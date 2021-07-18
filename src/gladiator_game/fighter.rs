pub trait Fighter {
    fn is_alive(&self) -> bool;

    fn lose_life(&mut self, lost_life: i32);

    fn get_life(&self) -> i32;

    fn gain_experience(&mut self, gained_experience: i32);

    fn get_experience(&self) -> i32;

    fn get_speed(&self) -> i32;

    fn get_cost(&self) -> i32;

    fn get_attack_damage(&self) -> i32;

    fn defend(&mut self, damage: i32);
}

pub struct Soldier {
    life: i32,
    experience: i32,
}

impl Soldier {
    pub fn new() -> Self {
        Soldier {
            life: 3,
            experience: 0
        }
    }
}

impl Fighter for Soldier {
    fn is_alive(&self) -> bool {
        self.life > 0
    }

    fn lose_life(&mut self, lost_life: i32) {
        assert!(lost_life >= 0, "lost_life must be 0 or greater");
        self.life -= lost_life;
    }

    fn get_life(&self) -> i32 {
        self.life
    }

    fn gain_experience(&mut self, gained_experience: i32) {
        assert!(gained_experience >= 0, "gained_experience must be 0 or greater");
        self.experience += gained_experience;
    }

    fn get_experience(&self) -> i32 {
        self.experience
    }

    fn get_speed(&self) -> i32 {
        1 - self.experience
    }

    fn get_cost(&self) -> i32 {
        1
    }

    fn get_attack_damage(&self) -> i32 {
        1 + self.experience
    }

    fn defend(&mut self, damage: i32) {
        assert!(damage >= 0, "damage value must be 0 or greater");
        if damage > self.experience {
            self.lose_life(1);
        }
    }
}

pub struct Archer {
    life: i32,
    experience: i32,
}

impl Archer {
    pub fn new() -> Self {
        Archer {
            life: 3,
            experience: 0
        }
    }
}

impl Fighter for Archer {
    fn is_alive(&self) -> bool {
        self.life > 0
    }

    fn lose_life(&mut self, lost_life: i32) {
        assert!(lost_life >= 0, "lost_life must be 0 or greater");
        self.life -= lost_life;
    }

    fn get_life(&self) -> i32 {
        self.life
    }

    fn gain_experience(&mut self, gained_experience: i32) {
        assert!(gained_experience >= 0, "gained_experience must be 0 or greater");
        self.experience += gained_experience;
    }

    fn get_experience(&self) -> i32 {
        self.experience
    }

    fn get_speed(&self) -> i32 {
        3
    }

    fn get_cost(&self) -> i32 {
        2
    }

    fn get_attack_damage(&self) -> i32 {
        1 + 2 * self.experience
    }

    fn defend(&mut self, damage: i32) {
        assert!(damage >= 0, "damage value must be 0 or greater");
        self.lose_life(1);
    }
}

pub struct Cavalry {
    life: i32,
    experience: i32,
}

impl Cavalry {
    pub fn new() -> Self {
        Cavalry {
            life: 4,
            experience: 0
        }
    }
}

impl Fighter for Cavalry {
    fn is_alive(&self) -> bool {
        self.life > 0
    }

    fn lose_life(&mut self, lost_life: i32) {
        assert!(lost_life >= 0, "lost_life must be 0 or greater");
        self.life -= lost_life;
    }

    fn get_life(&self) -> i32 {
        self.life
    }

    fn gain_experience(&mut self, gained_experience: i32) {
        assert!(gained_experience >= 0, "gained_experience must be 0 or greater");
        self.experience += gained_experience;
    }

    fn get_experience(&self) -> i32 {
        self.experience
    }

    fn get_speed(&self) -> i32 {
        2 + self.experience
    }

    fn get_cost(&self) -> i32 {
        3
    }

    fn get_attack_damage(&self) -> i32 {
        1 + self.experience
    }

    fn defend(&mut self, damage: i32) {
        assert!(damage >= 0, "damage value must be 0 or greater");
        if damage > self.experience/2 {
            self.lose_life(1);
        }
    }
}