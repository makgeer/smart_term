use std::io::{self, Write, BufRead};
use crate::terminal::{CommandHistory, CommandExecutor, CommandParser, TerminalConfig, TerminalState};
use crate::utils::privileges::{PrivilegeManager, PrivilegeLevel};
use crate::ui::ncurses_like::NcursesLikeUI;
use crate::utils::helpers;

pub struct Terminal {
    prompt: String,
    history: CommandHistory,
    executor: CommandExecutor,
    parser: CommandParser,
    config: TerminalConfig,
    state: TerminalState,
}

impl Terminal {
    pub fn new() -> Self {
        let config = TerminalConfig::default();
        let state = TerminalState::default();
        let executor = CommandExecutor::new();
        let parser = CommandParser::new();
        let history = CommandHistory::new(config.history_size);
        
        let prompt = Self::build_prompt(&state);
        
        Self {
            prompt,
            history,
            executor,
            parser,
            config,
            state,
        }
    }
    
    pub fn with_config(config: TerminalConfig) -> Self {
        let state = TerminalState::default();
        let executor = CommandExecutor::new();
        let parser = CommandParser::new();
        let history = CommandHistory::new(config.history_size);
        
        let prompt = if config.prompt.is_empty() {
            Self::build_prompt(&state)
        } else {
            config.prompt.clone()
        };
        
        Self {
            prompt,
            history,
            executor,
            parser,
            config,
            state,
        }
    }
    
    pub fn run(&mut self) {
        if self.config.show_welcome {
            self.show_welcome_message();
        }
        
        let stdin = io::stdin();
        
        while self.state.is_running {
            self.show_prompt();
            
            if let Some(input) = stdin.lock().lines().next() {
                match input {
                    Ok(line) => self.process_input(&line),
                    Err(e) => eprintln!("Ошибка чтения: {}", e),
                }
            }
        }
    }
    
    fn build_prompt(state: &TerminalState) -> String {
        let privilege_level = PrivilegeManager::check_privileges();
        let user_indicator = match privilege_level {
            PrivilegeLevel::Root => "🔴",
            PrivilegeLevel::Admin => "🔴", 
            PrivilegeLevel::User => "🟢",
            PrivilegeLevel::Unknown => "⚪",
        };
        
        if cfg!(target_os = "windows") {
            format!("{} {}$ ", user_indicator, state.current_directory)
        } else {
            // Unix-style prompt с цветами
            format!(
                "\x1b[1;32m{} {}@{}:\x1b[1;34m{}\x1b[0m$ ",
                user_indicator,
                state.username,
                state.hostname,
                state.current_directory
            )
        }
    }
    
    fn show_welcome_message(&self) {
        let privilege_level = PrivilegeManager::check_privileges();
        
        println!("╔══════════════════════════════════════════════════════════════════════════════╗");
        println!("║ 🚀 SMART TERMINAL v{} - УНИВЕРСАЛЬНЫЙ ТЕРМИНАЛ С ПСЕВДОГРАФИКОЙ         ║", env!("CARGO_PKG_VERSION"));
        println!("╠══════════════════════════════════════════════════════════════════════════════╣");
        println!("║ 💻 Пользователь: {:<30} Уровень прав: {:<12} ║", 
            self.state.username, format!("{:?}", privilege_level));
        println!("║ 📁 Текущая директория: {:<50} ║", self.state.current_directory);
        
        if !PrivilegeManager::is_elevated() {
            println!("║ ⚠️  Для некоторых команд могут потребоваться повышенные права{:20} ║", "");
            println!("║    Используйте 'elevate' для перезапуска с повышенными правами{:18} ║", "");
        }
        
        println!("╠══════════════════════════════════════════════════════════════════════════════╣");
        println!("║ 🎯 ОСНОВНЫЕ ВОЗМОЖНОСТИ:                                                    ║");
        println!("║                                                                              ║");
        println!("║  📚 Встроенная справка по 100+ Bash командам                                ║");
        println!("║  🖥️  Псевдографический интерфейс в стиле Far/MC (Ctrl+U)                    ║");
        println!("║  🔐 Автоматическое управление правами (sudo/Admin)                          ║");
        println!("║  🐍 Встроенные интерпретаторы: Rust, Python, Java                           ║");
        println!("║  📝 Micro-like редактор с подсветкой синтаксиса                             ║");
        println!("║  🔧 Git интеграция с визуальным статусом                                    ║");
        println!("║  🌐 Кросс-платформенность: Linux, Windows, macOS, BSD                       ║");
        println!("║  📖 Полная история команд с поиском                                         ║");
        println!("║                                                                              ║");
        println!("╠══════════════════════════════════════════════════════════════════════════════╣");
        println!("║ 🎮 ГОРЯЧИЕ КЛАВИШИ И КОМАНДЫ:                                               ║");
        println!("║                                                                              ║");
        println!("║  Ctrl+U        - Переключение в псевдографический режим                     ║");
        println!("║  Tab           - Автодополнение (в разработке)                              ║");
        println!("║  Стрелки ↑↓    - Навигация по истории команд                                ║");
        println!("║  help          - Справка по терминалу                                       ║");
        println!("║  bash-help     - Полная справка по Bash                                     ║");
        println!("║  bash-quick    - Быстрая справка (часто используемые команды)               ║");
        println!("║  help <команда>- Справка по конкретной команде                              ║");
        println!("║  elevate       - Перезапуск с повышенными правами                           ║");
        println!("║  privileges    - Показать текущий уровень прав                              ║");
        println!("║  history       - Показать историю команд                                    ║");
        println!("║  clear         - Очистить экран                                             ║");
        println!("║  exit/quit     - Выход из терминала                                         ║");
        println!("║                                                                              ║");
        println!("╠══════════════════════════════════════════════════════════════════════════════╣");
        println!("║ 💡 ПРИМЕРЫ ИСПОЛЬЗОВАНИЯ:                                                   ║");
        println!("║                                                                              ║");
        println!("║  > help ls               - Справка по команде ls                            ║");
        println!("║  > !python print('hello')- Выполнить Python код                             ║");
        println!("║  > !edit file.txt        - Редактировать файл                               ║");
        println!("║  > gs                    - Git статус (если в репозитории)                  ║");
        println!("║  > bash-help             - Полная справка по Bash                           ║");
        println!("║  > Ctrl+U                - Перейти в режим файлового менеджера              ║");
        println!("║                                                                              ║");
        println!("╚══════════════════════════════════════════════════════════════════════════════╝");
        println!();
    }
    
