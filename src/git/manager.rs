use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};

/// Ошибки Git операций
#[derive(Debug, Clone)]
pub enum GitError {
    CommandFailed(String),
    ParseError(String),
    NoRepository,
    IOError(String),
}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::CommandFailed(msg) => write!(f, "Git command failed: {}", msg),
            GitError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            GitError::NoRepository => write!(f, "Not a git repository"),
            GitError::IOError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for GitError {}

impl From<GitError> for String {
    fn from(error: GitError) -> String {
        error.to_string()
    }
}

/// Информация о коммите
#[derive(Debug, Clone)]
pub struct GitCommit {
    pub hash: String,
    pub author: String,
    pub email: String,
    pub date: String,
    pub message: String,
    pub summary: String,
}

/// Информация о статусе файла
#[derive(Debug, Clone, PartialEq)]
pub enum FileStatus {
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Untracked,
    Conflicted,
}

/// Статус файла в репозитории
#[derive(Debug, Clone)]
pub struct GitFileStatus {
    pub path: PathBuf,
    pub status: FileStatus,
    pub staged: bool,
}

/// Статус репозитория
#[derive(Debug, Clone)]
pub struct RepositoryStatus {
    pub branch: String,
    pub upstream: Option<String>,
    pub ahead: usize,
    pub behind: usize,
    pub staged_files: Vec<GitFileStatus>,
    pub unstaged_files: Vec<GitFileStatus>,
    pub untracked_files: Vec<PathBuf>,
}

/// Менеджер для работы с Git
#[derive(Debug)]
pub struct GitManager {
    repo_path: PathBuf,
}

impl GitManager {
    /// Создает новый Git менеджер для указанного пути
    pub fn new(path: &Path) -> Result<Self, GitError> {
        let repo_path = Self::find_repository_root(path)?;
        Ok(Self { repo_path })
    }

    /// Находит корень git репозитория
    fn find_repository_root(path: &Path) -> Result<PathBuf, GitError> {
        let mut current = path.to_path_buf();
        
        loop {
            let git_dir = current.join(".git");
            if git_dir.exists() {
                return Ok(current);
            }
            
            if !current.pop() {
                return Err(GitError::NoRepository);
            }
        }
    }

    /// Выполняет git команду
    fn run_git_command(&self, args: &[&str]) -> Result<String, GitError> {
        let output = Command::new("git")
            .args(args)
            .current_dir(&self.repo_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| GitError::IOError(format!("Failed to execute git command: {}", e)))?;

        if output.status.success() {
            String::from_utf8(output.stdout)
                .map_err(|e| GitError::ParseError(format!("Invalid UTF-8 in output: {}", e)))
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(GitError::CommandFailed(error_msg.into_owned()))
        }
    }

    /// Получает историю коммитов
    pub fn get_log(&self, limit: Option<usize>) -> Result<Vec<GitCommit>, GitError> {
        let mut args = vec!["log", "--oneline", "--decorate", "--format=%H|%an|%ae|%ad|%s"];
        
        if let Some(limit) = limit {
            // Используем форматирование напрямую в push
            args.push(Box::leak(format!("-{}", limit).into_boxed_str()));
        }
        
        let output = self.run_git_command(&args)?;
        let mut commits = Vec::new();

        for line in output.lines() {
            let parts: Vec<&str> = line.splitn(5, '|').collect();
            if parts.len() == 5 {
                let commit = GitCommit {
                    hash: parts[0].to_string(),
                    author: parts[1].to_string(),
                    email: parts[2].to_string(),
                    date: parts[3].to_string(),
                    message: parts[4].to_string(),
                    summary: parts[4].chars().take(50).collect(),
                };
                commits.push(commit);
            }
        }

        Ok(commits)
    }

