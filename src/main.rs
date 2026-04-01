use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::hint::HistoryHinter;
use rustyline::{Completer, Editor, Helper, Highlighter, Hinter, Result, Validator};
use std::io::{self, Write}; // <--- 1. Añadimos esto para el flush del cursor

// --- IMPORTACIÓN DE MÓDULOS ---
mod builtins;
mod core;
mod ui;
mod utils;

use crate::core::parser::execute_pipeline; 
use crate::ui::colors::{BOLD, RESET};
use crate::ui::prompt::generate_prompt;

// --- HELPER PARA AUTOCOMPLETADO (TAB) ---
#[derive(Helper, Completer, Hinter, Highlighter, Validator)]
struct ZhellmiHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
}

// 2. Esta función activa el cursor parpadeante enviando el código ANSI al Kernel
fn setup_terminal() {
    let mut stdout = io::stdout();
    // \x1b[?25h -> Muestra el cursor
    // \x1b[5 q  -> Fuerza el estilo "guion parpadeante" (blink underline)
    print!("\x1b[?25h\x1b[5 q"); 
    let _ = stdout.flush();
}

fn main() -> Result<()> {
    crate::core::environ::init_zami_environment();
    let h = ZhellmiHelper {
        completer: FilenameCompleter::new(),
        hinter: HistoryHinter::new(),
    };

    let config = rustyline::Config::builder().build();
    let mut rl: Editor<ZhellmiHelper, rustyline::history::DefaultHistory> =
        Editor::with_config(config)?;
    rl.set_helper(Some(h));

    let _ = rl.load_history("history.txt");

    println!("🛡️ {}Zhellmi 1.0.1{} - developed for ZAMI", BOLD, RESET);

    setup_terminal(); // <--- 3. Lo llamamos una sola vez aquí

    loop {
        let prompt = generate_prompt();

        match rl.readline(&prompt) {
            Ok(line) => {
                let input = line.trim();

                if input.is_empty() || input.starts_with('#') {
                    continue;
                }

                let _ = rl.add_history_entry(input);

                execute_pipeline(input);
            }
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(ReadlineError::Eof) => {
                let _ = rl.save_history("history.txt");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
