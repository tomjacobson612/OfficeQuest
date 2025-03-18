mod models;
use ggez::conf::Conf;
use ggez::{event, ContextBuilder};
use models::card::Card;
use models::enemy::Enemy;
use models::player::Player;
use models::state::State;
fn main() {
    let test_player = Player::create_test_player();
    let test_enemy = Enemy::random_enemy();

    let state = State::create_test_state(test_player, test_enemy);

    //Event Loop
    let c = Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("OfficeQuest", "Tom Jacobson")
        .default_conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state);
}