    fn show_prompt(&self) {
        print!("{}", self.prompt);
        if let Err(e) = io::stdout().flush() {
            eprintln!("Ошибка вывода: {}", e);
        }
    }
    
    fn process_input(&mut self, input: &str) {
        let input = input.trim();
        
        if input.is_empty() {
            return;
        }
        
        // Проверяем специальные комбинации клавиш
        if input == "\x15" { // Ctrl+U
            self.activate_ui_mode();
            return;
        }
        
        // Добавляем в историю (кроме специальных комбинаций)
        // ИСПРАВЛЕНИЕ: убрана некорректная escape-последовательность
        if !input.starts_with('\x15') && !input.chars().next().map_or(false, |c| c.is_control()) {
            self.history.add(input.to_string());
        }
        
        // Обрабатываем специальные команды
        match input {
            "exit" | "quit" => {
                helpers::print_success("До свидания!");
                self.state.is_running = false;
                return;
            }
            "clear" => {
                helpers::clear_screen();
                return;
            }
            "history" => {
                self.show_history();
                return;
            }
            "help" | "bash-help" | "bash-quick" => {
                let _ = self.executor.execute_internal_command(input);
                return;
            }
            "elevate" => {
                self.elevate_privileges();
                return;
            }
            "privileges" => {
                self.show_privileges();
                return;
            }
            "ui" | "gui" | "graphics" => {
                self.activate_ui_mode();
                return;
            }
            "nowelcome" => {
                helpers::print_info("Приветственное сообщение отключено. Используйте 'welcome' для показа.");
                return;
            }
            "welcome" => {
                self.show_welcome_message();
                return;
            }
            _ => {}
        }
        
        // Если команда начинается с "help " - обрабатываем как запрос справки
        if input.starts_with("help ") {
            let _ = self.executor.execute_internal_command(input);
            return;
        }
        
        // Парсим и выполняем команду
        match self.parser.parse(input) {
            Ok(command) => {
                match self.executor.execute(&command) {
                    Ok(result) => {
                        if !result.output.is_empty() {
                            println!("{}", result.output);
                        }
                        if let Some(error) = result.error {
                            helpers::print_error(&error);
                        }
                        self.state.last_exit_code = if result.success { 0 } else { 1 };
                    }
                    Err(e) => {
                        helpers::print_error(&e);
                        self.state.last_exit_code = 1;
                    }
                }
            }
            Err(e) => {
                helpers::print_error(&e);
                self.state.last_exit_code = 1;
            }
        }
        
        // Обновляем промпт после выполнения команды
        self.update_prompt();
    }
    
    fn activate_ui_mode(&self) {
        helpers::print_info("Переход в псевдографический режим...");
        println!("💡 В псевдографическом режиме используйте:");
        println!("   Tab - переключение между панелями");
        println!("   F1-F10 - функциональные клавиши");
        println!("   Ctrl+Q - возврат в текстовый режим");
        println!("   : - ввод команд (как в Vim)");
        println!();
        
        match NcursesLikeUI::new() {
            Ok(mut ui) => {
                if let Err(e) = ui.run() {
                    helpers::print_error(&format!("Ошибка в графическом режиме: {}", e));
                    helpers::print_info("Возврат в текстовый режим...");
                }
            }
            Err(e) => {
                helpers::print_error(&format!("Не удалось запустить графический режим: {}", e));
                helpers::print_info("Убедитесь, что терминал поддерживает необходимые функции.");
            }
        }
        
        // Показываем краткое сообщение после возврата
        helpers::print_success("Возврат в текстовый режим. Для справки введите 'help'");
    }
    
    fn elevate_privileges(&self) {
        helpers::print_info("Запрос повышенных привилегий...");
        match PrivilegeManager::request_elevation() {
            Ok(_) => {
                helpers::print_success("Успешно. Перезапустите терминал.");
            }
            Err(e) => {
                helpers::print_error(&format!("Не удалось получить повышенные права: {}", e));
            }
        }
    }
    
    fn show_privileges(&self) {
        let current = PrivilegeManager::check_privileges();
        println!("Текущий уровень прав: {:?}", current);
        println!("Повышены ли права: {}", PrivilegeManager::is_elevated());
    }
    
    fn show_history(&self) {
        println!("История команд:");
        for (i, cmd) in self.history.get_all().iter().enumerate() {
            println!("{:4}: {}", i + 1, cmd);
        }
    }
    
    fn update_prompt(&mut self) {
        // Обновляем текущую директорию
        if let Ok(current_dir) = std::env::current_dir() {
            self.state.current_directory = current_dir.to_string_lossy().to_string();
        }
        
        // Перестраиваем промпт
        self.prompt = Self::build_prompt(&self.state);
    }
    
    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
    }
    
    pub fn get_state(&self) -> &TerminalState {
        &self.state
    }
    
    pub fn get_config(&self) -> &TerminalConfig {
        &self.config
    }
}
