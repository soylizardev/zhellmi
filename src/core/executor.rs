use crate::builtins::handle_builtin;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn execute_command(cmd: &str, args: &[&str], output_file: Option<&str>) -> bool {
    // 1. Builtins (solo si no hay redirección, para evitar conflictos)
    if handle_builtin(cmd, args, output_file) {
        return true;
    }

    // DETECCIÓN DE ENTORNO: 
    // Si /mnt/zami existe, estamos en Fedora. Si no, estamos en ZAMI real (raíz /).
    let zami_root = if Path::new("/mnt/zami").exists() {
        "/mnt/zami"
    } else {
        ""
    };

    let mut resolved_path: Option<String> = None;

    if cmd.starts_with('/') || cmd.starts_with('.') {
        resolved_path = Some(format!("{}{}", zami_root, cmd));
    } else {
        let path_var = env::var("PATH").unwrap_or_else(|_| "/bin:/usr/bin:/tools/bin".to_string());
        for p in path_var.split(':') {
            let mut full = PathBuf::from(zami_root);
            full.push(p.trim_start_matches('/'));
            full.push(cmd);
            if full.exists() && full.is_file() {
                resolved_path = Some(full.to_string_lossy().into_owned());
                break;
            }
        }
    }

    if let Some(path) = resolved_path {
        let mut command = Command::new(&path);
        command.args(args).envs(env::vars());

        // Manejo de Redirección >
        if let Some(file_path) = output_file {
            if let Ok(file) = File::create(file_path) {
                command.stdout(Stdio::from(file));
            }
        }

        match command.status() {
            Ok(s) => s.success(),
            Err(_) => false,
        }
    } else {
        intentar_busybox(cmd, args, zami_root, output_file)
    }
}

pub fn execute_pipe_command(cmd_str: &str) -> bool {
    let commands: Vec<&str> = cmd_str.split('|').collect();
    let mut prev_stdout = Stdio::inherit();

    // Detectar root para pipes también
    let zami_root = if Path::new("/mnt/zami").exists() { "/mnt/zami" } else { "" };

    for (i, c) in commands.iter().enumerate() {
        let parts: Vec<&str> = c.split_whitespace().collect();
        if parts.is_empty() { return false; }
        
        let cmd = parts[0];
        let args = &parts[1..];
        let is_last = i == commands.len() - 1;

        let stdout_type = if is_last { Stdio::inherit() } else { Stdio::piped() };

        // Intentamos resolver la ruta del comando para el pipe
        let mut full_cmd_path = cmd.to_string();
        if !cmd.starts_with('/') && !cmd.starts_with('.') {
             let path_var = env::var("PATH").unwrap_or_else(|_| "/bin:/usr/bin".to_string());
             for p in path_var.split(':') {
                let mut p_check = PathBuf::from(zami_root);
                p_check.push(p.trim_start_matches('/'));
                p_check.push(cmd);
                if p_check.exists() {
                    full_cmd_path = p_check.to_string_lossy().into_owned();
                    break;
                }
             }
        } else {
            full_cmd_path = format!("{}{}", zami_root, cmd);
        }

        let child = Command::new(full_cmd_path)
            .args(args)
            .stdin(prev_stdout)
            .stdout(stdout_type)
            .spawn();

        match child {
            Ok(mut output) => {
                if !is_last {
                    prev_stdout = Stdio::from(output.stdout.take().unwrap());
                } else {
                    return output.wait().map(|s| s.success()).unwrap_or(false);
                }
            }
            Err(_) => return false,
        }
    }
    true
}

fn intentar_busybox(cmd: &str, args: &[&str], zami_root: &str, output_file: Option<&str>) -> bool {
    // Intentamos las dos rutas posibles de busybox en ZAMI
    let paths = [
        format!("{}/bin/busybox", zami_root),
        format!("{}/usr/bin/busybox", zami_root),
    ];

    let mut final_path = None;
    for p in &paths {
        if Path::new(p).exists() {
            final_path = Some(p.clone());
            break;
        }
    }

    if let Some(path) = final_path {
        let mut command = Command::new(path);
        // BusyBox necesita el comando como primer argumento
        let mut full_args = vec![cmd];
        full_args.extend(args);
        command.args(&full_args).envs(env::vars());

        if let Some(file_path) = output_file {
            if let Ok(file) = File::create(file_path) {
                command.stdout(Stdio::from(file));
            }
        }

        command.status().map(|s| s.success()).unwrap_or(false)
    } else {
        false
    }
}
