use tracing::{info, debug};
use tracing_subscriber;

mod context;
mod commands;
mod completion;
mod config_tree;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::sync::{Arc, Mutex};

use context::CliContext;
use completion::ContextualCompleter;
use rustyline::history::FileHistory;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let context = Arc::new(Mutex::new(CliContext::UserExec));
    let completer = ContextualCompleter {
        context: context.clone(),
    };

    //let mut rl = Editor::new();
    let mut rl = Editor::<ContextualCompleter, FileHistory>::new().expect("Failed to create editor");
    rl.set_helper(Some(completer));
    rl.load_history("history.txt").ok();

    info!("CLI started");

    loop {
        let prompt = {
            let guard = context.lock().unwrap();
            guard.prompt()
        };

        match rl.readline(&prompt) {
            Ok(line) => {
                let line: &str = line.trim();
                if line.is_empty() {
                    continue;
                }
                rl.add_history_entry(line.to_owned());
                debug!("User input: {}", line);

                let tokens: Vec<&str> = line.split_whitespace().collect();

                let mut ctx_guard = context.lock().unwrap();
                match commands::dispatch(tokens, &mut *ctx_guard) {
                    Ok(continue_running) => {
                        if !continue_running {
                            break;
                        }
                    }
                    Err(e) => print!("{}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
            }
            Err(ReadlineError::Eof) => {
                println!("\nexit");
                break;
            }
            Err(err) => {
                eprintln!("Readline error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history("history.txt").ok();
    info!("CLI exited");
}
