pub mod guessing_number;
use crate::helper::terminal::show_menu;

pub fn perform() {
    loop {
        let title = String::from("Games");
        let itens = ["Adivinhe o número", "Forca"];

        let item_selected = show_menu(&title, &itens, false);

        match item_selected {
            1 => guessing_number::perform(),
            2 => println!("Forca"),
            _ => {
                println!("voltando...");
                break;
            }
        }
    }
}
