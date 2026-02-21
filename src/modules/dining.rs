use crate::db::Db;
use chrono::Utc;
use rusqlite::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Restaurant {
    pub id: String,
    pub name: String,
    pub cuisine_type: Option<String>,
    pub location: Option<String>,
    pub price_range: Option<String>,
    pub visited_flag: bool,
    pub rating: Option<i64>,
}

pub struct DiningService<'a> {
    db: &'a Db,
}

impl<'a> DiningService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn get_restaurants(&self) -> Result<Vec<Restaurant>> {
        let mut stmt = self.db.conn.prepare(
            "SELECT id, name, cuisine_type, location, price_range, visited_flag, rating FROM restaurants WHERE deleted_at IS NULL ORDER BY name ASC"
        )?;

        let restaurants = stmt
            .query_map([], |row| {
                let visited_int: i64 = row.get(5)?;
                Ok(Restaurant {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    cuisine_type: row.get(2)?,
                    location: row.get(3)?,
                    price_range: row.get(4)?,
                    visited_flag: visited_int != 0,
                    rating: row.get(6)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(restaurants)
    }

    pub fn add_restaurant(
        &self,
        name: &str,
        cuisine_type: Option<&str>,
        location: Option<&str>,
        visited: bool,
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let visited_int = if visited { 1 } else { 0 };

        self.db.conn.execute(
            "INSERT INTO restaurants (id, name, cuisine_type, location, visited_flag, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (id, name, cuisine_type, location, visited_int, &now, &now),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dining_service() {
        let db = Db::new(":memory:").unwrap();
        db.init().unwrap();

        let service = DiningService::new(&db);

        service
            .add_restaurant(
                "Pizza Neapolitana",
                Some("Italian"),
                Some("Downtown"),
                false,
            )
            .unwrap();
        service
            .add_restaurant("Sushi Spot", Some("Japanese"), Some("Uptown"), true)
            .unwrap();

        let restaurants = service.get_restaurants().unwrap();
        assert_eq!(restaurants.len(), 2);

        // Ordered by name
        assert_eq!(restaurants[0].name, "Pizza Neapolitana");
        assert_eq!(restaurants[0].visited_flag, false);
        assert_eq!(restaurants[1].name, "Sushi Spot");
        assert_eq!(restaurants[1].visited_flag, true);
    }
}
