use crate::db::Db;
use chrono::Utc;
use rusqlite::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub account_type: String, // checking, savings, credit, etc.
    pub currency_code: String,
    pub current_balance_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub category_type: String, // income or expense
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub account_id: String,
    pub amount_cents: i64,
    pub currency_code: String,
    pub date: String,
    pub merchant: String,
    pub category_name: Option<String>,
}

pub struct FinanceService<'a> {
    db: &'a Db,
}

impl<'a> FinanceService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn get_accounts(&self) -> Result<Vec<Account>> {
        let mut stmt = self.db.conn.prepare(
            "SELECT id, name, account_type, currency_code, current_balance_cents FROM accounts WHERE deleted_at IS NULL"
        )?;

        let accounts = stmt
            .query_map([], |row| {
                Ok(Account {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    account_type: row.get(2)?,
                    currency_code: row.get(3)?,
                    current_balance_cents: row.get(4)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(accounts)
    }

    pub fn create_account(
        &self,
        name: &str,
        account_type: &str,
        starting_balance_cents: i64,
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        self.db.conn.execute(
            "INSERT INTO accounts (id, name, account_type, currency_code, current_balance_cents, created_at, updated_at)
             VALUES (?1, ?2, ?3, 'INR', ?4, ?5, ?6)",
            (id, name, account_type, starting_balance_cents, &now, &now),
        )?;
        Ok(())
    }

    pub fn get_total_balance(&self) -> f64 {
        let result: Result<i64> = self.db.conn.query_row(
            "SELECT COALESCE(SUM(current_balance_cents), 0) FROM accounts WHERE deleted_at IS NULL",
            [],
            |row| row.get(0),
        );
        match result {
            Ok(cents) => cents as f64 / 100.0,
            Err(_) => 0.0,
        }
    }

    pub fn get_categories(&self) -> Result<Vec<Category>> {
        let mut stmt = self
            .db
            .conn
            .prepare("SELECT id, name, type, color FROM categories WHERE deleted_at IS NULL")?;

        let categories = stmt
            .query_map([], |row| {
                Ok(Category {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    category_type: row.get(2)?,
                    color: row.get(3)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(categories)
    }

    pub fn create_category(&self, name: &str, category_type: &str, color: &str) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        self.db.conn.execute(
            "INSERT INTO categories (id, name, type, color, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (id, name, category_type, color, &now, &now),
        )?;
        Ok(())
    }

    pub fn get_transactions(&self, limit: usize) -> Result<Vec<Transaction>> {
        let mut stmt = self.db.conn.prepare(
            "SELECT t.id, t.account_id, t.amount_cents, t.currency_code, t.date, t.merchant, c.name as category_name 
             FROM transactions t
             LEFT JOIN categories c ON t.category_id = c.id
             WHERE t.deleted_at IS NULL
             ORDER BY t.date DESC
             LIMIT ?1"
        )?;

        let transactions = stmt
            .query_map([limit], |row| {
                Ok(Transaction {
                    id: row.get(0)?,
                    account_id: row.get(1)?,
                    amount_cents: row.get(2)?,
                    currency_code: row.get(3)?,
                    date: row.get(4)?,
                    merchant: row.get(5)?,
                    category_name: row.get(6).unwrap_or(None),
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(transactions)
    }

    pub fn create_transaction(
        &self,
        account_id: &str,
        amount_cents: i64,
        merchant: &str,
        date: &str,
        category_id: Option<&str>,
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let tx = self.db.conn.unchecked_transaction()?;

        // 1. Insert the transaction
        tx.execute(
            "INSERT INTO transactions (id, account_id, amount_cents, currency_code, date, merchant, category_id, is_pending, created_at, updated_at)
             VALUES (?1, ?2, ?3, 'INR', ?4, ?5, ?6, 0, ?7, ?8)",
             (id, account_id, amount_cents, date, merchant, category_id, &now, &now),
        )?;

        // 2. Adjust the account balance
        tx.execute(
            "UPDATE accounts SET current_balance_cents = current_balance_cents + ?1, updated_at = ?2 WHERE id = ?3",
            (amount_cents, &now, account_id)
        )?;

        tx.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finance_service() {
        let db = Db::new(":memory:").unwrap();
        db.init().unwrap();

        let service = FinanceService::new(&db);

        service
            .create_account("My Checking", "checking", 50000)
            .unwrap();
        service
            .create_account("My Savings", "savings", 100000)
            .unwrap();

        let accounts = service.get_accounts().unwrap();
        assert_eq!(accounts.len(), 2);

        let checking_id = &accounts[0].id;

        service
            .create_category("Groceries", "expense", "#FF0000")
            .unwrap();
        let categories = service.get_categories().unwrap();
        assert_eq!(categories.len(), 1);
        let grocery_id = &categories[0].id;

        // Add a transaction of -$100.00
        service
            .create_transaction(
                checking_id,
                -10000,
                "Whole Foods",
                "2023-11-01T00:00:00Z",
                Some(grocery_id),
            )
            .unwrap();

        let txs = service.get_transactions(10).unwrap();
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].merchant, "Whole Foods");
        assert_eq!(txs[0].category_name.as_deref(), Some("Groceries"));

        let total = service.get_total_balance();
        // 500 + 1000 - 100 = 1400
        assert_eq!(total, 1400.0);
    }
}