    /// Получает текущий статус репозитория
    pub fn get_status(&self) -> Result<RepositoryStatus, GitError> {
        // Получаем информацию о ветке
        let branch_output = self.run_git_command(&["branch", "--show-current"])?;
        let branch = branch_output.trim().to_string();

        // Получаем информацию об upstream
        let upstream_output = self.run_git_command(&["rev-parse", "--abbrev-ref", "@{upstream}"]);
        let upstream = upstream_output.ok().map(|s| s.trim().to_string());

        // Получаем информацию о расхождении с upstream
        let (ahead, behind) = if upstream.is_some() {
            let count_output = self.run_git_command(&["rev-list", "--count", "--left-right", "@{upstream}...HEAD"]).unwrap_or_default();
            let counts: Vec<&str> = count_output.trim().split('\t').collect();
            (
                counts.get(0).and_then(|s| s.parse().ok()).unwrap_or(0),
                counts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
            )
        } else {
            (0, 0)
        };

        // Получаем статус файлов
        let status_output = self.run_git_command(&["status", "--porcelain=v1"])?;
        let mut staged_files = Vec::new();
        let mut unstaged_files = Vec::new();
        let mut untracked_files = Vec::new();

        for line in status_output.lines() {
            if line.len() >= 3 {
                let status_code = &line[0..2];
                let file_path = &line[3..].trim();

                match status_code {
                    " M" => unstaged_files.push(GitFileStatus {
                        path: PathBuf::from(file_path),
                        status: FileStatus::Modified,
                        staged: false,
                    }),
                    "M " => staged_files.push(GitFileStatus {
                        path: PathBuf::from(file_path),
                        status: FileStatus::Modified,
                        staged: true,
                    }),
                    "MM" => {
                        staged_files.push(GitFileStatus {
                            path: PathBuf::from(file_path),
                            status: FileStatus::Modified,
                            staged: true,
                        });
                        unstaged_files.push(GitFileStatus {
                            path: PathBuf::from(file_path),
                            status: FileStatus::Modified,
                            staged: false,
                        });
                    }
                    " A" => unstaged_files.push(GitFileStatus {
                        path: PathBuf::from(file_path),
                        status: FileStatus::Added,
                        staged: false,
                    }),
                    "A " => staged_files.push(GitFileStatus {
                        path: PathBuf::from(file_path),
                        status: FileStatus::Added,
                        staged: true,
                    }),
                    " D" => unstaged_files.push(GitFileStatus {
                        path: PathBuf::from(file_path),
                        status: FileStatus::Deleted,
                        staged: false,
                    }),
                    "D " => staged_files.push(GitFileStatus {
                        path: PathBuf::from(file_path),
                        status: FileStatus::Deleted,
                        staged: true,
                    }),
                    "??" => untracked_files.push(PathBuf::from(file_path)),
                    _ => {} // Игнорируем другие статусы
                }
            }
        }

        Ok(RepositoryStatus {
            branch,
            upstream,
            ahead,
            behind,
            staged_files,
            unstaged_files,
            untracked_files,
        })
    }

    /// Добавляет файлы в индекс
    pub fn add_files(&self, files: &[&Path]) -> Result<(), GitError> {
        if files.is_empty() {
            return Ok(());
        }

        let mut args = vec!["add"];
        for file in files {
            if let Some(file_str) = file.to_str() {
                args.push(file_str);
            } else {
                return Err(GitError::ParseError("Invalid file path".to_string()));
            }
        }

        self.run_git_command(&args)?;
        Ok(())
    }

    /// Создает коммит
    pub fn commit(&self, message: &str) -> Result<(), GitError> {
        self.run_git_command(&["commit", "-m", message])?;
        Ok(())
    }

    /// Отправляет изменения в удаленный репозиторий
    pub fn push(&self) -> Result<(), GitError> {
        self.run_git_command(&["push"])?;
        Ok(())
    }

    /// Получает изменения из удаленного репозитория
    pub fn pull(&self) -> Result<(), GitError> {
        self.run_git_command(&["pull"])?;
        Ok(())
    }

    /// Переключается на другую ветку
    pub fn checkout(&self, branch: &str) -> Result<(), GitError> {
        self.run_git_command(&["checkout", branch])?;
        Ok(())
    }

    /// Создает новую ветку
    pub fn create_branch(&self, branch: &str) -> Result<(), GitError> {
        self.run_git_command(&["checkout", "-b", branch])?;
        Ok(())
    }

    /// Получает список веток
    pub fn get_branches(&self) -> Result<Vec<String>, GitError> {
        let output = self.run_git_command(&["branch", "--format=%(refname:short)"])?;
        let branches: Vec<String> = output
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        Ok(branches)
    }

    /// Получает текущую ветку
    pub fn get_current_branch(&self) -> Result<String, GitError> {
        let output = self.run_git_command(&["branch", "--show-current"])?;
        Ok(output.trim().to_string())
    }

