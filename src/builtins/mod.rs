pub mod cd;
pub mod help;
pub mod stats;

use std::ffi::CStr;
use std::io::{self, Write};
// Esta función centraliza el chequeo de comandos internos.
// Devuelve true si el comando era un builtin y se ejecutó.
//

fn builtin_whoami() {
    unsafe {
        let uid = libc::getuid();
        let pw = libc::getpwuid(uid);
        if !pw.is_null() {
            let name = CStr::from_ptr((*pw).pw_name).to_string_lossy();
            println!("{}", name);
        } else {
            println!("unknown");
        }
    }
}

pub fn builtin_zsudo(args: &[&str]) {
    if args.is_empty() {
        println!("zsudo: uso: zsudo <comando> [argumentos]");
        return;
    }

    let uid = unsafe { libc::getuid() };
    if uid != 0 {
        println!("zsudo: ❌ El usuario actual no tiene rango de Administrador ZAMI.");
        return;
    }

    // Si pasó la validación, extraemos el comando y sus argumentos
    let cmd = args[0];
    let cmd_args = &args[1..];

    // Llamamos al ejecutor principal para que procese el comando "con permisos"
    crate::core::executor::execute_command(cmd, cmd_args);
}

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
        "whoami" => {
            builtin_whoami();
            true // Retornamos true: "Yo me encargué de este comando"
        }
        "zsudo" => {
            builtin_zsudo(args);
            true
        }
        "zreboot" => {
            stats::run_reboot();
            true
        }
        _ => false, // No es un comando interno, pasar al executor
    }
}
