//! Git types and errors

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Repository not found at path: {path}")]
    RepositoryNotFound { path: String },

    #[error("Not a valid Git repository: {path}")]
    InvalidRepository { path: String },

    #[error("Git command failed: {command} (exit code: {exit_code})")]
    CommandFailed {
        command: String,
        exit_code: i32,
        stderr: String,
    },

    #[error("Working directory is not clean. Uncommitted changes detected")]
    WorkingDirectoryDirty,

    #[error("Branch '{branch}' does not exist")]
    BranchNotFound { branch: String },

    #[error("Branch '{branch}' already exists")]
    BranchAlreadyExists { branch: String },

    #[error("Merge conflict detected in files: {files:?}")]
    MergeConflict { files: Vec<String> },

    #[error("Remote '{remote}' not found")]
    RemoteNotFound { remote: String },

    #[error("Operation aborted: {reason}")]
    OperationAborted { reason: String },

    #[error("Git process failed to start: {source}")]
    ProcessSpawnError { source: std::io::Error },

    #[error("Invalid branch name: {name}. {reason}")]
    InvalidBranchName { name: String, reason: String },

    #[error("Plugin execution failed: {plugin_name}. {reason}")]
    PluginFailed { plugin_name: String, reason: String },

    #[error("Network error: {operation}. {reason}")]
    NetworkError { operation: String, reason: String },
}

/// Result type for Git operations
pub type GitResult<T> = Result<T, GitError>;

/// Real-time output from Git commands with metadata and progress
#[derive(Debug, Clone)]
pub enum GitOutput {
    /// Standard output line with timestamp
    Stdout {
        line: String,
        timestamp: std::time::Instant,
    },
    /// Error output line with timestamp
    Stderr {
        line: String,
        timestamp: std::time::Instant,
    },
    /// Progress information for operations like clone, fetch, push
    Progress {
        operation: String,
        current: u64,
        total: Option<u64>,
        message: String,
    },
    /// Command completed with final result
    Completed {
        success: bool,
        exit_code: i32,
        command: String,
    },
    /// Step information for TUI workflow display
    WorkflowStep { step: String, status: StepStatus },
    /// Plugin hook execution
    PluginHook {
        hook_name: String,
        plugin: String,
        status: PluginStatus,
    },
}

/// Status of workflow steps for TUI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed { reason: String },
    Skipped { reason: String },
}

/// Status of plugin execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginStatus {
    Starting,
    Running,
    Completed { output: String },
    Failed { error: String },
    Skipped { reason: String },
}

/// Plugin hooks that can be triggered during workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginHook {
    // Feature workflow hooks
    PreFeatureStart {
        feature_name: String,
    },
    PostFeatureStart {
        feature_name: String,
        branch: String,
    },
    PreFeatureFinish {
        feature_name: String,
        branch: String,
    },
    PreFeatureMerge {
        feature_name: String,
        source_branch: String,
        target_branch: String,
    },
    PostFeatureMerge {
        feature_name: String,
        source_branch: String,
        target_branch: String,
    },
    PostFeatureFinish {
        feature_name: String,
    },

    // Release workflow hooks
    PreReleaseStart {
        version: String,
    },
    PostReleaseStart {
        version: String,
        branch: String,
    },
    PreReleaseFinish {
        version: String,
        branch: String,
    },
    PreReleaseTag {
        version: String,
        tag: String,
    },
    PostReleaseTag {
        version: String,
        tag: String,
    },
    PreReleaseMergeToMain {
        version: String,
        branch: String,
    },
    PostReleaseMergeToMain {
        version: String,
        branch: String,
    },
    PreReleaseMergeToDevelop {
        version: String,
        branch: String,
    },
    PostReleaseMergeToDevelop {
        version: String,
        branch: String,
    },
    PostReleaseToFinish {
        version: String,
        tag: String,
    },

    // Hotfix workflow hooks
    PreHotfixStart {
        version: String,
    },
    PostHotfixStart {
        version: String,
        branch: String,
    },
    PreHotfixFinish {
        version: String,
        branch: String,
    },
    PreHotfixTag {
        version: String,
        tag: String,
    },
    PostHotfixTag {
        version: String,
        tag: String,
    },
    PreHotfixMergeToMain {
        version: String,
        branch: String,
    },
    PostHotfixMergeToMain {
        version: String,
        branch: String,
    },
    PreHotfixMergeToDevelop {
        version: String,
        branch: String,
    },
    PostHotfixMergeToDevelop {
        version: String,
        branch: String,
    },
    PostHotfixFinish {
        version: String,
        tag: String,
    },

    // General Git operation hooks
    PrePush {
        branch: String,
        remote: String,
    },
    PostPush {
        branch: String,
        remote: String,
    },
    PreMerge {
        source_branch: String,
        target_branch: String,
    },
    PostMerge {
        source_branch: String,
        target_branch: String,
    },
    OnConflict {
        files: Vec<String>,
    },
    PreCommit {
        message: String,
    },
    PostCommit {
        commit_hash: String,
    },
}

/// Validation levels for dangerous operations
#[derive(Debug, Clone, Copy)]
pub enum ValidationLevel {
    /// No validation (use with caution)
    None,
    /// Basic validation (working directory clean, branch exists, etc.)
    Basic,
    /// Strict validation (includes remote checks, conflict prediction)
    Strict,
}

/// Configuration for Git operations
#[derive(Debug, Clone)]
pub struct GitConfig {
    pub repository_path: PathBuf,
    pub validation_level: ValidationLevel,
    pub dry_run: bool,
    pub timeout_seconds: Option<u64>,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub default_remote: String,
    pub plugin_hooks_enabled: bool,
    pub progress_reporting: bool,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            repository_path: PathBuf::from("."),
            validation_level: ValidationLevel::Basic,
            dry_run: false,
            timeout_seconds: Some(300),
            user_name: None,
            user_email: None,
            default_remote: "origin".to_string(),
            plugin_hooks_enabled: true,
            progress_reporting: true,
        }
    }
}

/// Remote operation types for progress tracking
#[derive(Debug, Clone)]
pub enum RemoteOperation {
    Clone,
    Fetch,
    Pull,
    Push,
}
