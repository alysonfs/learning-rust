use rpassword::prompt_password;
use std::{io::Write, process::Command};

fn underline_title_menu(title_menu: &str) {
    let size = title_menu.len();
    println!("{}", String::from("=").repeat(size));
}

fn show_itens(itens: &[&str]) {
    for (i, item) in itens.iter().enumerate() {
        println!("{} - {}", i + 1, item)
    }
}

pub fn show_menu(title: &str, itens: &[&str], exit: bool) -> u32 {
    clear();
    let title_menu = String::from("Aprenda Rust :: ") + title;
    println!("{}", title_menu);
    underline_title_menu(&title_menu);
    show_itens(itens);
    exit_or_back(exit);
    std::io::stdout().flush().unwrap();

    print!("\nEscolha uma opção: ");
    std::io::stdout().flush().unwrap();

    let mut item_selected_input = String::new();
    std::io::stdin()
        .read_line(&mut item_selected_input)
        .unwrap();

    let item_selected: Result<u32, _> = item_selected_input.trim().parse();

    match item_selected {
        Ok(item) => item,
        _ => 0,
    }
}

pub fn clear() {
    let output = Command::new("clear").status();

    match output {
        Ok(_) => (),
        Err(error) => eprintln!("Failed to clear terminal: {}", error),
    }
}

pub fn await_enter() {
    prompt_password("\nPressione <enter>").unwrap();
}

pub fn exit_or_back(exit: bool) {
    println!("{}", if exit { "* - Sair" } else { "* - Voltar" });
}
