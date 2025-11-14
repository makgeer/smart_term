pub mod bash_help;

pub use bash_help::BashHelp;

/// Основные категории справки
#[derive(Debug, Clone)]
pub enum HelpCategory {
    FileOperations,
    TextProcessing,
    SystemOperations,
    NetworkCommands,
    ProcessManagement,
    UserManagement,
    BuiltinCommands,
}

impl HelpCategory {
    pub fn name(&self) -> &str {
        match self {
            HelpCategory::FileOperations => "Файловые операции",
            HelpCategory::TextProcessing => "Обработка текста",
            HelpCategory::SystemOperations => "Системные операции",
            HelpCategory::NetworkCommands => "Сетевые команды",
            HelpCategory::ProcessManagement => "Управление процессами",
            HelpCategory::UserManagement => "Управление пользователями",
            HelpCategory::BuiltinCommands => "Встроенные команды Bash",
        }
    }
    
    pub fn all_categories() -> Vec<HelpCategory> {
        vec![
            HelpCategory::FileOperations,
            HelpCategory::TextProcessing,
            HelpCategory::SystemOperations,
            HelpCategory::NetworkCommands,
            HelpCategory::ProcessManagement,
            HelpCategory::UserManagement,
            HelpCategory::BuiltinCommands,
        ]
    }
}
