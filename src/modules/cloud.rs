use crate::db::Db;
use std::path::Path;
use anyhow::{Result, anyhow};
use google_drive3::{DriveHub, api::File};
use google_drive3::hyper_rustls::HttpsConnectorBuilder;
use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use std::fs;
use std::io::Cursor;

// Specific types for google-drive3 6.0
type Connector = google_drive3::hyper_rustls::HttpsConnector<google_drive3::hyper_util::client::legacy::connect::HttpConnector>;
type Authenticator = yup_oauth2::authenticator::Authenticator<Connector>;
type Hub = DriveHub<Connector>;

pub struct CloudService<'a> {
    db: &'a Db,
}

impl<'a> CloudService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub async fn link_account(&self) -> Result<()> {
        let auth = self.get_authenticator().await?;
        let scopes = &["https://www.googleapis.com/auth/drive.file"];
        auth.token(scopes).await?;
        Ok(())
    }

    async fn get_authenticator(&self) -> Result<Authenticator> {
        let secret_path = Path::new("client_secret.json");
        if !secret_path.exists() {
            return Err(anyhow!("client_secret.json not found in root directory."));
        }

        let secret = yup_oauth2::read_application_secret(secret_path).await?;
        let token_dir = Path::new(".tokens");
        if !token_dir.exists() {
            fs::create_dir_all(token_dir)?;
        }

        let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
            .persist_tokens_to_disk(token_dir.join("google_drive_tokens.json"))
            .build()
            .await?;

        Ok(auth)
    }

    async fn get_drive_hub(&self) -> Result<Hub> {
        let auth = self.get_authenticator().await?;
        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()?
            .https_only()
            .enable_http1()
            .build();
        
        let client = google_drive3::hyper_util::client::legacy::Client::builder(google_drive3::hyper_util::rt::TokioExecutor::new())
            .build(connector);

        Ok(DriveHub::new(client, auth))
    }

    pub async fn sync_database(&self, db_path: &Path) -> Result<()> {
        let hub = self.get_drive_hub().await?;
        let folder_id = self.ensure_folder_exists(&hub, "Applications/myHome").await?;
        
        let backup_path = db_path.with_extension("db.backup");
        self.db.backup(&backup_path)?;

        let remote_file = self.find_file(&hub, &folder_id, "myhome.db").await?;
        
        match remote_file {
            Some(file) => {
                self.upload_file(&hub, &folder_id, &backup_path, "myhome.db", file.id.as_deref()).await?;
            }
            None => {
                self.upload_file(&hub, &folder_id, &backup_path, "myhome.db", None).await?;
            }
        }
        
        Ok(())
    }

    async fn ensure_folder_exists(&self, hub: &Hub, path: &str) -> Result<String> {
        let parts: Vec<&str> = path.split('/').collect();
        let mut parent_id = "root".to_string();

        for part in parts {
            let existing = self.find_file(hub, &parent_id, part).await?;
            parent_id = match existing {
                Some(file) => file.id.ok_or_else(|| anyhow!("Found folder without ID"))?,
                None => {
                    let mut folder = File::default();
                    folder.name = Some(part.to_string());
                    folder.mime_type = Some("application/vnd.google-apps.folder".to_string());
                    folder.parents = Some(vec![parent_id]);

                    // Workaround: Use upload() with empty cursor for metadata-only create
                    let (_, res) = hub.files().create(folder)
                        .upload(Cursor::new(vec![]), "application/octet-stream".parse().unwrap())
                        .await?;
                    res.id.ok_or_else(|| anyhow!("Failed to create folder"))?
                }
            };
        }

        Ok(parent_id)
    }

    async fn find_file(&self, hub: &Hub, parent_id: &str, name: &str) -> Result<Option<File>> {
        let query = format!("name = '{}' and '{}' in parents and trashed = false", name, parent_id);
        let (_, list) = hub.files().list()
            .q(&query)
            .spaces("drive")
            .doit().await?;

        Ok(list.files.and_then(|files| files.into_iter().next()))
    }

    async fn upload_file(&self, hub: &Hub, parent_id: &str, local_path: &Path, remote_name: &str, remote_id: Option<&str>) -> Result<()> {
        let mut file = File::default();
        file.name = Some(remote_name.to_string());
        
        let mut f = fs::File::open(local_path)?;

        if let Some(id) = remote_id {
            hub.files().update(file, id)
                .upload(&mut f, "application/octet-stream".parse().unwrap())
                .await?;
        } else {
            file.parents = Some(vec![parent_id.to_string()]);
            hub.files().create(file)
                .upload(&mut f, "application/octet-stream".parse().unwrap())
                .await?;
        }

        Ok(())
    }

    pub async fn invite_member(&self, email: &str) -> Result<()> {
        let hub = self.get_drive_hub().await?;
        let folder_id = self.ensure_folder_exists(&hub, "Applications/myHome").await?;

        let mut permission = google_drive3::api::Permission::default();
        permission.role = Some("writer".to_string());
        permission.type_ = Some("user".to_string());
        permission.email_address = Some(email.to_string());

        hub.permissions().create(permission, &folder_id).doit().await?;
        Ok(())
    }

    pub fn get_sync_status(&self) -> String {
        let token_path = Path::new(".tokens/google_drive_tokens.json");
        if token_path.exists() {
            "Linked & Ready".to_string()
        } else {
            "Not Linked".to_string()
        }
    }

    pub fn get_last_sync_time(&self) -> String {
        // In a real app we'd store this in the settings or a separate file
        "Never".to_string()
    }
}

// Ensure Send/Sync for Slint
unsafe impl<'a> Send for CloudService<'a> {}
unsafe impl<'a> Sync for CloudService<'a> {}
