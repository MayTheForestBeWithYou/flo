use crate::gitflow::steps::{checkout, delete_branch, merge};
use crate::gitflow::workflow::{Workflow, WorkflowContext};

pub fn finish_feature_workflow(feature_name: &str) -> Workflow {
    Workflow::builder("Finish Feature", WorkflowContext::new())
        .step(checkout(feature_name))
        .step(merge(feature_name, false))
        .step(delete_branch(feature_name, false))
        .build()
}
