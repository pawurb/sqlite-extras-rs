use sqlx::{sqlite::SqliteRow, Row};

use super::shared::Query;

#[derive(Debug, Clone)]
pub struct SequenceNumber {
    pub table_name: String,
    pub seq: u64,
}

impl Query for SequenceNumber {
    fn new(row: &SqliteRow) -> Self {
        Self {
            table_name: row.get(0),
            seq: row.get(1),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.table_name, self.seq]
    }

    fn headers() -> prettytable::Row {
        row!["table_name", "seq"]
    }

    fn read_file() -> String {
        include_str!("../sql/sequence_number.sql").to_string()
    }
}
