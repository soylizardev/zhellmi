use std::env;

pub fn init_zami_environment() {
    // 1. LIMPIEZA: Quitamos variables de Fedora que podrían confundir a los binarios de ZAMI
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

    // 2. DETECCIÓN DE PODER: ¿Quién está ejecutando Zhellmi?
    let uid = unsafe { libc::getuid() };

    if uid == 0 {
        // SI ERES ROOT: Puedes usar el HOME de root legalmente
        unsafe {
            env::set_var("USER", "root");
            env::set_var("HOME", "/root");
        }
    } else {
        // SI ERES USUARIO NORMAL: No podemos usar /root porque el Kernel nos bloqueará
        unsafe {
            env::set_var("USER", "zami_user");
        }
        // Mantenemos el HOME real de Fedora por ahora para que 'cd ~' funcione.
        // Esto evita el Error 13 porque lizar-dev SÍ tiene permiso en su propio HOME.
        if let Ok(real_home) = env::var("HOME") {
            unsafe {
                env::set_var("HOME", real_home);
            }
        }
    }

    // 3. AISLAMIENTO TÉCNICO: Esto es lo que hace a ZAMI independiente
    // Forzamos a que los programas busquen librerías y binarios SOLO en la isla
    unsafe {
        env::set_var("LD_LIBRARY_PATH", "/usr/lib:/lib:/tools/lib");
        env::set_var("PATH", "/bin:/usr/bin:/sbin:/tools/bin");
    }
}
