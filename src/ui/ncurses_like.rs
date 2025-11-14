use crate::ui::{
    panels::{Panel, PanelType, FileType}, 
    widgets::{FilePanelWidget, CommandLineWidget, StatusBarWidget},
    keybindings::{KeyBindings, KeyPress},
    git_widget::GitWidget,
    screen,
    UIColor
};
use crate::ui::widgets::print_at;
use std::io::{self, Write};

pub struct NcursesLikeUI {
    left_panel: FilePanelWidget,
    right_panel: FilePanelWidget,
    command_line: CommandLineWidget,
    status_bar: StatusBarWidget,
    git_widget: GitWidget,
    keybindings: KeyBindings,
    screen_width: u16,
    screen_height: u16,
    active_panel: ActivePanel,
    mode: UIMode,
    running: bool,
}

#[derive(Debug, Clone)]
pub enum ActivePanel {
    Left,
    Right,
    CommandLine,
}

#[derive(Debug, Clone)]
pub enum UIMode {
    Normal,
    Command,
    Menu,
    Search,
}

impl NcursesLikeUI {
    pub fn new() -> Result<Self, String> {
        let (width, height) = crate::utils::helpers::get_terminal_size()
            .map_err(|e| format!("Не удалось получить размер терминала: {}", e))?;
        
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("Не удалось получить текущую директорию: {}", e))?;
        
        let left_panel = Panel::new(current_dir.clone(), PanelType::FileManager);
        let right_panel = Panel::new(current_dir, PanelType::FileManager);
        
