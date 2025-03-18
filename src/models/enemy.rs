use colored::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
#[derive(Clone)]
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
        let enemies = [
            Enemy::hr_rep(),
            Enemy::jealous_coworker(),
            Enemy::horrible_boss(),
        ];
        enemies.choose(&mut thread_rng()).unwrap().clone()
    }

    // Specific Enemy Constructors
    pub fn hr_rep() -> Enemy {
        Enemy {
            name: "HR Rep".to_string(),
            hp: 4,
            hp_max: 4,
            actions: vec![Intent::Damage { amount: 1 }, Intent::Healing { amount: 1 }],
        }
    }

    pub fn jealous_coworker() -> Enemy {
        Enemy {
            name: "Jealous Coworker".to_string(),
            hp: 7,
            hp_max: 7,
            actions: vec![Intent::Damage { amount: 3 }, Intent::Healing { amount: 2 }],
        }
    }

    pub fn horrible_boss() -> Enemy {
        Enemy {
            name: "Horrible Boss".to_string(),
            hp: 10,
            hp_max: 10,
            actions: vec![Intent::Damage { amount: 4 }, Intent::Healing { amount: 3 }],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_alive_true() {
        let enemy = Enemy::hr_rep();
        assert!(enemy.is_alive());
    }

    #[test]
    fn test_take_damage_success() {
        let mut enemy = Enemy::jealous_coworker();
        enemy.take_damage(3);
        assert_eq!(enemy.hp, 4);
    }

    #[test]
    fn test_heal_success() {
        let mut enemy = Enemy::horrible_boss();
        enemy.take_damage(5);
        enemy.heal(3);
        assert_eq!(enemy.hp, 8);
    }
}
