use std::path::Path;
use std::process::{Command, Output};
use thiserror::Error;

/// A structured error type for all possible Git command failures.
#[derive(Debug, Error)]
pub enum GitError {
    /// The `git` command could not be executed at all (e.g., not in PATH).
    #[error("Failed to execute git command: {0}")]
    ExecutionError(#[from] std::io::Error),

    /// The `git` command ran, but exited with a non-zero status code.
    /// indicating an error. `stderr` is captured.
    #[error(
        "Git command failed with exit code {status}:\n--- stderr ---\n{stderr}\n--- stdout ---\n{stdout}"
    )]
    CommandFailed {
        status: std::process::ExitStatus,
        stdout: String,
        stderr: String,
    },

    /// A specific, common error  case we want to handle gracefully.
    #[error("Merge conflict detected. Please resolve conflicts before proceeding.")]
    MergeConflict,
}

pub type GitResult<T> = Result<T, GitError>;

/// Constructs a `Command` for a git operation without executing it.
/// This is used by `run` and by our unit tests.
fn build_command<I, S>(cwd: &Path, args: I) -> Command
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut command = Command::new("git");
    command.current_dir(cwd).args(args);
    command
}

/// Execute an arbitrary Git command.
fn run<I, S>(cwd: &Path, args: I) -> GitResult<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let output = build_command(cwd, args).output()?;
    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if stderr.contains("CONFLICT") || stdout.contains("CONFLICT") {
            return Err(GitError::MergeConflict);
        }

        return Err(GitError::CommandFailed {
            status: output.status,
            stdout,
            stderr,
        });
    }

    Ok(output)
}

pub fn checkout_new_branch(repo_path: &Path, branch: &str) -> GitResult<()> {
    run(repo_path, ["checkout", "-b", branch])?;
    Ok(())
}

pub fn checkout(repo_path: &Path, branch: &str) -> GitResult<()> {
    run(repo_path, ["checkout", branch])?;
    Ok(())
}

pub fn fetch_all(repo_path: &Path) -> GitResult<()> {
    run(repo_path, ["fetch", "--all", "--prune"])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    mod unit_tests {
        use crate::git::build_command;
        use std::ffi::OsStr;
        use std::path::Path;

        #[test]
        fn test_build_command_constructs_correctly() {
            let cwd = Path::new("./fake/path");
            let args = ["checkout", "develop"];
            let command = build_command(cwd, args);

            assert_eq!(
                command.get_program(),
                "git",
                "The  program being run should always be 'git'"
            );
            assert_eq!(
                command.get_current_dir(),
                Some(cwd),
                "The command should run in the specified CWD"
            );

            let actual_args: Vec<&OsStr> = command.get_args().collect();
            let expected_args: Vec<&OsStr> = vec!["checkout".as_ref(), "develop".as_ref()];
            assert_eq!(
                actual_args, expected_args,
                "The arguments should be exactly in order 'checkout develop'"
            );
        }
    }

    #[cfg(feature = "test-integration")]
    mod integration_tests {
        use crate::git::{GitError, checkout, checkout_new_branch};
        use std::fs;
        use std::process::Command;
        use tempfile::{TempDir, tempdir};

        fn create_test_repo() -> TempDir {
            let dir = tempdir().unwrap();
            let path = dir.path();

            Command::new("git")
                .current_dir(path)
                .arg("init")
                .output()
                .unwrap();
            fs::write(path.join("README.md"), "hello").unwrap();
            Command::new("git")
                .current_dir(path)
                .arg("add")
                .arg(".")
                .output()
                .unwrap();
            Command::new("git")
                .current_dir(path)
                .arg("commit")
                .arg("-m")
                .arg("Initial commit")
                .output()
                .unwrap();

            dir
        }

        #[test]
        fn test_checkout_new_branch_successfully() {
            let repo = create_test_repo();
            let repo_path = repo.path();

            let result = checkout_new_branch(repo_path, "develop");
            assert!(result.is_ok(), "checkout should succeed");

            let head = fs::read_to_string(repo_path.join(".git/HEAD")).unwrap();
            assert!(
                head.contains("refs/heads/develop"),
                "HEAD should point to the new branch"
            );
        }

        #[test]
        fn test_checkout_nonexistent_branch_fails() {
            let repo = create_test_repo();
            let repo_path = repo.path();

            let result = checkout(repo_path, "nonexistent-branch");
            assert!(
                result.is_err(),
                "checkout of a nonexistent branch should fail"
            );

            if let Err(GitError::CommandFailed { stderr, .. }) = result {
                assert!(stderr.contains("did not match any file(s) known to git"));
            } else {
                panic!("Expected GitError::CommandFailed, but got a different error: {result:?}")
            }
        }
    }
}
