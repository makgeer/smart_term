use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    cursor::{MoveTo, Show},
    event::{self, Event, KeyCode, KeyModifiers},
    style::{Print, ResetColor},
    Result as CrosstermResult,
};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Terminal,
    FileManager,
    Editor,
    GitStatus,
    PseudoGraphics,
}

pub struct TerminalState {
    pub history: VecDeque<String>,
    pub current_input: String,
    pub cursor_position: usize,
    pub output_lines: VecDeque<String>,
    pub history_index: Option<usize>,
    pub prompt: String,
}

impl Default for TerminalState {
    fn default() -> Self {
        Self {
            history: VecDeque::with_capacity(100),
            current_input: String::new(),
            cursor_position: 0,
            output_lines: VecDeque::with_capacity(1000),
            history_index: None,
            prompt: "smart-term> ".to_string(),
        }
    }
}

pub struct AppState {
    pub mode: AppMode,
    pub current_file: Option<PathBuf>,
    pub current_directory: PathBuf,
    pub should_quit: bool,
    pub terminal: TerminalState,
}

impl Default for AppState {
    fn default() -> Self {
        let mut terminal = TerminalState::default();
        terminal.output_lines.push_back("Smart Term - Кроссплатформенный терминал".to_string());
        terminal.output_lines.push_back("Введите 'help' для списка команд".to_string());
        terminal.output_lines.push_back("".to_string());

        Self {
            mode: AppMode::Terminal,
            current_file: None,
            current_directory: std::env::current_dir().unwrap_or_default(),
            should_quit: false,
            terminal,
        }
    }
}

fn main() -> CrosstermResult<()> {
    // Настройка терминала
    enable_raw_mode()?;
    
    let mut state = AppState::default();
    
    // Основной цикл приложения
    while !state.should_quit {
        // Отрисовка UI
        draw_ui(&state)?;
        state.current_directory = std::env::current_dir().unwrap_or_default();

        // Обработка событий
        if let Event::Key(key_event) = event::read()? {
            handle_key_event(key_event, &mut state)?;
        }
    }

    // Очистка терминала
    execute!(io::stdout(), Show, ResetColor)?;
    disable_raw_mode()?;

    Ok(())
}

fn draw_ui(state: &AppState) -> CrosstermResult<()> {
    let (width, height) = size().unwrap_or((80, 24));
    
    // Очистка экрана
    execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
    
    // Верхняя рамка
    execute!(io::stdout(), Print("┌"), Print(&"─".repeat((width - 2) as usize)), Print("┐"))?;
    execute!(io::stdout(), MoveTo(0, 1), Print("│"), Print(&format!(" Smart Term - {} ", mode_name(&state.mode))), Print("│"))?;
    execute!(io::stdout(), MoveTo(0, 2), Print("├"), Print(&"─".repeat((width - 2) as usize)), Print("┤"))?;
    
    match state.mode {
        AppMode::Terminal => {
            draw_terminal_ui(state, width, height)?;
        }
        _ => {
            draw_other_modes(state, width, height)?;
        }
    }
    
    // Нижняя рамка
    execute!(io::stdout(), MoveTo(0, height - 3), Print("├"), Print(&"─".repeat((width - 2) as usize)), Print("┤"))?;
    execute!(io::stdout(), MoveTo(0, height - 2), Print("│"), Print(&format!(" {:<width$} ", get_help_text(&state.mode), width = (width - 4) as usize)), Print("│"))?;
    execute!(io::stdout(), MoveTo(0, height - 1), Print("└"), Print(&"─".repeat((width - 2) as usize)), Print("┘"))?;
    
    io::stdout().flush()?;
    Ok(())
}

