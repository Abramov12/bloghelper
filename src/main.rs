mod cli;

use clap::Parser;
use cli::Cli;
use anyhow::Result;
fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        )
        .with_ansi(true)           // цветной вывод в терминале
        .init();
    let cli = Cli::parse();
    cli::run(cli)?;
    Ok(())
    
}
