use crate::db::Db;
use rusqlite::params;
use serde::{Deserialize, Serialize};

/// Represents the global application settings.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub currency_code: String,
    pub currency_symbol: String,
    pub user_name: String,
    pub theme: String,
}

/// Represents a currency option for the UI.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrencyInfo {
    pub code: String,
    pub symbol: String,
}

pub struct SettingsService<'a> {
    db: &'a Db,
}

impl<'a> SettingsService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    /// Fetches the current application settings.
    /// Initializes defaults if no settings are found.
    pub fn get_settings(&self) -> Result<AppSettings, rusqlite::Error> {
        let conn = &self.db.conn;
        let mut stmt = conn.prepare("SELECT currency_code, currency_symbol, user_name, theme FROM app_preferences WHERE id = 1")?;

        let result = stmt.query_row([], |row| {
            Ok(AppSettings {
                currency_code: row.get(0)?,
                currency_symbol: row.get(1)?,
                user_name: row.get(2)?,
                theme: row.get(3)?,
            })
        });

        match result {
            Ok(settings) => Ok(settings),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                self.initialize_defaults()?;
                self.get_settings()
            }
            Err(e) => Err(e),
        }
    }

    fn initialize_defaults(&self) -> Result<(), rusqlite::Error> {
        let conn = &self.db.conn;
        conn.execute(
            "INSERT OR IGNORE INTO app_preferences (id, currency_code, currency_symbol, user_name, theme) 
             VALUES (1, 'INR', '₹', 'Lokesh', 'Dark')",
            [],
        )?;
        Ok(())
    }

    /// Updates the application's currency settings.
    pub fn update_currency(&self, code: &str, symbol: &str) -> Result<(), rusqlite::Error> {
        let conn = &self.db.conn;
        conn.execute(
            "UPDATE app_preferences SET currency_code = ?1, currency_symbol = ?2 WHERE id = 1",
            params![code, symbol],
        )?;
        Ok(())
    }

    /// Updates the user profile settings.
    pub fn update_profile(&self, user_name: &str, theme: &str) -> Result<(), rusqlite::Error> {
        let conn = &self.db.conn;
        conn.execute(
            "UPDATE app_preferences SET user_name = ?1, theme = ?2 WHERE id = 1",
            params![user_name, theme],
        )?;
        Ok(())
    }

    /// Returns a list of supported currencies.
    pub fn get_currency_list(&self) -> Vec<CurrencyInfo> {
        vec![
            CurrencyInfo {
                code: "INR".to_string(),
                symbol: "₹".to_string(),
            },
            CurrencyInfo {
                code: "USD".to_string(),
                symbol: "$".to_string(),
            },
            CurrencyInfo {
                code: "EUR".to_string(),
                symbol: "€".to_string(),
            },
            CurrencyInfo {
                code: "GBP".to_string(),
                symbol: "£".to_string(),
            },
            CurrencyInfo {
                code: "JPY".to_string(),
                symbol: "¥".to_string(),
            },
            CurrencyInfo {
                code: "AUD".to_string(),
                symbol: "A$".to_string(),
            },
            CurrencyInfo {
                code: "CAD".to_string(),
                symbol: "C$".to_string(),
            },
            CurrencyInfo {
                code: "CHF".to_string(),
                symbol: "Fr".to_string(),
            },
            CurrencyInfo {
                code: "CNY".to_string(),
                symbol: "¥".to_string(),
            },
            CurrencyInfo {
                code: "SAR".to_string(),
                symbol: "﷼".to_string(),
            },
            CurrencyInfo {
                code: "SGD".to_string(),
                symbol: "S$".to_string(),
            },
            CurrencyInfo {
                code: "NZD".to_string(),
                symbol: "NZ$".to_string(),
            },
            CurrencyInfo {
                code: "AED".to_string(),
                symbol: "د.إ".to_string(),
            },
            CurrencyInfo {
                code: "RUB".to_string(),
                symbol: "₽".to_string(),
            },
            CurrencyInfo {
                code: "BRL".to_string(),
                symbol: "R$".to_string(),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_service_full() {
        let db = Db::new(":memory:").unwrap();
        db.init().unwrap();
        let service = SettingsService::new(&db);

        // Test Default Initialization
        let settings = service.get_settings().expect("Should initialize defaults");
        assert_eq!(settings.currency_code, "INR");
        assert_eq!(settings.user_name, "Lokesh");
        assert_eq!(settings.theme, "Dark");

        // Test Update Currency
        service
            .update_currency("USD", "$")
            .expect("Should update currency");
        let updated_currency = service.get_settings().unwrap();
        assert_eq!(updated_currency.currency_code, "USD");
        assert_eq!(updated_currency.currency_symbol, "$");

        // Test Update Profile
        service
            .update_profile("Antigravity", "Light")
            .expect("Should update profile");
        let updated_profile = service.get_settings().unwrap();
        assert_eq!(updated_profile.user_name, "Antigravity");
        assert_eq!(updated_profile.theme, "Light");

        // Test Currency List
        let list = service.get_currency_list();
        assert!(!list.is_empty());
        assert!(list.iter().any(|c| c.code == "GBP"));
    }
}