fn draw_terminal_ui(state: &AppState, width: u16, height: u16) -> CrosstermResult<()> {
    let terminal_area_height = height - 6;
    
    // Выводим историю команд (снизу вверх)
    let lines_to_show = terminal_area_height.saturating_sub(1) as usize;
    let start_index = if state.terminal.output_lines.len() > lines_to_show {
        state.terminal.output_lines.len() - lines_to_show
    } else {
        0
    };
    
    for (i, line) in state.terminal.output_lines.iter().skip(start_index).enumerate() {
        if i < lines_to_show {
            let y = 3 + i as u16;
            let display_line = if line.len() > (width - 4) as usize {
                &line[..(width - 4) as usize]
            } else {
                line
            };
            execute!(io::stdout(), MoveTo(2, y), Print(display_line))?;
        }
    }
    
    // Промпт и текущая строка ввода
    let input_y = height - 4;
    execute!(io::stdout(), MoveTo(2, input_y), Print(&state.terminal.prompt))?;
    
    // Выводим текущий ввод
    let input_text = &state.terminal.current_input;
    let display_input = if input_text.len() > (width - 20) as usize {
        &input_text[input_text.len() - (width - 20) as usize..]
    } else {
        input_text
    };
    execute!(io::stdout(), Print(display_input))?;
    
    // Позиция курсора в строке ввода
    let cursor_x = 2 + state.terminal.prompt.len() + state.terminal.cursor_position as usize;
    if cursor_x < (width - 1) as usize {
        execute!(io::stdout(), MoveTo(cursor_x as u16, input_y), Print("_"))?;
    }
    
    Ok(())
}

fn draw_other_modes(state: &AppState, width: u16, height: u16) -> CrosstermResult<()> {
    // Основная область для других режимов
    let content_height = height - 6;
    for i in 0..content_height {
        execute!(io::stdout(), MoveTo(0, 3 + i), Print("│"))?;
        execute!(io::stdout(), MoveTo(width - 1, 3 + i), Print("│"))?;
    }
    
    match state.mode {
        AppMode::FileManager => {
            execute!(io::stdout(), MoveTo(2, 4), Print("📁 Файловый менеджер"))?;
            execute!(io::stdout(), MoveTo(2, 6), Print(&format!("📂 Текущая директория: {:?}", state.current_directory)))?;
            
            // Показываем файлы в директории
            if let Ok(entries) = std::fs::read_dir(&state.current_directory) {
                for (i, entry) in entries.enumerate().take(8) {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        let name = path.file_name().unwrap_or_default().to_string_lossy();
                        let is_dir = path.is_dir();
                        let marker = if i == 0 { "> " } else { "  " };
                        let icon = if is_dir { "📁" } else { "📄" };
                        execute!(io::stdout(), MoveTo(2, 8 + i as u16), Print(&format!("{}{}{}", marker, icon, name)))?;
                    }
                }
            }
        }
        AppMode::Editor => {
            execute!(io::stdout(), MoveTo(2, 4), Print("✏️ Редактор (в разработке)"))?;
            execute!(io::stdout(), MoveTo(2, 6), Print("Поддержка синтаксиса: Python, Rust, JS"))?;
            execute!(io::stdout(), MoveTo(2, 8), Print("Горячие клавиши: F2 - сохранить, Esc - выйти"))?;
            if let Some(file) = &state.current_file {
                execute!(io::stdout(), MoveTo(2, 10), Print(&format!("📄 Файл: {:?}", file)))?;
            }
        }
        AppMode::GitStatus => {
            execute!(io::stdout(), MoveTo(2, 4), Print("🔗 Git статус:"))?;
            
            if is_git_repo(&state.current_directory) {
                execute!(io::stdout(), MoveTo(2, 6), Print("✅ Git репозиторий найден"))?;
                
                if let Ok(output) = Command::new("git")
                    .args(&["branch", "--show-current"])
                    .current_dir(&state.current_directory)
                    .output() {
                    
                    if let Ok(branch) = String::from_utf8(output.stdout) {
                        let branch = branch.trim();
                        execute!(io::stdout(), MoveTo(2, 8), Print(&format!("🌿 Ветка: {}", branch)))?;
                    }
                }
                
                if let Ok(output) = Command::new("git")
                    .args(&["status", "--porcelain"])
                    .current_dir(&state.current_directory)
                    .output() {
                    
                    let status = String::from_utf8_lossy(&output.stdout);
                    let lines: Vec<&str> = status.lines().collect();
                    execute!(io::stdout(), MoveTo(2, 10), Print(&format!("📊 Изменений: {}", lines.len())))?;
                    
                    for (i, line) in lines.iter().enumerate().take(5) {
                        execute!(io::stdout(), MoveTo(2, 12 + i as u16), Print(&format!("  {}", line)))?;
                    }
                }
            } else {
                execute!(io::stdout(), MoveTo(2, 6), Print("❌ Не Git репозиторий"))?;
                execute!(io::stdout(), MoveTo(2, 8), Print("💡 Используйте 'git init' для инициализации"))?;
            }
        }
        AppMode::PseudoGraphics => {
            execute!(io::stdout(), MoveTo(2, 4), Print("🎨 Псевдографика (MC/VC стиль)"))?;
            execute!(io::stdout(), MoveTo(2, 6), Print("┌─────────────────────────────────────────────────┐"))?;
            execute!(io::stdout(), MoveTo(2, 7), Print("│ Панель 1                    │ Панель 2     │"))?;
            execute!(io::stdout(), MoveTo(2, 8), Print("├─────────────────────────────────┼──────────────┤"))?;
            execute!(io::stdout(), MoveTo(2, 9), Print("│ файлы...                     │ превью...    │"))?;
            execute!(io::stdout(), MoveTo(2, 10), Print("│                               │              │"))?;
            execute!(io::stdout(), MoveTo(2, 11), Print("└─────────────────────────────────────────────────┘"))?;
            execute!(io::stdout(), MoveTo(2, 13), Print("Включена поддержка псевдографики!"))?;
        }
        AppMode::Terminal => {
            // Этот случай не должен вызываться для Terminal
            execute!(io::stdout(), MoveTo(2, 4), Print("Ошибка: Terminal режим не должен обрабатываться здесь"))?;
        }
    }
    
    Ok(())
}

