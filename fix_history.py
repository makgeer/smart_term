content = '''use std::collections::VecDeque;
use std::time::SystemTime;

const DEFAULT_MAX_SIZE: usize = 1000;

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub command: String,
    pub timestamp: SystemTime,
    pub duration: Option<std::time::Duration>,
}

impl HistoryEntry {
    pub fn new(command: String) -> Self {
        Self {
            command,
            timestamp: SystemTime::now(),
            duration: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct History {
    entries: VecDeque<HistoryEntry>,
    max_size: usize,
    current_index: usize,
}

impl History {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(max_size),
            max_size,
            current_index: 0,
        }
    }

    pub fn add(&mut self, entry: HistoryEntry) {
        if self.entries.len() >= self.max_size {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
        self.current_index = self.entries.len();
    }

    pub fn add_command(&mut self, command: String) {
        self.add(HistoryEntry::new(command));
    }

    pub fn get_previous(&mut self) -> Option<&HistoryEntry> {
        if self.entries.is_empty() {
            return None;
        }
        if self.current_index > 0 {
            self.current_index -= 1;
        }
        self.entries.get(self.current_index)
    }

    pub fn get_next(&mut self) -> Option<&HistoryEntry> {
        if self.entries.is_empty() {
            return None;
        }
        if self.current_index < self.entries.len() - 1 {
            self.current_index += 1;
        }
        self.entries.get(self.current_index)
    }

    pub fn search(&self, pattern: &str) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.command.contains(pattern))
            .collect()
    }

    pub fn get_all(&self) -> Vec<&HistoryEntry> {
        self.entries.iter().collect()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.current_index = 0;
    }
}
'''
with open('src/terminal/history.rs', 'w', encoding='utf-8') as f:
    f.write(content)
