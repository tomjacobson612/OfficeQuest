mod models;
use models::card;
use models::card::Card;
use models::player::Player;
use models::enemy::Enemy;
use models::event::{EventEffect, Event};
use ggez::event::{EventHandler};
use ggez::{Context, event, ContextBuilder, GameResult, GameError};
use ggez::conf::Conf;
use ggez::input::keyboard;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let test_card2: Card = card::friday_meeting();

    let test_player = Player {
        name: String::from("Tom"),
        hp: 10,
        hp_max: 10,
        energy: 3,
        energy_max: 3,
        hand: vec![test_card2, card::pizza_party()],
        deck: vec![],
        discard: vec![],
        hand_displayed: false,
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
        event_list: vec![Event::audit()],
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
#[derive(Debug, PartialEq, Clone)]
pub enum Turn {
    PlayerTurn,
    EnemyTurn,
    GameOver,
}

pub struct State {
    pub state: GameState,
    pub player: Player,
    pub enemy: Enemy,
    pub turn: Option<Turn>,
    pub event_list: Vec<Event>,
    pub current_event: Option<Event>,
    pub event_in_progress: bool,
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    Combat,
    Event,
    GameOver,
}

impl State{
    fn print_game_state(&self) {
        if let Some(turn) = self.turn.as_ref() {
            println!("Turn: {:?}", turn);
            println!("------------------");
        }
        println!("Current Energy: {}/{}", self.player.energy, self.player.energy_max);
        println!("Player Health: {}/{}", self.player.hp, self.player.hp_max);
        println!("Enemy Health: {}", self.enemy.hp);
        println!("------------------");
    }

    fn handle_combat(&mut self, ctx: &mut Context) -> GameResult {
        if self.player.hand_displayed == false {
            self.print_game_state();
            self.player.print_hand();
            self.player.hand_displayed = true;
        }

        for i in 0..9 {
            let key = match i {
                0 => keyboard::KeyCode::Key1,
                1 => keyboard::KeyCode::Key2,
                2 => keyboard::KeyCode::Key3,
                3 => keyboard::KeyCode::Key4,
                4 => keyboard::KeyCode::Key5,
                5 => keyboard::KeyCode::Key6,
                6 => keyboard::KeyCode::Key7,
                7 => keyboard::KeyCode::Key8,
                8 => keyboard::KeyCode::Key9,
                _ => unreachable!(),
            };

            if ctx.keyboard.is_key_just_pressed(key) {
                if i < self.player.hand.len() {
                    self.player.play_card(&mut self.enemy, i);
                    self.print_game_state();
                    self.player.hand_displayed = false;
                    if self.enemy.is_dead() {
                        self.trigger_random_event();
                        return Ok(());
                    }
                } else {
                    println!("Invalid card selection.");
                }
                return Ok(());
            }
        }

        if  ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::Space) {
            println!("You end your turn.");
            self.player.energy = self.player.energy_max;
            self.player.hand_displayed = false;
            self.enemy_turn();
        }
        Ok(())
    }

    fn apply_event(&mut self, event: &Event) {
        println!("Event: {}", event.name);
        println!("{}", event.event_flavor_text);
        match event.effect {
            EventEffect::GainHP => self.player.hp += 10,
            EventEffect::LoseHP => self.player.hp -= 5,
            EventEffect::GainMaxHp => self.player.energy_max += 1,
            _ => println!("Effect not yet implemented"),
        }
    }

    fn trigger_random_event(&mut self) {
        if let Some(event) = self.event_list.choose(&mut thread_rng()).cloned() {
            self.current_event = Some(event);
            self.state = GameState::Event;
        }
    }

    fn handle_event(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(event) = self.current_event.clone() {
            if !self.event_in_progress { 
                self.apply_event(&event);
                println!("Press Spacebar to continue..."); 
                self.event_in_progress = true;
            }

            if ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::Space) {
                self.current_event = None;
                self.start_combat();
                self.event_in_progress = false;
            }
        }
        Ok(())
    }

    fn start_combat(&mut self) {
        self.state = GameState::Combat;
        self.player.energy = self.player.energy_max;
        self.enemy.hp = 10;
    }

    fn enemy_turn(&mut self) {
        println!("Enemy attacks!");
        println!("------------------");
        self.player.hp -= 1;
        if self.player.hp <= 0 {
            self.state = GameState::GameOver;
        }
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.state {
            GameState::Combat => self.handle_combat(ctx),
            GameState::Event => self.handle_event(ctx),
            GameState::GameOver => {
                println!("You lose :(");
                Ok(())
            }
        }
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}