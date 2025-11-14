use std::process::{Command, Stdio};
use std::collections::HashMap;
use crate::terminal::{CommandResult, CommandType, ParsedCommand};
use crate::utils::privileges::PrivilegeManager;
use crate::editor::micro_like::MicroEditor;
use crate::git::GitManager;
use crate::help::bash_help::BashHelp;

pub struct CommandExecutor {
    bash_aliases: HashMap<String, String>,
    // Нужно передать путь и обработать Result
    git: GitManager::new(Path::new(".")).expect("Failed to create GitManager"),
}

impl CommandExecutor {
    pub fn new() -> Self {
        let mut aliases = HashMap::new();
        
        // Базовые aliases
        aliases.insert("ll".to_string(), "ls -al".to_string());
        aliases.insert("la".to_string(), "ls -A".to_string());
        aliases.insert("l".to_string(), "ls -CF".to_string());
        aliases.insert("..".to_string(), "cd ..".to_string());
        aliases.insert("...".to_string(), "cd ../..".to_string());
        
        // Git aliases
        aliases.insert("gs".to_string(), "git status --short".to_string());
        aliases.insert("ga".to_string(), "git add .".to_string());
        aliases.insert("gc".to_string(), "git commit -m update".to_string());
        aliases.insert("gp".to_string(), "git push".to_string());
        
        Self {
            bash_aliases: aliases,
            // Нужно передать путь и обработать Result
            git: GitManager::new(Path::new(".")).expect("Failed to create GitManager"),
        }
    }
    
    pub fn execute(&self, command: &ParsedCommand) -> Result<CommandResult, String> {
        match &command.cmd_type {
            CommandType::System(cmd, args) => {
                self.execute_system_command(cmd, args)
            }
            CommandType::Bash(code) => {
                self.execute_bash_command(code)
            }
            CommandType::Rust(code) => {
                self.execute_rust_code(code)
            }
            CommandType::Python(code) => {
                self.execute_python_code(code)
            }
            CommandType::Java(code) => {
                self.execute_java_code(code)
            }
            CommandType::Editor(filename) => {
                self.execute_editor_command(filename)
            }
            CommandType::Git(sub_cmd, args) => {
                self.execute_git_command(sub_cmd, args)
            }
            CommandType::Crypto(sub_cmd, args) => {
                self.execute_crypto_command(sub_cmd, args)
            }
            CommandType::Internal(cmd) => {
                self.execute_internal_command(cmd)
            }
        }
    }
    
