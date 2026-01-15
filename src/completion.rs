use crate::context::CliContext;
use std::sync::{Arc, Mutex};
use rustyline::completion::Completer;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::highlight::Highlighter;
use rustyline::Helper;
use rustyline::Result as ReadlineResult;
use rustyline::Context;

use crate::commands::{
    user_exec, priv_exec, global_config, interface_config
};

pub struct ContextualCompleter {
    pub context: Arc<Mutex<CliContext>>,
}

impl Completer for ContextualCompleter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> ReadlineResult<(usize, Vec<Self::Candidate>)> {
        let start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let prefix = &line[start..pos];

        let current_ctx = {
            let guard = self.context.lock().unwrap();
            guard.clone()
        };

        let candidates = match &current_ctx {
            CliContext::UserExec => user_exec::available_commands(),
            CliContext::PrivilegedExec => priv_exec::available_commands(),
            CliContext::GlobalConfig { .. } => global_config::available_commands(),
            CliContext::InterfaceConfig { .. } => interface_config::available_commands(),
        };

        let matches: Vec<String> = candidates
            .into_iter()
            .filter(|cmd| cmd.starts_with(prefix))
            .map(|s| s.to_string())
            .collect();

        Ok((start, matches))
    }
}

// Implement other required traits with default behavior
impl Hinter for ContextualCompleter {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Highlighter for ContextualCompleter {
    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        std::borrow::Cow::Borrowed(hint)
    }

    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
        std::borrow::Cow::Borrowed(line)
    }
}

impl Validator for ContextualCompleter {
    fn validate(&self, _ctx: &mut rustyline::validate::ValidationContext) -> rustyline::Result<rustyline::validate::ValidationResult> {
        Ok(rustyline::validate::ValidationResult::Valid(None))
    }
}

// Now it implements Helper!
impl Helper for ContextualCompleter {}
