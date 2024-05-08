mod core;
mod game;
mod helper;

use helper::terminal::show_menu;
use std::process::exit;

fn main() {
    loop {
        let itens = ["Games", "Saúde"];
        let item_selected = show_menu("Início", &itens, true);

        match item_selected {
            1 => game::perform(),
            2 => println!("b"),
            _ => exit(0),
        }
    }
}
