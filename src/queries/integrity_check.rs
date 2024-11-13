use sqlx::{sqlite::SqliteRow, Row};

use super::shared::Query;

#[derive(Debug, Clone)]
pub struct IntegrityCheck {
    pub message: String,
}

impl Query for IntegrityCheck {
    fn new(row: &SqliteRow) -> Self {
        Self {
            message: row.get(0),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.message]
    }

    fn headers() -> prettytable::Row {
        row!["message"]
    }

    fn read_file() -> String {
        include_str!("../sql/integrity_check.sql").to_string()
    }
}
