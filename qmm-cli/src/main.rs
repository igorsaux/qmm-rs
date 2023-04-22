mod cli_player;

use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use qmm_player::QuestPlayer;
use qmm_syntax::qmm::{parse_qmm, Quest};

use crate::cli_player::CliQuestPlayer;

#[derive(Debug, Clone, Parser)]
struct Cli {
    /// Path to a quest file (.qmm)
    pub quest: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
enum Command {
    Dump {
        /// Dump path
        path: PathBuf,
    },
    Play,
}

fn dump(quest: Quest, path: &Path) {
    let mut f = File::create(path).unwrap();
    f.write_all(format!("{quest:#?}").as_bytes()).unwrap();
}

fn play(quest: Quest) {
    let quest_player = QuestPlayer::new(&quest, 1).unwrap();
    let cli_player = CliQuestPlayer::new(quest_player);
    cli_player.run();
}

fn main() {
    let args = Cli::parse();

    let mut quest_file = File::open(args.quest).unwrap();
    let mut quest_data = Vec::new();
    quest_file.read_to_end(&mut quest_data).unwrap();

    let quest = match parse_qmm(&quest_data) {
        Ok(quest) => quest,
        Err(err) => {
            println!("Got error: {err}\n{err:#?}");
            return;
        }
    };

    match args.command {
        Command::Dump { path } => dump(quest, &path),
        Command::Play => play(quest),
    }
}