fn mode_name(mode: &AppMode) -> &'static str {
    match mode {
        AppMode::Terminal => "Терминал",
        AppMode::FileManager => "Файловый менеджер",
        AppMode::Editor => "Редактор",
        AppMode::GitStatus => "Git статус",
        AppMode::PseudoGraphics => "Псевдографика",
    }
}

fn get_help_text(mode: &AppMode) -> &'static str {
    match mode {
        AppMode::Terminal => "Tab: Режимы | Ctrl+G: Git | F4: Редактор | Ctrl+P: Псевдографика | Ctrl+Q: Выход",
        AppMode::FileManager => "Enter: Открыть | Tab: Переключение | Esc: Терминал | Ctrl+Q: Выход",
        AppMode::Editor => "F2: Сохранить | Esc: Терминал | Ctrl+Q: Выход",
        AppMode::GitStatus => "Tab: Переключение | Esc: Терминал | Ctrl+Q: Выход",
        AppMode::PseudoGraphics => "Tab: Переключение | Esc: Терминал | Ctrl+Q: Выход",
    }
}

fn handle_key_event(
    key_event: crossterm::event::KeyEvent,
    state: &mut AppState,
) -> CrosstermResult<()> {
    match state.mode {
        AppMode::Terminal => handle_terminal_input(key_event, state)?,
        _ => handle_other_modes_input(key_event, state)?,
    }
    
    Ok(())
}

