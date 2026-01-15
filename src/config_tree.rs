use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
#[derive(PartialEq)]
pub struct InterfaceConfig {
    pub description: Option<String>,
    pub ip_address: Option<String>,
    pub shutdown: bool,
}

#[derive(Debug, Default, Clone)]
#[derive(PartialEq)]
pub struct GlobalConfig {
    pub hostname: String,
    pub interfaces: HashMap<String, InterfaceConfig>,
}

impl GlobalConfig {
    pub fn new() -> Self {
        Self {
            hostname: "Router".to_string(),
            ..Default::default()
        }
    }

    pub fn to_running_config(&self) -> String {
        let mut lines = vec![format!("hostname {}", self.hostname)];

        for (name, iface) in &self.interfaces {
            lines.push(format!("interface {}", name));
            if let Some(desc) = &iface.description {
                lines.push(format!(" description {}", desc));
            }
            if let Some(ip) = &iface.ip_address {
                lines.push(format!(" ip address {}", ip));
            }
            if iface.shutdown {
                lines.push(" shutdown".to_string());
            }
        }

        lines.join("\n") + "\n"
    }
}
