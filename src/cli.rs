use clap::{Parser, Subcommand};

use tracing::info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Все доступные команды
#[derive(Subcommand)]
pub enum Commands {
    /// Создать новый черновик поста для Instagram или TikTok
    New {
        /// Тема поста (например: "как снимать рилс без штатива")
        topic: String,
    },

    
    /// Показать версию программы
    Version,
}
pub fn run(cli: Cli) -> anyhow::Result<()>{
    match cli.command
    {
        Some(Commands::New { topic }) => {
            info!("Создаём новый пост по теме: {}", topic);
            println!("✅ Получена команда 'new'");
            println!("Тема: {}", topic);
            // Здесь позже будет генерация шаблона
        }

        Some(Commands::Version) => {
            println!("bloghelper версия {}", env!("CARGO_PKG_VERSION"));
        }

        None => {
            println!("Используйте: --help");
        }
    }

    Ok(())
}