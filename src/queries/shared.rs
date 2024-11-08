use sqlx::sqlite::SqliteRow;

pub trait Query {
    fn new(row: &SqliteRow) -> Self;
    fn to_row(&self) -> prettytable::Row;
    fn headers() -> prettytable::Row;
    fn read_file() -> String;
    fn description() -> String {
        Self::read_file().lines().take(1).collect()
    }
}
