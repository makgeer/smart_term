use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum PrivilegeLevel {
    User,
    Admin,    // Windows Administrator
    Root,     // Unix root
    Unknown,
}

pub struct PrivilegeManager;

impl PrivilegeManager {
    /// Проверить текущий уровень привилегий
    pub fn check_privileges() -> PrivilegeLevel {
        #[cfg(target_os = "windows")]
        {
            windows::check_windows_privileges()
        }
        
        #[cfg(target_family = "unix")]
        {
            unix::check_unix_privileges()
        }
        
        #[cfg(not(any(target_os = "windows", target_family = "unix")))]
        {
            PrivilegeLevel::Unknown
        }
    }
    
    /// Проверить, есть ли повышенные привилегии
    pub fn is_elevated() -> bool {
        let level = Self::check_privileges();
        level == PrivilegeLevel::Admin || level == PrivilegeLevel::Root
    }
    
    /// Запросить повышение привилегий
    pub fn request_elevation() -> Result<(), String> {
        if Self::is_elevated() {
            return Ok(());
        }
        
        #[cfg(target_os = "windows")]
        {
            windows::elevate_windows()
        }
        
        #[cfg(target_family = "unix")]
        {
            unix::elevate_unix()
        }
        
        #[cfg(not(any(target_os = "windows", target_family = "unix")))]
        {
            Err("Платформа не поддерживается".to_string())
        }
    }
    
    /// Выполнить команду с повышенными привилегиями
    pub fn run_elevated_command(command: &str, args: &[String]) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            windows::run_elevated_windows(command, args)
        }
        
        #[cfg(target_family = "unix")]
        {
            unix::run_elevated_unix(command, args)
        }
        
        #[cfg(not(any(target_os = "windows", target_family = "unix")))]
        {
            Err("Платформа не поддерживается".to_string())
        }
    }
}

// Windows-specific implementation
#[cfg(target_os = "windows")]
mod windows {
    use super::PrivilegeLevel;
    use std::process::{Command, Stdio};
    
    pub fn check_windows_privileges() -> PrivilegeLevel {
        // Простая проверка через whoami /groups
        let output = Command::new("whoami")
            .args(&["/groups"])
            .output();
            
        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.contains("S-1-16-12288") || stdout.contains("S-1-16-16384") {
                    PrivilegeLevel::Admin
                } else {
                    PrivilegeLevel::User
                }
            }
            _ => PrivilegeLevel::User,
        }
    }
    
    pub fn elevate_windows() -> Result<(), String> {
        let exe_path = std::env::current_exe()
            .map_err(|e| format!("Не удалось получить путь к исполняемому файлу: {}", e))?;
            
        let output = Command::new("powershell")
            .args(&[
                "-Command",
                "Start-Process",
                &exe_path.to_string_lossy(),
                "-Verb",
                "RunAs",
                "-WorkingDirectory",
                &std::env::current_dir().unwrap().to_string_lossy()
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();
            
        match output {
            Ok(_) => {
                println!("✅ Запуск с правами администратора...");
                std::process::exit(0);
            }
            Err(e) => Err(format!("Не удалось запустить с повышенными правами: {}", e))
        }
    }
    
    pub fn run_elevated_windows(command: &str, args: &[String]) -> Result<(), String> {
        let args_str = args.join(" ");
        
        let output = Command::new("powershell")
            .args(&[
                "-Command",
                "Start-Process",
                command,
                "-ArgumentList",
                &format!("\"{}\"", args_str),
                "-Verb",
                "RunAs",
                "-Wait"
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Не удалось запустить команду: {}", e))?
            .wait_with_output()
            .map_err(|e| format!("Ошибка выполнения: {}", e))?;
            
        if !output.stdout.is_empty() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
        
        Ok(())
    }
}

// Unix-specific implementation (Linux, macOS, BSD)
#[cfg(target_family = "unix")]
mod unix {
    use super::PrivilegeLevel;
    use std::process::{Command, Stdio};
    
    pub fn check_unix_privileges() -> PrivilegeLevel {
        unsafe {
            let uid = libc::getuid();
            if uid == 0 {
                PrivilegeLevel::Root
            } else {
                PrivilegeLevel::User
            }
        }
    }
    
    pub fn elevate_unix() -> Result<(), String> {
        let exe_path = std::env::current_exe()
            .map_err(|e| format!("Не удалось получить путь к исполняемому файлу: {}", e))?;
            
        // Пытаемся использовать sudo или doas (для BSD)
        let sudo_command = if Command::new("doas").arg("--version").output().is_ok() {
            "doas"
        } else {
            "sudo"
        };
        
        let mut cmd = Command::new(sudo_command);
        cmd.arg(&exe_path);
        
        // Сохраняем текущую директорию
        if let Ok(current_dir) = std::env::current_dir() {
            cmd.current_dir(current_dir);
        }
        
        // Передаем аргументы командной строки
        cmd.args(std::env::args().skip(1));
        
        match cmd.spawn() {
            Ok(_) => {
                println!("✅ Запуск с правами root...");
                std::process::exit(0);
            }
            Err(e) => Err(format!("Не удалось запустить с повышенными правами: {}. Попробуйте запустить с sudo/doas вручную.", e))
        }
    }
    
    pub fn run_elevated_unix(command: &str, args: &[String]) -> Result<(), String> {
        // Определяем доступную команду для повышения прав
        let sudo_command = if Command::new("doas").arg("--version").output().is_ok() {
            "doas"
        } else {
            "sudo"
        };
        
        let mut cmd = Command::new(sudo_command);
        cmd.arg(command);
        cmd.args(args);
        
        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Не удалось запустить команду: {}", e))?
            .wait_with_output()
            .map_err(|e| format!("Ошибка выполнения: {}", e))?;
            
        if !output.stdout.is_empty() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
        
        Ok(())
    }
}
