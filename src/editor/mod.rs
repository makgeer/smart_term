//! Модуль редактора для работы с текстовыми файлами

pub mod micro_like;
pub mod text_buffer;
pub mod syntax;

use std::path::Path;

/// Результат операций редактора
pub type EditorResult<T> = Result<T, String>;

/// Тип файла для подсветки синтаксиса
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Html,
    Css,
    Markdown,
    Toml,
    Json,
    Yaml,
    PlainText,
    Unknown,
}

impl FileType {
    /// Определяет тип файла по расширению
    pub fn from_path(path: &Path) -> Self {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => FileType::Rust,
            Some("py") => FileType::Python,
            Some("js") => FileType::JavaScript,
            Some("ts") => FileType::TypeScript,
            Some("html") | Some("htm") => FileType::Html,
            Some("css") => FileType::Css,
            Some("md") => FileType::Markdown,
            Some("toml") => FileType::Toml,
            Some("json") => FileType::Json,
            Some("yaml") | Some("yml") => FileType::Yaml,
            Some("txt") => FileType::PlainText,
            _ => FileType::Unknown,
        }
    }

    /// Возвращает расширения файлов для этого типа
    pub fn extensions(&self) -> Vec<&'static str> {
        match self {
            FileType::Rust => vec!["rs"],
            FileType::Python => vec!["py"],
            FileType::JavaScript => vec!["js"],
            FileType::TypeScript => vec!["ts"],
            FileType::Html => vec!["html", "htm"],
            FileType::Css => vec!["css"],
            FileType::Markdown => vec!["md"],
            FileType::Toml => vec!["toml"],
            FileType::Json => vec!["json"],
            FileType::Yaml => vec!["yaml", "yml"],
            FileType::PlainText => vec!["txt"],
            FileType::Unknown => vec![],
        }
    }

    /// Возвращает имя языка для подсветки синтаксиса
    pub fn language_name(&self) -> &'static str {
        match self {
            FileType::Rust => "Rust",
            FileType::Python => "Python",
            FileType::JavaScript => "JavaScript",
            FileType::TypeScript => "TypeScript",
            FileType::Html => "HTML",
            FileType::Css => "CSS",
            FileType::Markdown => "Markdown",
            FileType::Toml => "TOML",
            FileType::Json => "JSON",
            FileType::Yaml => "YAML",
            FileType::PlainText => "Plain Text",
            FileType::Unknown => "Unknown",
        }
    }
}

/// Позиция в тексте (строка, колонка)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextPosition {
    pub line: usize,
    pub column: usize,
}

impl TextPosition {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }
}

/// Выделение текста
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextSelection {
    pub start: TextPosition,
    pub end: TextPosition,
}

impl TextSelection {
    pub fn new(start: TextPosition, end: TextPosition) -> Self {
        Self { start, end }
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn normalize(&self) -> Self {
        if self.start.line < self.end.line || 
           (self.start.line == self.end.line && self.start.column <= self.end.column) {
            self.clone()
        } else {
            Self::new(self.end, self.start)
        }
    }
}

/// Настройки редактора
#[derive(Debug, Clone)]
pub struct EditorSettings {
    pub tab_size: usize,
    pub show_line_numbers: bool,
    pub syntax_highlighting: bool,
    pub word_wrap: bool,
    pub auto_indent: bool,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            tab_size: 4,
            show_line_numbers: true,
            syntax_highlighting: true,
            word_wrap: false,
            auto_indent: true,
        }
    }
}

/// Тrait для редакторов
pub trait TextEditor {
    /// Открывает файл для редактирования
    fn open_file(&mut self, filename: &str) -> EditorResult<()>;
    
    /// Сохраняет файл
    fn save_file(&self) -> EditorResult<()>;
    
    /// Сохраняет файл с новым именем
    fn save_file_as(&mut self, filename: &str) -> EditorResult<()>;
    
    /// Запускает основной цикл редактора
    fn run(&mut self) -> EditorResult<()>;
    
    /// Получает текущую позицию курсора
    fn cursor_position(&self) -> TextPosition;
    
    /// Устанавливает позицию курсора
    fn set_cursor_position(&mut self, position: TextPosition);
    
    /// Получает текущее выделение
    fn selection(&self) -> Option<TextSelection>;
    
    /// Устанавливает выделение
    fn set_selection(&mut self, selection: Option<TextSelection>);
    
    /// Получает содержимое редактора
    fn content(&self) -> &[String];
    
    /// Вставляет текст в текущую позицию
    fn insert_text(&mut self, text: &str) -> EditorResult<()>;
    
    /// Удаляет выделенный текст
    fn delete_selection(&mut self) -> EditorResult<()>;
    
