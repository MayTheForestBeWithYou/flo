use crate::git::command::GitCommand;
use crate::git::types::GitError::CommandFailed;
use crate::git::types::{GitConfig, GitError, GitOutput, GitResult, RemoteOperation};
use std::path::PathBuf;
use tokio::sync::mpsc;

pub struct GitRemote {
    config: GitConfig,
}

impl GitRemote {
    pub fn new(config: GitConfig) -> Self {
        Self { config }
    }

    pub async fn fetch(&self, remote: Option<&str>) -> GitResult<mpsc::Receiver<GitOutput>> {
        let remote_name = remote.unwrap_or(&self.config.default_remote);

        let mut cmd = GitCommand::new(&self.config.repository_path)
            .with_config(self.config.clone())
            .arg("fetch")
            .remote_operation(RemoteOperation::Fetch)
            .workflow_step(&format!("🌊 Fetching from {remote_name}"));

        if let Some(r) = remote {
            cmd = cmd.arg(r);
        }

        cmd.execute_streaming().await
    }

    pub async fn pull(
        &self,
        remote: Option<&str>,
        branch: Option<&str>,
    ) -> GitResult<mpsc::Receiver<GitOutput>> {
        let mut cmd = GitCommand::new(&self.config.repository_path)
            .with_config(self.config.clone())
            .arg("pull")
            .remote_operation(RemoteOperation::Pull)
            .workflow_step("🌊 Pulling latest changes");

        if let Some(r) = remote {
            cmd = cmd.arg(r);
        }
        if let Some(b) = branch {
            cmd = cmd.arg(b);
        }

        cmd.execute_streaming().await
    }

    pub async fn push(&self, remote: &str, branch: &str) -> GitResult<mpsc::Receiver<GitOutput>> {
        let (tx, rx) = mpsc::channel(1000);

        let mut push_rx = GitCommand::new(&self.config.repository_path)
            .with_config(self.config.clone())
            .args(["push", remote, branch])
            .remote_operation(RemoteOperation::Push)
            .workflow_step(format!("🚀 Pushing {branch} to {remote}"))
            .execute_streaming()
            .await?;

        let branch_name = branch.to_string();
        let remote_name = remote.to_string();

        tokio::spawn(async move {
            let mut push_success = false;
            while let Some(output) = push_rx.recv().await {
                if let GitOutput::Completed { success, .. } = output {
                    push_success = success;
                    let _ = tx.send(output).await;
                    break;
                } else {
                    let _ = tx.send(output).await;
                }
            }
        });
        Ok(rx)
    }

    pub async fn clone_repository(
        url: &str,
        destination: &PathBuf,
        branch: Option<&str>,
    ) -> GitResult<mpsc::Receiver<GitOutput>> {
        let mut cmd = GitCommand::new(destination.parent().unwrap_or(destination))
            .arg("clone")
            .arg(url)
            .arg(destination.to_string_lossy().as_ref())
            .remote_operation(RemoteOperation::Clone)
            .workflow_step(&format!("📥 Cloning repository from {url}"));

        if let Some(b) = branch {
            cmd = cmd.args(["-b", b]);
        }

        cmd.execute_streaming().await
    }

    pub async fn push_tags(&self, remote: &str) -> GitResult<mpsc::Receiver<GitOutput>> {
        GitCommand::new(&self.config.repository_path)
            .with_config(self.config.clone())
            .args(["push", remote, "--tags"])
            .remote_operation(RemoteOperation::Push)
            .workflow_step(&format!("🏷️ Pushing tags to {remote}"))
            .execute_streaming()
            .await
    }

    pub async fn get_remote_url(&self, remote: &str) -> GitResult<String> {
        let mut rx = GitCommand::new(&self.config.repository_path)
            .with_config(self.config.clone())
            .args(["remote", "get-url", remote])
            .execute_streaming()
            .await?;

        let mut url = String::new();
        let mut success = false;

        while let Some(output) = rx.recv().await {
            match output {
                GitOutput::Stdout { line, .. } => {
                    url = line.trim().to_string();
                }
                GitOutput::Completed { success: s, .. } => {
                    success = s;
                    break;
                }
                _ => {}
            }
        }

        if success && !url.is_empty() {
            Ok(url)
        } else {
            Err(GitError::RemoteNotFound {
                remote: remote.to_string(),
            })
        }
    }

    pub async fn list_remotes(&self) -> GitResult<Vec<String>> {
        let mut rx = GitCommand::new(&self.config.repository_path)
            .with_config(self.config.clone())
            .args(["remote"])
            .execute_streaming()
            .await?;

        let mut remotes = Vec::new();
        let mut success = false;

        while let Some(output) = rx.recv().await {
            match output {
                GitOutput::Stdout { line, .. } => {
                    if !line.trim().is_empty() {
                        remotes.push(line.trim().to_string());
                    }
                }
                GitOutput::Completed { success: s, .. } => {
                    success = s;
                    break;
                }
                _ => {}
            }
        }

        if success {
            Ok(remotes)
        } else {
            Err(CommandFailed {
                command: "git remote".to_string(),
                exit_code: 1,
                stderr: "Failed to list remotes".to_string(),
            })
        }
    }
}
