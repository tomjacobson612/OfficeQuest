use crate::models::card::Card;
use crate::models::enemy::Enemy;

pub struct Player {
    pub name: String,
    pub hp: i32,
    pub energy: i32,
    pub hand: Vec<Card>,
    pub deck: Vec<Card>,
    pub hand_displayed: bool,
}

impl Player {
    pub fn print_deck(&self) {
        println!("{}'s Deck:", self.name);
        for card in &self.deck {
            card.print();
            println!("------------------");
        }
    }

    pub fn print_hand(&self){
        let position: u32 = 0;
        println!("{}'s Hand:", self.name);
        for card in &self.hand {
            println!("Position: {}", position);
            card.print();
            println!("*********************************");
        }
    }

    pub fn display_energy(&self){
        println!("Current Energy: {}", self.energy);
        println!("------------------");
    }

    pub fn play_card(&mut self, enemy: &mut Enemy, hand_position: usize){
        let card = self.hand.remove(hand_position);
        println!("Chosen Card is: ");
        card.print();
        
        if self.energy < card.mana_cost{
            println!("You do not have enough energy to play that card!");
            return;
        }else{
            self.energy -= card.mana_cost;
            self.hp -= card.self_damage;
            enemy.hp -= card.damage;
            return;} 
    }
}