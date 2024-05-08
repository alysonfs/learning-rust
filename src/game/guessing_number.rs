use std::{cmp::Ordering, io};

use rand::Rng;

use crate::{
    core::guess_role::MAX_NUMBER,
    helper::terminal::{await_enter, clear},
};

pub fn perform() {
    clear();
    println!("Descubra o número secreto!");
    let secret_number = rand::thread_rng().gen_range(1..=MAX_NUMBER);

    println!("O numero secreto é: {}", secret_number);
    await_enter();
    loop {
        println!("Informe seu palpite:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu palpite foi: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                clear();
                break;
            }
        }
    }
}
