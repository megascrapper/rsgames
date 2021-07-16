use std::collections::HashMap;
use std::io;

use rand::Rng;

use crate::errors::GameError;

pub struct DiceGamePlayer {
    name: String,
    score: i32,
}

impl DiceGamePlayer {
    pub fn new(name: String) -> DiceGamePlayer {
        DiceGamePlayer {
            name,
            score: 0,
        }
    }
}

pub struct DiceGame {
    players: Vec<DiceGamePlayer>,
    final_score_list: HashMap<String, i32>,
    round: u32,
}

impl DiceGame {
    pub fn new() -> Result<DiceGame, GameError> {
        let mut players = vec![];
        let mut player_count = String::new();

        println!("How many players?");
        io::stdin().read_line(&mut player_count).expect("Cannot read line");
        if let Ok(player_count) = player_count.trim().parse() {
            if player_count < 2 {
                Err(GameError("Player count must be at least 2".to_string()))
            } else {
                for i in 1..=player_count {
                    let mut player_name = String::new();
                    println!("Enter player name for player {}: ", i);
                    io::stdin().read_line(&mut player_name).expect("Cannot read line");
                    players.push(DiceGamePlayer::new(player_name.trim().to_string()));
                }
                Ok(DiceGame {
                    players,
                    final_score_list: HashMap::new(),
                    round: 0,
                })
            }
        } else {
            Err(GameError("Could not create a new dice game".to_string()))
        }
    }

    pub fn play(&mut self) {
        for _ in 0..5 {
            self.play_round();
        }

        // eliminating draw players until there's one winner
        loop {
            let highest_score = self.players.iter().max_by_key(|p| p.score).unwrap().score;
            self.players = self.players.drain(..).filter(|p| p.score == highest_score).collect();
            if self.players.len() > 1 {
                self.play_round();
            } else {
                break;
            }
        }

        assert_eq!(self.players.len(), 1);
        println!("The winner is {} with {} points", self.players[0].name, self.players[0].score);
    }

    fn play_round(&mut self) {
        self.round += 1;
        println!("================================================================================");
        println!("Round {}", self.round);
        for player in self.players.iter_mut() {
            println!("Now playing: {}", player.name);
            let die1 = roll_dice();
            let die2 = roll_dice();
            let dice_sum = die1 + die2;
            if dice_sum % 2 == 0 {
                player.score += 10;
            } else {
                player.score += 5;
            }
            if die1 == die2 {
                player.score += roll_dice() as i32;
            }
            println!("Player {} now has {} points\n", player.name, player.score)
        }
    }
}

fn roll_dice() -> u8 {
    rand::thread_rng().gen_range(1..=6)
}