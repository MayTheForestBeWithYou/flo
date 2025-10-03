//! Low-level Git command execution with streaming output

use super::types::GitConfig;
use super::types::{GitError, RemoteOperation, StepStatus};
use super::types::{GitOutput, GitResult};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct GitCommand {
    args: Vec<String>,
    config: GitConfig,
    workflow_context: Option<String>,
    remote_operation: Option<RemoteOperation>,
}

impl GitCommand {
    pub fn new<P: Into<PathBuf>>(repository_path: P) -> Self {
        Self {
            args: vec!["git".to_string()],
            config: GitConfig {
                repository_path: repository_path.into(),
                ..Default::default()
            },
            workflow_context: None,
            remote_operation: None,
        }
    }

    pub fn with_config(mut self, config: GitConfig) -> Self {
        self.config = config;
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.args
            .extend(args.into_iter().map(|s| s.as_ref().to_string()));
        self
    }

    pub fn arg<S: AsRef<str>>(mut self, arg: S) -> Self {
        self.args.push(arg.as_ref().to_string());
        self
    }

    pub fn workflow_step<S: Into<String>>(mut self, step: S) -> Self {
        self.workflow_context = Some(step.into());
        self
    }

    pub fn remote_operation(mut self, operation: RemoteOperation) -> Self {
        self.remote_operation = Some(operation);
        self
    }

    pub async fn execute_streaming(self) -> GitResult<mpsc::Receiver<GitOutput>> {
        let (tx, rx) = mpsc::channel(1000);
        let command_str = self.args.join(" ");

        if let Some(step) = &self.workflow_context {
            let _ = tx
                .send(GitOutput::WorkflowStep {
                    step: step.clone(),
                    status: StepStatus::Running,
                })
                .await;
        }

        if self.config.dry_run {
            let _ = tx
                .send(GitOutput::Stdout {
                    line: format!("🦊 [DRY RUN] Would execute: {command_str}"),
                    timestamp: std::time::Instant::now(),
                })
                .await;

            if let Some(step) = &self.workflow_context {
                let _ = tx
                    .send(GitOutput::WorkflowStep {
                        step: step.clone(),
                        status: StepStatus::Completed,
                    })
                    .await;
            }

            let _ = tx
                .send(GitOutput::Completed {
                    success: true,
                    exit_code: 0,
                    command: command_str,
                })
                .await;

            return Ok(rx);
        }

        if !self.config.repository_path.exists() {
            return Err(GitError::RepositoryNotFound {
                path: self.config.repository_path.to_string_lossy().to_string(),
            });
        }

        let command = self.clone();
        let workflow_context = command.workflow_context.clone();
        tokio::spawn(async move {
            if let Err(e) = &command.execute_internal(tx.clone()).await {
                let _ = tx
                    .send(GitOutput::Stderr {
                        line: format!("🦊 Error: {e}"),
                        timestamp: std::time::Instant::now(),
                    })
                    .await;

                if let Some(step) = workflow_context {
                    let _ = tx
                        .send(GitOutput::WorkflowStep {
                            step: step.clone(),
                            status: StepStatus::Failed {
                                reason: e.to_string(),
                            },
                        })
                        .await;
                }
            }
        });

        Ok(rx)
    }

    async fn execute_internal(self, tx: mpsc::Sender<GitOutput>) -> GitResult<()> {
        let mut cmd = TokioCommand::new(&self.args[0]);
        cmd.args(&self.args[1..])
            .current_dir(&self.config.repository_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        cmd.env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_PAGER", "")
            .env("GIT_EDITOR", "true")
            .env("LANG", "en_US.UTF-8");

        if self.remote_operation.is_some() && self.config.progress_reporting {
            cmd.env("GIT_PROGRESS", "1");
        }

        if let Some(name) = &self.config.user_name {
            cmd.env("GIT_AUTHOR_NAME", name)
                .env("GIT_COMMITTER_NAME", name);
        }
        if let Some(email) = &self.config.user_email {
            cmd.env("GIT_AUTHOR_EMAIL", email)
                .env("GIT_COMMITTER_EMAIL", email);
        }

        let mut child = cmd
            .spawn()
            .map_err(|e| GitError::ProcessSpawnError { source: e })?;

        if let Some(stdout) = child.stdout.take() {
            let tx_stdout = tx.clone();
            let remote_op = self.remote_operation.clone();
            tokio::spawn(async move {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    if let Some(ref op) = remote_op {
                        if let Some(progress) = Self::parse_progress_line(&line, op) {
                            let _ = tx_stdout.send(progress).await;
                            continue;
                        }
                    }

                    let _ = tx_stdout
                        .send(GitOutput::Stdout {
                            line,
                            timestamp: std::time::Instant::now(),
                        })
                        .await;
                }
            });
        }

        if let Some(stderr) = child.stderr.take() {
            let tx_stderr = tx.clone();
            let remote_op = self.remote_operation.clone();
            tokio::spawn(async move {
                let reader = BufReader::new(stderr);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    if let Some(ref op) = remote_op {
                        if let Some(progress) = Self::parse_progress_line(&line, op) {
                            let _ = tx_stderr.send(progress).await;
                            continue;
                        }
                    }

                    let _ = tx_stderr
                        .send(GitOutput::Stderr {
                            line,
                            timestamp: std::time::Instant::now(),
                        })
                        .await;
                }
            });
        }

