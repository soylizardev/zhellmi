use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::hint::HistoryHinter;
use rustyline::{Completer, Editor, Helper, Highlighter, Hinter, Result, Validator};
use std::io::{self, Write};

// --- IMPORTACIÓN DE MÓDULOS ---
mod core;
mod builtins;
mod ui;
mod utils;

use crate::core::executor::execute_command;
use crate::core::parser::parse_input; // Importamos tu parser modular
use crate::ui::prompt::generate_prompt;
use crate::ui::colors::{BOLD, RESET};

// --- HELPER PARA AUTOCOMPLETADO (TAB) ---
#[derive(Helper, Completer, Hinter, Highlighter, Validator)]
struct ZhellmiHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter, 
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
}

fn main() -> Result<()> {
    // 1. Configuración del Helper y Rustyline
    let h = ZhellmiHelper {
        completer: FilenameCompleter::new(),
        hinter: HistoryHinter::new(),
    };

    let config = rustyline::Config::builder().build();
    let mut rl: Editor<ZhellmiHelper, rustyline::history::DefaultHistory> =
        Editor::with_config(config)?;
    rl.set_helper(Some(h));

    // 2. Cargar historial
    let _ = rl.load_history("history.txt");

    // 3. Bienvenida Z-Series
    println!("🛡️ {}Zhellmi 0.0.3{} - developed for ZAMI", BOLD, RESET);

    // 4. LOOP PRINCIPAL
    loop {
        let prompt = generate_prompt();

        match rl.readline(&prompt) {
            Ok(line) => {
                let input = line.trim();
                
                // Ignorar vacíos o comentarios
                if input.is_empty() || input.starts_with('#') {
                    continue;
                }

                // Guardar en historial
                let _ = rl.add_history_entry(input);
                
                // --- PROCESAR ENTRADA CON EL PARSER MODULAR ---
                if let Some((cmd, args)) = parse_input(input) {
                    
                    // Manejo de comandos críticos del loop
                    if cmd == "exit" {
                        let _ = rl.save_history("history.txt");
                        break;
                    }

                    if cmd == "clear" {
                        print!("\x1b[2J\x1b[H");
                        let _ = io::stdout().flush();
                        continue;
                    }

                    // Enviamos al ejecutor (él decidirá si es Builtin o Binario de la Isla)
                    execute_command(cmd, &args);
                }
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
