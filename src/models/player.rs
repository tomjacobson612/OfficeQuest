use crate::models::card::Card;

pub struct Player {
    pub name: String,
    pub hp: i32,
    pub energy: i32,
    pub deck: Vec<Card>,
}

impl Player {
    pub fn print_deck(&self) {
        println!("{}'s Deck:", self.name);
        for card in &self.deck {
            card.print();
            println!("------------------");
        }
    }
}