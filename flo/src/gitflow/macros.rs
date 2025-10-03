#[macro_export]
macro_rules! define_git_step {
    (
        $(#[$outer:meta])*
        $struct_name:ident,
        ($($field_name:ident: $field_type:ty),*),
        ($self:ident, $repo:ident, $ctx:ident, $tx:ident) => $execute_body:block
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct $struct_name {
            $(pub $field_name: $field_type),*
        }

        impl $struct_name {
            pub fn new($($field_name: $field_type),*) -> Self {
                Self { $($field_name),* }
            }
        }

        #[async_trait::async_trait]
        impl $crate::gitflow::workflow::WorkflowStep for $struct_name {
            async fn execute(
                &$self,
                $repo: &std::sync::Arc<$crate::git::repository::GitRepository>,
                $ctx: &mut $crate::gitflow::workflow::WorkflowContext,
                $tx: &tokio::sync::mpsc::Sender<$crate::git::types::GitOutput>,
            ) -> $crate::git::types::GitResult<()> {
                // Future Enhancement: Add pre/post execution logging here
                $execute_body
            }
        }
    };
}
