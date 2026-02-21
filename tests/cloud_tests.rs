use myhome::db::Db;
use myhome::modules::cloud::CloudService;
use std::path::Path;

#[tokio::test]
async fn test_cloud_service_instantiation() {
    let db = Db::new(":memory:").unwrap();
    db.init().unwrap();
    let service = CloudService::new(&db);
    
    // Status should be "Not Linked" initially because tokens are missing
    assert_eq!(service.get_sync_status(), "Not Linked");
}

#[tokio::test]
async fn test_cloud_service_backup_flow() {
    let db_path = "test_cloud.db";
    let db = Db::new(db_path).unwrap();
    db.init().unwrap();
    let service = CloudService::new(&db);
    
    let db_path_buf = Path::new(db_path);
    // We can't actually sync without credentials, but we can verify the backup part of the flow
    // by mocking or just checking it doesn't crash before the network call
    let backup_path = db_path_buf.with_extension("db.backup");
    if backup_path.exists() {
        std::fs::remove_file(&backup_path).unwrap();
    }
    
    db.backup(&backup_path).unwrap();
    assert!(backup_path.exists());
    
    std::fs::remove_file(db_path).unwrap();
    std::fs::remove_file(backup_path).unwrap();
}
