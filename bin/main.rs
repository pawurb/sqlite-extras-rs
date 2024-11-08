use clap::{Parser, Subcommand};
use sqlite_extras::{
    index_size, render_table, table_size, IndexSize, Query, SQExtrasError, TableSize,
};

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "sqlite-extras: Sqlite database performance insights. Locks, index usage, buffer cache hit ratios, vacuum stats and more.

https://github.com/pawurb/sqlite-extras-rs"
)]
pub struct SQExtrasArgs {
    #[command(subcommand)]
    pub cmd: SQSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum SQSubcommand {
    #[command(about = extract_desc(&TableSize::description()))]
    TableSize,
    #[command(about = extract_desc(&IndexSize::description()))]
    IndexSize,
}

#[tokio::main]
async fn main() {
    match execute().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

async fn execute() -> Result<(), SQExtrasError> {
    let args = SQExtrasArgs::parse();
    match args.cmd {
        SQSubcommand::TableSize => render_table(table_size().await?),
        SQSubcommand::IndexSize => render_table(index_size().await?),
    }

    Ok(())
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
