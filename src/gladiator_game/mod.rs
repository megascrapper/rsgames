use std::io;

use crate::errors::GameError;
use crate::gladiator_game::army::{Army, FormationType};

mod fighter;
mod army;

pub struct GladiatorGame<'a> {
    army_1: Army<'a>,
    army_2: Army<'a>,
}

impl<'a> GladiatorGame<'_> {
    pub fn new() -> Result<GladiatorGame<'a>, GameError> {
        let mut player1_name = String::new();
        let mut player2_name = String::new();
        let mut budget_input = String::new();
        let mut game_mode_input = String::new();

        println!("Enter name for player 1: ");
        io::stdin().read_line(&mut player1_name).expect("Cannot read line");
        println!("Enter name for player 2: ");
        io::stdin().read_line(&mut player2_name).expect("Cannot read line");

        println!("Enter budget for both players: ");
        io::stdin().read_line(&mut budget_input).expect("Cannot read line");
        let budget = match budget_input.trim().parse() {
            Ok(n) => if n > 0 { n } else { return Err(GameError("Error: budget must be greater than 0".to_string())); },
            Err(_) => return Err(GameError("Error: invalid amount".to_string()))
        };

        println!("Select game mode: ");
        println!("(1) Gladiatorial combat\nThe unit at the front of the army will continuously fight until it dies. The next\nunit will then take over. Fighting stops when one of the army has no units remaining, and the winner is the \narmy with remaining units.");
        println!("(2) Fairer combat\nEach unit will fight for one turn and then retreats to the rear of the army. Fighting stops when one of the\narmy has no units remaining, and the winner is the army with remaining units.");
        io::stdin().read_line(&mut game_mode_input).expect("Cannot read line");
        let game_mode = match game_mode_input.trim().parse() {
            Ok(n) => match n {
                1 => FormationType::Stack,
                2 => FormationType::Queue,
                _ => return Err(GameError("Error: invalid selection".to_string()))
            },
            Err(_) => return Err(GameError("Error: invalid number".to_string()))
        };

        Ok(GladiatorGame {
            army_1: Army::new(player1_name.trim().to_string(), budget, &game_mode),
            army_2: Army::new(player2_name.trim().to_string(), budget, &game_mode),
        })
    }

    pub fn play(&mut self) {
        while !(self.army_1.is_empty() || self.army_2.is_empty()) {
            let mut unit1 = self.army_1.pop_unit().unwrap();
            let mut unit2 = self.army_2.pop_unit().unwrap();

            // actual battle
            if unit1.get_speed() > unit2.get_speed() {
                unit2.defend(unit1.get_attack_damage()); // unit1 attacks
                if unit2.is_alive() { // unit2 attacks
                    unit1.defend(unit2.get_attack_damage());
                }
            } else if unit1.get_speed() < unit2.get_speed() {
                unit1.defend(unit2.get_attack_damage()); // unit2 attacks
                if unit1.is_alive() {
                    unit2.defend(unit1.get_attack_damage()); // unit1 attacks
                }
            } else {
                unit2.defend(unit1.get_attack_damage()); // both units attack
                unit1.defend(unit2.get_attack_damage());
            }

            // gain xp/lose health
            if unit1.is_alive() && unit2.is_alive() { // if both units are alive, they both lose 1 health
                unit1.lose_life(1);
                unit2.lose_life(1);
            } else if unit1.is_alive() { // if only one unit is alive, they gain 1 experience
                unit1.gain_experience(1);
            } else {
                unit2.gain_experience(1);
            }

            // push back alive units
            if unit1.is_alive() {
                self.army_1.push_unit(unit1);
            }
            if unit2.is_alive() {
                self.army_2.push_unit(unit2);
            }
        }
        debug_assert!(self.army_1.is_empty() || self.army_2.is_empty());
        // determine winner
        if self.army_1.is_empty() && self.army_2.is_empty() {
            println!("Game is a draw");
        } else if self.army_2.is_empty() {
            println!("Player 1 ({}) won", self.army_1.name());
        } else {
            println!("Player 2 ({}) won", self.army_2.name());
        }
    }
}