use crate::commands::AppCommand;
use crate::git::repository::GitRepository;
use crate::git::types::{GitError, GitOutput, GitResult};
use crate::gitflow::orchestrator::GitFlowOrchestrator;
use std::path::PathBuf;
use tokio::sync::mpsc;

pub struct ExecContext {
    pub non_interactive: bool,
}

/// This is the main application controller. It is UI-agnostic.
/// It takes a semantic command and executes the core logic.
pub async fn execute(
    command: AppCommand,
    context: ExecContext,
) -> Result<mpsc::Receiver<GitOutput>, Box<dyn std::error::Error>> {
    if context.non_interactive {
        println!("Non interactive command");
    }

    let repo_path = PathBuf::from(".");
    let repo = GitRepository::new(repo_path);

    let orchestrator = GitFlowOrchestrator::new(repo);

    let receiver_result = match command {
        AppCommand::FeatureFinish { name } => orchestrator.finish_feature(&name),
        _ => Err({
            GitError::CommandFailed {
                command: "Failed".to_string(),
                exit_code: 1,
                stderr: "Failed".to_string(),
            }
        }),
    };

    Ok(receiver_result?)
}
