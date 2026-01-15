use crate::CliContext;

pub fn handle(tokens: Vec<&str>, context: &mut CliContext) -> Result<bool, String> {
    if tokens.is_empty() {
        return Ok(true);
    }

    match tokens[0] {
        "interface" => {
            if let Some(name) = tokens.get(1) {
                let interface_name = name.to_string();
                let config = context.get_global_config_mut().unwrap();
                config.interfaces.entry(interface_name.clone()).or_default();
                *context = CliContext::InterfaceConfig {
                    interface_name,
                    config: config.clone(),
                };
                println!();
                Ok(true)
            } else {
                Err("% Incomplete command.\n".to_string())
            }
        }
        "exit" => {
            if let Some(parent) = context.parent() {
                *context = parent;
            }
            Ok(true)
        }
        "end" => {
            // Don't move out of context; just reset to PrivExec
            *context = CliContext::PrivilegedExec;
            Ok(true)
        }
        _ => Err("% Invalid input detected at '^' marker.\n".to_string()),
    }
}

pub fn help() -> String {
    r#"Global configuration commands:
  interface <name>  Select an interface to configure
  exit              Exit current mode
  end               Return to privileged EXEC
  help              Interactive help
  ?                 List available commands"#
        .to_string()
}

pub fn available_commands() -> Vec<&'static str> {
    vec!["interface", "exit", "end", "help", "?"]
}
