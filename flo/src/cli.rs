use clap::{Parser, Subcommand};

/// Flo: Your friendly, interactive guide through the streams of `GitFlow`.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The command to run.
    #[command(subcommand)]
    pub command: Command,

    /// Run in non-interactive mode (for scripting).
    #[arg(long, global = true)]
    pub non_interactive: bool,

    /// Resume a previously failed operation.
    #[arg(long = "continue", global = true)]
    pub continue_operation: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialize a new repisotry for use with Flo and `GitFlow`.
    Init,

    /// Start, finish, or manage feature branches.
    Feature(FeatureCommand),

    /// Start, finish, or manage release branches.
    Release(ReleaseCommand),

    /// Start, finish, or manage hotfix branches.
    Hotfix(HotfixCommand),

    /// View the status of your Flo-managed repository.
    Status,

    /// Manage Flo plugins.
    #[command(subcommand)]
    Plugins(PluginCommand),

    /// View and debug your Flo configuration.
    #[command(subcommand)]
    Config(ConfigCommand),
}

/// Commands for managing feature branaches.
#[derive(Parser, Debug)]
pub struct FeatureCommand {
    #[command(subcommand)]
    pub command: FeatureSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum FeatureSubcommand {
    /// Start a new feature branch.
    Start {
        /// The name of the feature (e.g., 'add-login-button').
        name: String,
    },
    /// Finish the current or a specified feature branch.
    Finish {
        /// The name of the feature to finish. If omitted, uses the current branch.
        name: Option<String>,
    },
}

/// Commands for managing release branches.
#[derive(Parser, Debug)]
pub struct ReleaseCommand {
    #[command(subcommand)]
    pub command: ReleaseSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum ReleaseSubcommand {
    /// Start a new release branch.
    Start {
        /// The semantic version of the release (e.g., '1.2.0').
        /// If omitted, Flo will suggest the next version interactively.
        version: Option<String>,
    },
    /// Finalize and merge a release branch.
    Finish {
        /// The version of the release to finish (e.g., '1.2.0').
        /// If omitted, uses the current release branch.
        version: Option<String>,
    },
}

/// Commands for managing hotfix branches.
#[derive(Parser, Debug)]
pub struct HotfixCommand {
    #[command(subcommand)]
    pub command: HotfixSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum HotfixSubcommand {
    /// Start a new hotfix branch.
    Start {
        /// The semantic version of the hotfix (e.g., '1.2.1').
        /// If omitted, Flo will suggest the next version interactively.
        version: Option<String>,
    },
    /// Finalize and merge a hotfix branch.
    Finish {
        /// The version of the hotfix to finish (e.g., '1.2.1').
        /// If omitted, uses the current hotfix branch.
        version: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum PluginCommand {
    /// List all installed and detected plugins.
    List,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    /// Print the final merged configuration for the current project.
    Debug,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::match_wildcard_for_single_variants)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_start_parses_correctly() {
        let args = vec!["flo", "feature", "start", "my-cool-feature"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Command::Feature(feature_cmd) => match feature_cmd.command {
                FeatureSubcommand::Start { name } => {
                    assert_eq!(name, "my-cool-feature");
                }
                _ => panic!("Expected Feature::Start, but got something else."),
            },
            _ => panic!("Expected Command::Feature, but got something else."),
        }
    }

    #[test]
    fn test_feature_finish_with_optional_name() {
        let args = vec!["flo", "feature", "finish", "my-cool-feature"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Command::Feature(feature_cmd) => match feature_cmd.command {
                FeatureSubcommand::Finish { name } => {
                    assert_eq!(name, Some("my-cool-feature".to_string()));
                }
                _ => panic!("Expected Feature::Finish, but got something else."),
            },
            _ => panic!("Expected Command::Feature, but got something else."),
        }
    }

    #[test]
    fn test_feature_finish_without_optional_name() {
        let args = vec!["flo", "feature", "finish"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Command::Feature(feature_cmd) => match feature_cmd.command {
                FeatureSubcommand::Finish { name } => {
                    assert_eq!(name, None);
                }
                _ => panic!("Expected Feature::Finish, but got something else."),
            },
            _ => panic!("Expected Command::Feature, but got something else."),
        }
    }

    #[test]
    fn test_release_start_without_optional_name() {
        let args = vec!["flo", "release", "start"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Command::Release(release_cmd) => match release_cmd.command {
                ReleaseSubcommand::Start { version } => {
                    assert_eq!(version, None);
                }
                _ => panic!("Expected Release::Start, but got something else."),
            },
            _ => panic!("Expected Command::Release, but got something else."),
        }
    }

    #[test]
    fn test_release_start_with_optional_name() {
        let args = vec!["flo", "release", "start", "my-release"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Command::Release(release_cmd) => match release_cmd.command {
                ReleaseSubcommand::Start { version } => {
                    assert_eq!(version, Some("my-release".to_string()));
                }
                _ => panic!("Expected Release::Start, but got something else."),
            },
            _ => panic!("Expected Command::Release, but got something else."),
        }
    }

    #[test]
    fn test_release_finish_without_optional_name() {
        let args = vec!["flo", "release", "finish"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Command::Release(release_cmd) => match release_cmd.command {
                ReleaseSubcommand::Finish { version } => {
                    assert_eq!(version, None);
                }
                _ => panic!("Expected Release::Finish, but got something else."),
            },
            _ => panic!("Expected Command::Release, but got something else."),
        }
    }

    #[test]
    fn test_release_finish_with_optional_name() {
        let args = vec!["flo", "release", "finish", "my-release"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Command::Release(release_cmd) => match release_cmd.command {
                ReleaseSubcommand::Finish { version } => {
                    assert_eq!(version, Some("my-release".to_string()));
                }
                _ => panic!("Expected Release::Finish, but got something else."),
            },
            _ => panic!("Expected Command::Release, but got something else."),
        }
    }

    #[test]
    fn test_hotfix_start_without_optional_name() {
        let args = vec!["flo", "hotfix", "start"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Command::Hotfix(hotfix_cmd) => match hotfix_cmd.command {
                HotfixSubcommand::Start { version } => {
                    assert_eq!(version, None);
                }
                _ => panic!("Expected Hotfix::Start, but got something else."),
            },
            _ => panic!("Expected Command::Hotfix, but got something else."),
        }
    }

    #[test]
    fn test_hotfix_start_with_optional_name() {
        let args = vec!["flo", "hotfix", "start", "my-hotfix"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Command::Hotfix(hotfix_cmd) => match hotfix_cmd.command {
                HotfixSubcommand::Start { version } => {
                    assert_eq!(version, Some("my-hotfix".to_string()));
                }
                _ => panic!("Expected Hotfix::Start, but got something else."),
            },
            _ => panic!("Expected Command::Hotfix, but got something else."),
        }
    }

    #[test]
    fn test_hotfix_finish_without_optional_name() {
        let args = vec!["flo", "hotfix", "finish"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Command::Hotfix(hotfix_cmd) => match hotfix_cmd.command {
                HotfixSubcommand::Finish { version } => {
                    assert_eq!(version, None);
                }
                _ => panic!("Expected Hotfix::Finish, but got something else."),
            },
            _ => panic!("Expected Command::Hotfix, but got something else."),
        }
    }

    #[test]
    fn test_hotfix_finish_with_optional_name() {
        let args = vec!["flo", "hotfix", "finish", "my-hotfix"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Command::Hotfix(hotfix_cmd) => match hotfix_cmd.command {
                HotfixSubcommand::Finish { version } => {
                    assert_eq!(version, Some("my-hotfix".to_string()));
                }
                _ => panic!("Expected Hotfix::Finish, but got something else."),
            },
            _ => panic!("Expected Command::Hotfix, but got something else."),
        }
    }

    #[test]
    fn test_missing_required_argument_fails() {
        let args = vec!["flo", "feature", "start"];
        let cli = Cli::try_parse_from(args);
        assert!(
            cli.is_err(),
            "Parsing should fail when a required argument is missing."
        );
    }

    #[test]
    fn test_global_non_interactive_flag() {
        let args = vec!["flo", "--non-interactive", "status"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(cli.non_interactive);
    }

    #[test]
    fn test_global_continue_operation_flag() {
        let args = vec!["flo", "--continue", "feature", "finish"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(cli.continue_operation);
    }

    #[test]
    fn test_plugins_list_command() {
        let args = vec!["flo", "plugins", "list"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Command::Plugins(plugin_cmd) => {
                assert!(matches!(plugin_cmd, PluginCommand::List));
            }
            _ => panic!("Expected Plugins::List, but got something else."),
        }
    }

    #[test]
    fn test_config_command() {
        let args = vec!["flo", "config", "debug"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Command::Config(config_cmd) => {
                assert!(matches!(config_cmd, ConfigCommand::Debug));
            }
            _ => panic!("Expected ConfigCommand::Debug, but got something else."),
        }
    }
}
