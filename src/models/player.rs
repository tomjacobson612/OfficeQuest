use crate::models::card::Card;

pub struct Player {
    pub name: String,
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