use crate::config_tree::GlobalConfig;

#[derive(Debug, Clone, PartialEq)]
pub enum CliContext {
    UserExec,
    PrivilegedExec,
    GlobalConfig {
        config: GlobalConfig,
    },
    InterfaceConfig {
        interface_name: String,
        config: GlobalConfig,
    },
}

impl CliContext {
    pub fn prompt(&self) -> String {
        match self {
            CliContext::UserExec => "Router> ".to_string(),
            CliContext::PrivilegedExec => "Router# ".to_string(),
            CliContext::GlobalConfig { .. } => "Router(config)# ".to_string(),
            CliContext::InterfaceConfig { interface_name, .. } => {
                format!("Router(config-if-{})# ", interface_name)
            }
        }
    }

    pub fn parent(&self) -> Option<CliContext> {
        match self {
            CliContext::UserExec => None,
            CliContext::PrivilegedExec => Some(CliContext::UserExec),
            CliContext::GlobalConfig { config } => {
                Some(CliContext::PrivilegedExec)
            }
            CliContext::InterfaceConfig { config, .. } => {
                Some(CliContext::GlobalConfig { config: config.clone() })
            }
        }
    }

    pub fn into_global_config(self) -> GlobalConfig {
        match self {
            CliContext::GlobalConfig { config } => config,
            CliContext::InterfaceConfig { config, .. } => config,
            _ => panic!("Not in config mode"),
        }
    }

    pub fn from_global_config(config: GlobalConfig) -> Self {
        CliContext::GlobalConfig { config }
    }

    pub fn get_global_config(&self) -> Option<&GlobalConfig> {
        match self {
            CliContext::GlobalConfig { config } => Some(config),
            CliContext::InterfaceConfig { config, .. } => Some(config),
            _ => None,
        }
    }

    pub fn get_global_config_mut(&mut self) -> Option<&mut GlobalConfig> {
        match self {
            CliContext::GlobalConfig { config } => Some(config),
            CliContext::InterfaceConfig { config, .. } => Some(config),
            _ => None,
        }
    }
}
