use std::env;
use std::path::PathBuf;

// Expande el símbolo '~' al inicio de una ruta por el directorio HOME del usuario.
// Si no se encuentra la variable HOME, por defecto usa la raíz '/'.
pub fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with('~') {
        let home = env::var("HOME").unwrap_or_else(|_| "/".into());
        // Reemplaza el primer '~' con la ruta del HOME
        PathBuf::from(path.replacen('~', &home, 1))
    } else {
        PathBuf::from(path)
    }
}

pub fn expand_env_var(word: &str) -> String {
    if word.starts_with('$') {
        // Le quitamos el '$' inicial para buscar el nombre real
        let var_name = &word[1..];

        // Buscamos la variable en el sistema. Si no existe, devolvemos vacío.
        env::var(var_name).unwrap_or_else(|_| "".to_string())
    } else {
        // Si no es una variable, devolvemos la palabra tal cual
        word.to_string()
    }
}
