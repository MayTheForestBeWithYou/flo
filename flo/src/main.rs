mod cli;
mod git;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Command};
use std::path::PathBuf;

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
                return run_feature_start();
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

fn run_feature_start() -> Result<()> {
    let repo_path = PathBuf::from(".");

    git::checkout(&repo_path, "develop")
        .context("Failed to switch to the 'develop' branch. Is this a Flo repository?")?;
    git::fetch_all(&repo_path).context("Failed to fetch the latest changes from the remote.")?;
    println!("Successfully prepared 'develop' branch.");
    git::checkout_new_branch(&repo_path, "my-feature")?;
    Ok(())
}
