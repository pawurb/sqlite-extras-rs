use sqlx::{sqlite::SqliteRow, Row};

use super::shared::Query;

#[derive(Debug, Clone)]
pub struct TotalSize {
    pub name: String,
    pub value: u64,
}

impl Query for TotalSize {
    fn new(row: &SqliteRow) -> Self {
        Self {
            name: row.get(0),
            value: row.get(1),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.value]
    }

    fn headers() -> prettytable::Row {
        row!["name", "value"]
    }

    fn read_file() -> String {
        include_str!("../sql/total_size.sql").to_string()
    }
}