fn handle_terminal_input(
    key_event: crossterm::event::KeyEvent,
    state: &mut AppState,
) -> CrosstermResult<()> {
    match key_event.code {
        KeyCode::Char('q') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            state.should_quit = true;
        }
        KeyCode::Char('g') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            state.mode = AppMode::GitStatus;
        }
        KeyCode::F(4) => {
            state.mode = AppMode::Editor;
            state.current_file = Some(state.current_directory.join("new_file.txt"));
        }
        KeyCode::Tab => {
            state.mode = AppMode::FileManager;
        }
        KeyCode::Char('p') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            state.mode = AppMode::PseudoGraphics;
        }
        KeyCode::Enter => {
            execute_command(&mut state.terminal, &state.current_directory);
        }
        KeyCode::Backspace => {
            if state.terminal.cursor_position > 0 {
                state.terminal.cursor_position -= 1;
                if state.terminal.cursor_position < state.terminal.current_input.len() {
                    state.terminal.current_input.remove(state.terminal.cursor_position);
                }
            }
        }
        KeyCode::Delete => {
            if state.terminal.cursor_position < state.terminal.current_input.len() {
                state.terminal.current_input.remove(state.terminal.cursor_position);
            }
        }
        KeyCode::Left => {
            if state.terminal.cursor_position > 0 {
                state.terminal.cursor_position -= 1;
            }
        }
        KeyCode::Right => {
            if state.terminal.cursor_position < state.terminal.current_input.len() {
                state.terminal.cursor_position += 1;
            }
        }
        KeyCode::Home => {
            state.terminal.cursor_position = 0;
        }
        KeyCode::End => {
            state.terminal.cursor_position = state.terminal.current_input.len();
        }
        KeyCode::Up => {
            if let Some(idx) = state.terminal.history_index {
                if idx > 0 {
                    state.terminal.history_index = Some(idx - 1);
                    state.terminal.current_input = state.terminal.history[idx - 1].clone();
                    state.terminal.cursor_position = state.terminal.current_input.len();
                }
            } else if !state.terminal.history.is_empty() {
                state.terminal.history_index = Some(state.terminal.history.len() - 1);
                state.terminal.current_input = state.terminal.history.back().unwrap().clone();
                state.terminal.cursor_position = state.terminal.current_input.len();
            }
        }
        KeyCode::Down => {
            if let Some(idx) = state.terminal.history_index {
                if idx < state.terminal.history.len() - 1 {
                    state.terminal.history_index = Some(idx + 1);
                    state.terminal.current_input = state.terminal.history[idx + 1].clone();
                    state.terminal.cursor_position = state.terminal.current_input.len();
                } else {
                    state.terminal.history_index = None;
                    state.terminal.current_input.clear();
                    state.terminal.cursor_position = 0;
                }
            }
        }
        KeyCode::Char(ch) => {
            if ch == '\t' {
                // Автодополнение (базовое)
                state.terminal.current_input.push_str("    ");
                state.terminal.cursor_position += 4;
            } else {
                // Безопасная вставка символа
                if state.terminal.cursor_position <= state.terminal.current_input.len() {
                    state.terminal.current_input.insert(state.terminal.cursor_position, ch);
                    state.terminal.cursor_position += 1;
                }
            }
        }
        KeyCode::Esc => {
            state.terminal.current_input.clear();
            state.terminal.cursor_position = 0;
            state.terminal.history_index = None;
        }
        _ => {}
    }
    
    Ok(())
}

