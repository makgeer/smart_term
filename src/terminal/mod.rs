pub mod core;
pub mod history;
pub mod parser;
pub mod executor;

pub use core::Terminal;
pub use history::CommandHistory;
pub use parser::{CommandParser, ParsedCommand, CommandType};
pub use executor::CommandExecutor;

/// Конфигурация терминала
#[derive(Debug, Clone)]
pub struct TerminalConfig {
    pub prompt: String,
    pub history_size: usize,
    pub show_welcome: bool,
    pub enable_syntax_highlighting: bool,
    pub auto_completion: bool,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            prompt: "smart-term> ".to_string(),
            history_size: 100,
            show_welcome: true,
            enable_syntax_highlighting: true,
            auto_completion: false, // Пока не реализовано
        }
    }
}

/// Результат выполнения команды
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

impl CommandResult {
    pub fn new(success: bool, output: String, error: Option<String>) -> Self {
        Self { success, output, error }
    }
    
    pub fn success(output: String) -> Self {
        Self::new(true, output, None)
    }
    
    pub fn error(error: String) -> Self {
        Self::new(false, String::new(), Some(error))
    }
}

/// Состояние терминала
#[derive(Debug, Clone)]
pub struct TerminalState {
    pub current_directory: String,
    pub username: String,
    pub hostname: String,
    pub is_running: bool,
    pub last_exit_code: i32,
}

impl Default for TerminalState {
    fn default() -> Self {
        Self {
            current_directory: std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| "?".to_string()),
            username: whoami::username(),
            // Заменить устаревший вызов
            hostname: whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string()),
            is_running: true,
            last_exit_code: 0,
        }
    }
}
