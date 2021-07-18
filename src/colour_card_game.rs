use std::io;

use rand::Rng;

use crate::errors::GameError;

enum ColourCard {
    Red(u8),
    Black(u8),
    Yellow(u8),
}


struct Player {
    name: String,
    score: i32,
    cards: Vec<ColourCard>,
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name,
            score: 0,
            cards: Vec::new(),
        }
    }

    fn add_card(&mut self, card: ColourCard) {
        self.cards.push(card);
    }
}

pub struct ColourCardGame {
    player_1: Player,
    player_2: Player,
    card_deck: Vec<ColourCard>,
    round: u32,
}

impl ColourCardGame {
    pub fn new() -> Result<ColourCardGame, GameError> {
        println!("Enter player name for player 1: ");
        let mut player1_name = String::new();
        io::stdin().read_line(&mut player1_name).expect("Cannot read line");
        println!("Enter player name for player 2: ");
        let mut player2_name = String::new();
        io::stdin().read_line(&mut player2_name).expect("Cannot read line");

        Ok(ColourCardGame {
            player_1: Player::new(player1_name.trim().to_string()),
            player_2: Player::new(player2_name.trim().to_string()),
            card_deck: Self::generate_cards(),
            round: 0,
        })
    }

    pub fn play(&mut self) {
        while !self.card_deck.is_empty() {
            self.round += 1;
            println!("================================================================================");
            println!("Round {}", self.round);

            let player1_card = self.take_card();
            let player2_card = self.take_card();
            match self.determine_winner(&player1_card, &player2_card) {
                1 => {
                    self.player_1.score += 1;
                    self.player_1.add_card(player1_card);
                }
                2 => {
                    self.player_2.score += 1;
                    self.player_2.add_card(player2_card);
                }
                _ => panic!("invalid winner")
            }
            println!("Player 1 ({}) has {} points", self.player_1.name, self.player_1.score);
            println!("Player 2 ({}) has {} points", self.player_2.name, self.player_2.score);
        }

        if self.player_1.score > self.player_2.score {
            println!("Player 1 ({}) wins", self.player_1.name);
        } else if self.player_1.score < self.player_2.score {
            println!("Player 2 ({}) wins", self.player_2.name);
        } else {
            println!("Draw game");
        }
    }

    fn generate_cards() -> Vec<ColourCard> {
        let mut deck = Vec::with_capacity(30);
        for i in 1..=10 {
            deck.push(ColourCard::Red(i));
            deck.push(ColourCard::Black(i));
            deck.push(ColourCard::Yellow(i));
        }
        deck
    }

    fn take_card(&mut self) -> ColourCard {
        let index = rand::thread_rng().gen_range(0..self.card_deck.len());
        self.card_deck.remove(index)
    }

    fn determine_winner(&self, player1_card: &ColourCard, player2_card: &ColourCard) -> u8 {
        // red > black
        // yellow > red
        // black > yellow
        match (player1_card, player2_card) {
            (ColourCard::Red(_), ColourCard::Black(_)) => 1,
            (ColourCard::Red(_), ColourCard::Yellow(_)) => 2,
            (ColourCard::Black(_), ColourCard::Red(_)) => 2,
            (ColourCard::Black(_), ColourCard::Yellow(_)) => 1,
            (ColourCard::Yellow(_), ColourCard::Red(_)) => 1,
            (ColourCard::Yellow(_), ColourCard::Black(_)) => 2,
            (ColourCard::Red(a), ColourCard::Red(b)) => if a > b { 1 } else { 2 },
            (ColourCard::Black(a), ColourCard::Black(b)) => if a > b { 1 } else { 2 },
            (ColourCard::Yellow(a), ColourCard::Yellow(b)) => if a > b { 1 } else { 2 },
        }
    }
}