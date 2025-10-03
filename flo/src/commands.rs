/// This enum represents the pure, semantic intent of the user.
pub enum AppCommand {
    FeatureStart { name: String },
    FeatureFinish { name: String },
    ReleaseStart { version: Option<String> },
    ReleaseFinish { version: Option<String> },
    HotfixStart { version: Option<String> },
    HotfixFinish { version: Option<String> },
    Init {},
    Status {},
    PluginList {},
    Config {},
    Continue {},
}
