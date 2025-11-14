mod terminal;
mod ui;
mod editor;
mod git;
mod help;
mod utils;

use terminal::Terminal;

fn main() {
    // Установка красивого вывода паники
    better_panic::install();
    
    // Обработка аргументов командной строки
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "--version" | "-v" => {
                println!("smart-term v{}", env!("CARGO_PKG_VERSION"));
                println!("Умный терминал с псевдографикой и Git интеграцией");
                return;
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--ui" | "-u" => {
                // Запуск в псевдографическом режиме
                if let Err(e) = ui::ncurses_like::NcursesLikeUI::new().and_then(|mut ui| ui.run()) {
                    eprintln!("Ошибка UI: {}", e);
                }
                return;
            }
            _ => {}
        }
    }
    
    // Запуск основного терминала
    let mut terminal = Terminal::new();
    terminal.run();
}

fn print_help() {
    println!("smart-term - Умный терминал v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Использование:");
    println!("  smart-term              Запуск в текстовом режиме");
    println!("  smart-term --ui         Запуск в псевдографическом режиме");
    println!("  smart-term --version    Показать версию");
    println!("  smart-term --help       Показать эту справку");
    println!();
    println!("Горячие клавиши в текстовом режиме:");
    println!("  Ctrl+U    Переключение в псевдографический режим");
    println!("  Tab       Автодополнение (в разработке)");
    println!("  Стрелки   Навигация по истории команд");
    println!();
    println!("Документация: https://github.com/smart-term/smart-term");
    println!("Отчет об ошибках: https://github.com/smart-term/smart-term/issues");
}
