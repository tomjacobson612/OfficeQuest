use crate::models::enemy::Enemy;
use crate::models::event::{Event, EventEffect};
use crate::models::player::Player;
use ggez::input::keyboard;
use ggez::GameError;
use ggez::{Context, GameResult};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, PartialEq, Clone)]
pub enum Turn {
    PlayerTurn,
    EnemyTurn,
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

impl State {
    fn print_game_state(&self) {
        if let Some(turn) = self.turn.as_ref() {
            println!("Turn: {:?}", turn);
            println!("------------------");
        }
        println!(
            "Current Energy: {}/{}",
            self.player.energy, self.player.energy_max
        );
        println!("Player Health: {}/{}", self.player.hp, self.player.hp_max);
        println!("Enemy Health: {}", self.enemy.hp);
        println!("------------------");
    }

    fn handle_combat(&mut self, ctx: &mut Context) -> GameResult {
        if !self.player.hand_displayed {
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
        //End Player Turn
        if ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::Space) {
            println!("You end your turn.");
            self.player.energy = self.player.energy_max;
            self.player.hand_displayed = false;
            self.turn = Some(Turn::EnemyTurn);
        }
        Ok(())
    }

    pub fn process_event(&mut self, event: &Event) {
        println!("Event: {}", event.name);
        println!("{}", event.event_flavor_text);
        if let Some(options) = &event.options {
            for (i, option) in options.iter().enumerate() {
                println!("{}: {}", i + 1, option.choice_text);
            }
        }
    }

    pub fn apply_event_effects(&mut self, effect: &EventEffect) {
        match effect {
            EventEffect::GainHP { amount } => self.player.heal(*amount),
            EventEffect::LoseHP { amount } => {
                self.player.take_damage(*amount);
                if !self.player.is_alive() {
                    self.state = GameState::GameOver;
                }
            }
            EventEffect::GainMaxHp { amount } => {
                self.player.hp_max += amount;
                self.player.heal(*amount);
            }
            EventEffect::GainEnergyMax { amount } => self.player.energy_max += amount,
            EventEffect::GainCard { card: _ } => todo!(),
            EventEffect::LoseCard { card: _ } => todo!(),
            EventEffect::GainCurse { card: _ } => todo!(),
            EventEffect::None => todo!(),
        }
    }

    fn trigger_random_event(&mut self) {
        if let Some(event) = self.event_list.choose(&mut thread_rng()).cloned() {
            self.current_event = Some(event);
            self.state = GameState::Event;
        }
    }

    fn resolve_event(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(event) = self.current_event.clone() {
            if !self.event_in_progress {
                self.process_event(&event);

                if event.options.is_none() {
                    println!("Press Spacebar to continue...");
                }
                self.event_in_progress = true;
            }

            if ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::Space) && event.options.is_none()
            {
                self.apply_event_effects(&event.effect);
                self.current_event = None;
                self.start_combat();
                self.event_in_progress = false;
            } else if let Some(options) = &event.options {
                for i in 0..options.len() {
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
                        if i < options.len() {
                            self.apply_event_effects(&options[i].effect);
                            println!("{}", options[i].outcome_text);
                            let event = self.current_event.as_mut();
                            event.unwrap().outcome_shown = true;
                            println!("Press Spacebar to continue...");
                        }
                        break;
                    }
                }
            }
            if ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::Space) && event.outcome_shown {
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
            GameState::Combat => {
                if self.turn == Some(Turn::PlayerTurn){
                    self.handle_combat(ctx)?;
                } else {
                    self.enemy_turn();
                    self.turn = Some(Turn::PlayerTurn);
                }
                Ok(())
            },
            GameState::Event => self.resolve_event(ctx),
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
