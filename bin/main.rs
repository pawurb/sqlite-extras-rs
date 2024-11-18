use clap::{Parser, Subcommand};
use sqlite_extras::{
    compile_options, index_size, integrity_check, pragma, render_table, sequence_number,
    table_size, total_size, CompileOptions, IndexSize, IntegrityCheck, Pragma, Query,
    SQExtrasError, SequenceNumber, TableSize, TotalSize,
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
    #[command(about = &TableSize::description())]
    TableSize,
    #[command(about = &IndexSize::description())]
    IndexSize,
    #[command(about = &IntegrityCheck::description())]
    IntegrityCheck,
    #[command(about = &Pragma::description())]
    Pragma,
    #[command(about = &TotalSize::description())]
    TotalSize,
    #[command(about = &CompileOptions::description())]
    CompileOptions,
    #[command(about = &SequenceNumber::description())]
    SequenceNumber,
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
        SQSubcommand::IntegrityCheck => render_table(integrity_check().await?),
        SQSubcommand::Pragma => render_table(pragma().await?),
        SQSubcommand::TotalSize => render_table(total_size().await?),
        SQSubcommand::CompileOptions => render_table(compile_options().await?),
        SQSubcommand::SequenceNumber => render_table(sequence_number().await?),
    }

    Ok(())
}
