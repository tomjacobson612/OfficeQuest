mod models;
use ggez::conf::Conf;
use ggez::{event, ContextBuilder};
use models::card::Card;
use models::enemy::Enemy;
use models::event::Event;
use models::player::Player;
use models::state::{GameState, State, Turn};
fn main() {
    let test_card2: Card = Card::friday_meeting();

    let test_player = Player {
        name: String::from("Tom"),
        hp: 10,
        hp_max: 10,
        energy: 3,
        energy_max: 3,
        hand: vec![test_card2, Card::pizza_party()],
        deck: vec![],
        discard: vec![],
        hand_displayed: false,
        persistent_effects: vec![],
    };

    let test_enemy = Enemy {
        name: String::from("HR Rep"),
        hp: 1,
        actions: vec![],
    };

    let state = State {
        state: GameState::Combat,
        player: test_player,
        enemy: test_enemy,
        turn: Some(Turn::PlayerTurn),
        event_list: vec![Event::sketchy_janitor(), Event::quarterly_raise(), Event::skip_lunch()],
        current_event: None,
        event_in_progress: false,
    };

    //Event Loop
    let c = Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("OfficeQuest", "Tom Jacobson")
        .default_conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state);
}
