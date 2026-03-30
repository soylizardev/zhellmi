use std::env;
use crate::utils::expand_tilde;

/// Ejecuta la lógica del comando 'cd'
pub fn run(args: &[&str]) {
    // Si no hay argumentos, intentamos ir al HOME.
    // Si no hay HOME definido, vamos a la raíz '/' de la isla.
    let target_str = args
        .get(0)
        .map(|s| s.to_string())
        .unwrap_or_else(|| env::var("HOME").unwrap_or_else(|_| "/".into()));

    // Usamos nuestra función centralizada para entender el símbolo '~'
    let target_path = expand_tilde(&target_str);

    // Intentamos el cambio de directorio
    if let Err(e) = env::set_current_dir(&target_path) {
        eprintln!("zhellmi: cd: {}: {}", target_str, e);
    }
}