    /// Проверяет, является ли путь git репозиторием
    pub fn is_repository(path: &Path) -> bool {
        Self::find_repository_root(path).is_ok()
    }

    /// Получает diff для файла
    pub fn get_file_diff(&self, file_path: &Path) -> Result<String, GitError> {
        if let Some(file_str) = file_path.to_str() {
            self.run_git_command(&["diff", file_str])
        } else {
            Err(GitError::ParseError("Invalid file path".to_string()))
        }
    }

    /// Получает staged diff для файла
    pub fn get_staged_diff(&self, file_path: &Path) -> Result<String, GitError> {
        if let Some(file_str) = file_path.to_str() {
            self.run_git_command(&["diff", "--staged", file_str])
        } else {
            Err(GitError::ParseError("Invalid file path".to_string()))
        }
    }

    /// Отменяет изменения в файле
    pub fn discard_changes(&self, file_path: &Path) -> Result<(), GitError> {
        if let Some(file_str) = file_path.to_str() {
            self.run_git_command(&["checkout", "--", file_str])?;
            Ok(())
        } else {
            Err(GitError::ParseError("Invalid file path".to_string()))
        }
    }

    /// Удаляет файл из индекса
    pub fn unstage_file(&self, file_path: &Path) -> Result<(), GitError> {
        if let Some(file_str) = file_path.to_str() {
            self.run_git_command(&["reset", "HEAD", "--", file_str])?;
            Ok(())
        } else {
            Err(GitError::ParseError("Invalid file path".to_string()))
        }
    }

    /// Получает визуальное представление статуса (для UI)
    pub fn get_visual_status(&self) -> Result<String, GitError> {
        let status = self.get_status()?;
        let mut output = String::new();
        
        output.push_str(&format!("На ветке: {}\n", status.branch));
        
        if let Some(upstream) = status.upstream {
            output.push_str(&format!("Ветка отслеживает: {}\n", upstream));
            if status.ahead > 0 || status.behind > 0 {
                output.push_str(&format!("Опережает на {} коммитов, отстает на {} коммитов\n", 
                    status.ahead, status.behind));
            }
        }
        
        if !status.staged_files.is_empty() {
            output.push_str("Изменения для коммита:\n");
            for file in status.staged_files {
                let status_char = match file.status {
                    FileStatus::Modified => "M",
                    FileStatus::Added => "A",
                    FileStatus::Deleted => "D",
                    _ => "?",
                };
                output.push_str(&format!("  {} {}\n", status_char, file.path.display()));
            }
        }
        
        if !status.unstaged_files.is_empty() {
            output.push_str("Не проиндексированные изменения:\n");
            for file in status.unstaged_files {
                let status_char = match file.status {
                    FileStatus::Modified => "M",
                    FileStatus::Added => "A",
                    FileStatus::Deleted => "D",
                    _ => "?",
                };
                output.push_str(&format!("  {} {}\n", status_char, file.path.display()));
            }
        }
        
        if !status.untracked_files.is_empty() {
            output.push_str("Неотслеживаемые файлы:\n");
            for file in status.untracked_files {
                output.push_str(&format!("  ? {}\n", file.display()));
            }
        }
        
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_find_repository_root() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_path = temp_dir.path();
        
        // Создаем mock git репозиторий
        fs::create_dir_all(repo_path.join(".git")).unwrap();
        
        let result = GitManager::find_repository_root(repo_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), repo_path);
    }

    #[test]
    fn test_not_a_repository() {
        let temp_dir = tempfile::tempdir().unwrap();
        let result = GitManager::find_repository_root(temp_dir.path());
        assert!(matches!(result, Err(GitError::NoRepository)));
    }

    #[test]
    fn test_git_manager_creation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_path = temp_dir.path();
        fs::create_dir_all(repo_path.join(".git")).unwrap();
        
        let manager = GitManager::new(repo_path);
        assert!(manager.is_ok());
    }

    #[test]
    fn test_is_repository() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_path = temp_dir.path();
        
        assert!(!GitManager::is_repository(repo_path));
        
        fs::create_dir_all(repo_path.join(".git")).unwrap();
        assert!(GitManager::is_repository(repo_path));
    }
}
