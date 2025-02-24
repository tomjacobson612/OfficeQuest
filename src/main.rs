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
        hand: vec![test_card2, card::pizza_party()],
        deck: vec![test_card],
        hand_displayed: false,
    };

    let test_enemy = Enemy {
        name: String::from("HR Rep"),
        hp: 5,
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
            println!("------------------");
        }
        println!("Player Health: {}", self.player.hp);
        println!("Enemy Health: {}", self.enemy.hp);
        println!("------------------");
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let prev_turn = self.turn.clone();
        match self.turn {
            None => {todo!()}
            Some(Turn::PlayerTurn) => {
                if self.player.hand_displayed == false{
                    self.print_game_state();
                    self.player.display_energy();
                    self.player.print_hand();
                    self.player.hand_displayed = true;
                }

                for (key, idx) in [
                    (keyboard::KeyCode::Key1, 0),
                    (keyboard::KeyCode::Key2, 1),
                    (keyboard::KeyCode::Key3, 2),
                    (keyboard::KeyCode::Key4, 3),
                    (keyboard::KeyCode::Key5, 4),
                    (keyboard::KeyCode::Key6, 5),
                    (keyboard::KeyCode::Key7, 6),
                    (keyboard::KeyCode::Key8, 7),
                    (keyboard::KeyCode::Key9, 8),
                ].iter(){
                    if ctx.keyboard.is_key_just_pressed(*key) {
                        if idx < &self.player.hand.len() {
                            println!("Index: {}", idx);
                            self.player.play_card(&mut self.enemy, *idx);
                            self.print_game_state();
                            self.player.hand_displayed = false;
                        } else {
                            println!("Invalid card selection.");
                        }
                        break;
                    }
                }

                //End Player Turn Button
                if ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::Space) {
                    println!("You end your turn.");
                    self.player.energy = 3;
                    self.player.hand_displayed = false;
                    self.turn = Some(Turn::EnemyTurn);
                }
            }

            Some(Turn::EnemyTurn) => {
                println!("Enemy attacks!");
                println!("------------------");
                self.player.hp -= 1;
                if self.player.hp <= 0 {
                    self.turn = Some(Turn::GameOver);
                } else {
                    self.turn = Some(Turn::PlayerTurn);}}

            Some(Turn::GameOver) => {
                println!("You lose :(");}}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())}
}