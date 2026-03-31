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

    let zami_root = "/mnt/zami";
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

    for (i, c) in commands.iter().enumerate() {
        let parts: Vec<&str> = c.split_whitespace().collect();
        if parts.is_empty() { return false; }
        
        let cmd = parts[0];
        let args = &parts[1..];
        let is_last = i == commands.len() - 1;

        let stdout_type = if is_last { Stdio::inherit() } else { Stdio::piped() };

        let child = Command::new(cmd) // Nota: Aquí podrías usar la lógica de resolved_path para ser más estricto
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
    let busybox_path = format!("{}/usr/bin/busybox", zami_root);
    if !Path::new(&busybox_path).exists() { return false; }

    let mut command = Command::new(&busybox_path);
    let mut full_args = vec![cmd];
    full_args.extend(args);
    command.args(&full_args).envs(env::vars());

    if let Some(file_path) = output_file {
        if let Ok(file) = File::create(file_path) {
            command.stdout(Stdio::from(file));
        }
    }

    command.status().map(|s| s.success()).unwrap_or(false)
}
