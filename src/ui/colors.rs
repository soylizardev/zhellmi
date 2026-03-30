// --- PALETA ZAMI (Tus colores exactos) ---
pub const ZM_HEX: &str = "#ab3428";     // Rojo ZAMI
pub const USR_HEX: &str = "#f49e4c";    // Naranja Usuario
pub const EQP_HEX: &str = "#f5ee9e";    // Crema Equipo/Host
pub const RUTA_HEX: &str = "#3b8ea5";   // Azul Ruta

// --- ESTILOS ANSI ---
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const BLACK_BOLD: &str = "\x1b[1;38;5;16m"; // Texto negro para bloques claros

/// Convierte Hex a color de FONDO (Código ANSI 48;2)
/// Usado para los bloques sólidos del prompt
pub fn hex_to_bg_ansi(hex: &str) -> String {
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(255);
    format!("\x1b[48;2;{};{};{}m", r, g, b)
}

/// Convierte Hex a color de TEXTO (Código ANSI 38;2)
/// Usado para el comando 'ls' y otros textos resaltados
pub fn hex_to_ansi(hex: &str) -> String {
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(255);
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}
