mod queries;

#[macro_use]
extern crate prettytable;
use std::{env, fmt};

use prettytable::{Cell, Row as TableRow, Table};
pub use queries::{index_size::IndexSize, shared::Query, table_size::TableSize};
use sqlx::SqlitePool;

pub fn render_table<T: Query>(items: Vec<T>) {
    let mut table = Table::new();
    table.add_row(T::headers());

    let columns_count = T::headers().len();

    for item in items {
        table.add_row(item.to_row());
    }
    table.set_titles(TableRow::new(vec![
        Cell::new(T::description().as_str()).style_spec(format!("H{}", columns_count).as_str())
    ]));
    table.printstd();
}

pub async fn table_size() -> Result<Vec<TableSize>, SQExtrasError> {
    get_rows().await
}

pub async fn index_size() -> Result<Vec<IndexSize>, SQExtrasError> {
    get_rows().await
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SQExtrasError {
    MissingConfigVars(),
    DbConnectionError(String),
    Unknown(String),
}

impl fmt::Display for SQExtrasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Self::MissingConfigVars() => {
                "Both $DATABASE_URL and $SQLITE_EXTRAS_DATABASE_URL are not set."
            }
            Self::DbConnectionError(e) => &format!("Cannot connect to database: '{}'", e),
            Self::Unknown(e) => &format!("Unknown sqlite-extras error: '{}'", e),
        };

        write!(f, "{}", msg)
    }
}

fn db_url() -> Result<String, SQExtrasError> {
    env::var("SQLITE_EXTRAS_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .map_err(|_| SQExtrasError::MissingConfigVars())
}

async fn get_rows<T: Query>() -> Result<Vec<T>, SQExtrasError> {
    let conn = match SqlitePool::connect(db_url()?.as_str()).await {
        Ok(conn) => conn,
        Err(e) => return Err(SQExtrasError::DbConnectionError(format!("{}", e))),
    };

    let query = T::read_file();

    Ok(match sqlx::query(&query).fetch_all(&conn).await {
        Ok(rows) => rows.iter().map(T::new).collect(),
        Err(e) => return Err(SQExtrasError::Unknown(format!("{}", e))),
    })
}
