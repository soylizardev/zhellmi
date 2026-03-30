use crate::utils::expand_env_var;

pub fn parse_input(line: &str) -> Option<(String, Vec<String>)> {
    let mut parts = line.split_whitespace();

    // Obtenemos el comando principal y lo expandimos
    // (por si alguien escribe `$COMANDO` en lugar de un nombre directo)
    let raw_cmd = parts.next()?;
    let cmd = expand_env_var(raw_cmd);

    if cmd.is_empty() {
        return None; // Si la variable estaba vacía y no hay comando, abortamos
    }

    // Mapeamos el resto de los argumentos pasándolos por el expansor
    let args: Vec<String> = parts.map(expand_env_var).collect();

    Some((cmd, args))
}
