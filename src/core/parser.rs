use std::io::{self, Write};
use crate::core::executor;
use crate::utils::expand_env_var;

pub fn execute_pipeline(line: &str) {
    let sequences: Vec<&str> = line.split(';').collect();

    for seq in sequences {
        let seq = seq.trim();
        if seq.is_empty() { continue; }

        let and_cmds: Vec<&str> = seq.split("&&").collect();
        let mut success = true;

        for cmd_str in and_cmds {
            let cmd_str = cmd_str.trim();
            if cmd_str.is_empty() || !success { break; }

            // 1. Prioridad: Pipes
            if cmd_str.contains('|') {
                success = executor::execute_pipe_command(cmd_str);
                continue;
            }

            // 2. Redirección >
            let parts: Vec<&str> = cmd_str.split('>').collect();
            let actual_cmd_part = parts[0].trim();
            let mut output_file = None;
            if parts.len() > 1 {
                output_file = Some(parts[1].trim());
            }

            let mut words = actual_cmd_part.split_whitespace();
            let raw_cmd = match words.next() {
                Some(c) => c,
                None => continue,
            };

            let cmd = expand_env_var(raw_cmd);
            let args: Vec<String> = words.map(expand_env_var).collect();
            let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

            if cmd == "exit" { std::process::exit(0); }
            if cmd == "clear" {
                print!("\x1b[2J\x1b[H");
                let _ = io::stdout().flush();
                success = true;
                continue;
            }

            success = executor::execute_command(&cmd, &args_ref, output_file);
        }
    }
}
