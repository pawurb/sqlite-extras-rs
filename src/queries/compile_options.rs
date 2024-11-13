use sqlx::{sqlite::SqliteRow, Row};

use super::shared::Query;

#[derive(Debug, Clone)]
pub struct CompileOptions {
    pub value: String,
}

impl Query for CompileOptions {
    fn new(row: &SqliteRow) -> Self {
        Self { value: row.get(0) }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.value]
    }

    fn headers() -> prettytable::Row {
        row!["value"]
    }

    fn read_file() -> String {
        include_str!("../sql/compile_options.sql").to_string()
    }
}
