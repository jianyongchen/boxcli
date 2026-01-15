use crate::context::CliContext;

pub fn handle(tokens: Vec<&str>, context: &mut CliContext) -> Result<bool, String> {
    if tokens.is_empty() {
        return Ok(true);
    }

    match tokens[0] {
        "enable" => {
            *context = CliContext::PrivilegedExec;
            println!();
            Ok(true)
        }
        "exit" | "quit" => Ok(false),
        _ => Err("% Invalid input detected at '^' marker.\n".to_string()),
    }
}

pub fn help() -> String {
    r#"User EXEC commands:
  enable    Enter privileged EXEC mode
  exit      Exit from the EXEC
  quit      Quit the CLI
  help      Description of the interactive help system
  ?         Provide list of commands"#
        .to_string()
}

pub fn available_commands() -> Vec<&'static str> {
    vec!["enable", "exit", "quit", "help", "?"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enable() {
        let mut ctx = CliContext::UserExec;
        let result = handle(vec!["enable"], &mut ctx);
        assert!(result.is_ok());
        assert_eq!(ctx, CliContext::PrivilegedExec);
    }
}
