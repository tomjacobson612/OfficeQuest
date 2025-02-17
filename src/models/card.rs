pub struct Card {
    pub name: String,
    pub mana_cost: i32,
    pub damage: i32,
    pub self_damage: i32,
    pub textbox: String,
    pub flavor_text: String,
}

impl Card {
    pub fn print(&self) {
        println!("Card Name: {}", self.name);
        println!("Mana Cost: {}", self.mana_cost);
        println!("Damage: {}", self.damage);
        println!("Self Damage: {}", self.self_damage);
        println!("Textbox: {}", self.textbox);
        println!("Flavor Text: \"{}\"", self.flavor_text);
    }
}

pub fn friday_meeting() -> Card {
    Card {
        name: "Friday Meeting".to_string(),
        mana_cost: 1,
        damage: 4,
        self_damage: 2,
        textbox: "Deal 4 damage to any target, take 2 damage.".to_string(),
        flavor_text: "Who scheduled the meeting for Friday at 5pm?".to_string(),
    }
}
