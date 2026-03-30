use std::env;
use std::ffi::CStr;

pub fn init_zami_environment() {
    // 1. Limpieza de Fedora
    let black_list = [
        "SESSION_MANAGER",
        "XDG_RUNTIME_DIR",
        "WAYLAND_DISPLAY",
        "DISPLAY",
    ];
    for var in black_list {
        unsafe {
            env::remove_var(var);
        }
    }

    // 2. OBTENER IDENTIDAD REAL DESDE EL KERNEL
    let mut real_user_name = "zami_user".to_string(); // Fallback
    unsafe {
        let uid = libc::getuid();
        let pw = libc::getpwuid(uid);
        if !pw.is_null() {
            // Convertimos el puntero de C a un String de Rust
            real_user_name = CStr::from_ptr((*pw).pw_name).to_string_lossy().into_owned();
        }
    }

    // 3. CONFIGURAR VARIABLES BASADAS EN LA REALIDAD
    unsafe {
        env::set_var("USER", &real_user_name);
    }

    // Si es root (UID 0), el HOME es /root. Si no, usamos el HOME del sistema.
    let uid = unsafe { libc::getuid() };
    if uid == 0 {
        unsafe {
            env::set_var("HOME", "/root");
        }
    } else if let Ok(h) = env::var("HOME") {
        unsafe {
            env::set_var("HOME", h);
        }
    }

    // 4. AISLAMIENTO TÉCNICO (El ADN de ZAMI)
    unsafe {
        env::set_var("LD_LIBRARY_PATH", "/usr/lib:/lib:/tools/lib");
        env::set_var("PATH", "/bin:/usr/bin:/sbin:/tools/bin");
    }
}

