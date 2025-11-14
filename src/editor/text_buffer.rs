use crate::editor::{EditorResult, TextPosition, TextSelection};

/// Буфер для хранения и управления текстом
#[derive(Debug, Clone)]
pub struct TextBuffer {
    lines: Vec<String>,
    cursor: TextPosition,
    selection: Option<TextSelection>,
    undo_stack: Vec<TextBuffer>,
    redo_stack: Vec<TextBuffer>,
    max_undo_steps: usize,
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self {
            lines: vec![String::new()],
            cursor: TextPosition::zero(),
            selection: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_undo_steps: 100,
        }
    }
}

impl TextBuffer {
    /// Создает новый пустой текстовый буфер
    pub fn new() -> Self {
        Self::default()
    }

    /// Создает текстовый буфер из строки
    pub fn from_string(text: &str) -> Self {
        let lines = if text.is_empty() {
            vec![String::new()]
        } else {
            text.lines().map(|s| s.to_string()).collect()
        };

        Self {
            lines,
            ..Self::default()
        }
    }

    /// Возвращает содержимое буфера как строку
    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }

    /// Возвращает ссылку на строки буфера
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    /// Возвращает количество строк в буфере
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Возвращает длину конкретной строки
    pub fn line_length(&self, line: usize) -> Option<usize> {
        self.lines.get(line).map(|s| s.len())
    }

    /// Возвращает текущую позицию курсора
    pub fn cursor_position(&self) -> TextPosition {
        self.cursor
    }

    /// Устанавливает позицию курсора
    pub fn set_cursor_position(&mut self, position: TextPosition) -> EditorResult<()> {
        if position.line >= self.lines.len() {
            return Err(format!("Номер строки {} за пределами буфера", position.line));
        }

        let max_column = self.lines[position.line].len();
        if position.column > max_column {
            return Err(format!("Колонка {} за пределами строки {}", position.column, position.line));
        }

        self.cursor = position;
        self.clear_selection();
        Ok(())
    }

    /// Перемещает курсор относительно текущей позиции
    pub fn move_cursor(&mut self, delta_line: isize, delta_column: isize) -> EditorResult<()> {
        let new_line = (self.cursor.line as isize + delta_line).max(0) as usize;
        let new_line = new_line.min(self.lines.len().saturating_sub(1));

        let max_column = self.lines[new_line].len();
        let new_column = (self.cursor.column as isize + delta_column).max(0) as usize;
        let new_column = new_column.min(max_column);

        self.set_cursor_position(TextPosition::new(new_line, new_column))
    }

    /// Возвращает текущее выделение
    pub fn selection(&self) -> Option<&TextSelection> {
        self.selection.as_ref()
    }

    /// Устанавливает выделение
    pub fn set_selection(&mut self, selection: Option<TextSelection>) {
        self.selection = selection;
    }

    /// Очищает выделение
    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    /// Вставляет символ в текущую позицию курсора
    pub fn insert_char(&mut self, c: char) -> EditorResult<()> {
        self.save_undo_state();
        
        if let Some(selection) = self.selection.take() {
            self.delete_selection_impl(&selection)?;
        }

        let line = &mut self.lines[self.cursor.line];
        line.insert(self.cursor.column, c);
        self.cursor.column += 1;

        self.redo_stack.clear();
        Ok(())
    }

    /// Вставляет строку в текущую позицию курсора
    pub fn insert_string(&mut self, text: &str) -> EditorResult<()> {
        self.save_undo_state();

        if let Some(selection) = self.selection.take() {
            self.delete_selection_impl(&selection)?;
        }

        if text.contains('\n') {
            // Многострочная вставка
            let lines: Vec<&str> = text.lines().collect();
            self.insert_multiline(&lines)?;
        } else {
            // Однострочная вставка
            let line = &mut self.lines[self.cursor.line];
            line.insert_str(self.cursor.column, text);
            self.cursor.column += text.len();
        }

        self.redo_stack.clear();
        Ok(())
    }

    /// Удаляет символ перед курсором (Backspace)
    pub fn backspace(&mut self) -> EditorResult<()> {
        self.save_undo_state();

        if let Some(selection) = self.selection.take() {
            return self.delete_selection_impl(&selection);
        }

        if self.cursor.column > 0 {
            // Удаление в пределах строки
            let line = &mut self.lines[self.cursor.line];
            line.remove(self.cursor.column - 1);
            self.cursor.column -= 1;
        } else if self.cursor.line > 0 {
            // Слияние с предыдущей строкой
            let current_line = self.lines.remove(self.cursor.line);
            self.cursor.line -= 1;
            let prev_line_len = self.lines[self.cursor.line].len();
            self.lines[self.cursor.line].push_str(&current_line);
            self.cursor.column = prev_line_len;
        }

        self.redo_stack.clear();
        Ok(())
    }

    /// Удаляет символ после курсора (Delete)
    pub fn delete(&mut self) -> EditorResult<()> {
        self.save_undo_state();

        if let Some(selection) = self.selection.take() {
            return self.delete_selection_impl(&selection);
        }

        let current_line_len = self.lines[self.cursor.line].len();
        
        if self.cursor.column < current_line_len {
            // Удаление в пределах строки
            let line = &mut self.lines[self.cursor.line];
            line.remove(self.cursor.column);
        } else if self.cursor.line < self.lines.len() - 1 {
            // Слияние со следующей строкой
            let next_line = self.lines.remove(self.cursor.line + 1);
            self.lines[self.cursor.line].push_str(&next_line);
        }

        self.redo_stack.clear();
        Ok(())
    }

    /// Разбивает строку в позиции курсора (Enter)
    pub fn split_line(&mut self) -> EditorResult<()> {
        self.save_undo_state();

        if let Some(selection) = self.selection.take() {
            self.delete_selection_impl(&selection)?;
        }

        let current_line = self.lines[self.cursor.line].clone();
        let (left, right) = current_line.split_at(self.cursor.column);
        
        // Создаем копии строк до модификации вектора
        let left_str = left.to_string();
        let right_str = right.to_string();
        
        self.lines[self.cursor.line] = left_str;
        self.lines.insert(self.cursor.line + 1, right_str);
        
        self.cursor.line += 1;
        self.cursor.column = 0;

        self.redo_stack.clear();
        Ok(())
    }

    /// Удаляет выделенный текст
    pub fn delete_selection(&mut self) -> EditorResult<()> {
        if let Some(selection) = self.selection.take() {
            self.save_undo_state();
            self.delete_selection_impl(&selection)?;
            self.redo_stack.clear();
        }
        Ok(())
    }

    /// Получает выделенный текст
    pub fn get_selected_text(&self) -> Option<String> {
        self.selection.as_ref().map(|selection| {
            let normalized = selection.normalize();
            let mut result = String::new();
            
            for line in normalized.start.line..=normalized.end.line {
                if let Some(line_text) = self.lines.get(line) {
                    if line == normalized.start.line && line == normalized.end.line {
                        // Выделение в пределах одной строки
                        let start = normalized.start.column.min(line_text.len());
                        let end = normalized.end.column.min(line_text.len());
                        if start < end {
                            result.push_str(&line_text[start..end]);
                        }
                    } else if line == normalized.start.line {
                        // Первая строка выделения
                        let start = normalized.start.column.min(line_text.len());
                        result.push_str(&line_text[start..]);
                        result.push('\n');
                    } else if line == normalized.end.line {
                        // Последняя строка выделения
                        let end = normalized.end.column.min(line_text.len());
                        result.push_str(&line_text[..end]);
                    } else {
                        // Промежуточные строки
                        result.push_str(line_text);
                        result.push('\n');
                    }
                }
            }
            
            result
        })
    }

    /// Отменяет последнее действие
    pub fn undo(&mut self) -> EditorResult<()> {
        if let Some(previous_state) = self.undo_stack.pop() {
            let current_state = std::mem::replace(self, previous_state);
            self.redo_stack.push(current_state);
            
            if self.redo_stack.len() > self.max_undo_steps {
                self.redo_stack.remove(0);
            }
            Ok(())
        } else {
            Err("Нет действий для отмены".to_string())
        }
    }

    /// Повторяет последнее отмененное действие
    pub fn redo(&mut self) -> EditorResult<()> {
        if let Some(next_state) = self.redo_stack.pop() {
            let current_state = std::mem::replace(self, next_state);
            self.undo_stack.push(current_state);
            
            if self.undo_stack.len() > self.max_undo_steps {
                self.undo_stack.remove(0);
            }
            Ok(())
        } else {
            Err("Нет действий для повтора".to_string())
        }
    }

    /// Очищает историю undo/redo
    pub fn clear_history(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    // --- Приватные методы ---

    /// Сохраняет текущее состояние для undo
    fn save_undo_state(&mut self) {
        let current_state = self.clone();
        self.undo_stack.push(current_state);
        
        if self.undo_stack.len() > self.max_undo_steps {
            self.undo_stack.remove(0);
        }
    }

    /// Вставляет многострочный текст
    fn insert_multiline(&mut self, lines: &[&str]) -> EditorResult<()> {
        if lines.is_empty() {
            return Ok(());
        }

        let current_line = self.lines[self.cursor.line].clone();
        let (left_part, right_part) = current_line.split_at(self.cursor.column);
        
        let mut new_lines = Vec::with_capacity(lines.len());
        
        // Первая строка
        new_lines.push(format!("{}{}", left_part, lines[0]));
        
        // Промежуточные строки
        for &line in &lines[1..lines.len() - 1] {
            new_lines.push(line.to_string());
        }
        
        // Последняя строка
        if lines.len() > 1 {
            new_lines.push(format!("{}{}", lines[lines.len() - 1], right_part));
        } else {
            new_lines.push(right_part.to_string());
        }

        // Заменяем строки в буфере
        self.lines.splice(self.cursor.line..=self.cursor.line, new_lines);
        
        // Обновляем позицию курсора
        self.cursor.line += lines.len() - 1;
        self.cursor.column = lines[lines.len() - 1].len();
        
        Ok(())
    }

    /// Удаляет выделенный текст (внутренняя реализация)
    fn delete_selection_impl(&mut self, selection: &TextSelection) -> EditorResult<()> {
        let normalized = selection.normalize();
        
        if normalized.start.line == normalized.end.line {
            // Удаление в пределах одной строки
            let line = &mut self.lines[normalized.start.line];
            let start = normalized.start.column.min(line.len());
            let end = normalized.end.column.min(line.len());
            
            if start < end {
                line.replace_range(start..end, "");
            }
            
            self.cursor = normalized.start;
        } else {
            // Многострочное удаление
            let first_line_part = if normalized.start.column > 0 {
                self.lines[normalized.start.line][..normalized.start.column].to_string()
            } else {
                String::new()
            };
            
            let last_line_part = if normalized.end.column < self.lines[normalized.end.line].len() {
                self.lines[normalized.end.line][normalized.end.column..].to_string()
            } else {
                String::new()
            };
            
            // Собираем новую строку
            let new_line = format!("{}{}", first_line_part, last_line_part);
            
            // Удаляем диапазон строк
            self.lines.splice(
                normalized.start.line..=normalized.end.line,
                std::iter::once(new_line),
            );
            
            self.cursor = TextPosition::new(normalized.start.line, normalized.start.column);
        }
        
        // Убеждаемся, что буфер не пуст
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buffer = TextBuffer::new();
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.lines()[0], "");
        assert_eq!(buffer.cursor_position(), TextPosition::zero());
    }

    #[test]
    fn test_from_string() {
        let buffer = TextBuffer::from_string("hello\nworld");
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.lines()[0], "hello");
        assert_eq!(buffer.lines()[1], "world");
    }

    #[test]
    fn test_insert_char() {
        let mut buffer = TextBuffer::new();
        buffer.insert_char('a').unwrap();
        buffer.insert_char('b').unwrap();
        assert_eq!(buffer.lines()[0], "ab");
        assert_eq!(buffer.cursor_position().column, 2);
    }

    #[test]
    fn test_insert_string() {
        let mut buffer = TextBuffer::new();
        buffer.insert_string("hello").unwrap();
        assert_eq!(buffer.lines()[0], "hello");
        assert_eq!(buffer.cursor_position().column, 5);
    }

    #[test]
    fn test_backspace() {
        let mut buffer = TextBuffer::from_string("hello");
        buffer.set_cursor_position(TextPosition::new(0, 3)).unwrap();
        buffer.backspace().unwrap();
        assert_eq!(buffer.lines()[0], "helo");
        assert_eq!(buffer.cursor_position().column, 2);
    }

    #[test]
    fn test_split_line() {
        let mut buffer = TextBuffer::from_string("hello world");
        buffer.set_cursor_position(TextPosition::new(0, 5)).unwrap();
        buffer.split_line().unwrap();
        
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.lines()[0], "hello");
        assert_eq!(buffer.lines()[1], " world");
        assert_eq!(buffer.cursor_position(), TextPosition::new(1, 0));
    }

    #[test]
    fn test_undo_redo() {
        let mut buffer = TextBuffer::new();
        buffer.insert_char('a').unwrap();
        buffer.insert_char('b').unwrap();
        
        assert_eq!(buffer.lines()[0], "ab");
        
        buffer.undo().unwrap();
        assert_eq!(buffer.lines()[0], "a");
        
        buffer.redo().unwrap();
        assert_eq!(buffer.lines()[0], "ab");
    }

    #[test]
    fn test_selection() {
        let mut buffer = TextBuffer::from_string("hello world");
        let selection = TextSelection::new(
            TextPosition::new(0, 0),
            TextPosition::new(0, 5)
        );
        buffer.set_selection(Some(selection));
        
        let selected_text = buffer.get_selected_text();
        assert_eq!(selected_text, Some("hello".to_string()));
    }
}
