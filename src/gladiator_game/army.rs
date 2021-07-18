use std::collections::VecDeque;
use std::io;

use crate::gladiator_game::fighter::{Archer, Cavalry, Fighter, Soldier};

enum ArmyFormation<'a> {
    Stack(Vec<Box<dyn Fighter + 'a>>),
    Queue(VecDeque<Box<dyn Fighter + 'a>>),
}

pub enum FormationType {
    Stack,
    Queue,
}

pub struct Army<'a> {
    name: String,
    budget: i32,
    force: ArmyFormation<'a>,
}

impl<'a> Army<'a> {
    pub fn new(name: String, budget: i32, formation: &FormationType) -> Self {
        loop {
            let mut input_string = String::new();
            println!("Player {} choose your army as S A C", name);
            println!("where S is the number of soldiers\nA is the number of archers\nC is the number of cavalries");
            io::stdin().read_line(&mut input_string).expect("Cannot read line");
            let input_list: Vec<&str> = input_string.trim().split_whitespace().collect();
            if input_list.len() != 3 {
                eprintln!("Error: invalid format");
                continue;
            }
            let s = match input_list[0].parse::<i32>() {
                Ok(n) => n,
                Err(_) => continue,
            };
            let a = match input_list[1].parse::<i32>() {
                Ok(n) => n,
                Err(_) => continue,
            };
            let c = match input_list[2].parse::<i32>() {
                Ok(n) => n,
                Err(_) => continue,
            };
            if Self::correct_army_given(budget, s, a, c) {
                return Army {
                    name,
                    budget,
                    force: Self::assign_army(s, a, c, formation),
                };
            }
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_empty(&self) -> bool {
        match &self.force {
            ArmyFormation::Stack(s) => s.is_empty(),
            ArmyFormation::Queue(q) => q.is_empty(),
        }
    }

    pub fn push_unit(&mut self, unit: Box<dyn Fighter + 'a>) {
        match &mut self.force {
            ArmyFormation::Stack(s) => s.push(unit),
            ArmyFormation::Queue(q) => q.push_back(unit),
        }
    }

    pub fn pop_unit(&mut self) -> Option<Box<dyn Fighter + 'a>> {
        match &mut self.force {
            ArmyFormation::Stack(s) => s.pop(),
            ArmyFormation::Queue(q) => q.pop_front(),
        }
    }

    fn correct_army_given(budget: i32, soldiers: i32, archers: i32, cavalry: i32) -> bool {
        if soldiers < 0 || archers < 0 || cavalry < 0 {
            false
        } else {
            (1 * soldiers + 2 * archers + 3 * cavalry) <= budget
        }
    }

    fn assign_army(sold: i32, arch: i32, cav: i32, formation_type: &FormationType) -> ArmyFormation<'a> {
        let len = (sold + arch + cav) as usize;
        match formation_type {
            FormationType::Stack => {
                //  Stack ordering from bottom: cavalry, archer, soldier
                let mut s: Vec<Box<dyn Fighter>> = Vec::with_capacity(len);
                for _ in 0..cav {
                    s.push(Box::new(Cavalry::new()));
                }
                for _ in 0..arch {
                    s.push(Box::new(Archer::new()));
                }
                for _ in 0..sold {
                    s.push(Box::new(Soldier::new()));
                }
                ArmyFormation::Stack(s)
            },
            FormationType::Queue => {
                let mut q: VecDeque<Box<dyn Fighter>> = VecDeque::with_capacity(len);
                for _ in 0..sold {
                    q.push_back(Box::new(Soldier::new()));
                }
                for _ in 0..arch {
                    q.push_back(Box::new(Archer::new()));
                }
                for _ in 0..cav {
                    q.push_back(Box::new(Cavalry::new()));
                }
                ArmyFormation::Queue(q)
            }
        }
    }
}