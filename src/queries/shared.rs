use sqlx::sqlite::SqliteRow;

pub trait Query {
    fn new(row: &SqliteRow) -> Self;
    fn to_row(&self) -> prettytable::Row;
    fn headers() -> prettytable::Row;
    fn read_file() -> String;
    fn description() -> String {
        let file_content = Self::read_file();
        let desc = file_content.lines().take(1).next().unwrap_or_default();
        extract_desc(desc)
    }
}

fn extract_desc(desc: &str) -> String {
    if let (Some(start), Some(end)) = (desc.find("/*"), desc.find("*/")) {
        let extracted = &desc[start + 2..end];
        let mut trimmed = extracted.trim().to_string();
        if trimmed.ends_with('.') {
            trimmed.pop();
        }
        trimmed
    } else {
        desc.to_string()
    }
}
