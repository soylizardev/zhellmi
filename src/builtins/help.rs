use crate::ui::colors::{BOLD, RESET};

/// Imprime el manual de auxilio de Zhellmi en la terminal
pub fn run() {
    println!("\n{}🛡️ MANUAL DE AUXILIO ZHELLMI (Z-SERIES){}", BOLD, RESET);
    
    println!("\n{}Identidad y Navegación:{}", BOLD, RESET);
    println!("  ls [ruta]   - Lista contenido (verde = directorio)");
    println!("  cd <ruta>   - Cambia el directorio (soporta ~)");
    println!("  pwd         - Muestra la ruta absoluta");
    println!("  clear       - Limpia la pantalla por completo");

    println!("\n{}Diagnóstico y Supervivencia:{}", BOLD, RESET);
    println!("  zstat       - Salud en tiempo real: RAM y Uptime");
    println!("  zhelp       - Muestra este manual de auxilio");
    println!("  zsync       - Fuerza volcado de buffers al disco");
    println!("  zreboot     - Orden inmediata de reinicio al hardware");

    println!("\n{}Control de Sesión:{}", BOLD, RESET);
    println!("  exit        - Cierra Zhellmi y guarda el historial\n");
}
