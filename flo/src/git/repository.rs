//! High-level Git repository operations

use crate::git::command::GitCommand;
use crate::git::types::GitError::InvalidBranchName;
use crate::git::types::{GitConfig, GitError, GitOutput, GitResult, ValidationLevel};
use std::path::PathBuf;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct GitRepository {
    config: GitConfig,
}

impl GitRepository {
    pub fn new<P: Into<PathBuf>>(repository_path: P) -> Self {
        Self {
            config: GitConfig {
                repository_path: repository_path.into(),
                ..Default::default()
            },
        }
    }

    pub fn with_config(mut self, config: GitConfig) -> Self {
        self.config = config;
        self
    }

    pub fn config(&self) -> &GitConfig {
        &self.config
    }

    pub async fn validate(&self) -> GitResult<()> {
        if !self.config.repository_path.exists() {
            return Err(GitError::RepositoryNotFound {
                path: self.config.repository_path.to_string_lossy().to_string(),
            });
        }

        let mut rx = GitCommand::new(&self.config.repository_path)
            .with_config(self.config.clone())
            .args(["rev-parse", "--git-dir"])
            .execute_streaming()
            .await?;

        let mut sucess = false;
        while let Some(output) = rx.recv().await {
            if let GitOutput::Completed { success: s, .. } = output {
                sucess = s;
                break;
            }
        }

        if !sucess {
            return Err(GitError::InvalidRepository {
                path: self.config.repository_path.to_string_lossy().to_string(),
            });
        }

        Ok(())
    }

    pub async fn ensure_clean_working_directory(&self) -> GitResult<()> {
        if matches!(self.config.validation_level, ValidationLevel::None) {
            return Ok(());
        }

        let mut rx = GitCommand::new(&self.config.repository_path)
            .with_config(self.config.clone())
            .args(["status", "--porcelain"])
            .workflow_step("Checking working directory status")
            .execute_streaming()
            .await?;

        let mut stdout_lines = Vec::new();
        let mut success = false;

        while let Some(output) = rx.recv().await {
            match output {
                GitOutput::Stdout { line, .. } => stdout_lines.push(line),
                GitOutput::Completed { success: s, .. } => {
                    success = s;
                    break;
                }
                _ => {}
            }
        }

        if !success {
            return Err(GitError::CommandFailed {
                command: "git status --porcelain".to_string(),
                exit_code: 1,
                stderr: "Failed to check repository status".to_string(),
            });
        }

        if !stdout_lines.is_empty() {
            return Err(GitError::WorkingDirectoryDirty);
        }

        Ok(())
    }

    pub async fn current_branch(&self) -> GitResult<String> {
        let mut rx = GitCommand::new(&self.config.repository_path)
            .with_config(self.config.clone())
            .args(["symbolic-ref", "--short", "HEAD"])
            .execute_streaming()
            .await?;

        let mut branch_name = String::new();
        let mut success = false;

        while let Some(output) = rx.recv().await {
            match output {
                GitOutput::Stdout { line, .. } => {
                    branch_name = line.trim().to_string();
                }
                GitOutput::Completed { success: s, .. } => {
                    success = s;
                    break;
                }
                _ => {}
            }
        }

        if success && !branch_name.is_empty() {
            Ok(branch_name)
        } else {
            Err(GitError::CommandFailed {
                command: "git symbolic-ref --short HEAD".to_string(),
                exit_code: 1,
                stderr: "Could not determine current branch".to_string(),
            })
        }
    }

    pub async fn branch_exists(&self, branch_name: &str) -> GitResult<bool> {
        let mut rx = GitCommand::new(&self.config.repository_path)
            .with_config(self.config.clone())
            .args([
                "show-ref",
                "--verify",
                "--quiet",
                &format!("refs/heads/{branch_name}"),
            ])
            .execute_streaming()
            .await?;

        while let Some(output) = rx.recv().await {
            if let GitOutput::Completed { success, .. } = output {
                return Ok(success);
            }
        }

        Ok(false)
    }

