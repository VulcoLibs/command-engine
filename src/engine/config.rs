#[derive(Clone, Debug)]
pub struct EngineConfig {
    pub help_caller: String,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            help_caller: "help".to_string()
        }
    }
}

unsafe impl Sync for EngineConfig {}
unsafe impl Send for EngineConfig {}