        let output = if let Some(timeout) = self.config.timeout_seconds {
            tokio::time::timeout(std::time::Duration::from_secs(timeout), child.wait())
                .await
                .map_err(|_| GitError::OperationAborted {
                    reason: format!("Command timed out after {timeout} seconds"),
                })?
        } else {
            child.wait().await
        }
        .map_err(|e| GitError::ProcessSpawnError { source: e })?;

        let success = output.success();
        let exit_code = output.code().unwrap_or(-1);
        let command_str = self.args.join(" ");

        if let Some(step) = &self.workflow_context {
            let status = if success {
                StepStatus::Completed
            } else {
                StepStatus::Failed {
                    reason: format!("Command failed with exit code {exit_code}"),
                }
            };
            let _ = tx
                .send(GitOutput::WorkflowStep {
                    step: step.clone(),
                    status,
                })
                .await;
        }

        let _ = tx
            .send(GitOutput::Completed {
                success,
                exit_code,
                command: command_str.clone(),
            })
            .await;

        if !success {
            return Err(GitError::CommandFailed {
                command: command_str,
                exit_code,
                stderr: "See streamed output above".to_string(),
            });
        }

        Ok(())
    }

    fn parse_progress_line(line: &str, operation: &RemoteOperation) -> Option<GitOutput> {
        if line.contains("%") && line.contains("(") && line.contains(")") {
            if let Some(percent_start) = line.rfind(' ') {
                if let Some(percent_end) = line[percent_start..].find('%') {
                    if let Ok(percent) =
                        line[percent_start + 1..percent_start + percent_end].parse::<u64>()
                    {
                        return Some(GitOutput::Progress {
                            operation: format!("{operation:?}"),
                            current: percent,
                            total: Some(100),
                            message: line.to_string(),
                        });
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::types::{GitOutput, RemoteOperation};
    
    #[test]
    fn test_parse_progress_receiving_objects() {
        let line = "Receiving objects: 75% (150/200), 1.25 MiB | 5.00 MiB/s, done.";
        let operation = RemoteOperation::Clone;
        
        let result = GitCommand::parse_progress_line(line, &operation);
        
        assert!(result.is_some(), "Should have successfully parsed the progress line.");
        
        if let Some(GitOutput::Progress { operation, current, total, ..}) = result {
            assert_eq!(operation, "Clone");
            assert_eq!(current, 75);
            assert_eq!(total, Some(100));
        } else {
            panic!("Parsed result was not a GitOutput::Progress variant.");
        }
    }
    
    #[test]
    fn test_parse_progress_non_matching_line() {
        let line = "Cloning into 'my-repo'...";
        let operation = RemoteOperation::Clone;
        
        let result = GitCommand::parse_progress_line(line, &operation);
        
        assert!(result.is_none());
    }
    
    #[tokio::test]
    async fn test_dry_run_produces_correct_output() {
        let mut config = GitConfig::default();
        config.dry_run = true;
        
        let temp_dir = tempfile::TempDir::new().unwrap();
        
        let mut rx = GitCommand::new(temp_dir.path())
            .with_config(config)
            .args(["status", "--porcelain"])
            .execute_streaming()
            .await
            .expect("Executing a dry-run command should not fail.");
        
        let mut found_dry_run_output = false;
        let mut output_lines = Vec::new();
        
        while let Some(output) = rx.recv().await {
            if let GitOutput::Stdout { line, .. } = &output {
                if line.contains("[DRY RUN]") && line.contains("git status --porcelain") {
                    found_dry_run_output = true;
                }
            }
            output_lines.push(output);
        }
        
        assert!(found_dry_run_output);
        println!("{:?}", output_lines);
        
        let last_output = output_lines.last().unwrap();
        if let GitOutput::Completed { success, exit_code, .. } = last_output {
            assert!(success);
            assert_eq!(*exit_code, 0)
        } else {
            panic!("The last output of a command stream should be GitOutput::Completed");
        }
    }
}
