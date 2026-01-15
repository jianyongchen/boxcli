use crate::context::CliContext;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "", no_binary_name = true, disable_help_flag = true)]
struct PrivExecInput {
    #[command(subcommand)]
    command: Option<PrivExecCommand>,
}

#[derive(Subcommand, Debug)]
enum PrivExecCommand {
    Disable,
    #[command(alias = "conf")]
    Configure {
        #[arg(value_enum)]
        terminal: TerminalMode,
    },
    Show {
        #[arg(value_enum)]
        target: Option<ShowTarget>,
    },
    Exit,
    End,
    //Help,
    //QuestionMark,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum TerminalMode {
    Terminal,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum ShowTarget {
    #[value(alias = "run")]
    RunningConfig,
}

pub fn handle(tokens: Vec<&str>, context: &mut CliContext) -> Result<bool, String> {
    // Handle alias: "conf t" → "configure terminal"
    let tokens: Vec<String> = tokens
        .into_iter()
        .map(|s| {
            if s == "conf" {
                "configure".to_string()
            } else if s == "t" {
                "terminal".to_string()
            } else {
                s.to_string()
            }
        })
        .collect();
    let tokens_ref: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();

    let mut argv:Vec<&str> = vec![];
    argv.extend(tokens_ref.iter());

    //println!("{:?}", argv);
    let input = match PrivExecInput::try_parse_from(argv) {
        Ok(input) => input,
        Err(e) => {
            return Err(format!("{}\n", e));
        }
    };

    match input.command {
        Some(PrivExecCommand::Disable) => {
            *context = CliContext::UserExec;
            Ok(true)
        }
        Some(PrivExecCommand::Configure { terminal: TerminalMode::Terminal }) => {
            let config = context.get_global_config().cloned().unwrap_or_default();
            *context = CliContext::GlobalConfig { config };
            println!();
            Ok(true)
        }
        Some(PrivExecCommand::Show { target: Some(ShowTarget::RunningConfig) }) => {
            if let Some(cfg) = context.get_global_config() {
                print!("{}", cfg.to_running_config());
            } else {
                println!("No configuration.");
            }
            Ok(true)
        }
        Some(PrivExecCommand::Show { target: None }) => {
            println!("Available: running-config");
            Ok(true)
        }
        Some(PrivExecCommand::Exit) => {
            *context = CliContext::UserExec;
            Ok(true)
        }
        Some(PrivExecCommand::End) => {
            *context = CliContext::PrivilegedExec;
            Ok(true)
        }
        //Some(PrivExecCommand::Help | PrivExecCommand::QuestionMark) => {
        //    Ok(true) // handled upstream
        //}
        None => Ok(true),
    }
}

pub fn help() -> String {
    r#"Privileged EXEC commands:
  configure terminal  Enter configuration mode (alias: conf t)
  disable             Turn off privileged commands
  show running-config Show current configuration (alias: show run)
  exit                Exit from the EXEC
  end                 Exit to privileged EXEC mode
  help                Interactive help
  ?                   List available commands"#
        .to_string()
}

pub fn available_commands() -> Vec<&'static str> {
    vec![
        "configure terminal",
        "conf t",
        "disable",
        "show running-config",
        "show run",
        "exit",
        "end",
        "help",
        "?",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configure_terminal() {
        let mut ctx = CliContext::PrivilegedExec;
        let result = handle(vec!["configure", "terminal"], &mut ctx);
        assert!(result.is_ok());
        matches!(ctx, CliContext::GlobalConfig { .. });
    }

    #[test]
    fn test_alias_conf_t() {
        let mut ctx = CliContext::PrivilegedExec;
        let result = handle(vec!["conf", "t"], &mut ctx);
        assert!(result.is_ok());
        matches!(ctx, CliContext::GlobalConfig { .. });
    }
}
