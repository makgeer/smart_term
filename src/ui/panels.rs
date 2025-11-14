use std::path::PathBuf;
use crate::utils::filesystem;

#[derive(Debug, Clone)]
pub struct Panel {
    pub current_path: PathBuf,
    pub files: Vec<FileEntry>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub is_active: bool,
    pub panel_type: PanelType,
}

#[derive(Debug, Clone)]
pub enum PanelType {
    FileManager,
    CommandHistory,
    QuickView,
    TreeView,
}

impl Panel {
    pub fn new(path: PathBuf, panel_type: PanelType) -> Self {
        let files = Self::load_directory_listing(&path);
        
        Self {
            current_path: path,
            files,
            selected_index: 0,
            scroll_offset: 0,
            is_active: true,
            panel_type,
        }
    }
    
    pub fn load_directory_listing(path: &PathBuf) -> Vec<FileEntry> {
        let mut entries = Vec::new();
        
        // Добавляем ".." для навигации вверх
        if path.parent().is_some() {
            entries.push(FileEntry::new(
                "..".to_string(),
                0,
                FileType::Directory,
            ));
        }
        
        if let Ok(read_dir) = std::fs::read_dir(path) {
            for entry in read_dir.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    
                    // Пропускаем скрытые файлы (начинающиеся с .)
                    if file_name.starts_with('.') {
                        continue;
                    }
                    
                    let file_type = if metadata.is_dir() {
                        FileType::Directory
                    } else if metadata.is_file() {
                        FileType::File
                    } else {
                        FileType::Symlink
                    };
                    
                    let size = if metadata.is_file() {
                        metadata.len()
                    } else {
                        0
                    };
                    
                    entries.push(FileEntry::new(file_name, size, file_type));
                }
            }
        }
        
        entries.sort_by(|a, b| {
            // Сначала директории, потом файлы
            if a.file_type != b.file_type {
                a.file_type.cmp(&b.file_type).reverse()
            } else {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });
        
        entries
    }
    
    pub fn move_selection(&mut self, direction: i32) {
    let new_index = self.selected_index as i32 + direction;
    if new_index >= 0 && new_index < self.files.len() as i32 {
        self.selected_index = new_index as usize;
        
        // Переменная объявлена в нужной области видимости
        let visible_count = self.get_visible_files(20).len();
        
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + visible_count {
            self.scroll_offset = self.selected_index - visible_count + 1;
        }
    }
}
    
    pub fn get_selected_file(&self) -> Option<&FileEntry> {
        self.files.get(self.selected_index)
    }
    
    pub fn refresh(&mut self) {
        self.files = Self::load_directory_listing(&self.current_path);
        self.selected_index = self.selected_index.min(self.files.len().saturating_sub(1));
    }
    
    pub fn change_directory(&mut self, new_path: PathBuf) -> Result<(), String> {
        if new_path.exists() && new_path.is_dir() {
            self.current_path = new_path;
            self.refresh();
            self.selected_index = 0;
            Ok(())
        } else {
            Err("Директория не существует".to_string())
        }
    }
    
    pub fn get_visible_files(&self, height: usize) -> &[FileEntry] {
        let end = (self.scroll_offset + height).min(self.files.len());
        &self.files[self.scroll_offset..end]
    }

    pub fn go_up_directory(&mut self) -> Result<(), String> {
        if let Some(parent) = self.current_path.parent() {
            self.change_directory(parent.to_path_buf())
        } else {
            Err("Уже в корневой директории".to_string())
        }
    }

    pub fn refresh_files(&mut self) -> Result<(), String> {
        // Перечитываем файлы из текущей директории
        self.files = Self::read_directory(&self.current_path)?;
        self.selected_index = 0;
        self.scroll_offset = 0;
        Ok(())
    }
                
            // Авто-скроллинг если нужно
            let visible_count = self.get_visible_files(20).len(); // примерная высота
            if self.selected_index < self.scroll_offset {
                self.scroll_offset = self.selected_index;
            } else if self.selected_index >= self.scroll_offset + visible_count {
                self.scroll_offset = self.selected_index - visible_count + 1;
            }
        
    }


impl Panel {
    fn read_directory(path: &PathBuf) -> Result<Vec<FileEntry>, String> {
        // Реализация чтения директории
        let mut files = Vec::new();
        let entries = std::fs::read_dir(path)
            .map_err(|e| format!("Ошибка чтения директории: {}", e))?;
            
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let metadata = entry.metadata().unwrap();
                let file_type = if metadata.is_dir() {
                    FileType::Directory
                } else if metadata.file_type().is_symlink() {
                    FileType::Symlink
                } else {
                    FileType::File
                };
                
                files.push(FileEntry {
                    name: path.file_name().unwrap().to_string_lossy().to_string(),
                    size: metadata.len(),
                    file_type,
                });
            }
        }
        Ok(files)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
    pub file_type: FileType,
}

impl FileEntry {
    pub fn new(name: String, size: u64, file_type: FileType) -> Self {
        Self { name, size, file_type }
    }
    
    pub fn get_icon(&self) -> &str {
        match self.file_type {
            FileType::Directory => "📁",
            FileType::File => "📄",
            FileType::Symlink => "🔗",
        }
    }
    
    pub fn get_display_name(&self) -> String {
        if self.name == ".." {
            ".. [Наверх]".to_string()
        } else {
            self.name.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileType {
    Directory,
    File,
    Symlink,
}
