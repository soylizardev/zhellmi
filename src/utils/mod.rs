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

// Podrás añadir aquí funciones futuras como:
// - logs de errores en archivos específicos de ZAMI.
// - Validadores de permisos de archivos.
// - Parsers de tiempo para el uptime.
