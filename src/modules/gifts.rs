use crate::db::Db;
use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub relationship: Option<String>,
    pub date_of_birth: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GiftIdea {
    pub id: String,
    pub person_id: String,
    pub description: String,
    pub estimated_price_cents: Option<i64>,
    pub status: String,
}

pub struct GiftsService<'a> {
    db: &'a Db,
}

impl<'a> GiftsService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn add_person(
        &self,
        name: &str,
        relationship: Option<&str>,
        date_of_birth: Option<&str>,
    ) -> Result<String, rusqlite::Error> {
        let conn = &self.db.conn;
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO people (id, name, relationship, date_of_birth, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, name, relationship, date_of_birth, now, now],
        )?;
        Ok(id)
    }

    pub fn get_people(&self) -> Result<Vec<Person>, rusqlite::Error> {
        let conn = &self.db.conn;
        let mut stmt = conn.prepare(
            "SELECT id, name, relationship, date_of_birth FROM people WHERE deleted_at IS NULL",
        )?;

        let iter = stmt.query_map([], |row| -> rusqlite::Result<Person> {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                relationship: row.get(2)?,
                date_of_birth: row.get(3)?,
            })
        })?;

        let mut people = Vec::new();
        for person in iter {
            people.push(person?);
        }
        Ok(people)
    }

    pub fn add_gift_idea(
        &self,
        person_id: &str,
        description: &str,
        estimated_price_cents: Option<i64>,
    ) -> Result<String, rusqlite::Error> {
        let conn = &self.db.conn;
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO gift_ideas (id, person_id, description, estimated_price_cents, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                id,
                person_id,
                description,
                estimated_price_cents,
                now,
                now
            ],
        )?;
        Ok(id)
    }

    pub fn get_gift_ideas(&self) -> Result<Vec<GiftIdea>, rusqlite::Error> {
        let conn = &self.db.conn;
        let mut stmt = conn.prepare(
            "SELECT id, person_id, description, estimated_price_cents, status FROM gift_ideas WHERE deleted_at IS NULL",
        )?;

        let iter = stmt.query_map([], |row| -> rusqlite::Result<GiftIdea> {
            Ok(GiftIdea {
                id: row.get(0)?,
                person_id: row.get(1)?,
                description: row.get(2)?,
                estimated_price_cents: row.get(3)?,
                status: row.get(4)?,
            })
        })?;

        let mut ideas = Vec::new();
        for idea in iter {
            ideas.push(idea?);
        }
        Ok(ideas)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gifts_service() {
        let db = Db::new(":memory:").unwrap();
        db.init().unwrap();
        let service = GiftsService::new(&db);

        // Add a person
        let person_id = service
            .add_person("Alice", Some("Friend"), Some("1990-01-01"))
            .unwrap();

        let people = service.get_people().unwrap();
        assert_eq!(people.len(), 1);
        assert_eq!(people[0].name, "Alice");

        // Add a gift idea
        service
            .add_gift_idea(&person_id, "Book", Some(2000))
            .unwrap();

        let ideas = service.get_gift_ideas().unwrap();
        assert_eq!(ideas.len(), 1);
        assert_eq!(ideas[0].description, "Book");
    }
}
