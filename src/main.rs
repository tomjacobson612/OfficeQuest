mod models;
use models::card;
use models::card::Card;
use models::player::Player;
use models::enemy::Enemy;
use ggez::*;
use ggez::input::keyboard;

fn main() {
    let test_card: Card = card::friday_meeting();
    let test_card2: Card = card::friday_meeting();

    let test_player = Player {
        name: String::from("Tom"),
        hp: 10,
        energy: 3,
        deck: vec![test_card],
    };

    let test_enemy = Enemy {
        name: String::from("HR Rep"),
        hp: 5,
        actions: vec![test_card2],
    };

    let state = State {
        player: test_player,
        enemy: test_enemy,
        turn: Some(Turn::PlayerTurn)
    };

    //Event Loop
    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("OfficeQuest", "Tom Jacobson")
    .default_conf(c)
    .build()
    .unwrap();

    event::run(ctx, event_loop, state);
}
#[derive(Debug, PartialEq, Clone)]
pub enum Turn {
    PlayerTurn,
    EnemyTurn,
    //CombatEnd,
    GameOver,
}

pub struct State {
    pub player: Player,
    pub enemy: Enemy,
    pub turn: Option<Turn>,
}

impl State{
    fn print_game_state(&self) {
        if let Some(turn) = self.turn.as_ref() {
            println!("Turn: {:?}", turn);
        }
        println!("Player Health: {}", self.player.hp);
        println!("Enemy Health: {}", self.enemy.hp);
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let prev_turn = self.turn.clone();
        match self.turn {
            None => {todo!()}
            Some(Turn::PlayerTurn) => {
                if ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::Space) {
                    println!("Player attacks!");
                    self.enemy.hp -= 1;
                    self.turn = Some(Turn::EnemyTurn);
                }
            }

            Some(Turn::EnemyTurn) => {
                println!("Enemy attacks!");
                self.player.hp -= 1;
                if self.player.hp <= 0 {
                    self.turn = Some(Turn::GameOver);
                } else {
                    self.turn = Some(Turn::PlayerTurn);}}

            Some(Turn::GameOver) => {
                println!("You lose :(");}}
        
        if prev_turn != self.turn{
            self.print_game_state();}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())}
}