use crate::builtins::handle_builtin;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn execute_command(cmd: &str, args: &[&str]) {
    // 0. CAPA CERO: Builtins internos (cd, exit, help, etc.)
    if handle_builtin(cmd, args) {
        return;
    }

    let zami_root = "/mnt/zami";
    let mut resolved_path: Option<String> = None;

    // 1. RESOLUCIÓN DE RUTA: ¿Dónde está el binario?
    if cmd.starts_with('/') || cmd.starts_with('.') {
        // Caso A: Ruta absoluta o relativa
        resolved_path = Some(format!("{}{}", zami_root, cmd));
    } else {
        // Caso B: Búsqueda dinámica en el $PATH
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

    // 2. CHECKPOINT DE SEGURIDAD (La Aduana)
    if let Some(path) = resolved_path {
        // --- FILTRO PROTECTOR DEL TOOLCHAIN ---
        // Si la ruta final contiene "/tools/", verificamos UID
        if path.contains("/tools/") {
            let uid = unsafe { libc::getuid() };
            if uid != 0 {
                eprintln!(
                    "Zm: 🛡️ Permiso denegado. El Toolchain es sagrado y solo root puede tocarlo."
                );
                return; // Bloqueo total: el comando no se ejecuta
            }
        }

        // Si pasó la aduana y el archivo existe, lo ejecutamos
        if Path::new(&path).exists() {
            run_process(&path, args);
            return;
        }
    }

    // 3. CAPA DE SIMBIOSIS (BusyBox)
    // Si no encontramos un binario nativo en la isla, recurrimos a BusyBox
    intentar_busybox(cmd, args, zami_root);
}

fn run_process(path: &str, args: &[&str]) {
    let status = Command::new(path).args(args).envs(env::vars()).status();

    match status {
        Ok(s) if !s.success() => {
            // Error interno del comando (ya impreso por el proceso hijo)
        }
        Err(e) => println!("Zm: error crítico al ejecutar {}: {}", path, e),
        _ => {}
    }
}

fn intentar_busybox(cmd: &str, args: &[&str], zami_root: &str) {
    let busybox_path = format!("{}/usr/bin/busybox", zami_root);

    // Verificamos si BusyBox existe antes de llamarlo
    if !Path::new(&busybox_path).exists() {
        eprintln!("zhellmi: comando no encontrado: '{}'", cmd);
        return;
    }

    let mut full_args = vec![cmd];
    full_args.extend(args);

    let status = Command::new(&busybox_path)
        .args(&full_args)
        .envs(env::vars())
        .status();

    if let Err(_) = status {
        eprintln!(
            "zhellmi: error al invocar la simbiosis con BusyBox para '{}'",
            cmd
        );
    }
}

