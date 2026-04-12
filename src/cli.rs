use clap::{Parser, Subcommand};

//use tracing::info;

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
pub fn run(cli: Cli) -> Result<(),()>{
    match cli.command
    {
        Some(Commands::New { topic }) => {
            print!("Команда {topic} выполнена");
        },

        Some(Commands::Version) => print!("Версия 2"),
        
        None => ()
    }

    Result::Ok(())
}