    fn execute_system_command(&self, cmd: &str, args: &[String]) -> Result<CommandResult, String> {
        // Проверяем aliases
        let final_cmd = if let Some(alias) = self.bash_aliases.get(cmd) {
            alias.clone()
        } else {
            cmd.to_string()
        };
        
        // Разбираем команду с учетом alias
        let parts: Vec<&str> = final_cmd.split_whitespace().collect();
        let actual_cmd = parts[0];
        let mut all_args: Vec<String> = parts[1..].iter().map(|&s| s.to_string()).collect();
        all_args.extend_from_slice(args);
        
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &final_cmd])
                .args(args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
        } else {
            Command::new(actual_cmd)
                .args(&all_args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
        };
        
        match output {
            Ok(child) => {
                let output = child.wait_with_output()
                    .map_err(|e| format!("Ошибка выполнения команды: {}", e))?;
                
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                
                if output.status.success() {
                    Ok(CommandResult::success(stdout))
                } else {
                    Ok(CommandResult::error(if stderr.is_empty() { stdout } else { stderr }))
                }
            }
            Err(e) => {
                // Если команда не найдена, пробуем выполнить через shell
                self.execute_bash_command(&format!("{} {}", cmd, args.join(" ")))
            }
        }
    }
    
    fn execute_bash_command(&self, command: &str) -> Result<CommandResult, String> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", command])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
        } else {
            Command::new("bash")
                .args(&["-c", command])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
        };
        
        match output {
            Ok(child) => {
                let output = child.wait_with_output()
                    .map_err(|e| format!("Ошибка выполнения команды: {}", e))?;
                
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                
                if output.status.success() {
                    Ok(CommandResult::success(stdout))
                } else {
                    Ok(CommandResult::error(if stderr.is_empty() { stdout } else { stderr }))
                }
            }
            Err(e) => Err(format!("Не удалось выполнить команду: {}", e))
        }
    }
    
    fn execute_rust_code(&self, code: &str) -> Result<CommandResult, String> {
        // Простая реализация Rust REPL
        let output = format!("[RUST] Выполнение Rust кода: {}", code);
        Ok(CommandResult::success(output))
    }
    
    fn execute_python_code(&self, code: &str) -> Result<CommandResult, String> {
        let output = Command::new("python")
            .args(&["-c", code])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("Ошибка выполнения Python: {}", e))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        if output.status.success() {
            Ok(CommandResult::success(stdout))
        } else {
            Ok(CommandResult::error(if stderr.is_empty() { stdout } else { stderr }))
        }
    }
    
    fn execute_java_code(&self, code: &str) -> Result<CommandResult, String> {
        // Упрощенная Java поддержка
        let output = format!("[JAVA] Выполнение Java кода: {}", code);
        Ok(CommandResult::success(output))
    }
    
    fn execute_editor_command(&self, filename: &str) -> Result<CommandResult, String> {
        let mut editor = MicroEditor::new();
        
        match editor.open_file(filename) {
            Ok(()) => {
                match editor.run() {
                    Ok(result) => {
                        if result.saved {
                            Ok(CommandResult::success(format!("Файл {} сохранен", filename)))
                        } else {
                            Ok(CommandResult::success(format!("Редактирование {} завершено", filename)))
                        }
                    }
                    Err(e) => Ok(CommandResult::error(format!("Ошибка редактора: {}", e)))
                }
            }
            Err(e) => Ok(CommandResult::error(e))
        }
    }
    
    fn execute_git_command(&self, subcommand: &str, args: &[String]) -> Result<CommandResult, String> {
        match subcommand {
            "status" => {
                let output = self.git.status()?;
                Ok(CommandResult::success(output))
            }
            "add" => {
                let output = if args.is_empty() {
                    self.git.add(&[".".to_string()])?
                } else {
                    self.git.add(args)?
                };
                Ok(CommandResult::success(output))
            }
            "commit" => {
                let message = if args.len() >= 2 && args[0] == "-m" {
                    &args[1]
                } else {
                    "update"
                };
                let output = self.git.commit(message)?;
                Ok(CommandResult::success(output))
            }
            "push" => {
                let output = self.git.push()?;
                Ok(CommandResult::success(output))
            }
            "pull" => {
                let output = self.git.pull()?;
                Ok(CommandResult::success(output))
            }
            "log" => {
                let limit = args.iter()
                    .find(|a| a.starts_with('-') && a[1..].chars().all(|c| c.is_digit(10)))
                    .and_then(|a| a[1..].parse().ok());
                let output = self.git.log(limit)?;
                Ok(CommandResult::success(output))
            }
            "diff" => {
                let file = args.first().map(|s| s.as_str());
                let output = self.git.diff(file)?;
                Ok(CommandResult::success(output))
            }
            "branch" => {
                let output = self.git.branch()?;
                Ok(CommandResult::success(output))
            }
            "checkout" => {
                if let Some(branch) = args.first() {
                    let output = self.git.checkout(branch)?;
                    Ok(CommandResult::success(output))
                } else {
                    Ok(CommandResult::error("Не указана ветка".to_string()))
                }
            }
            _ => {
                Ok(CommandResult::error(format!("Неизвестная git команда: {}", subcommand)))
            }
        }
    }
    
    fn execute_crypto_command(&self, sub_cmd: &str, args: &[String]) -> Result<CommandResult, String> {
        // Заглушка для криптографических функций
        let output = format!("[CRYPTO] Команда: {}, Аргументы: {:?}", sub_cmd, args);
        Ok(CommandResult::success(output))
    }
    
    pub fn execute_internal_command(&self, cmd: &str) -> Result<CommandResult, String> {
        match cmd {
            "help" => {
                let help = r#"
Smart Term - основные команды:

  Системные команды:
    <command> [args]    - выполнить системную команду
    ls, cd, pwd, etc.   - стандартные Unix команды

  Специальные команды:
    !rust <code>        - выполнить Rust код
    !python <code>      - выполнить Python код
    !java <code>        - выполнить Java код
    !edit <file>        - редактировать файл
    !bash <command>     - выполнить Bash команду

  Git команды:
    git <command>       - выполнить git команду
    gs                  - git status --short
    ga                  - git add .
    gc                  - git commit -m "update"
    gp                  - git push

  Встроенные команды:
    help                - эта справка
    bash-help           - полная справка по Bash
    bash-quick          - быстрые команды Bash
    history             - история команд
    clear               - очистить экран
    elevate             - перезапуск с правами root
    privileges          - показать уровень прав
    ui                  - псевдографический режим
    exit/quit           - выход

  Горячие клавиши:
    Ctrl+U              - переключение в UI режим
    Стрелки ↑↓          - навигация по истории
                "#;
                Ok(CommandResult::success(help.to_string()))
            }
            "bash-help" => {
                let help = BashHelp::get_full_help();
                Ok(CommandResult::success(help))
            }
            "bash-quick" => {
                let help = BashHelp::get_quick_reference();
                Ok(CommandResult::success(help))
            }
            cmd if cmd.starts_with("help ") => {
                let command = &cmd[5..];
                if let Some(help_text) = BashHelp::search_command(command) {
                    Ok(CommandResult::success(help_text))
                } else {
                    Ok(CommandResult::error(format!("Команда '{}' не найдена в справке", command)))
                }
            }
            _ => Ok(CommandResult::error(format!("Неизвестная внутренняя команда: {}", cmd)))
        }
    }
    
    pub fn add_alias(&mut self, alias: String, command: String) {
        self.bash_aliases.insert(alias, command);
    }
    
    pub fn get_aliases(&self) -> &HashMap<String, String> {
        &self.bash_aliases
    }
}
