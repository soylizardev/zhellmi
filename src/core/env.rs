use std::env;
use std::path::PathBuf;

// Devuelve la ruta actual formateada para el prompt (sustituyendo HOME por ~)
pub fn get_formatted_path() -> String {
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
    let home = env::var("HOME").unwrap_or_else(|_| "".into());
    let path_str = cwd.to_str().unwrap_or("/");

    if !home.is_empty() && path_str.starts_with(&home) {
        path_str.replace(&home, "~")
    } else {
        path_str.to_string()
    }
}
// (Futuro) Aquí podrías añadir funciones para cargar un archivo .zami_env
// o para gestionar el PATH específico de la isla.
