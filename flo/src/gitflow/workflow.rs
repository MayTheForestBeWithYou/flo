use crate::git::repository::GitRepository;
use crate::git::types::{GitOutput, GitResult};
use crate::gitflow::steps::Step;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::mpsc;

pub type WorkflowContext = std::collections::HashMap<String, String>;

/// A single, executable step within a larger workflow.
#[async_trait]
pub trait WorkflowStep: Debug + Send + Sync {
    /// Executes the logic for this step.
    async fn execute(
        &self,
        repo: &Arc<GitRepository>,
        context: &mut WorkflowContext,
        tx: &mpsc::Sender<GitOutput>,
    ) -> GitResult<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub context: WorkflowContext,
    pub steps: Vec<Step>,
}

impl Workflow {
    pub fn builder(name: impl Into<String>, context: WorkflowContext) -> WorkflowBuilder {
        WorkflowBuilder::new(name, context)
    }
}

pub struct WorkflowBuilder {
    name: String,
    context: WorkflowContext,
    steps: Vec<Step>,
}

impl WorkflowBuilder {
    pub fn new(name: impl Into<String>, context: WorkflowContext) -> Self {
        Self {
            name: name.into(),
            context,
            steps: Vec::new(),
        }
    }

    /// Adds any struct that implements `WorkflowStep` to the workflow.
    pub fn step(mut self, step: Step) -> Self {
        self.steps.push(step);
        self
    }

    pub fn build(self) -> Workflow {
        Workflow {
            name: self.name,
            context: self.context,
            steps: self.steps,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    pub workflow: Workflow,
    pub next_step_index: usize,
    pub status: WorkflowStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatus {
    Running,
    Completed,
    Failed,
}
