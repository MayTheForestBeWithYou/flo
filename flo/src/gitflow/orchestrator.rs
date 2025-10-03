use crate::git::repository::GitRepository;
use crate::git::types::{GitOutput, GitResult, StepStatus};
use crate::gitflow::workflow;
use crate::gitflow::workflow::WorkflowStep;
use crate::gitflow::workflows::feature::finish_feature_workflow;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

/// The GitFlow Orchestrator.
///
/// This struct is the central coordinator for all high-level Git workflows.
/// It holds shared services and delegates the definition and execution of
/// workflows to the builder pattern.
pub struct GitFlowOrchestrator {
    repository: Arc<GitRepository>,
}

impl GitFlowOrchestrator {
    pub fn new(repo: GitRepository) -> Self {
        Self {
            repository: Arc::new(repo),
        }
    }

    // --- Public Workflow methods ---
    // These methods are the public API for the orchestrator. Each one follows
    // the same two-step pattern:
    // 1. DEFINE the specific workflow recipe by calling a builder function.
    // 2. EXECUTE the recipe using the generic `execute_workflow` runner.

    pub fn finish_feature(&self, feature_name: &str) -> GitResult<Receiver<GitOutput>> {
        let workflow = finish_feature_workflow(feature_name);

        self.execute_workflow(workflow)
    }

    // --- Private Generic Executor ---

    /// Takes any `Workflow` object and executes its steps in a new async task.
    /// This is the single, centralized execution engine for all workflows.
    fn execute_workflow(
        &self,
        workflow: workflow::Workflow,
    ) -> GitResult<mpsc::Receiver<GitOutput>> {
        let (tx, rx) = mpsc::channel(1000);

        let repo_handle = Arc::clone(&self.repository);

        let mut context = workflow.context;
        let workflow_name = workflow.name;

        tokio::spawn(async move {
            for step in workflow.steps {
                let result = step.execute(&repo_handle, &mut context, &tx).await;

                if let Err(e) = result {
                    let _ = tx
                        .send(GitOutput::WorkflowStep {
                            step: format!("❌ Workflow '{workflow_name}' failed."),
                            status: StepStatus::Failed {
                                reason: e.to_string(),
                            },
                        })
                        .await;
                    return;
                }
            }
        });

        Ok(rx)
    }
}
