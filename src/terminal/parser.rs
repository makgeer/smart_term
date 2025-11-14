use crate::terminal::CommandResult;

#[derive(Debug, Clone)]
pub enum CommandType {
    System(String, Vec<String>),  // команда и аргументы
    Rust(String),                 // код на Rust
    Python(String),               // код на Python
    Java(String),                 // код на Java
    Bash(String),                 // Bash команда/скрипт
    Editor(String),               // редактирование файла
    Git(String, Vec<String>),     // git команда и аргументы
    Crypto(String, Vec<String>),  // крипто-команда и аргументы
    Internal(String),             // внутренняя команда
}

#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub cmd_type: CommandType,
    pub raw_input: String,
}

pub struct CommandParser;

impl CommandParser {
    pub fn new() -> Self {
        Self
    }
    
    pub fn parse(&self, input: &str) -> Result<ParsedCommand, String> {
        let input = input.trim();
        
        if input.is_empty() {
            return Err("Пустая команда".to_string());
        }
        
        // Определяем тип команды по префиксу
        if input.starts_with("!rust ") {
            let code = input[6..].trim().to_string();
            if code.is_empty() {
                return Err("Пустой Rust код".to_string());
            }
            Ok(ParsedCommand {
                cmd_type: CommandType::Rust(code),
                raw_input: input.to_string(),
            })
        }
        else if input.starts_with("!python ") {
            let code = input[8..].trim().to_string();
            if code.is_empty() {
                return Err("Пустой Python код".to_string());
            }
            Ok(ParsedCommand {
                cmd_type: CommandType::Python(code),
                raw_input: input.to_string(),
            })
        }
        else if input.starts_with("!java ") {
            let code = input[6..].trim().to_string();
            if code.is_empty() {
                return Err("Пустой Java код".to_string());
            }
            Ok(ParsedCommand {
                cmd_type: CommandType::Java(code),
                raw_input: input.to_string(),
            })
        }
        else if input.starts_with("!bash ") {
            let code = input[6..].trim().to_string();
            Ok(ParsedCommand {
                cmd_type: CommandType::Bash(code),
                raw_input: input.to_string(),
            })
        }
        else if input.starts_with("!edit ") || input.starts_with("!micro ") {
            let filename = input.split_whitespace().nth(1)
                .ok_or("Не указано имя файла")?;
            Ok(ParsedCommand {
                cmd_type: CommandType::Editor(filename.to_string()),
                raw_input: input.to_string(),
            })
        }
        else if input.starts_with("!crypt ") {
            let args_str = input[7..].trim();
            let args: Vec<String> = args_str.split_whitespace()
                .map(|s| s.to_string())
                .collect();
            
            if args.is_empty() {
                return Err("Не указана крипто-команда".to_string());
            }
            
            Ok(ParsedCommand {
                cmd_type: CommandType::Crypto(args[0].clone(), args[1..].to_vec()),
                raw_input: input.to_string(),
            })
        }
        else if input.starts_with("git ") {
            let git_args: Vec<&str> = input[4..].split_whitespace().collect();
            if git_args.is_empty() {
                return Err("Не указана git команда".to_string());
            }
            
            Ok(ParsedCommand {
                cmd_type: CommandType::Git(
                    git_args[0].to_string(), 
                    git_args[1..].iter().map(|s| s.to_string()).collect()
                ),
                raw_input: input.to_string(),
            })
        }
        else {
            // Проверяем специальные Git алиасы
            match input {
                "gs" | "gst" => Ok(ParsedCommand {
                    cmd_type: CommandType::Git("status".to_string(), vec!["--short".to_string()]),
                    raw_input: "git status --short".to_string(),
                }),
                "ga" => Ok(ParsedCommand {
                    cmd_type: CommandType::Git("add".to_string(), vec![".".to_string()]),
                    raw_input: "git add .".to_string(),
                }),
                "gc" => Ok(ParsedCommand {
                    cmd_type: CommandType::Git("commit".to_string(), vec!["-m".to_string(), "update".to_string()]),
                    raw_input: "git commit -m update".to_string(),
                }),
                "gp" => Ok(ParsedCommand {
                    cmd_type: CommandType::Git("push".to_string(), vec![]),
                    raw_input: "git push".to_string(),
                }),
                "gl" => Ok(ParsedCommand {
                    cmd_type: CommandType::Git("log".to_string(), vec!["--oneline".to_string(), "-10".to_string()]),
                    raw_input: "git log --oneline -10".to_string(),
                }),
                _ => {
                    // По умолчанию - системная команда или Bash
                    let parts: Vec<&str> = input.split_whitespace().collect();
                    if parts.is_empty() {
                        return Err("Пустая команда".to_string());
                    }
                    
                    let cmd = parts[0].to_string();
                    let args = parts[1..].iter().map(|&s| s.to_string()).collect();
                    
                    // Проверяем, является ли команда внутренней
                    if self.is_internal_command(&cmd) {
                        Ok(ParsedCommand {
                            cmd_type: CommandType::Internal(input.to_string()),
                            raw_input: input.to_string(),
                        })
                    } else {
                        Ok(ParsedCommand {
                            cmd_type: CommandType::System(cmd, args),
                            raw_input: input.to_string(),
                        })
                    }
                }
            }
        }
    }
    
    fn is_internal_command(&self, cmd: &str) -> bool {
        matches!(cmd, 
            "help" | "bash-help" | "bash-quick" | "history" | "clear" | 
            "exit" | "quit" | "elevate" | "privileges" | "ui" | "gui" |
            "nowelcome" | "welcome"
        )
    }
    
    pub fn parse_args(&self, args: &[String]) -> Vec<String> {
        args.to_vec()
    }
}
