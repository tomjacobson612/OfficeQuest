use crate::Card;
#[derive(Debug, PartialEq, Clone)]
pub struct Event{
    pub name: String,
    pub event_flavor_text: String,
    pub effect: EventEffect,
}
#[derive(Debug, PartialEq, Clone)]
pub enum EventEffect{
    GainHP { amount: i32 },
    LoseHP { amount: i32 },
    GainMaxHp { amount: i32 },
    GainGold { amount: i32 },
    LoseGold { amount: i32 },
    GainEnergyMax { amount: i32 },
    GainCard { card: Card },
    LoseCard { card: Card },
    GainCurse { card: Card },
}

impl  Event{
    pub fn audit() -> Event {
        Event{
            name: "Random Audit".to_string(),
            event_flavor_text: "Random Audit Flavor Text".to_string(),
            effect: EventEffect::LoseGold{amount: 5},
        }
    }

    pub fn skip_lunch() -> Event {
        Event{
            name: "Skip Lunch".to_string(),
            event_flavor_text: r#"
            Due to budgeting constraints you have been asked to work through lunch.
            As you stare forlornly at your computer screen, your stomach rumbles.
            'You can't skip lunch... you can't do that' you mutter to yourself.
            Lose 2HP.
            "#
            .to_string(),
            effect: EventEffect::LoseHP {amount: 2},
        }}

    pub fn quarterly_raise() -> Event {
        Event{
            name: "Quarterly Raise".to_string(),
            event_flavor_text: r#"
            Your performance has been stellar this quarter, we would like
            to offer you a 3% raise, adjusted for inflation.
            Heal 2HP.
            "#
            .to_string(),
            effect: EventEffect::GainHP { amount: 2 },
        }}
}