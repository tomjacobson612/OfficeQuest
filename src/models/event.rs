use crate::Card;
#[derive(Debug, PartialEq, Clone)]
pub struct Event {
    pub name: String,
    pub event_flavor_text: String,
    pub effect: EventEffect,
    pub options: Option<Vec<EventOption>>,
    pub option_chosen: Option<usize>,
    pub outcome_shown: bool,
}
#[derive(Debug, PartialEq, Clone)]
pub enum EventEffect {
    GainHP { amount: i32 },
    LoseHP { amount: i32 },
    GainMaxHp { amount: i32 },
    GainEnergyMax { amount: i32 },
    GainCard { card: Card },
    LoseCard { card: Card },
    GainCurse { card: Card },
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventOption {
    pub choice_text: String,
    pub effect: EventEffect,
    pub outcome_text: String,
}

impl Event {
    pub fn skip_lunch() -> Event {
        Event {
            name: "Skip Lunch".to_string(),
            event_flavor_text: r#"
            Due to budgeting constraints you have been asked to work through lunch.
            As you stare forlornly at your computer screen, your stomach rumbles.
            'You can't skip lunch... you can't do that' you mutter to yourself.
            Lose 2HP.
            "#
            .to_string(),
            effect: EventEffect::LoseHP { amount: 2 },
            options: None,
            option_chosen: None,
            outcome_shown: false,
        }
    }

    pub fn quarterly_raise() -> Event {
        Event {
            name: "Quarterly Raise".to_string(),
            event_flavor_text: r#"
            Your performance has been stellar this quarter, we would like
            to offer you a 3% raise, adjusted for inflation.
            Heal 2HP.
            "#
            .to_string(),
            effect: EventEffect::GainHP { amount: 2 },
            options: None,
            option_chosen: None,
            outcome_shown: false,
        }
    }

    pub fn sketchy_janitor() -> Event {
        Event{
            name: "Sketchy Janitor".to_string(),
            event_flavor_text: r#"
            'Psssst... over here.'
            You turn and see someone from maintenence peeking out from what looks like a broom closet.
            'Get in here quick.'
            What do you do?
            "#
            .to_string(),
            effect: EventEffect::None,
            options: Some(vec![
                EventOption {
                    choice_text: "Follow him in.".to_string(),
                    effect: EventEffect::GainHP { amount: 5 },
                    outcome_text: r#"
                    You follow him into the cramped closet and for a moment your world turns black as the door shuts behind you.
                    *Click*. You wince as the lights turn on.
                    'They gave us a 12ft long sub as a quarterly bonus.' he says, pointing to a sandwhich that seems to wrap around
                    the entirety of the small closet, like a strange square inner tube.
                    'Thanks...' you say hesistantly as he cuts you off a piece. Surprisingly it's quite tasty!
                    Gain 5hp.
                    "#
                    .to_string(),
                },
                EventOption {
                    choice_text: "Pretend you heard someone calling your name and quickly make an exit.".to_string(),
                    effect: EventEffect::LoseHP { amount: 1 },
                    outcome_text: r#"
                    'What was that? You need me RIGHT NOW?'
                    'Sorry man.' you apologize lamely and quickly make your escape. Unfortunately you stub your toe on the way out. 
                    Lose 1 hp.
                    "#
                    .to_string(),
                },
            ]),
            option_chosen: None,
            outcome_shown: false,
        }
    }
}
