use std::env;
use std::path::PathBuf;
use std::process::Command;
// ¡ESTA LÍNEA ES LA MAGIA QUE CONECTA TODO!
use crate::builtins::handle_builtin; 

/// Intenta ejecutar un comando buscando primero en Zhellmi, luego en la isla ZAMI y por último en BusyBox
pub fn execute_command(cmd: &str, args: &[&str]) {
    // 0. CAPA CERO: ¿Es un comando interno de Zhellmi? (ls, cd, zhelp, clear, etc.)
    if handle_builtin(cmd, args) {
        return; // Si es interno, se ejecuta y terminamos aquí. No buscamos en el disco.
    }

    // 1. PRIMERA CAPA: Definimos las rutas sagradas de la Isla ZAMI
    let zami_root = "/mnt/zami";
    let search_paths = [
        "/bin",
        "/usr/bin",
        "/usr/local/bin",
        "/sbin",
        "/tools/bin", // Tu Toolchain de construcción
    ];

    let mut binary_path = None;

    // 2. BUSCADOR DINÁMICO: ¿Existe el binario en la isla?
    for p in search_paths {
        let mut full_path = PathBuf::from(zami_root);
        full_path.push(p.trim_start_matches('/'));
        full_path.push(cmd);

        if full_path.exists() && full_path.is_file() {
            binary_path = Some(full_path);
            break;
        }
    }

    // 3. TERCERA CAPA: LÓGICA DE DESPACHO EXTERNO
    match binary_path {
        Some(path) => {
            // Caso A: Binario nativo de ZAMI
            let status = Command::new(path)
                .args(args)
                .envs(env::vars())
                .status();

            if let Err(_e) = status {
                eprintln!("zhellmi: error al ejecutar binario real: {}", _e);
            }
        }
        None => {
            // Caso B: Simbiosis con BusyBox
            let busybox_path = format!("{}/usr/bin/busybox", zami_root);
            
            let mut full_args = vec![cmd];
            full_args.extend(args);

            let status = Command::new(&busybox_path)
                .args(&full_args)
                .envs(env::vars())
                .status();

            if let Err(_e) = status {
                eprintln!("zhellmi: comando no encontrado: '{}'", cmd);
            }
        }
    }
}
