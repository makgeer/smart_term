content = '''use crate::editor::EditorResult;
use crate::ui::keybindings::KeyPress;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{stdout, Write};

pub struct MicroEditor {
    file_path: Option<String>,
    content: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
    offset_x: usize,
    offset_y: usize,
    screen_width: u16,
    screen_height: u16,
    should_quit: bool,
}

impl MicroEditor {
    pub fn new() -> Self {
        let (screen_width, screen_height) = terminal::size().unwrap_or((80, 24));
        
        Self {
            file_path: None,
            content: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
            offset_x: 0,
            offset_y: 0,
            screen_width,
            screen_height,
            should_quit: false,
        }
    }

    pub fn open_file(&mut self, filename: &str) -> EditorResult<()> {
        match std::fs::read_to_string(filename) {
            Ok(content) => {
                self.file_path = Some(filename.to_string());
                self.content = if content.is_empty() {
                    vec![String::new()]
                } else {
                    content.lines().map(|s| s.to_string()).collect()
                };
                self.cursor_x = 0;
                self.cursor_y = 0;
                self.offset_x = 0;
                self.offset_y = 0;
                Ok(())
            }
            Err(e) => {
                let error_msg = format!("Failed to open file {}: {}", filename, e);
                Err(error_msg)
            }
        }
    }

    pub fn run(&mut self) -> EditorResult<()> {
        self.setup_terminal()?;

        while !self.should_quit {
            self.draw()?;
            
            if let Some(key) = self.read_key()? {
                self.handle_key(key);
            }
        }

        self.cleanup_terminal()?;
        Ok(())
    }

    fn setup_terminal(&self) -> EditorResult<()> {
        enable_raw_mode()
            .map_err(|e| format!("Failed to enable raw mode: {}", e))?;
        
        execute!(
            stdout(),
            Hide,
            Clear(ClearType::All),
        ).map_err(|e| format!("Failed to hide cursor: {}", e))?;

        Ok(())
    }

    fn cleanup_terminal(&self) -> EditorResult<()> {
        execute!(
            stdout(),
            Show,
            MoveTo(0, 0),
            ResetColor,
        ).map_err(|e| format!("Failed to show cursor: {}", e))?;

        disable_raw_mode()
            .map_err(|e| format!("Failed to disable raw mode: {}", e))?;

        Ok(())
    }

    fn draw(&self) -> EditorResult<()> {
        execute!(stdout(), MoveTo(0, 0), Clear(ClearType::All))
            .map_err(|e| format!("Screen clear error: {}", e))?;

        for (line_num, line) in self.content.iter().enumerate().skip(self.offset_y).take(self.screen_height as usize - 2) {
            if line_num >= self.content.len() {
                break;
            }

            let display_line = if line.len() > self.offset_x {
                &line[self.offset_x..]
            } else {
                ""
            };

            let truncated_line = if display_line.len() > self.screen_width as usize {
                &display_line[..self.screen_width as usize]
            } else {
                display_line
            };

            execute!(
                stdout(),
                MoveTo(0, line_num as u16 - self.offset_y as u16),
                Print(truncated_line)
            ).map_err(|e| format!("Line render error: {}", e))?;
        }

        let status_line = self.screen_height - 1;
        let file_info = self.file_path.as_ref().map_or("New File".to_string(), |p| p.clone());
        let cursor_info = format!("Line: {}, Column: {}", self.cursor_y + 1, self.cursor_x + 1);
        
        execute!(
            stdout(),
            MoveTo(0, status_line),
            SetBackgroundColor(Color::Blue),
            SetForegroundColor(Color::White),
            Clear(ClearType::CurrentLine),
            Print(format!("{} | {}", file_info, cursor_info))
        ).map_err(|e| format!("Status bar render error: {}", e))?;

        let cursor_screen_x = (self.cursor_x - self.offset_x) as u16;
        let cursor_screen_y = (self.cursor_y - self.offset_y) as u16;
        
        execute!(
            stdout(),
            MoveTo(cursor_screen_x, cursor_screen_y),
        ).map_err(|e| format!("Cursor positioning error: {}", e))?;

        stdout().flush().map_err(|e| format!("Buffer flush error: {}", e))?;

        Ok(())
    }

    fn handle_key(&mut self, key: KeyPress) {
        match key {
            KeyPress::Key(key_event) if key_event.modifiers.contains(KeyModifiers::CONTROL) 
                && matches!(key_event.code, KeyCode::Char('q')) => {
                self.should_quit = true;
            }
            KeyPress::Up => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.cursor_x.min(self.content[self.cursor_y].len());
                }
            }
            KeyPress::Down => {
                if self.cursor_y < self.content.len() - 1 {
                    self.cursor_y += 1;
                    self.cursor_x = self.cursor_x.min(self.content[self.cursor_y].len());
                }
            }
            KeyPress::Left => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.content[self.cursor_y].len();
                }
            }
            KeyPress::Right => {
                if self.cursor_x < self.content[self.cursor_y].len() {
                    self.cursor_x += 1;
                } else if self.cursor_y < self.content.len() - 1 {
                    self.cursor_y += 1;
                    self.cursor_x = 0;
                }
            }
            KeyPress::Backspace => {
                if self.cursor_x > 0 {
                    let current_line = &mut self.content[self.cursor_y];
                    current_line.remove(self.cursor_x - 1);
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    let current_line = self.content.remove(self.cursor_y);
                    self.cursor_y -= 1;
                    self.cursor_x = self.content[self.cursor_y].len();
                    self.content[self.cursor_y].push_str(&current_line);
                }
            }
            KeyPress::Enter => {
                let current_line = self.content[self.cursor_y].clone();
                let (left, right) = current_line.split_at(self.cursor_x);
                
                self.content[self.cursor_y] = left.to_string();
                self.content.insert(self.cursor_y + 1, right.to_string());
                
                self.cursor_y += 1;
                self.cursor_x = 0;
            }
            KeyPress::Char(c) => {
                let current_line = &mut self.content[self.cursor_y];
                current_line.insert(self.cursor_x, c);
                self.cursor_x += 1;
            }
            _ => {}
        }

        self.update_scroll();
    }

    fn update_scroll(&mut self) {
        if self.cursor_y < self.offset_y {
            self.offset_y = self.cursor_y;
        } else if self.cursor_y >= self.offset_y + self.screen_height as usize - 2 {
            self.offset_y = self.cursor_y - (self.screen_height as usize - 2) + 1;
        }

        if self.cursor_x < self.offset_x {
            self.offset_x = self.cursor_x;
        } else if self.cursor_x >= self.offset_x + self.screen_width as usize {
            self.offset_x = self.cursor_x - self.screen_width as usize + 1;
        }
    }

    fn read_key(&self) -> EditorResult<Option<KeyPress>> {
        if event::poll(std::time::Duration::from_millis(100))
            .map_err(|e| format!("Event poll error: {}", e))?
        {
            if let Event::Key(key_event) = event::read()
                .map_err(|e| format!("Event read error: {}", e))?
            {
                let key_press = match key_event {
                    KeyEvent {
                        code: KeyCode::Char(c),
                        modifiers: KeyModifiers::ALT,
                        ..
                    } => KeyPress::Alt(c),
                    KeyEvent {
                        code: KeyCode::Char(c),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => KeyPress::Ctrl(c),
                    KeyEvent {
                        code: KeyCode::Char(c),
                        ..
                    } => KeyPress::Char(c),
                    KeyEvent {
                        code: KeyCode::Up, ..
                    } => KeyPress::Up,
                    KeyEvent {
                        code: KeyCode::Down, ..
                    } => KeyPress::Down,
                    KeyEvent {
                        code: KeyCode::Left, ..
                    } => KeyPress::Left,
                    KeyEvent {
                        code: KeyCode::Right, ..
                    } => KeyPress::Right,
                    KeyEvent {
                        code: KeyCode::Enter, ..
                    } => KeyPress::Enter,
                    KeyEvent {
                        code: KeyCode::Backspace, ..
                    } => KeyPress::Backspace,
                    KeyEvent {
                        code: KeyCode::Tab, ..
                    } => KeyPress::Tab,
                    KeyEvent {
                        code: KeyCode::Delete, ..
                    } => KeyPress::Delete,
                    KeyEvent {
                        code: KeyCode::Home, ..
                    } => KeyPress::Home,
                    KeyEvent {
                        code: KeyCode::End, ..
                    } => KeyPress::End,
                    KeyEvent {
                        code: KeyCode::PageUp, ..
                    } => KeyPress::PageUp,
                    KeyEvent {
                        code: KeyCode::PageDown, ..
                    } => KeyPress::PageDown,
                    KeyEvent {
                        code: KeyCode::Esc, ..
                    } => KeyPress::Esc,
                    _ => return Ok(None),
                };
                return Ok(Some(key_press));
            }
        }
        Ok(None)
    }

    pub fn save_file(&self) -> EditorResult<()> {
        if let Some(ref path) = self.file_path {
            let content = self.content.join("\\n");
            std::fs::write(path, content)
                .map_err(|e| format!("Failed to save file {}: {}", path, e))?;
            Ok(())
        } else {
            Err("File has no path for saving".to_string())
        }
    }

    pub fn save_file_as(&mut self, filename: &str) -> EditorResult<()> {
        self.file_path = Some(filename.to_string());
        self.save_file()
    }
}
'''
with open('src/editor/micro_like.rs', 'w', encoding='utf-8') as f:
    f.write(content)
