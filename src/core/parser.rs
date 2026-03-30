/// Toma la línea cruda de la terminal y devuelve un Option con (comando, argumentos)
/// Si la línea es un comentario o está vacía, devuelve None.
pub fn parse_input(line: &str) -> Option<(&str, Vec<&str>)> {
    let trimmed = line.trim();

    // 1. Regla de Oro: Ignorar vacíos y comentarios
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    // 2. Segmentación: Separamos por espacios en blanco
    let parts: Vec<&str> = trimmed.split_whitespace().collect();

    // 3. Extracción: El primer elemento es el comando, el resto son argumentos
    if parts.is_empty() {
        return None;
    }

    let command = parts[0];
    let args = parts[1..].to_vec();

    Some((command, args))
}
