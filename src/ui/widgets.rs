use crate::ui::{UIColor, panels::{Panel, FileEntry, FileType}};
use crate::utils::filesystem;
use std::io::{self, Write};

pub trait Widget {
    fn draw(&self, x: u16, y: u16, width: u16, height: u16);
    fn handle_input(&mut self, key: crate::ui::keybindings::KeyPress) -> bool;
}

#[derive(Debug, Clone)]
pub struct FilePanelWidget {
    pub panel: Panel,
    pub show_hidden: bool,
}

impl FilePanelWidget {
    pub fn new(panel: Panel) -> Self {
        Self {
            panel,
            show_hidden: false,
        }
    }
    
    pub fn draw(&self, x: u16, y: u16, width: u16, height: u16) {
        // Рамка панели
        self.draw_border(x, y, width, height);
        
        // Заголовок с путем
        self.draw_header(x, y, width);
        
        // Содержимое директории
        self.draw_files(x, y + 1, width, height - 2);
        
        // Статус бар
        self.draw_status(x, y + height - 1, width);
    }
    
    fn draw_border(&self, x: u16, y: u16, width: u16, height: u16) {
        let border_color = if self.panel.is_active {
            UIColor::Cyan
        } else {
            UIColor::White
        };
        
        // Углы
        print_at(x, y, "┌", border_color);
        print_at(x + width - 1, y, "┐", border_color);
        print_at(x, y + height - 1, "└", border_color);
        print_at(x + width - 1, y + height - 1, "┘", border_color);
        
        // Горизонтальные линии
        for i in 1..width-1 {
            print_at(x + i, y, "─", border_color);
            print_at(x + i, y + height - 1, "─", border_color);
        }
        
        // Вертикальные линии
        for i in 1..height-1 {
            print_at(x, y + i, "│", border_color);
            print_at(x + width - 1, y + i, "│", border_color);
        }
    }
    
    fn draw_header(&self, x: u16, y: u16, width: u16) {
        let path_str = self.panel.current_path.to_string_lossy();
        let truncated_path = if path_str.len() > width as usize - 4 {
            format!("…{}", &path_str[path_str.len() - (width as usize - 4) + 1..])
        } else {
            path_str.to_string()
        };
        
        let header_text = if self.panel.is_active {
            format!(" {} ", truncated_path)
        } else {
            truncated_path
        };
        
        print_at(x + 2, y, &header_text, UIColor::Yellow);
    }
    
    fn draw_files(&self, x: u16, y: u16, width: u16, height: u16) {
        let visible_files = self.panel.get_visible_files(height as usize);
        
        for (i, file) in visible_files.iter().enumerate() {
            let is_selected = i + self.panel.scroll_offset == self.panel.selected_index;
            self.draw_file_entry(x, y + i as u16, width, file, is_selected);
        }
        
        // Заполнить оставшееся пространство
        for i in visible_files.len()..height as usize {
            print_at(x + 1, y + i as u16, &" ".repeat(width as usize - 2), UIColor::White);
        }
    }
    
    fn draw_file_entry(&self, x: u16, y: u16, width: u16, file: &FileEntry, is_selected: bool) {
        let bg_color = if is_selected { 
            UIColor::Blue 
        } else { 
            UIColor::Black 
        };
        
        let text_color = match file.file_type {
            FileType::Directory => UIColor::Cyan,
            FileType::Symlink => UIColor::Magenta,
            FileType::File => UIColor::White,
        };
        
        let icon = file.get_icon();
        let mut display_text = format!("{} {}", icon, file.get_display_name());
        
        // Если файл, показываем размер
        if let FileType::File = file.file_type {
            let size_str = crate::utils::human_readable_size(file.size); // ИСПРАВЛЕНО: прямой вызов
            if display_text.len() + size_str.len() + 3 < width as usize {
                display_text = format!("{:width$} {}", display_text, size_str, 
                    width = width as usize - size_str.len() - 3);
            }
        }
        
        // Обрезаем если слишком длинное
        if display_text.len() > width as usize - 2 {
            display_text = format!("{}…", &display_text[..width as usize - 3]);
        }
        
        // Заполняем оставшееся пространство
        let padding = " ".repeat(width as usize - 2 - display_text.len());
        let full_text = format!("{}{}", display_text, padding);
        
        print_at_with_bg(x + 1, y, &full_text, text_color, bg_color);
    }
    
    fn draw_status(&self, x: u16, y: u16, width: u16) {
        let selected = self.panel.get_selected_file();
        let status = if let Some(file) = selected {
            match file.file_type {
                FileType::Directory => "DIR".to_string(),
                FileType::File => crate::utils::human_readable_size(file.size), // ИСПРАВЛЕНО
                FileType::Symlink => "LINK".to_string(),
            }
        } else {
            "".to_string()
        };
        
        let free_space = filesystem::get_free_space(&self.panel.current_path);
        let status_line = format!("{} Free:{}", status, free_space);
        
        print_at_with_bg(x + 1, y, &status_line, UIColor::Green, UIColor::DarkGray);
    }

    // ДОБАВЛЕНЫ НЕДОСТАЮЩИЕ МЕТОДЫ
    pub fn move_selection(&mut self, direction: i32) {
        self.panel.move_selection(direction);
    }

