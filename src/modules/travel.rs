use crate::db::Db;
use chrono::Utc;
use rusqlite::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trip {
    pub id: String,
    pub name: String,
    pub destination: String,
    pub start_date: String,
    pub end_date: String,
    pub trip_type: Option<String>,
    pub status: String,
}

pub struct TravelService<'a> {
    db: &'a Db,
}

impl<'a> TravelService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn get_trips(&self) -> Result<Vec<Trip>> {
        let mut stmt = self.db.conn.prepare(
            "SELECT id, name, destination, start_date, end_date, trip_type, status FROM trips WHERE deleted_at IS NULL ORDER BY start_date ASC"
        )?;

        let trips = stmt
            .query_map([], |row| {
                Ok(Trip {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    destination: row.get(2)?,
                    start_date: row.get(3)?,
                    end_date: row.get(4)?,
                    trip_type: row.get(5)?,
                    status: row.get(6)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(trips)
    }

    pub fn add_trip(
        &self,
        name: &str,
        destination: &str,
        start_date: &str,
        end_date: &str,
        trip_type: Option<&str>,
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        self.db.conn.execute(
            "INSERT INTO trips (id, name, destination, start_date, end_date, trip_type, status, currency_code, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'planning', 'INR', ?7, ?8)",
            (id, name, destination, start_date, end_date, trip_type, &now, &now),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_travel_service() {
        let db = Db::new(":memory:").unwrap();
        db.init().unwrap();

        let service = TravelService::new(&db);

        service
            .add_trip(
                "Summer Vacation",
                "Paris",
                "2024-06-01",
                "2024-06-15",
                Some("vacation"),
            )
            .unwrap();
        service
            .add_trip(
                "Work Conference",
                "New York",
                "2024-04-10",
                "2024-04-14",
                Some("business"),
            )
            .unwrap();

        let trips = service.get_trips().unwrap();
        assert_eq!(trips.len(), 2);

        // Ordered by start date
        assert_eq!(trips[0].destination, "New York");
        assert_eq!(trips[1].destination, "Paris");
    }
}
