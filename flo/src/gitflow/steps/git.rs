use crate::define_git_step;

define_git_step!(
    GitCheckout,
    (branch: String),
    (self, repo, _ctx, tx) => {
        let mut rx = repo.checkout(&self.branch).await?;
        repo.forward_output(&mut rx, tx).await
    }
);

define_git_step!(
    GitCreateBranch,
    (branch_name: String, base_branch: String),
    (self, repo, _ctx, tx) => {
        let mut rx = repo.create_branch(&self.branch_name, Some(&self.base_branch)).await?;
        repo.forward_output(&mut rx, tx).await
    }
);

define_git_step!(
    GitMerge,
    (source_branch: String, merge_strategy: bool),
    (self, repo, _ctx, tx) => {
        let mut rx = repo.merge(&self.source_branch, self.merge_strategy).await?;
        repo.forward_output(&mut rx, tx).await
    }
);

define_git_step!(
    GitDeleteBranch,
    (branch: String, force: bool),
    (self, repo, _ctx, tx) => {
        let mut rx = repo.delete_branch(&self.branch, self.force).await?;
        repo.forward_output(&mut rx, tx).await
    }
);

define_git_step!(
    GitCreateTag,
    (tag_name: String, message: String),
    (self, repo, _ctx, tx) => {
        let mut rx = repo.create_tag(&self.tag_name, Some(&self.message)).await?;
        repo.forward_output(&mut rx, tx).await
    }
);
