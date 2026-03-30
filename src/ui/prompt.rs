use std::env;
use std::fs;
use crate::core::env::get_formatted_path; // <--- Importamos la función que daba warning
use crate::ui::colors::*; // Asumiendo que aquí están tus constantes de colores y hex_to_bg_ansi

/// Genera el string completo del prompt con bloques de colores ANSI.
pub fn generate_prompt() -> String {
    // 1. Obtener datos del sistema
    let user = env::var("USER").unwrap_or_else(|_| "root".into());
    let hostname = fs::read_to_string("/proc/sys/kernel/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "zami".into());
    
    // 2. USAMOS LA FUNCIÓN DE CORE (Esto quita el warning en core/env.rs)
    let path_display = get_formatted_path();

    // 3. Determinar el símbolo según privilegios
    let uid = unsafe { libc::getuid() };
    let prompt_char = if uid == 0 { "#" } else { "$" };

    // 4. Construcción del Prompt con bloques Z-Series
    format!(
        "{}{} Zm {}{} {} {}{} {} {}{} {} {} {} ",
        hex_to_bg_ansi(ZM_HEX),   BLACK_BOLD,                    // Bloque ZAMI
        hex_to_bg_ansi(USR_HEX),  BLACK_BOLD, user,              // Bloque Usuario
        hex_to_bg_ansi(EQP_HEX),  BLACK_BOLD, hostname,          // Bloque Host
        hex_to_bg_ansi(RUTA_HEX), BLACK_BOLD, path_display,      // Bloque Ruta
        RESET, prompt_char
    )
}
