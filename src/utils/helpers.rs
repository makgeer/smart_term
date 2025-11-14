use std::io::{self, Write};

/// Вывод цветного текста
pub struct Color;

impl Color {
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const MAGENTA: &'static str = "\x1b[35m";
    pub const CYAN: &'static str = "\x1b[36m";
    pub const WHITE: &'static str = "\x1b[37m";
    pub const RESET: &'static str = "\x1b[0m";
    pub const BOLD: &'static str = "\x1b[1m";
}

/// Вывести цветное сообщение
pub fn print_color(text: &str, color: &str) {
    print!("{}{}{}", color, text, Color::RESET);
    io::stdout().flush().unwrap();
}

/// Вывести цветное сообщение с переводом строки
pub fn println_color(text: &str, color: &str) {
    println!("{}{}{}", color, text, Color::RESET);
}

/// Вывести сообщение об ошибке
pub fn print_error(msg: &str) {
    println_color(&format!("❌ {}", msg), Color::RED);
}

/// Вывести сообщение об успехе
pub fn print_success(msg: &str) {
    println_color(&format!("✅ {}", msg), Color::GREEN);
}

/// Вывести предупреждение
pub fn print_warning(msg: &str) {
    println_color(&format!("⚠️  {}", msg), Color::YELLOW);
}

/// Вывести информационное сообщение
pub fn print_info(msg: &str) {
    println_color(&format!("💡 {}", msg), Color::CYAN);
}

/// Получить размер терминала
pub fn get_terminal_size() -> Result<(u16, u16), String> {
    #[cfg(unix)]
    {
        unsafe {
            let mut size: libc::winsize = std::mem::zeroed();
            if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut size) == 0 {
                Ok((size.ws_col, size.ws_row))
            } else {
                Ok((80, 24)) // Стандартный размер
            }
        }
    }
    
    #[cfg(not(unix))]
    {
        Ok((80, 24)) // Заглушка для не-Unix систем
    }
}

/// Очистить экран
pub fn clear_screen() {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().unwrap();
}

/// Спросить у пользователя подтверждение
pub fn ask_confirm(prompt: &str) -> bool {
    print!("{} [y/N]: ", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        matches!(input.trim().to_lowercase().as_str(), "y" | "yes" | "да")
    } else {
        false
    }
}

/// Читать ввод пользователя
pub fn read_input() -> Result<String, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Ошибка чтения ввода: {}", e))?;
    Ok(input.trim().to_string())
}

/// Форматировать размер файла в человеко-читаемый вид
pub fn human_readable_size(size: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}

/// Проверить, существует ли файл/директория
pub fn path_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

/// Создать директорию если не существует
pub fn ensure_dir(path: &str) -> Result<(), String> {
    std::fs::create_dir_all(path)
        .map_err(|e| format!("Не удалось создать директорию {}: {}", path, e))
}

/// Получить домашнюю директорию пользователя
pub fn get_home_dir() -> Result<String, String> {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| "Не удалось определить домашнюю директорию".to_string())
}

/// Получить текущую рабочую директорию
pub fn get_current_dir() -> String {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "Неизвестно".to_string())
}

/// Разделить строку на аргументы (поддержка кавычек)
pub fn split_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = None;
    
    for ch in input.chars() {
        match ch {
            '"' | '\'' => {
                if in_quotes && quote_char == Some(ch) {
                    in_quotes = false;
                    quote_char = None;
                } else if !in_quotes {
                    in_quotes = true;
                    quote_char = Some(ch);
                } else {
                    current.push(ch);
                }
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }
    
    if !current.is_empty() {
        args.push(current);
    }
    
    args
}

/// Экранирование специальных символов для shell
pub fn escape_shell_arg(arg: &str) -> String {
    if arg.contains(' ') || arg.contains('"') || arg.contains('\'') {
        format!("\"{}\"", arg.replace('"', "\\\""))
    } else {
        arg.to_string()
    }
}
