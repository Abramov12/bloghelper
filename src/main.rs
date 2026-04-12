mod cli;

use clap::Parser;
use cli::Cli;
use anyhow::Result;
fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("Приложение запущено");

    
    cli::run(cli).ok();
        
    
    Ok(())
    
}