    /// Отменяет последнее действие
    fn undo(&mut self) -> EditorResult<()>;
    
    /// Повторяет последнее отмененное действие
    fn redo(&mut self) -> EditorResult<()>;
}

/// Базовые операции с текстом
pub mod text_operations {
    use super::EditorResult;

    /// Разбивает текст на строки с обработкой разных форматов концов строк
    pub fn split_lines(text: &str) -> Vec<String> {
        text.lines()
            .map(|line| line.to_string())
            .collect()
    }

    /// Объединяет строки в текст с унифицированными концами строк
    pub fn join_lines(lines: &[String]) -> String {
        lines.join("\n")
    }

    /// Вычисляет отступ для строки
    pub fn calculate_indent(line: &str) -> usize {
        line.chars()
            .take_while(|c| c.is_whitespace())
            .map(|c| if c == '\t' { 4 } else { 1 }) // Таб = 4 пробела
            .sum()
    }

    /// Нормализует отступы (заменяет табы на пробелы)
    pub fn normalize_indent(text: &str, tab_size: usize) -> String {
        text.replace("\t", &" ".repeat(tab_size))
    }

    /// Находит позицию в тексте по индексу символа
    pub fn find_position(lines: &[String], char_index: usize) -> EditorResult<(usize, usize)> {
        let mut current_index = 0;
        
        for (line_num, line) in lines.iter().enumerate() {
            let line_length = line.len() + 1; // +1 для символа новой строки
            
            if char_index < current_index + line_length {
                let column = char_index - current_index;
                return Ok((line_num, column.min(line.len())));
            }
            
            current_index += line_length;
        }
        
        Err("Индекс за пределами текста".to_string())
    }

    /// Находит индекс символа по позиции
    pub fn find_char_index(lines: &[String], line: usize, column: usize) -> EditorResult<usize> {
        if line >= lines.len() {
            return Err("Номер строки за пределами текста".to_string());
        }
        
        let mut index = 0;
        
        for (i, current_line) in lines.iter().enumerate() {
            if i == line {
                let col = column.min(current_line.len());
                return Ok(index + col);
            }
            index += current_line.len() + 1; // +1 для символа новой строки
        }
        
        Ok(index)
    }
}

/// Утилиты для работы с редактором
pub mod utils {
    use super::{EditorResult, FileType};
    use std::path::Path;

    /// Проверяет, является ли файл текстовым
    pub fn is_text_file(path: &Path) -> bool {
        if let Ok(metadata) = std::fs::metadata(path) {
            if metadata.len() > 10 * 1024 * 1024 { // 10MB лимит
                return false;
            }
        }

        // Проверяем по расширению
        match FileType::from_path(path) {
            FileType::Unknown => false,
            _ => true,
        }
    }

    /// Читает файл с проверкой кодировки
    pub fn read_text_file(path: &Path) -> EditorResult<String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Не удалось прочитать файл {}: {}", path.display(), e))?;

        // Базовая проверка на бинарный файл
        if content.contains('\0') {
            return Err("Файл похож на бинарный".to_string());
        }

        Ok(content)
    }

    /// Создает резервную копию файла
    pub fn create_backup(path: &Path) -> EditorResult<()> {
        let backup_path = path.with_extension("bak");
        std::fs::copy(path, backup_path)
            .map_err(|e| format!("Не удалось создать резервную копию: {}", e))?;
        Ok(())
    }

    /// Проверяет права на запись в файл
    pub fn check_write_permission(path: &Path) -> bool {
        if let Ok(metadata) = std::fs::metadata(path) {
            !metadata.permissions().readonly()
        } else {
            // Если файла нет, проверяем права на запись в директорию
            if let Some(parent) = path.parent() {
                std::fs::metadata(parent)
                    .map(|m| !m.permissions().readonly())
                    .unwrap_or(false)
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_detection() {
        assert_eq!(FileType::from_path(Path::new("test.rs")), FileType::Rust);
        assert_eq!(FileType::from_path(Path::new("script.py")), FileType::Python);
        assert_eq!(FileType::from_path(Path::new("file.txt")), FileType::PlainText);
        assert_eq!(FileType::from_path(Path::new("unknown.xyz")), FileType::Unknown);
    }

    #[test]
    fn test_text_position() {
        let pos = TextPosition::new(5, 10);
        assert_eq!(pos.line, 5);
        assert_eq!(pos.column, 10);
    }

    #[test]
    fn test_text_operations() {
        let text = "line1\nline2\nline3";
        let lines = text_operations::split_lines(text);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line1");
        
        let joined = text_operations::join_lines(&lines);
        assert_eq!(joined, text);
        
        let indent = text_operations::calculate_indent("    text");
        assert_eq!(indent, 4);
    }
}
