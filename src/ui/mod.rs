pub mod ncurses_like;
pub mod panels;
pub mod widgets;
pub mod keybindings;
pub mod git_widget;

pub use ncurses_like::NcursesLikeUI;
pub use panels::{Panel, PanelType, FileEntry, FileType};
pub use widgets::{FilePanelWidget, CommandLineWidget, StatusBarWidget, MessageType};
pub use keybindings::{KeyBindings, KeyPress};
pub use git_widget::GitWidget;

/// Основные цвета для UI
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UIColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    DarkGray,
    Gray,
}

impl UIColor {
    pub fn to_fg_color(&self) -> i16 {
        match self {
            UIColor::Black => 30,
            UIColor::Red => 31,
            UIColor::Green => 32,
            UIColor::Yellow => 33,
            UIColor::Blue => 34,
            UIColor::Magenta => 35,
            UIColor::Cyan => 36,
            UIColor::White => 37,
            UIColor::DarkGray => 90,
            UIColor::Gray => 37, // Добавить обработку Gray
            // ИЛИ добавить wildcard:
            // _ => 37,
        }
    }


    pub fn to_ansi_bg(&self) -> u8 {
        match self {
            UIColor::Black => 40,
            UIColor::Red => 41,
            UIColor::Green => 42,
            UIColor::Yellow => 43,
            UIColor::Blue => 44,
            UIColor::Magenta => 45,
            UIColor::Cyan => 46,
            UIColor::White => 47,
            UIColor::DarkGray => 100,
        }
    }
}

/// Утилиты для работы с экраном
pub mod screen {
    use std::io::{self, Write};

    /// Очистить экран
    pub fn clear() {
        print!("\x1b[2J\x1b[H");
        io::stdout().flush().unwrap();
    }

    /// Переместить курсор
    pub fn move_cursor(x: u16, y: u16) {
        print!("\x1b[{};{}H", y + 1, x + 1);
        io::stdout().flush().unwrap();
    }

    /// Скрыть курсор
    pub fn hide_cursor() {
        print!("\x1b[?25l");
        io::stdout().flush().unwrap();
    }

    /// Показать курсор
    pub fn show_cursor() {
        print!("\x1b[?25h");
        io::stdout().flush().unwrap();
    }

    /// Очистить строку
    pub fn clear_line() {
        print!("\x1b[K");
        io::stdout().flush().unwrap();
    }

    /// Включить альтернативный буфер
    pub fn enable_alt_buffer() {
        print!("\x1b[?1049h");
        io::stdout().flush().unwrap();
    }

    /// Выключить альтернативный буфер
    pub fn disable_alt_buffer() {
        print!("\x1b[?1049l");
        io::stdout().flush().unwrap();
    }
}
