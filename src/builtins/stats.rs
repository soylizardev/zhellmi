use std::fs;
use std::io::{self, Write};
use crate::utils::expand_tilde;
use crate::ui::colors::{BOLD, RESET, EQP_HEX, hex_to_ansi};

/// Muestra estadísticas básicas de memoria y uptime (Z-SERIES)
pub fn run_diagnostics() {
    println!("\n{}--- DIAGNÓSTICO Z-SERIES ---{}", BOLD, RESET);
    
    if let Ok(mem) = fs::read_to_string("/proc/meminfo") {
        for line in mem.lines().take(3) {
            println!("  {}", line);
        }
    }
    
    if let Ok(uptime) = fs::read_to_string("/proc/uptime") {
        let secs = uptime.split_whitespace().next().unwrap_or("0");
        println!("  Uptime del sistema: {}s", secs);
    }
    println!();
}

/// Sincroniza buffers de disco (Syscall sync)
pub fn run_sync() {
    print!("Sincronizando buffers de disco...");
    let _ = io::stdout().flush();
    unsafe {
        libc::sync();
    }
    println!(" [ OK ]");
}

/// Envía señal de reinicio al Kernel
pub fn run_reboot() {
    println!("Enviando señal de reinicio al Kernel...");
    unsafe {
        libc::reboot(libc::LINUX_REBOOT_CMD_RESTART);
    }
}

/// Implementación nativa de ls con colores ZAMI
pub fn builtin_ls(args: &[&str]) {
    let target_str = args.get(0).unwrap_or(&".");
    let target_path = expand_tilde(target_str);

    match fs::read_dir(&target_path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let name = entry.file_name().into_string().unwrap_or_default();

                if name.starts_with('.') {
                    continue;
                }

                if let Ok(meta) = entry.metadata() {
                    if meta.is_dir() {
                        print!("{}{}/{}  ", hex_to_ansi(EQP_HEX), name, RESET);
                    } else {
                        print!("{}  ", name);
                    }
                }
            }
            println!();
        }
        Err(e) => eprintln!("ls: {}: {}", target_str, e),
    }
}
