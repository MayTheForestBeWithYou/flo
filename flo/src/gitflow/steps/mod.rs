pub mod git;

use crate::git::repository::GitRepository;
use crate::git::types::{GitOutput, GitResult};
use crate::gitflow::steps::git::{
    GitCheckout, GitCreateBranch, GitCreateTag, GitDeleteBranch, GitMerge,
};
use crate::gitflow::workflow::{WorkflowContext, WorkflowStep};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Step {
    Checkout(GitCheckout),
    CreateBranch(GitCreateBranch),
    Merge(GitMerge),
    DeleteBranch(GitDeleteBranch),
    CreateTag(GitCreateTag),
}

#[async_trait]
impl WorkflowStep for Step {
    async fn execute(
        &self,
        repo: &Arc<GitRepository>,
        context: &mut WorkflowContext,
        tx: &Sender<GitOutput>,
    ) -> GitResult<()> {
        match self {
            Step::Checkout(s) => s.execute(repo, context, tx).await,
            Step::CreateBranch(s) => s.execute(repo, context, tx).await,
            Step::Merge(s) => s.execute(repo, context, tx).await,
            Step::DeleteBranch(s) => s.execute(repo, context, tx).await,
            Step::CreateTag(s) => s.execute(repo, context, tx).await,
        }
    }
}

pub fn checkout(branch: impl Into<String>) -> Step {
    Step::Checkout(GitCheckout::new(branch.into()))
}

pub fn create_branch(name: impl Into<String>, base: impl Into<String>) -> Step {
    Step::CreateBranch(GitCreateBranch::new(name.into(), base.into()))
}

pub fn merge(branch: impl Into<String>, strategy: bool) -> Step {
    Step::Merge(GitMerge::new(branch.into(), strategy))
}

pub fn delete_branch(branch: impl Into<String>, force: bool) -> Step {
    Step::DeleteBranch(GitDeleteBranch::new(branch.into(), force))
}

pub fn create_tag(name: impl Into<String>, tag: impl Into<String>) -> Step {
    Step::CreateTag(GitCreateTag::new(name.into(), tag.into()))
}
