#[derive(Debug, Clone)]
pub struct CommandHistory {
    commands: Vec<String>,
    max_size: usize,
    current_index: usize,
}

impl CommandHistory {
    pub fn new(max_size: usize) -> Self {
        Self {
            commands: Vec::with_capacity(max_size),
            max_size,
            current_index: 0,
        }
    }
    
    pub fn add(&mut self, command: String) {
        let command = command.trim().to_string();
        
        // Не добавляем пустые команды и дубликаты подряд
        if !command.is_empty() && self.commands.last() != Some(&command) {
            if self.commands.len() >= self.max_size {
                self.commands.remove(0);
            }
            self.commands.push(command);
        }
        self.current_index = self.commands.len();
    }
    
    pub fn get_previous(&mut self) -> Option<&String> {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.commands.get(self.current_index)
        } else {
            None
        }
    }
    
    pub fn get_next(&mut self) -> Option<&String> {
        if self.current_index < self.commands.len() - 1 {
            self.current_index += 1;
            self.commands.get(self.current_index)
        } else {
            self.current_index = self.commands.len();
            None
        }
    }
    
    pub fn get_all(&self) -> &Vec<String> {
        &self.commands
    }
    
    pub fn clear(&mut self) {
        self.commands.clear();
        self.current_index = 0;
    }
    
    pub fn search(&self, query: &str) -> Vec<&String> {
        self.commands
            .iter()
            .filter(|cmd| cmd.contains(query))
            .collect()
    }
    
    pub fn len(&self) -> usize {
        self.commands.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
    
    pub fn get_by_index(&self, index: usize) -> Option<&String> {
        self.commands.get(index)
    }
    
    pub fn remove(&mut self, index: usize) -> Option<String> {
        if index < self.commands.len() {
            let removed = self.commands.remove(index);
            if self.current_index >= index {
                self.current_index = self.current_index.saturating_sub(1);
            }
            Some(removed)
        } else {
            None
        }
    }
}
