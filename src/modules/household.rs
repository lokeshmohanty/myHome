use crate::db::Db;
use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Member {
    pub id: String,
    pub name: String,
    pub relationship: String,
    pub date_of_birth: Option<String>,
    pub profile_photo_path: Option<String>,
    pub is_primary: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub id: String,
    pub member_id: Option<String>,
    pub name: String,
    pub document_type: String,
    pub document_number: Option<String>,
    pub issue_date: Option<String>,
    pub expiry_date: Option<String>,
    pub issuing_authority: Option<String>,
}

pub struct HouseholdService<'a> {
    db: &'a Db,
}

impl<'a> HouseholdService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn add_member(
        &self,
        name: &str,
        relationship: &str,
        date_of_birth: Option<&str>,
        is_primary: bool,
    ) -> Result<String, rusqlite::Error> {
        let conn = &self.db.conn;
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO members (id, name, relationship, date_of_birth, is_primary, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                id,
                name,
                relationship,
                date_of_birth,
                is_primary as i32,
                now,
                now
            ],
        )?;
        Ok(id)
    }

    pub fn get_members(&self) -> Result<Vec<Member>, rusqlite::Error> {
        let conn = &self.db.conn;
        let mut stmt = conn.prepare(
            "SELECT id, name, relationship, date_of_birth, profile_photo_path, is_primary 
             FROM members WHERE deleted_at IS NULL",
        )?;

        let iter = stmt.query_map([], |row| -> rusqlite::Result<Member> {
            let is_primary_int: i32 = row.get(5)?;
            Ok(Member {
                id: row.get(0)?,
                name: row.get(1)?,
                relationship: row.get(2)?,
                date_of_birth: row.get(3)?,
                profile_photo_path: row.get(4)?,
                is_primary: is_primary_int != 0,
            })
        })?;

        let mut members = Vec::new();
        for member in iter {
            members.push(member?);
        }
        Ok(members)
    }

    pub fn add_document(
        &self,
        member_id: Option<&str>,
        name: &str,
        document_type: &str,
        document_number: Option<&str>,
        expiry_date: Option<&str>,
    ) -> Result<String, rusqlite::Error> {
        let conn = &self.db.conn;
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO documents (id, member_id, name, document_type, document_number, expiry_date, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                id,
                member_id,
                name,
                document_type,
                document_number,
                expiry_date,
                now,
                now
            ],
        )?;
        Ok(id)
    }

    pub fn get_documents(&self) -> Result<Vec<Document>, rusqlite::Error> {
        let conn = &self.db.conn;
        let mut stmt = conn.prepare(
            "SELECT id, member_id, name, document_type, document_number, issue_date, expiry_date, issuing_authority 
             FROM documents WHERE deleted_at IS NULL",
        )?;

        let iter = stmt.query_map([], |row| -> rusqlite::Result<Document> {
            Ok(Document {
                id: row.get(0)?,
                member_id: row.get(1)?,
                name: row.get(2)?,
                document_type: row.get(3)?,
                document_number: row.get(4)?,
                issue_date: row.get(5)?,
                expiry_date: row.get(6)?,
                issuing_authority: row.get(7)?,
            })
        })?;

        let mut docs = Vec::new();
        for doc in iter {
            docs.push(doc?);
        }
        Ok(docs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_household_service() {
        let db = Db::new(":memory:").unwrap();
        db.init().unwrap();
        let service = HouseholdService::new(&db);

        // Add a member
        let member_id = service
            .add_member("Bob", "Spouse", Some("1980-05-10"), true)
            .unwrap();

        let members = service.get_members().unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].name, "Bob");

        // Add a document
        service
            .add_document(
                Some(&member_id),
                "Passport",
                "ID",
                Some("A123"),
                Some("2030-01-01"),
            )
            .unwrap();

        let docs = service.get_documents().unwrap();
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].name, "Passport");
    }
}
