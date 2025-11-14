use crate::git::GitManager;
use crate::ui::{UIColor, screen};
use std::io::{self, Write};

#[derive(Debug)]
pub struct GitWidget {
    git: GitManager,
    is_visible: bool,
}

impl GitWidget {
    pub fn new() -> Self {
        Self {
            git: GitManager::new(),
            is_visible: false,
        }
    }
    
    pub fn toggle_visibility(&mut self) {
    self.is_visible = !self.is_visible;
    }
    
    pub fn draw(&self, x: u16, y: u16, width: u16, height: u16) {
        if !self.is_visible {
            return;
        }
        
        // Рамка Git панели
        self.draw_border(x, y, width, height);
        
        // Заголовок
        let title = " Git Status ";
        print_at(x + 1, y, title, UIColor::Yellow);
        
        // Получаем статус
        match self.git.get_visual_status() {
            Ok(status) => {
                self.draw_status(x, y + 1, width, height - 1, &status);
            }
            Err(_) => {
                print_at(x + 2, y + 1, "Not a git repository", UIColor::Red);
            }
        }
    }
    
    fn draw_border(&self, x: u16, y: u16, width: u16, height: u16) {
        // Углы
        print_at(x, y, "┌", UIColor::Green);
        print_at(x + width - 1, y, "┐", UIColor::Green);
        print_at(x, y + height - 1, "└", UIColor::Green);
        print_at(x + width - 1, y + height - 1, "┘", UIColor::Green);
        
        // Горизонтальные линии
        for i in 1..width-1 {
            print_at(x + i, y, "─", UIColor::Green);
            print_at(x + i, y + height - 1, "─", UIColor::Green);
        }
        
        // Вертикальные линии
        for i in 1..height-1 {
            print_at(x, y + i, "│", UIColor::Green);
            print_at(x + width - 1, y + i, "│", UIColor::Green);
        }
    }
    
    fn draw_status(&self, x: u16, y: u16, width: u16, height: u16, status: &crate::git::GitStatus) {
        let mut current_y = y;
        
        // Ветка
        let branch_text = format!("Branch: {}", status.branch);
        print_at(x + 2, current_y, &branch_text, UIColor::Cyan);
        current_y += 1;
        
        current_y += 1; // Пустая строка
        
        // Staged файлы
        if !status.staged.is_empty() {
            print_at(x + 2, current_y, "Staged:", UIColor::Green);
            current_y += 1;
            
            for file in &status.staged {
                if current_y >= y + height - 1 {
                    break;
                }
                print_at(x + 4, current_y, &format!("✓ {}", file), UIColor::Green);
                current_y += 1;
            }
            current_y += 1;
        }
        
        // Unstaged файлы
        if !status.unstaged.is_empty() {
            print_at(x + 2, current_y, "Modified:", UIColor::Yellow);
            current_y += 1;
            
            for file in &status.unstaged {
                if current_y >= y + height - 1 {
                    break;
                }
                print_at(x + 4, current_y, &format!("• {}", file), UIColor::Yellow);
                current_y += 1;
            }
            current_y += 1;
        }
        
        // Untracked файлы
        if !status.untracked.is_empty() {
            print_at(x + 2, current_y, "Untracked:", UIColor::Red);
            current_y += 1;
            
            for file in &status.untracked {
                if current_y >= y + height - 1 {
                    break;
                }
                print_at(x + 4, current_y, &format!("? {}", file), UIColor::Red);
                current_y += 1;
            }
        }
    }
    
    pub fn is_visible(&self) -> bool {
        self.is_visible
    }
    
    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }
}

// Вспомогательные функции для вывода
pub fn print_at(x: u16, y: u16, text: &str, color: UIColor) {
    screen::move_cursor(x, y);
    print!("\x1b[{}m{}", color.to_ansi_fg(), text);
    print!("\x1b[0m");
    io::stdout().flush().unwrap();
}

pub fn print_at_char(x: u16, y: u16, ch: char, color: UIColor) {
    screen::move_cursor(x, y);
    print!("\x1b[{}m{}", color.to_ansi_fg(), ch);
    print!("\x1b[0m");
    io::stdout().flush().unwrap();
}
