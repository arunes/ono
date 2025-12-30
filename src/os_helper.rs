use std::{env, path::PathBuf};

#[derive(Debug)]
enum OS {
    Linux,
    Windows,
    Mac,
}

#[derive(Debug)]
enum Shell {
    Bash,
    Zsh,
    Fish,
    Pwrshl,
    Cmd,
    Unknown,
}

static DATA_DIR: &str = "ono";

fn get_os() -> OS {
    if cfg!(target_os = "windows") {
        OS::Windows
    } else if cfg!(target_os = "macos") {
        OS::Mac
    } else {
        OS::Linux
    }
}

pub fn get_data_dir() -> String {
    dirs::data_dir()
        .unwrap_or(PathBuf::from("./"))
        .join(DATA_DIR)
        .into_os_string()
        .into_string()
        .unwrap_or(format!("./{}", DATA_DIR))
}

pub fn get_editor() -> String {
    log::info!("Getting user's default editor");
    match get_os() {
        OS::Linux | OS::Mac => env::var("VISUAL")
            .or_else(|_| env::var("EDITOR"))
            .unwrap_or_else(|_| "vi".to_string()),
        OS::Windows => env::var("EDITOR").unwrap_or_else(|_| "notepad.exe".to_string()),
    }
}

fn get_shell() -> Shell {
    log::info!("Getting user's default shell");
    let shell_str = match get_os() {
        OS::Linux => env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string()),
        OS::Mac => env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string()),
        OS::Windows => env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string()),
    };
    log::info!("User's shell: {shell_str}");

    let lower = shell_str.to_lowercase();
    if lower.contains("zsh") {
        Shell::Zsh
    } else if lower.contains("fish") {
        Shell::Fish
    } else if lower.contains("bash") {
        Shell::Bash
    } else if lower.contains("pwsh") || lower.contains("powershell") {
        Shell::Pwrshl
    } else if lower.contains("cmd.exe") {
        Shell::Cmd
    } else {
        log::warn!("Unknown shell: {lower}");
        Shell::Unknown
    }
}

pub fn get_history_file() -> Option<String> {
    log::info!("Getting user's shell history file.");
    // 1. If user has HISTFILE set, prioritize that
    if let Ok(env_path) = env::var("HISTFILE") {
        return Some(env_path);
    }

    // 2. Determine shell type and map to history file
    let shell = match get_shell() {
        Shell::Zsh => {
            let home = dirs::home_dir()?;
            Some(home.join(".zsh_history"))
        }
        Shell::Bash => {
            let home = dirs::home_dir()?;
            Some(home.join(".bash_history"))
        }
        Shell::Fish => {
            let data = dirs::data_dir()?;
            Some(data.join("fish/fish_history"))
        }
        // Even if the shell is cmd, look for PowerShell history, cmd doesn't have history file
        Shell::Pwrshl | Shell::Cmd => {
            if cfg!(target_os = "windows") {
                let config = dirs::config_dir()?;
                Some(config.join("Microsoft/Windows/PowerShell/PSReadLine/ConsoleHost_history.txt"))
            } else {
                let data = dirs::data_dir()?;
                Some(data.join("powershell/PSReadLine/ConsoleHost_history.txt"))
            }
        }
        Shell::Unknown => None,
    };

    log::info!("User's shell set as {shell:?}");
    shell.map(|v| v.into_os_string().into_string().unwrap())
}
