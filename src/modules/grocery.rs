use crate::db::Db;
use chrono::Utc;
use rusqlite::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroceryItem {
    pub id: String,
    pub name: String,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub category: Option<String>,
    pub is_purchased: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: String,
    pub name: String,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub location_id: Option<String>,
    pub expiration_date: Option<String>,
}

pub struct GroceryService<'a> {
    db: &'a Db,
}

impl<'a> GroceryService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn get_grocery_list(&self) -> Result<Vec<GroceryItem>> {
        let mut stmt = self.db.conn.prepare(
            "SELECT id, name, quantity, unit, category, is_checked FROM shopping_list_items WHERE is_checked = 0 ORDER BY created_at DESC"
        )?;

        // In SQLite, boolean is 0 or 1
        let items = stmt
            .query_map([], |row| {
                let is_checked_int: i32 = row.get(5)?;
                Ok(GroceryItem {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    quantity: row.get(2)?,
                    unit: row.get(3)?,
                    category: row.get(4)?,
                    is_purchased: is_checked_int == 1,
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(items)
    }

    pub fn add_grocery_item(&self, name: &str, category: Option<&str>) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let list_id = "default_list";

        // Insert a default list if it doesn't exist
        let _ = self.db.conn.execute(
            "INSERT OR IGNORE INTO shopping_lists (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            (list_id, "Main Grocery List", &now, &now)
        );

        self.db.conn.execute(
            "INSERT INTO shopping_list_items (id, list_id, name, quantity, unit, is_checked, category, created_at, updated_at)
             VALUES (?1, ?2, ?3, 1.0, 'unit', 0, ?4, ?5, ?6)",
            (id, list_id, name, category, &now, &now),
        )?;
        Ok(())
    }

    pub fn get_inventory(&self) -> Result<Vec<InventoryItem>> {
        let mut stmt = self.db.conn.prepare(
            "SELECT id, name, quantity, unit, location, expiry_date FROM inventory_items WHERE deleted_at IS NULL ORDER BY created_at DESC"
        )?;

        let items = stmt
            .query_map([], |row| {
                Ok(InventoryItem {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    quantity: row.get(2)?,
                    unit: row.get(3)?,
                    location_id: row.get(4)?,
                    expiration_date: row.get(5)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(items)
    }

    pub fn add_inventory_item(&self, name: &str, quantity: f64) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        self.db.conn.execute(
            "INSERT INTO inventory_items (id, name, quantity, unit, created_at, updated_at)
             VALUES (?1, ?2, ?3, 'unit', ?4, ?5)",
            (id, name, quantity, &now, &now),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grocery_service() {
        let db = Db::new(":memory:").unwrap();
        db.init().unwrap();

        let service = GroceryService::new(&db);

        service.add_grocery_item("Milk", Some("Dairy")).unwrap();
        service.add_grocery_item("Eggs", Some("Dairy")).unwrap();

        let list = service.get_grocery_list().unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].name, "Eggs"); // Assuming ORDER BY created_at DESC -> Eggs was added last, wait, SQLite memory might yield same timestamp.

        service.add_inventory_item("Pasta", 3.0).unwrap();
        let inventory = service.get_inventory().unwrap();
        assert_eq!(inventory.len(), 1);
        assert_eq!(inventory[0].name, "Pasta");
        assert_eq!(inventory[0].quantity, Some(3.0));
    }
}
