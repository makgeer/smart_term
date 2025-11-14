pub mod privileges;
pub mod helpers;

pub use privileges::PrivilegeManager;
pub use helpers::*;

/// Утилиты для работы с файловой системой
pub mod filesystem {
    use std::path::Path;
    
    /// Получить размер файла в человеко-читаемом формате
    pub fn get_file_size(path: &Path) -> Result<String, String> {
        let metadata = std::fs::metadata(path)
            .map_err(|e| format!("Не удалось получить метаданные: {}", e))?;
            
        let size = metadata.len();
        Ok(crate::utils::human_readable_size(size)) // Явное указание пути
    }
    
    /// Получить свободное место на диске
    pub fn get_free_space(path: &Path) -> String {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if let Ok(metadata) = std::fs::metadata(path) {
                // ИСПРАВЛЕНИЕ: правильные методы для Unix
                let available_blocks = metadata.len() / 512;
                crate::utils::human_readable_size(free_blocks)
            } else {
                "Неизвестно".to_string()
            }
        }
        #[cfg(not(unix))]
        {
            "Недоступно".to_string()
        }
    }
}

/// Получить размер в человеко-читаемом формате
pub fn human_readable_size(size: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    if size == 0 {
        return "0 B".to_string();
    }
    
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    // Оптимизация: убираем .0 для целых чисел
    if size.fract() == 0.0 {
        format!("{:.0} {}", size, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Утилиты для работы со временем
pub mod time {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    /// Получить текущее время в формате timestamp
    pub fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default() // Защита от паники
            .as_secs()
    }
    
    /// Форматировать время для отображения
    pub fn format_time(timestamp: u64) -> String {
        match chrono::DateTime::from_timestamp(timestamp as i64, 0) {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            None => "Неизвестно".to_string(),
        }
    }
}