    pub fn get_selected_item(&self) -> Option<&FileEntry> {
        self.panel.get_selected_file()
    }

    pub fn change_directory(&mut self, dir_name: &str) -> Result<(), String> {
        self.panel.change_directory(std::path::PathBuf::from(dir_name))
    }

    pub fn go_up_directory(&mut self) -> Result<(), String> {
        // Нужно добавить этот метод в Panel
        if let Some(parent) = self.panel.current_path.parent() {
            self.panel.change_directory(parent.to_path_buf())
        } else {
            Err("Уже в корневой директории".to_string())
        }
    }

    // ДОБАВИТЬ ЕЩЕ МЕТОДЫ которые могут понадобиться:
    pub fn refresh(&mut self) -> Result<(), String> {
        self.panel.refresh_files()
    }

    pub fn get_current_path(&self) -> &std::path::Path {
        &self.panel.current_path
    }

    pub fn toggle_hidden_files(&mut self) {
        self.show_hidden = !self.show_hidden;
        // Нужно обновить отображение файлов
        let _ = self.panel.refresh_files();
    }
}

#[derive(Debug, Clone)]
pub struct CommandLineWidget {
    pub input: String,
    pub cursor_pos: usize,
    pub history_index: Option<usize>,
}

impl CommandLineWidget {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor_pos: 0,
            history_index: None,
        }
    }
    
    pub fn draw(&self, x: u16, y: u16, width: u16) {
        let prompt = "> ";
        let input_width = width as usize - prompt.len() - 1;
        
        // Обрезаем ввод если слишком длинный
        let display_input = if self.input.len() > input_width {
            let start = self.input.len().saturating_sub(input_width);
            &self.input[start..]
        } else {
            &self.input
        };
        
        let line = format!("{}{}", prompt, display_input);
        print_at(x, y, &line, UIColor::White);
        
        // Курсор
        let cursor_x = x + prompt.len() as u16 + (self.cursor_pos.min(input_width)) as u16;
        crate::ui::screen::move_cursor(cursor_x, y);
    }
    
    pub fn insert_char(&mut self, c: char) {
        if self.cursor_pos <= self.input.len() {
            self.input.insert(self.cursor_pos, c);
            self.cursor_pos += 1;
        }
    }
    
    pub fn delete_backward(&mut self) {
        if self.cursor_pos > 0 {
            self.input.remove(self.cursor_pos - 1);
            self.cursor_pos -= 1;
        }
    }
    
    pub fn backspace(&mut self) {
        if !self.input.is_empty() && self.cursor_pos > 0 {
            self.input.remove(self.cursor_pos - 1);
            self.cursor_pos -= 1;
        }
    }

    pub fn get_text(&self) -> &str {
        &self.input
    }

    pub fn set_text(&mut self, text: &str) {
        self.input = text.to_string();
        self.cursor_pos = self.input.len();
    }

    pub fn clear(&mut self) {
        self.input.clear();
        self.cursor_pos = 0;
        self.history_index = None;
    }
}

#[derive(Debug, Clone)]
pub struct StatusBarWidget {
    pub message: String,
    pub message_type: MessageType,
}

impl StatusBarWidget {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            message_type: MessageType::Info,
        }
    }
    
    pub fn draw(&self, x: u16, y: u16, width: u16) {
        let color = match self.message_type {
            MessageType::Info => UIColor::White,
            MessageType::Warning => UIColor::Yellow,
            MessageType::Error => UIColor::Red,
            MessageType::Success => UIColor::Green,
        };
        
        let padded_message = if self.message.len() > width as usize {
            format!("{}…", &self.message[..width as usize - 1])
        } else {
            format!("{:width$}", self.message, width = width as usize)
        };
        
        print_at_with_bg(x, y, &padded_message, color, UIColor::DarkGray);
    }
    
    pub fn set_message(&mut self, message: String, message_type: MessageType) {
        self.message = message;
        self.message_type = message_type;
    }

    pub fn set_info(&mut self, message: &str) {
        self.set_message(message.to_string(), MessageType::Info);
    }

    pub fn set_error(&mut self, message: &str) {
        self.set_message(message.to_string(), MessageType::Error);
    }

    pub fn set_warning(&mut self, message: &str) {
        self.set_message(message.to_string(), MessageType::Warning);
    }

    pub fn set_success(&mut self, message: &str) {
        self.set_message(message.to_string(), MessageType::Success);
    }
    
    pub fn clear(&mut self) {
        self.message.clear();
        self.message_type = MessageType::Info;
    }
}

#[derive(Debug, Clone)]
pub enum MessageType {
    Info,
    Warning,
    Error,
    Success,
}

// Вспомогательные функции для вывода
pub fn print_at(x: u16, y: u16, text: &str, color: UIColor) {
    print_at_with_bg(x, y, text, color, UIColor::Black);
}

pub fn print_at_with_bg(x: u16, y: u16, text: &str, fg: UIColor, bg: UIColor) {
    crate::ui::screen::move_cursor(x, y);
    print!("\x1b[{};{}m{}", fg.to_ansi_fg(), bg.to_ansi_bg(), text);
    print!("\x1b[0m"); // Сброс цвета
    io::stdout().flush().unwrap();
}
