pub mod cd;
pub mod help;
pub mod stats;

use std::ffi::CStr;
use std::io::{self, Write};

// Esta función centraliza el chequeo de comandos internos.
// Devuelve true si el comando era un builtin y se ejecutó.

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

pub fn builtin_zsudo(args: &[&str], output_file: Option<&str>) {
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
    crate::core::executor::execute_command(cmd, cmd_args, output_file);
}

pub fn builtin_export(args: &[&str]) {
    if args.is_empty() {
        // Si solo escriben 'export', mostramos todas las variables (como una shell real)
        for (key, val) in std::env::vars() {
            println!("declare -x {}=\"{}\"", key, val);
        }
        return;
    }

    for arg in args {
        // Buscamos el formato CLAVE=VALOR
        if let Some((key, value)) = arg.split_once('=') {
            unsafe {
                std::env::set_var(key, value);
            }
        } else {
            eprintln!("Zm: export: formato incorrecto. Uso: export CLAVE=VALOR");
        }
    }
}

pub fn handle_builtin(cmd: &str, args: &[&str], output_file: Option<&str>) -> bool {
    match cmd {
        "cd" => {
            cd::run(args);
            true
        }
        "ls" => {
            // Si el primer argumento empieza con '-', delegamos a BusyBox (retornamos false)
            if let Some(arg) = args.get(0) {
                if arg.starts_with('-') {
                    return false; 
                }
            }
            // Si no hay flags, usamos tu ls con colores ZAMI
            stats::builtin_ls(args);
            true
        }
        "export" => {
            builtin_export(args);
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
            builtin_zsudo(args, output_file); // Le pasamos el archivo
            true
        }
        "zreboot" => {
            stats::run_reboot();
            true
        }
        _ => false, // No es un comando interno, pasar al executor
    }
}
