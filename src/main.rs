use std::io;
use crate::dice_game::DiceGame;

mod dice_game;
mod errors;

fn main() {
    println!("Hello, world!\n");
    loop {
        println!("Menu:");
        println!("(1) Dice game");
        println!("(0) Quit\n");

        println!("Select menu item:");
        let mut selection = String::new();
        io::stdin().read_line(&mut selection).expect("Cannot read line");

        if let Ok(selection) = selection.trim().parse() {
            match selection {
                0 => break,
                1 => {
                    match DiceGame::new() {
                        Ok(mut game) => game.play(),
                        Err(e) => println!("Error: {}", e)
                    }
                }
                _ => println!("Error: invalid selection"),
            }
        } else {
            println!("Error: enter a number");
        }
    }
    println!("Goodbye.");
}
