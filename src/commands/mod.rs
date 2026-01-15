pub mod user_exec;
pub mod priv_exec;
pub mod global_config;
pub mod interface_config;

use crate::context::CliContext;

pub fn dispatch(tokens: Vec<&str>, context: &mut CliContext) -> Result<bool, String> {
    if tokens.is_empty() {
        return Ok(true);
    }

    if tokens[0] == "help" || tokens[0] == "?" {
        print_help(context);
        return Ok(true);
    }

    match context {
        CliContext::UserExec => user_exec::handle(tokens, context),
        CliContext::PrivilegedExec => priv_exec::handle(tokens, context),
        CliContext::GlobalConfig { .. } => global_config::handle(tokens, context),
        CliContext::InterfaceConfig { .. } => interface_config::handle(tokens, context),
    }
}

fn print_help(context: &CliContext) {
    let help_text = match context {
        CliContext::UserExec => user_exec::help(),
        CliContext::PrivilegedExec => priv_exec::help(),
        CliContext::GlobalConfig { .. } => global_config::help(),
        CliContext::InterfaceConfig { .. } => interface_config::help(),
    };
    println!("{}", help_text);
}
