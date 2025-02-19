mod models;
use models::card;
use models::card::Card;
use models::player::Player;
use ggez::*;

fn main() {
    let test_card: Card = card::friday_meeting();
    let test_player = Player {
        name: String::from("Tom"),
        deck: vec![test_card],
    };

    let state = State {
        player: test_player,
    };

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("OfficeQuest", "Tom Jacobson")
    .default_conf(c)
    .build()
    .unwrap();

    event::run(ctx, event_loop, state);
}
pub struct State {
    pub player: Player,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())}

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.player.print_deck();
        Ok(())}
}