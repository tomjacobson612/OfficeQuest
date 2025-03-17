#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    pub card_type: CardType,
    pub name: String,
    pub mana_cost: i32,
    pub damage: i32,
    pub self_damage: i32,
    pub textbox: String,
    pub flavor_text: String,
}
#[derive(Debug, PartialEq, Clone)]
pub enum CardType {
    Basic,
    Persistant,
}

impl Card {
    pub fn _print_all(&self) {
        println!("Card Name: {}", self.name);
        println!("Mana Cost: {}", self.mana_cost);
        println!("Damage: {}", self.damage);
        println!("Self Damage: {}", self.self_damage);
        println!("Textbox: {}", self.textbox);
        println!("Flavor Text: \"{}\"", self.flavor_text);
    }

    pub fn print_player_view(&self) {
        println!("Card Name: {}", self.name);
        println!("Mana Cost: {}", self.mana_cost);
        println!("Textbox: {}", self.textbox);
        println!("Flavor Text: \"{}\"", self.flavor_text);
    }

    //Specific Card Constructors
    pub fn friday_meeting() -> Card {
        Card {
            card_type: CardType::Basic,
            name: "Friday Meeting".to_string(),
            mana_cost: 1,
            damage: 4,
            self_damage: 2,
            textbox: "Deal 4 damage to any target, take 2 damage.".to_string(),
            flavor_text: "Who scheduled the meeting for Friday at 5pm?".to_string(),
        }
    }

    pub fn pizza_party() -> Card {
        Card {
            card_type: CardType::Basic,
            name: "Pizza Party".to_string(),
            mana_cost: 2,
            damage: 0,
            self_damage: -2,
            textbox: "Heal 2hp.".to_string(),
            flavor_text: "In lieue of quarterly bonuses corporate is giving us a Pizza Party!"
                .to_string(),
        }
    }

    pub fn water_cooler() -> Card {
        Card {
            card_type: CardType::Persistant,
            name: "Water Cooler".to_string(),
            mana_cost: 3,
            damage: 0,
            self_damage: -1,
            textbox: "Heal 1hp at the end of each of your turns.".to_string(),
            flavor_text: "The town square of gossipmongers and time thieves alike.".to_string(),
        }
    }
}
