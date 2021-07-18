use std::io;
use crate::dice_game::DiceGame;
use crate::colour_card_game::ColourCardGame;
use crate::gladiator_game::GladiatorGame;


mod dice_game;
mod errors;
mod colour_card_game;
mod gladiator_game;

fn main() {
    println!("Hello, world!\n");
    loop {
        println!("Menu:");
        println!("(1) Dice game");
        println!("(2) Colour card game");
        println!("(3) Gladiator game");
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
                },
                2 => {
                    match ColourCardGame::new() {
                        Ok(mut game) => game.play(),
                        Err(e) => println!("Error: {}", e)
                    }
                },
                3 => {
                    match GladiatorGame::new() {
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
