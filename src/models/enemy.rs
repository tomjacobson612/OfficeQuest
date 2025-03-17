use colored::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Enemy {
    pub name: String,
    pub hp: i32,
    pub hp_max: i32,
    pub actions: Vec<Intent>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Intent {
    Damage { amount: i32 },
    Healing { amount: i32 },
}

impl Enemy {
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn choose_random_intent(&self) -> Option<&Intent> {
        self.actions.choose(&mut thread_rng())
    }

    pub fn take_damage(&mut self, damage: i32) {
        if self.is_alive() {
            self.hp = (self.hp - damage).max(0);
        }
    }

    pub fn deal_damage(&mut self, amount: i32) {
        println!("-------------------------------");
        println!(
            "{}",
            format!("{} deals {} damage!", self.name, amount).red()
        );
        println!("-------------------------------");
    }

    pub fn heal(&mut self, amount: i32) {
        println!("-------------------------------");
        println!("{}", format!("Enemy heals {} health!", amount).green());
        println!("-------------------------------");
        self.hp = (self.hp + amount).min(self.hp_max);
    }

    pub fn random_enemy() -> Enemy {
        return Enemy::hr_rep();
    }

    pub fn hr_rep() -> Enemy {
        Enemy {
            name: "HR Rep".to_string(),
            hp: 4,
            hp_max: 4,
            actions: vec![Intent::Damage { amount: 1 }, Intent::Healing { amount: 1 }],
        }
    }
}
