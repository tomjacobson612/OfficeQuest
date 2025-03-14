use crate::models::card::Card;
use crate::models::enemy::Enemy;

pub struct Player {
    pub name: String,
    pub hp: i32,
    pub hp_max: i32,
    pub energy: i32,
    pub energy_max: i32,
    pub hand: Vec<Card>,
    pub deck: Vec<Card>,
    pub discard: Vec<Card>,
    pub hand_displayed: bool,
    pub persistent_effects: Vec<Card>,
}

impl Player {
    pub fn print_deck(&self) {
        println!("{}'s Deck:", self.name);
        for card in &self.deck {
            card.print_all();
            println!("------------------");
        }
    }

    pub fn print_hand(&self){
        let mut position: u32 = 1;
        println!("{}'s Hand:", self.name);
        for card in &self.hand {
            println!("Press {} to play card.", position);
            position += 1;
            card.print_player_view();
            println!("*********************************");
        }
    }

    pub fn play_card(&mut self, enemy: &mut Enemy, hand_position: usize){
            let card = self.hand.remove(hand_position);
            println!("Chosen Card is: ");
            card.print_player_view();
        
        if self.energy < card.mana_cost{
            println!("You do not have enough energy to play that card!");
        }else{
            self.energy -= card.mana_cost;
            self.hp -= card.self_damage;
            enemy.hp -= card.damage;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn take_damage(&mut self, damage: i32) {
        if self.is_alive(){self.hp = (self.hp - damage).max(0);}
    }
    
    pub fn heal(&mut self, amount: i32){
        self.hp = (self.hp + amount).min(self.hp_max);
    }
}