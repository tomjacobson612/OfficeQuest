#[derive(Debug, PartialEq, Clone)]
pub struct Event{
    pub name: String,
    pub event_flavor_text: String,
    pub effect: EventEffect,
}
#[derive(Debug, PartialEq, Clone)]
pub enum EventEffect{
    GainHP,
    LoseHP,
    GainMaxHp,
    GainGold,
    LoseGold,
    GainEnergyMax,
    GainCard,
    LoseCard,
    GainCurse,
}
impl  Event{
    pub fn audit() -> Event {
        Event{
            name: "Random Audit".to_string(),
            event_flavor_text: "Random Audit Flavor Text".to_string(),
            effect: EventEffect::LoseGold,
        }
    }
}