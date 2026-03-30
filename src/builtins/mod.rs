pub mod cd;
pub mod help;
pub mod stats;

use std::io::{self, Write};

/// Esta función centraliza el chequeo de comandos internos.
/// Devuelve true si el comando era un builtin y se ejecutó.
pub fn handle_builtin(cmd: &str, args: &[&str]) -> bool {
    match cmd {
        "cd" => {
            cd::run(args);
            true
        }
        "ls" => {
            // Nota: Mantenemos ls aquí como builtin para aplicar tus colores ZAMI
            // de directorios sin depender de los flags de BusyBox.
            stats::builtin_ls(args); 
            true
        }
        "pwd" => {
            if let Ok(cwd) = std::env::current_dir() {
                println!("{}", cwd.display());
            }
            true
        }
        "clear" => {
            print!("\x1b[2J\x1b[H");
            let _ = io::stdout().flush();
            true
        }
        "zhelp" | "help" => {
            help::run();
            true
        }
        "zstat" => {
            stats::run_diagnostics();
            true
        }
        "zsync" => {
            stats::run_sync();
            true
        }
        "zreboot" => {
            stats::run_reboot();
            true
        }
        _ => false, // No es un comando interno, pasar al executor
    }
}
