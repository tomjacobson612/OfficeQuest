pub struct Event{
    pub name: String,
    pub event_flavor_text: String,
    pub effect: EventEffect,
}

pub enum EventEffect{
    GainHP,
    LoseHP,
    GainMaxHp,
    GainGold,
    LoseGold,
    GainEnergyMax,
    GainCard,
    GainCurse,
}