    pub fn validate_branch_name(&self, branch_name: &str) -> GitResult<()> {
        if matches!(self.config.validation_level, ValidationLevel::None) {
            return Ok(());
        }

        if branch_name.is_empty() {
            return Err(InvalidBranchName {
                name: branch_name.to_string(),
                reason: "Branch name cannot be empty".to_string(),
            });
        }

        let invalid_chars = [' ', '~', '^', ':', '?', '*', '[', '\\'];
        if branch_name.chars().any(|c| invalid_chars.contains(&c)) {
            return Err(InvalidBranchName {
                name: branch_name.to_string(),
                reason: "Branch name contains invalid characters".to_string(),
            });
        }

        if branch_name.starts_with("-") || branch_name.ends_with(".") {
            return Err(InvalidBranchName {
                name: branch_name.to_string(),
                reason: "Invalid branch name format".to_string(),
            });
        }

        Ok(())
    }

    pub async fn forward_output(
        &self,
        rx: &mut mpsc::Receiver<GitOutput>,
        tx: &mpsc::Sender<GitOutput>,
    ) -> GitResult<()> {
        while let Some(output) = rx.recv().await {
            if let GitOutput::Completed { success, .. } = output {
                let _ = tx.send(output).await;
                if !success {
                    return Err(GitError::CommandFailed {
                        command: "forwarded command".to_string(),
                        exit_code: 1,
                        stderr: "Operation failed".to_string(),
                    });
                }
                break;
            } else {
                let _ = tx.send(output).await;
            }
        }
        Ok(())
    }

    pub async fn create_branch(
        &self,
        branch_name: &str,
        from: Option<&str>,
    ) -> GitResult<mpsc::Receiver<GitOutput>> {
        self.validate_branch_name(branch_name)?;

        let mut cmd = GitCommand::new(&self.config.repository_path)
            .with_config(self.config.clone())
            .args(["checkout", "-b", branch_name]);

        if let Some(from_branch) = from {
            cmd = cmd.arg(from_branch);
        }

        cmd.workflow_step(&format!("🌊 Creating branch: {}", branch_name))
            .execute_streaming()
            .await
    }

    pub async fn checkout(&self, branch_name: &str) -> GitResult<mpsc::Receiver<GitOutput>> {
        GitCommand::new(&self.config.repository_path)
            .with_config(self.config.clone())
            .args(["checkout", branch_name])
            .workflow_step(&format!("🌊 Switchting to branch: {branch_name}"))
            .execute_streaming()
            .await
    }

    pub async fn merge(
        &self,
        branch_name: &str,
        no_ff: bool,
    ) -> GitResult<mpsc::Receiver<GitOutput>> {
        let mut args = vec!["merge"];
        if no_ff {
            args.push("--no-ff");
        }

        args.push(branch_name);

        GitCommand::new(&self.config.repository_path)
            .with_config(self.config.clone())
            .args(&args)
            .workflow_step(&format!("🌊 Merging branch: {branch_name}"))
            .execute_streaming()
            .await
    }

    pub async fn delete_branch(
        &self,
        branch_name: &str,
        force: bool,
    ) -> GitResult<mpsc::Receiver<GitOutput>> {
        let flag = if force { "-D" } else { "-d" };

        GitCommand::new(&self.config.repository_path)
            .with_config(self.config.clone())
            .args(["branch", flag, branch_name])
            .workflow_step(&format!("🧹 Deleting branch: {branch_name}"))
            .execute_streaming()
            .await
    }

    pub async fn create_tag(
        &self,
        tag_name: &str,
        message: Option<&str>,
    ) -> GitResult<mpsc::Receiver<GitOutput>> {
        let mut args = vec!["tag"];

        if let Some(msg) = message {
            args.extend(["-a", tag_name, "-m", msg]);
        } else {
            args.push(tag_name);
        }

        GitCommand::new(&self.config.repository_path)
            .with_config(self.config.clone())
            .args(&args)
            .workflow_step(&format!("🏷️ Creating tag: {tag_name}"))
            .execute_streaming()
            .await
    }
}
