mod cli;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};

fn main() {
    if let Err(err) = run_app() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}

#[allow(clippy::unnecessary_wraps)]
fn run_app() -> Result<()> {
    let cli = Cli::parse();

    println!("Parsed arguments: {cli:#?}");

    match cli.command {
        Command::Init => println!("Running 'init'..."),
        Command::Feature(cmd) => match cmd.command {
            cli::FeatureSubcommand::Start { name } => {
                println!("Running 'feature start' for feature: {name:?}");
            }
            cli::FeatureSubcommand::Finish { name } => {
                println!("Running 'feature finish' for feature: {name:?}");
            }
        },
        Command::Release(cmd) => match cmd.command {
            cli::ReleaseSubcommand::Start { version } => {
                println!("Running 'release start' for version: {version:?}");
            }
            cli::ReleaseSubcommand::Finish { version } => {
                println!("Running 'release finish' for version: {version:?}");
            }
        },
        Command::Hotfix(cmd) => match cmd.command {
            cli::HotfixSubcommand::Start { version } => {
                println!("Running 'hotfix start' for version: {version:?}");
            }
            cli::HotfixSubcommand::Finish { version } => {
                println!("Running 'hotfix finish' for version: {version:?}");
            }
        },
        Command::Status => println!("Running 'status'..."),
        Command::Plugins(cmd) => match cmd {
            cli::PluginCommand::List => println!("Running 'plugins list'..."),
        },
        Command::Config(cmd) => match cmd {
            cli::ConfigCommand::Debug => println!("Running 'config debug'..."),
        },
    }

    Ok(())
}
