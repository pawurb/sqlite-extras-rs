use sqlx::{sqlite::SqliteRow, Row};

use super::shared::Query;

#[derive(Debug, Clone)]
pub struct IndexSize {
    pub name: String,
    pub table_name: String,
    pub column_name: String,
    pub payload_size: u64,
    pub unused_size: u64,
    pub page_size: u64,
    pub cells: u64,
    pub pages: u64,
    pub max_payload_siz: u64,
}

impl Query for IndexSize {
    fn new(row: &SqliteRow) -> Self {
        Self {
            name: row.get(0),
            table_name: row.get(1),
            column_name: row.get(2),
            payload_size: row.get(3),
            unused_size: row.get(4),
            page_size: row.get(5),
            cells: row.get(6),
            pages: row.get(7),
            max_payload_siz: row.get(8),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.name,
            self.table_name,
            self.column_name,
            self.payload_size,
            self.unused_size,
            self.page_size,
            self.cells,
            self.pages,
            self.max_payload_siz
        ]
    }

    fn headers() -> prettytable::Row {
        row![
            "name",
            "table_name",
            "column_name",
            "payload_size",
            "unused_size",
            "page_size",
            "cells",
            "pages",
            "max_payload_siz"
        ]
    }

    fn read_file() -> String {
        include_str!("../sql/index_size.sql").to_string()
    }
}
