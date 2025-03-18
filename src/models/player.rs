use crate::models::card::Card;
use crate::models::enemy::Enemy;
use colored::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
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
    pub _persistent_effects: Vec<Card>,
}

impl Player {
    pub fn _print_deck(&self) {
        println!("{}'s Deck:", self.name);
        for card in &self.deck {
            card.print_player_view();
            println!("------------------");
        }
    }

    pub fn _print_discard(&self) {
        println!("{}'s Discard:", self.name);
        for card in &self.discard {
            card.print_player_view();
            println!("------------------");
        }
    }

    pub fn print_hand(&self) {
        let mut position: u32 = 1;
        println!("{}'s Hand:", self.name);

        let divider =
            "----------------------------------------------------------------------".cyan();
        let header_footer =
            "**********************************************************************".cyan();
        println!("{}", header_footer);

        for card in &self.hand {
            println!("Press {} to play card.", position);
            position += 1;
            card.print_player_view();
            println!("{}", divider);
        }
        println!("{}", header_footer);
    }

    pub fn play_card(&mut self, enemy: &mut Enemy, hand_position: usize) {
        let card = self.hand.remove(hand_position);
        println!("Chosen Card is: ");
        card.print_player_view();

        if self.energy < card.mana_cost {
            let text = "You don't have enough energy to play that card.".blue();
            println!("{}", text);
        } else {
            self.energy -= card.mana_cost;
            enemy.take_damage(card.damage);
            self.heal(card.self_damage);
            self.discard.push(card);
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn take_damage(&mut self, damage: i32) {
        if self.is_alive() {
            self.hp = (self.hp - damage).max(0);
        }
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp - amount).min(self.hp_max);
    }

    pub fn draw_cards(&mut self, num_cards: usize) {
        let mut rng = thread_rng();
        for _ in 0..num_cards {
            if self.deck.is_empty() {
                self.deck.append(&mut self.discard);
                self.deck.shuffle(&mut rng);
            }

            if let Some(card) = self.deck.pop() {
                self.hand.push(card);
            } else {
                println!("No more cards to draw!");
                break;
            }
        }
    }

    pub fn start_turn(&mut self) {
        self.energy = self.energy_max;
        self.draw_cards(3);
    }

    pub fn end_turn(&mut self) {
        self.discard.append(&mut self.hand);
        self.hand_displayed = false;
    }

    /// Cleans up player states at end of combat. Shuffles hand and discard pile back into deck.
    /// It also resets the hand displayed status.
    pub fn combat_cleanup(&mut self) {
        self.deck.append(&mut self.hand);
        self.deck.append(&mut self.discard);

        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
        self.hand_displayed = false;
    }

    pub fn create_test_player() -> Player {
        Player {
            name: String::from("Test Player"),
            hp: 10,
            hp_max: 10,
            energy: 3,
            energy_max: 3,
            hand: vec![],
            deck: vec![
                Card::friday_meeting(),
                Card::pizza_party(),
                Card::friday_meeting(),
                Card::pizza_party(),
                Card::friday_meeting(),
                Card::pizza_party(),
                Card::talk_to_manager(),
            ],
            discard: vec![],
            hand_displayed: false,
            _persistent_effects: vec![],
        }
    }
}
