use crate::context::CliContext;

pub fn handle(tokens: Vec<&str>, context: &mut CliContext) -> Result<bool, String> {
    if tokens.is_empty() {
        return Ok(true);
    }

    let interface_name = match context {
        CliContext::InterfaceConfig { interface_name, .. } => interface_name.clone(),
        _ => return Err("% Not in interface config mode.\n".to_string()),
    };

    if let Some(config) = context.get_global_config_mut() {
        let iface = config.interfaces.get_mut(&interface_name).unwrap();

        match tokens[0] {
            "ip" if tokens.get(1) == Some(&"address") => {
                if let Some(addr) = tokens.get(2) {
                    iface.ip_address = Some(addr.to_string());
                    println!("IP address set to {}", addr);
                } else {
                    return Err("% Incomplete command.\n".to_string());
                }
            }
            "description" => {
                if let Some(desc) = tokens.get(1) {
                    iface.description = Some(desc.to_string());
                } else {
                    return Err("% Incomplete command.\n".to_string());
                }
            }
            "shutdown" => {
                iface.shutdown = true;
            }
            "no" => {
                if tokens.get(1) == Some(&"shutdown") {
                    iface.shutdown = false;
                }
                // extend as needed
            }
            "exit" => {
                let new_config = config.clone();
                *context = CliContext::GlobalConfig { config: new_config };
                return Ok(true);
            }
            "end" => {
                *context = CliContext::PrivilegedExec;
                return Ok(true);
            }
            _ => return Err("% Invalid input detected at '^' marker.\n".to_string()),
        }
    }

    Ok(true)
}

pub fn help() -> String {
    r#"Interface configuration commands:
  ip address <addr>   Set IP address
  description <text>  Set interface description
  shutdown            Shutdown interface
  no shutdown         Enable interface
  exit                Exit current mode
  end                 Return to privileged EXEC
  help                Interactive help
  ?                   List available commands"#
        .to_string()
}

pub fn available_commands() -> Vec<&'static str> {
    vec![
        "ip address",
        "description",
        "shutdown",
        "no shutdown",
        "exit",
        "end",
        "help",
        "?",
    ]
}