fn handle_other_modes_input(
    key_event: crossterm::event::KeyEvent,
    state: &mut AppState,
) -> CrosstermResult<()> {
    match key_event.code {
        KeyCode::Char('q') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            state.should_quit = true;
        }
        KeyCode::F(10) => {
            state.should_quit = true;
        }
        KeyCode::Tab => {
            state.mode = AppMode::Terminal;
        }
        KeyCode::Esc => {
            state.mode = AppMode::Terminal;
        }
        KeyCode::Enter => {
            if state.mode == AppMode::FileManager {
                if let Ok(mut entries) = std::fs::read_dir(&state.current_directory) {
                    if let Some(entry) = entries.find_map(|e| e.ok()) {
                        let path = entry.path();
                        if path.is_file() {
                            state.current_file = Some(path);
                            state.mode = AppMode::Editor;
                        } else if path.is_dir() {
                            state.current_directory = path;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    
    Ok(())
}

fn execute_command(terminal: &mut TerminalState, current_dir: &PathBuf) {
    let command = terminal.current_input.trim().to_string();
    
    if command.is_empty() {
        terminal.output_lines.push_back(String::new());
        return;
    }
    
    // Добавляем команду в историю
    if !terminal.history.contains(&command) {
        terminal.history.push_back(command.clone());
        if terminal.history.len() > 100 {
            terminal.history.pop_front();
        }
    }
    
    // Обработка встроенных команд
    if command == "help" {
        terminal.output_lines.push_back("Smart Term - Встроенные команды:".to_string());
        terminal.output_lines.push_back("  help     - Показать эту справку".to_string());
        terminal.output_lines.push_back("  clear    - Очистить экран".to_string());
        terminal.output_lines.push_back("  pwd      - Показать текущую директорию".to_string());
        terminal.output_lines.push_back("  ls       - Список файлов".to_string());
        terminal.output_lines.push_back("  cd       - Сменить директорию".to_string());
        terminal.output_lines.push_back("  exit     - Выход из терминала".to_string());
        terminal.output_lines.push_back("  gui      - Переключить в GUI режим".to_string());
        terminal.output_lines.push_back("".to_string());
    } else if command == "clear" {
        terminal.output_lines.clear();
    } else if command == "pwd" {
        let current_path = std::env::current_dir().unwrap_or_default();
        terminal.output_lines.push_back(current_path.display().to_string());
    } else if command == "ls" {
        if let Ok(entries) = std::fs::read_dir(current_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                    let prefix = if is_dir { "📁 " } else { "📄 " };
                    terminal.output_lines.push_back(format!("{}{}", prefix, name));
                }
            }
        }
    } else if command.starts_with("cd ") {
        let path = &command[3..].trim();
        if *path == ".." {
            if let Some(parent) = current_dir.parent() {
                std::env::set_current_dir(parent).ok();
            }
        } else if *path == "~" {
            if let Some(home) = dirs::home_dir() {
                std::env::set_current_dir(home).ok();
            }
        } else {
            let new_path = current_dir.join(path);
            if new_path.exists() && new_path.is_dir() {
                std::env::set_current_dir(new_path).ok();
            } else {
                terminal.output_lines.push_back(format!("cd: {}: No such file or directory", path));
            }
        }
        let current_path = std::env::current_dir().unwrap_or_default();
        terminal.output_lines.push_back(current_path.display().to_string());
    } else if command == "exit" {
        terminal.output_lines.push_back("Для выхода из Smart Term используйте Ctrl+Q".to_string());
    } else if command == "gui" {
        terminal.output_lines.push_back("GUI режим уже активен. Используйте Tab для переключения режимов".to_string());
    } else {
        // Выполнение системной команды
        let parts: Vec<&str> = command.split_whitespace().collect();
        if !parts.is_empty() {
            let program = parts[0];
            let args = &parts[1..];
            
            match Command::new(program)
                .args(args)
                .current_dir(current_dir)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
            {
                Ok(child) => {
                    match child.wait_with_output() {
                        Ok(output) => {
                            if !output.stdout.is_empty() {
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                for line in stdout.lines() {
                                    terminal.output_lines.push_back(line.to_string());
                                }
                            }
                            if !output.stderr.is_empty() {
                                let stderr = String::from_utf8_lossy(&output.stderr);
                                for line in stderr.lines() {
                                    terminal.output_lines.push_back(format!("Error: {}", line));
                                }
                            }
                        }
                        Err(e) => {
                            terminal.output_lines.push_back(format!("Error executing command: {}", e));
                        }
                    }
                }
                Err(_e) => {
                    terminal.output_lines.push_back(format!("Command not found: {}", program));
                }
            }
        }
    }
    
    // Очищаем ввод
    terminal.current_input.clear();
    terminal.cursor_position = 0;
    terminal.history_index = None;
    
    // Добавляем пустую строку для разделения
    terminal.output_lines.push_back(String::new());
    
    // Ограничиваем размер истории
    if terminal.output_lines.len() > 1000 {
        terminal.output_lines.pop_front();
    }
}

fn is_git_repo(path: &PathBuf) -> bool {
    Command::new("git")
        .args(&["rev-parse", "--git-dir"])
        .current_dir(path)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}