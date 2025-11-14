#[derive(Debug, Clone, PartialEq)]
pub struct KeyBindings {
    pub bindings: Vec<KeyBinding>,
}

impl KeyBindings {
    pub fn new() -> Self {
        let mut bindings = Vec::new();
        
        // Навигация по файлам
        bindings.push(KeyBinding::new(KeyPress::Key(crossterm::event::KeyCode::Up)), "move_up", "Перемещение вверх"));
        bindings.push(KeyBinding::new(KeyPress::Key(crossterm::event::KeyCode::Down)), "move_down", "Перемещение вниз"));
        bindings.push(KeyBinding::new(KeyPress::Key(crossterm::event::KeyCode::PageUp)), "page_up", "Страница вверх"));
        bindings.push(KeyBinding::new(KeyPress::Key(crossterm::event::KeyCode::PageDown)), "page_down", "Страница вниз"));
        bindings.push(KeyBinding::new(KeyPress::Key(crossterm::event::KeyCode::Home)), "go_home", "В начало"));
        bindings.push(KeyBinding::new(KeyPress::Key(crossterm::event::KeyCode::End)), "go_end", "В конец"));
        
        // Основные действия
        bindings.push(KeyBinding::new(KeyPress::Key(Key::Char(' ')), "select_file", "Выбор файла"));
        bindings.push(KeyBinding::new(KeyPress::Key(Key::Enter), "open_file", "Открыть файл/директорию"));
        bindings.push(KeyBinding::new(KeyPress::Key(Key::Backspace), "go_up", "На уровень вверх"));
        
        // Панели
        bindings.push(KeyBinding::new(KeyPress::Key(Key::Tab), "switch_panel", "Переключение панелей"));
        bindings.push(KeyBinding::new(KeyPress::F(1), "left_panel", "Левая панель"));
        bindings.push(KeyBinding::new(KeyPress::F(2), "right_panel", "Правая панель"));
        
        // Функциональные клавиши
        bindings.push(KeyBinding::new(KeyPress::F(3), "view_file", "Просмотр файла"));
        bindings.push(KeyBinding::new(KeyPress::F(4), "edit_file", "Редактировать файл"));
        bindings.push(KeyBinding::new(KeyPress::F(5), "copy_file", "Копировать файл"));
        bindings.push(KeyBinding::new(KeyPress::F(6), "move_file", "Переместить файл"));
        bindings.push(KeyBinding::new(KeyPress::F(7), "mkdir", "Создать директорию"));
        bindings.push(KeyBinding::new(KeyPress::F(8), "delete_file", "Удалить файл"));
        bindings.push(KeyBinding::new(KeyPress::F(9), "menu", "Меню"));
        bindings.push(KeyBinding::new(KeyPress::F(10), "exit", "Выход"));
        
        // Комбинации клавиш
        bindings.push(KeyBinding::new(KeyPress::Ctrl('r'), "refresh", "Обновить"));
        bindings.push(KeyBinding::new(KeyPress::Ctrl('l'), "clear", "Очистить экран"));
        bindings.push(KeyBinding::new(KeyPress::Alt('h'), "show_hidden", "Показать скрытые"));
        bindings.push(KeyBinding::new(KeyPress::Ctrl('q'), "exit", "Выход в текстовый режим"));
        bindings.push(KeyBinding::new(KeyPress::Key(Key::Char(':')), "command_mode", "Командный режим"));
        bindings.push(KeyBinding::new(KeyPress::Ctrl('g'), "git_status", "Git статус"));
        
        Self { bindings }
    }
    
    pub fn find_binding(&self, key: &KeyPress) -> Option<&KeyBinding> {
        self.bindings.iter().find(|b| &b.key == key)
    }
    
    pub fn get_help(&self) -> String {
        let mut help = String::new();
        help.push_str("Горячие клавиши:\n");
        
        for binding in &self.bindings {
            help.push_str(&format!("{:15} - {}\n", binding.key.to_string(), binding.description));
        }
        
        help
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyBinding {
    pub key: KeyPress,
    pub action: String,
    pub description: String,
}

impl KeyBinding {
    pub fn new(key: KeyPress, action: &str, description: &str) -> Self {
        Self {
            key,
            action: action.to_string(),
            description: description.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyPress {
    Key(crossterm::event::KeyEvent),
    // Или определить свои варианты:
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Enter,
    Tab,
    Backspace,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Esc,
    F(u8),
    Alt(char),
    Ctrl(char),
    Shift(KeyPress), // для комбинаций
}

impl KeyPress {
    pub fn to_string(&self) -> String {
        match self {
            KeyPress::Key(Key::Char(c)) => format!("{}", c),
            KeyPress::Key(Key::Up) => "↑".to_string(),
            KeyPress::Key(Key::Down) => "↓".to_string(),
            KeyPress::Key(Key::Left) => "←".to_string(),
            KeyPress::Key(Key::Right) => "→".to_string(),
            KeyPress::Key(Key::Enter) => "Enter".to_string(),
            KeyPress::Key(Key::Tab) => "Tab".to_string(),
            KeyPress::Key(Key::Backspace) => "Backspace".to_string(),
            KeyPress::Key(Key::Delete) => "Del".to_string(),
            KeyPress::Key(Key::Home) => "Home".to_string(),
            KeyPress::Key(Key::End) => "End".to_string(),
            KeyPress::Key(Key::PageUp) => "PgUp".to_string(),
            KeyPress::Key(Key::PageDown) => "PgDown".to_string(),
            KeyPress::Key(Key::Esc) => "Esc".to_string(),
            KeyPress::Ctrl(c) => format!("Ctrl-{}", c),
            KeyPress::Alt(c) => format!("Alt-{}", c),
            KeyPress::Shift(key) => format!("Shift-{}", key.to_string()),
            KeyPress::F(n) => format!("F{}", n),
        }
    }
}
