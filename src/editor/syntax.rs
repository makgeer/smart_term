use crate::editor::FileType;

#[derive(Debug, Clone)]
pub struct SyntaxHighlighter {
    pub language: FileType,
    keywords: Vec<String>,
    builtins: Vec<String>,
    types: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub color: u8, // ANSI цвет
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            language: FileType::PlainText,
            keywords: Vec::new(),
            builtins: Vec::new(),
            types: Vec::new(),
        }
    }
    
    pub fn set_language_by_extension(&mut self, ext: &str) {
        self.language = FileType::from_path(Path::new(ext));
        self.load_keywords();
    }
    
    pub fn set_language(&mut self, language: FileType) {
        self.language = language;
        self.load_keywords();
    }
    
    fn load_keywords(&mut self) {
        self.keywords.clear();
        self.builtins.clear();
        self.types.clear();
        
        match self.language {
            FileType::Rust => {
                self.keywords = vec![
                    "fn", "let", "mut", "pub", "struct", "enum", "impl", "match",
                    "if", "else", "for", "while", "loop", "return", "break", "continue",
                    "use", "mod", "trait", "where", "self", "Self", "async", "await",
                    "const", "static", "unsafe", "extern", "crate", "super"
                ].iter().map(|s| s.to_string()).collect();
                
                self.types = vec![
                    "i32", "i64", "u32", "u64", "f32", "f64", "bool", "char", "str",
                    "String", "Vec", "Option", "Result", "Box", "Arc", "Mutex"
                ].iter().map(|s| s.to_string()).collect();
            }
            FileType::Python => {
                self.keywords = vec![
                    "def", "class", "if", "else", "elif", "for", "while", "import",
                    "from", "as", "return", "break", "continue", "pass", "yield",
                    "async", "await", "with", "lambda", "global", "nonlocal", "try",
                    "except", "finally", "raise", "assert", "del", "in", "is", "not",
                    "and", "or", "True", "False", "None"
                ].iter().map(|s| s.to_string()).collect();
                
                self.builtins = vec![
                    "print", "len", "range", "list", "dict", "set", "tuple", "str",
                    "int", "float", "bool", "type", "isinstance", "super", "open"
                ].iter().map(|s| s.to_string()).collect();
            }
            FileType::JavaScript => {
                self.keywords = vec![
                    "function", "var", "let", "const", "if", "else", "for", "while",
                    "return", "break", "continue", "switch", "case", "default",
                    "try", "catch", "finally", "throw", "class", "extends", "import",
                    "export", "from", "as", "async", "await", "this", "new", "delete",
                    "typeof", "instanceof", "in", "of", "true", "false", "null", "undefined"
                ].iter().map(|s| s.to_string()).collect();
            }
            _ => {}
        }
    }
    
    pub fn highlight_line(&self, line: &str, _line_num: usize) -> Option<Vec<Token>> {
        if self.language == FileType::PlainText {
            return None;
        }
        
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut in_string = false;
        let mut string_char = None;
        let mut in_comment = false;
        let mut in_number = false;
        
        for ch in line.chars() {
            if in_comment {
                current_token.push(ch);
                continue;
            }
            
            if in_string {
                current_token.push(ch);
                if ch == string_char.unwrap() {
                    tokens.push(Token {
                        text: current_token.clone(),
                        color: 32, // Зеленый для строк
                    });
                    current_token.clear();
                    in_string = false;
                    string_char = None;
                }
                continue;
            }
            
            if ch.is_whitespace() {
                if !current_token.is_empty() {
                    tokens.push(self.classify_token(&current_token));
                    current_token.clear();
                }
                tokens.push(Token {
                    text: ch.to_string(),
                    color: 0, // Белый для пробелов
                });
                in_number = false;
            } else if ch == '"' || ch == '\'' {
                if !current_token.is_empty() {
                    tokens.push(self.classify_token(&current_token));
                    current_token.clear();
                }
                in_string = true;
                string_char = Some(ch);
                current_token.push(ch);
            } else if ch == '#' || (ch == '/' && current_token.ends_with('/')) {
                if !current_token.is_empty() {
                    tokens.push(self.classify_token(&current_token.replace("/", "")));
                    current_token.clear();
                }
                in_comment = true;
                current_token.push(ch);
            } else if ch.is_digit(10) && current_token.is_empty() {
                in_number = true;
                current_token.push(ch);
            } else if in_number && (ch.is_digit(10) || ch == '.' || ch == 'x' || ch == 'b') {
                current_token.push(ch);
            } else {
                if in_number {
                    tokens.push(Token {
                        text: current_token.clone(),
                        color: 35, // Фиолетовый для чисел
                    });
                    current_token.clear();
                    in_number = false;
                }
                current_token.push(ch);
            }
        }
        
        // Оставшийся текст
        if !current_token.is_empty() {
            if in_comment {
                tokens.push(Token {
                    text: current_token,
                    color: 36, // Голубой для комментариев
                });
            } else if in_number {
                tokens.push(Token {
                    text: current_token,
                    color: 35, // Фиолетовый для чисел
                });
            } else {
                tokens.push(self.classify_token(&current_token));
            }
        }
        
        Some(tokens)
    }
    
    fn classify_token(&self, text: &str) -> Token {
        let color = match self.language {
            FileType::Rust => self.rust_token_color(text),
            FileType::Python => self.python_token_color(text),
            FileType::JavaScript => self.javascript_token_color(text),
            FileType::Html => self.html_token_color(text),
            FileType::Css => self.css_token_color(text),
            FileType::Markdown => self.markdown_token_color(text),
            _ => 0, // Белый
        };
        
        Token {
            text: text.to_string(),
            color,
        }
    }
    
    fn rust_token_color(&self, text: &str) -> u8 {
        if self.keywords.contains(&text.to_string()) {
            34 // Синий для ключевых слов
        } else if self.types.contains(&text.to_string()) {
            33 // Желтый для типов
        } else if text.starts_with("//") {
            36 // Голубой для комментариев
        } else {
            0 // Белый
        }
    }
    
    fn python_token_color(&self, text: &str) -> u8 {
        if self.keywords.contains(&text.to_string()) {
            34 // Синий
        } else if self.builtins.contains(&text.to_string()) {
            33 // Желтый
        } else if text.starts_with("#") {
            36 // Голубой
        } else {
            0 // Белый
        }
    }
    
    fn javascript_token_color(&self, text: &str) -> u8 {
        if self.keywords.contains(&text.to_string()) {
            34 // Синий
        } else if text.starts_with("//") {
            36 // Голубой
        } else {
            0 // Белый
        }
    }
    
    fn html_token_color(&self, text: &str) -> u8 {
        if text.starts_with("<!--") || text.starts_with("-->") {
            36 // Голубой для комментариев
        } else if text.starts_with("</") || text.starts_with("<") {
            34 // Синий для тегов
        } else {
            0 // Белый
        }
    }
    
    fn css_token_color(&self, text: &str) -> u8 {
        if text.starts_with("/*") || text.starts_with("*/") {
            36 // Голубой для комментариев
        } else if text.ends_with(':') {
            34 // Синий для свойств
        } else {
            0 // Белый
        }
    }
    
    fn markdown_token_color(&self, text: &str) -> u8 {
        if text.starts_with('#') {
            33 // Желтый для заголовков
        } else if text.starts_with('*') || text.starts_with('-') {
            32 // Зеленый для списков
        } else if text.starts_with('`') {
            35 // Фиолетовый для кода
        } else {
            0 // Белый
        }
    }
    
    pub fn get_language_name(&self) -> &'static str {
		match self.language {
			FileType::Rust => "Rust",
			FileType::Python => "Python",
			FileType::JavaScript => "JavaScript",
			FileType::TypeScript => "TypeScript",
			FileType::Html => "HTML",
			FileType::Css => "CSS",
			FileType::Markdown => "Markdown",
			FileType::Toml => "TOML",        // Добавить
			FileType::Json => "JSON",        // Добавить
			FileType::Yaml => "YAML",        // Добавить
			FileType::Unknown => "Unknown",
			_ => "Unknown", // wildcard для остальных случаев
		}
	}
} // ← Закрывающая скобка для impl SyntaxHighlighter