        Ok(Self {
            left_panel: FilePanelWidget::new(left_panel),
            right_panel: FilePanelWidget::new(right_panel),
            command_line: CommandLineWidget::new(),
            status_bar: StatusBarWidget::new(),
            git_widget: GitWidget::new(),
            keybindings: KeyBindings::new(),
            screen_width: width,
            screen_height: height,
            active_panel: ActivePanel::Left,
            mode: UIMode::Normal,
            running: true,
        })
    }
    
    pub fn run(&mut self) -> Result<(), String> {
        self.setup_terminal()?;
        
        while self.running {
            self.draw()?;
            
            if let Some(key) = self.read_key()? {
                if !self.handle_key(key) {
                    break;
                }
            }
        }
        
        self.cleanup_terminal()?;
        Ok(())
    }
    
    fn setup_terminal(&self) -> Result<(), String> {
        crossterm::terminal::enable_raw_mode()
            .map_err(|e| format!("Не удалось включить raw mode: {}", e))?;
        
        crossterm::execute!(
            io::stdout(),
            crossterm::cursor::Hide
        ).map_err(|e| format!("Не удалось скрыть курсор: {}", e))?;
        
        crossterm::execute!(
            io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        ).map_err(|e| format!("Не удалось очистить экран: {}", e))?;
        
        Ok(())
    }
    
    fn cleanup_terminal(&self) -> Result<(), String> {
        crossterm::execute!(
            io::stdout(),
            crossterm::cursor::Show
        ).map_err(|e| format!("Не удалось показать курсор: {}", e))?;
        
        crossterm::terminal::disable_raw_mode()
            .map_err(|e| format!("Не удалось выключить raw mode: {}", e))?;
        
        crossterm::execute!(
            io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        ).map_err(|e| format!("Не удалось очистить экран: {}", e))?;
        
        Ok(())
    }
    
    fn draw(&self) -> Result<(), String> {
        screen::clear();
        
        let panel_width = (self.screen_width - 3) / 2;
        let panel_height = self.screen_height - 3;
        
        // Левая панель
        self.left_panel.draw(1, 1, panel_width, panel_height);
        
        // Разделитель
        for y in 1..panel_height+1 {
            print_at(1 + panel_width, y, "│", UIColor::White);
        }
        
        // Правая панель
        self.right_panel.draw(2 + panel_width, 1, panel_width, panel_height);
        
        // Git виджет (если видим)
        if self.git_widget.is_visible() {
            let git_width = panel_width.min(40);
            let git_x = 2 + panel_width * 2 - git_width;
            self.git_widget.draw(git_x, 1, git_width, panel_height);
        }
        
        // Командная строка
        self.command_line.draw(1, self.screen_height - 2, self.screen_width);
        
        // Статус бар
        self.status_bar.draw(1, self.screen_height - 1, self.screen_width);
        
        io::stdout().flush().unwrap();
        Ok(())
    }
    
    fn handle_key(&mut self, key: KeyPress) -> bool {
        if let KeyPress::Ctrl('q') = key {
            self.running = false;
            return true;
        }
        
        match self.mode {
            UIMode::Normal => self.handle_normal_mode(key),
            UIMode::Command => self.handle_command_mode(key),
            UIMode::Menu => self.handle_menu_mode(key),
            UIMode::Search => self.handle_search_mode(key),
        }
    }
    
    fn handle_normal_mode(&mut self, key: KeyPress) -> bool {
        if let Some(binding) = self.keybindings.find_binding(&key) {
            match binding.action.as_str() {
                "move_up" => self.move_selection(-1),
                "move_down" => self.move_selection(1),
                "switch_panel" => self.switch_panel(),
                "open_file" => self.open_selected(),
                "go_up" => self.go_up_directory(),
                "exit" => { self.running = false; },
                "view_file" => self.view_file(),
                _ => {}
            }
        }
        true
    }
    
    fn handle_command_mode(&mut self, key: KeyPress) -> bool {
        match key {
            KeyPress::Esc => {
                self.mode = UIMode::Normal;
            }
            KeyPress::Enter => {
                self.execute_command();
                self.mode = UIMode::Normal;
            }
            KeyPress::Char(c) => {
                self.command_line.insert_char(c);
            }
            KeyPress::Backspace => {
                self.command_line.backspace();
            }
            _ => {}
        }
        true
    }
    
    fn handle_menu_mode(&mut self, key: KeyPress) -> bool {
        match key {
            KeyPress::Esc => {
                self.mode = UIMode::Normal;
            }
            _ => {}
        }
        true
    }
    
    fn handle_search_mode(&mut self, key: KeyPress) -> bool {
        match key {
            KeyPress::Esc => {
                self.mode = UIMode::Normal;
            }
            _ => {}
        }
        true
    }
    
    fn read_key(&self) -> Result<Option<KeyPress>, String> {
        use crossterm::event::{self, Event, KeyCode, KeyModifiers};
        
        if event::poll(std::time::Duration::from_millis(100))
            .map_err(|e| format!("Ошибка опроса событий: {}", e))? 
        {
            if let Event::Key(key_event) = event::read()
                .map_err(|e| format!("Ошибка чтения события: {}", e))? 
            {
                let key = match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char(c), KeyModifiers::CONTROL) => KeyPress::Ctrl(c),
                    (KeyCode::Char(c), _) => KeyPress::Char(c),
                    (KeyCode::Up, _) => KeyPress::Up,
                    (KeyCode::Down, _) => KeyPress::Down,
                    (KeyCode::Left, _) => KeyPress::Left,
                    (KeyCode::Right, _) => KeyPress::Right,
                    (KeyCode::Enter, _) => KeyPress::Enter,
                    (KeyCode::Esc, _) => KeyPress::Esc,
                    (KeyCode::Backspace, _) => KeyPress::Backspace,
                    (KeyCode::Tab, _) => KeyPress::Tab,
                    (KeyCode::Delete, _) => KeyPress::Delete,
                    (KeyCode::Home, _) => KeyPress::Home,
                    (KeyCode::End, _) => KeyPress::End,
                    (KeyCode::PageUp, _) => KeyPress::PageUp,
                    (KeyCode::PageDown, _) => KeyPress::PageDown,
                    (KeyCode::F(n), _) => KeyPress::F(n),
                    _ => return Ok(None),
                };
                return Ok(Some(key));
            }
        }
        Ok(None)
    }
    
    fn move_selection(&mut self, direction: i32) {
        match self.active_panel {
            ActivePanel::Left => self.left_panel.move_selection(direction),
            ActivePanel::Right => self.right_panel.move_selection(direction),
            ActivePanel::CommandLine => {}
        }
    }
    
    fn switch_panel(&mut self) {
        self.active_panel = match self.active_panel {
            ActivePanel::Left => ActivePanel::Right,
            ActivePanel::Right => ActivePanel::Left,
            ActivePanel::CommandLine => ActivePanel::Left,
        };
        
        let panel_name = match self.active_panel {
            ActivePanel::Left => "LEFT",
            ActivePanel::Right => "RIGHT", 
            ActivePanel::CommandLine => "COMMAND",
        };
        self.status_bar.set_info(&format!("Active panel: {}", panel_name));
    }
    
    fn open_selected(&mut self) {
        let panel = match self.active_panel {
            ActivePanel::Left => &mut self.left_panel,
            ActivePanel::Right => &mut self.right_panel,
            ActivePanel::CommandLine => return,
        };
        
        // Исправление проблемы заимствования
        let dir_name = if let Some(selected_item) = panel.get_selected_item() {
            if matches!(selected_item.file_type, FileType::Directory) {
                Some(selected_item.name.clone())
            } else {
                None
            }
        } else {
            None
        };

        if let Some(dir_name) = dir_name {
            if let Err(e) = panel.change_directory(&dir_name) {
                self.status_bar.set_error(&format!("Ошибка: {}", e));
            }
        } else {
            // Если это файл, переходим в режим команд
            if let Some(selected_item) = panel.get_selected_item() {
                self.mode = UIMode::Command;
                self.command_line.set_text(&format!("edit {}", selected_item.name));
                self.status_bar.set_info(&format!("Editing: {}", selected_item.name));
            }
        }
    }
    
    fn go_up_directory(&mut self) {
        match self.active_panel {
            ActivePanel::Left => {
                if let Err(e) = self.left_panel.go_up_directory() {
                    self.status_bar.set_error(&format!("Ошибка: {}", e));
                }
            }
            ActivePanel::Right => {
                if let Err(e) = self.right_panel.go_up_directory() {
                    self.status_bar.set_error(&format!("Ошибка: {}", e));
                }
            }
            ActivePanel::CommandLine => {
                self.command_line.clear();
            }
        }
    }
    
    fn view_file(&mut self) {
        let selected_item = match self.active_panel {
            ActivePanel::Left => self.left_panel.get_selected_item(),
            ActivePanel::Right => self.right_panel.get_selected_item(),
            ActivePanel::CommandLine => return,
        };

        if let Some(selected_item) = selected_item {
            if !matches!(selected_item.file_type, FileType::Directory) {
                self.mode = UIMode::Command;
                self.command_line.set_text(&format!("view {}", selected_item.name));
                self.status_bar.set_info(&format!("Viewing: {}", selected_item.name));
            }
        }
    }
    
    fn execute_command(&mut self) {
        let command = self.command_line.get_text().trim().to_string();
        if command.is_empty() {
            return;
        }
        
        self.status_bar.set_info(&format!("Executing: {}", command));
        
        match command.as_str() {
            "q" | "quit" => {
                self.running = false;
            }
            "git" => {
                self.git_widget.toggle_visibility();
            }
            cmd if cmd.starts_with("cd ") => {
                let path = cmd.trim_start_matches("cd ").trim();
                if let Err(e) = std::env::set_current_dir(path) {
                    self.status_bar.set_error(&format!("cd error: {}", e));
                }
            }
            _ => {
                match std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&command)
                    .output() 
                {
                    Ok(output) => {
                        if output.status.success() {
                            let result = String::from_utf8_lossy(&output.stdout);
                            self.status_bar.set_info(&format!("Success: {}", result.trim()));
                        } else {
                            let error = String::from_utf8_lossy(&output.stderr);
                            self.status_bar.set_error(&format!("Error: {}", error.trim()));
                        }
                    }
                    Err(e) => {
                        self.status_bar.set_error(&format!("Command failed: {}", e));
                    }
                }
            }
        }
        
        self.command_line.clear();
    }
}
