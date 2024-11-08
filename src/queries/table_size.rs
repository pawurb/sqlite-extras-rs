use sqlx::{sqlite::SqliteRow, Row};

use super::shared::Query;

#[derive(Debug, Clone)]
pub struct TableSize {
    pub name: String,
    pub payload_size: String,
    pub unused_size: String,
    pub vacuum_size: u64,
    pub page_size: u64,
    pub pages: u64,
    pub max_payload_size: u64,
}

impl Query for TableSize {
    fn new(row: &SqliteRow) -> Self {
        Self {
            name: row.get(0),
            payload_size: row.get(1),
            unused_size: row.get(2),
            vacuum_size: row.get(3),
            page_size: row.get(4),
            pages: row.get(5),
            max_payload_size: row.get(6),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.payload_size,
            self.unused_size,
            self.vacuum_size,
            self.page_size,
            self.pages,
            self.max_payload_size
        ]
    }

    fn headers() -> prettytable::Row {
        row![
            "payload_size",
            "unused_size",
            "vacuum_size",
            "page_size",
            "pages",
            "max_payload_size"
        ]
    }

    fn read_file() -> String {
        include_str!("../sql/index_size.sql").to_string()
    }
}
