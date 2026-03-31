use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::hint::HistoryHinter;
use rustyline::{Completer, Editor, Helper, Highlighter, Hinter, Result, Validator};

// --- IMPORTACIÓN DE MÓDULOS ---
mod builtins;
mod core;
mod ui;
mod utils;

use crate::core::parser::execute_pipeline; // Importamos el pipeline inteligente
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

    println!("🛡️ {}Zhellmi 0.9.0{} - developed for ZAMI", BOLD, RESET);

    loop {
        let prompt = generate_prompt();

        match rl.readline(&prompt) {
            Ok(line) => {
                let input = line.trim();

                if input.is_empty() || input.starts_with('#') {
                    continue;
                }

                let _ = rl.add_history_entry(input);

                // Mandamos toda la línea al cerebro lógico
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
