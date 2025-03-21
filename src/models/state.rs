use crate::models::enemy::{Enemy, Intent};
use crate::models::event::{Event, EventEffect};
use crate::models::player::Player;
use colored::*;
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
    pub turn_started: bool,
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    Combat,
    Event,
    GameOver,
}

impl State {
    /// Prints the current game state.
    /// # Example
    /// ```
    /// Player Health: 10/10
    /// Current Energy 3/3
    /// Enemy Health 4/4
    /// ```
    fn print_game_state(&self) {
        if let Some(turn) = self.turn.as_ref() {
            println!("Turn: {:?}", turn);
            println!("------------------");
        }
        println!(
            "Player Health: {}",
            format!("{}/{}", self.player.hp, self.player.hp_max).red()
        );
        println!(
            "Current Energy: {}",
            format!("{}/{}", self.player.energy, self.player.energy_max).blue()
        );
        println!(
            "{}'s Health: {}",
            self.enemy.name,
            format!("{}", self.enemy.hp).magenta()
        );
        println!("------------------");
    }

    /// Sets state to combat and turn to player turn (player is always first). Shuffles players deck.
    fn start_combat(&mut self) {
        self.state = GameState::Combat;
        self.turn = Some(Turn::PlayerTurn);

        let mut rng = thread_rng();
        self.player.deck.shuffle(&mut rng);
    }

    /// Resolves the players turn of combat. Allows for player of cards with number keys and ending of turn with spacebar.
    fn player_turn(&mut self, ctx: &mut Context) -> GameResult {
        if !self.player.is_alive() {
            let game_over = "You died.".red();
            println!("{}", game_over);
            self.state = GameState::GameOver;
            return Ok(());
        }

        if !self.turn_started {
            self.player.start_turn();
            self.turn_started = true;
        }

        if !self.player.hand_displayed {
            self.print_game_state();
            self.player.print_hand();

            let end = "Press spacebar to end your turn.".red();
            println!("{}", end);
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
                    if !self.enemy.is_alive() {
                        self.turn_started = false;
                        self.enemy_turn();
                        self.player.combat_cleanup();
                        self.trigger_random_event();
                        return Ok(());
                    }
                } else {
                    println!("Invalid card selection.");
                }
                return Ok(());
            }
        }

        if ctx.keyboard.is_key_just_pressed(keyboard::KeyCode::Space) {
            let text = "You end your turn.".yellow();
            println!("{}", text);
            self.player.end_turn();
            self.turn_started = false;
            self.turn = Some(Turn::EnemyTurn);
        }
        Ok(())
    }

    /// Resolves enemy combat turn. Chooses a random intent from list and resolves it. If enemy is dead a random event is triggered instead.
    fn enemy_turn(&mut self) {
        if self.enemy.hp <= 0 {
            println!("{}", format!("{} Defeated.", self.enemy.name).red());
            self.enemy = Enemy::random_enemy();
            self.state = GameState::Event;
        } else if let Some(intent) = self.enemy.choose_random_intent().cloned() {
            match intent {
                Intent::Damage { amount } => {
                    self.enemy.deal_damage(amount);
                    self.player.take_damage(amount);
                    if !self.player.is_alive() {
                        let game_over = "You died.".red();
                        println!("{}", game_over);
                        self.state = GameState::GameOver;
                    }
                }
                Intent::Healing { amount } => {
                    self.enemy.heal(amount);
                }
            }
        } else {
            println!("Enemy can't move!");
        }
    }

    /// Outputs an event's information to player. If options present presents them to the player.
    pub fn process_event(&mut self, event: &Event) {
        println!("Event: {}", event.name);
        println!("{}", event.event_flavor_text);
        if let Some(options) = &event.options {
            for (i, option) in options.iter().enumerate() {
                println!("{}: {}", i + 1, option.choice_text);
            }
        }
    }

    /// Resolves an event's effects. If options were given then that options effect will be applied.
    pub fn apply_event_effects(&mut self, effect: &EventEffect) {
        match effect {
            EventEffect::GainHP { amount } => self.player.heal(*amount),
            EventEffect::LoseHP { amount } => {
                self.player.take_damage(*amount);
                if !self.player.is_alive() {
                    self.state = GameState::GameOver;
                }
            }
            EventEffect::_GainMaxHp { amount } => {
                self.player.hp_max += amount;
                self.player.heal(*amount);
            }
            EventEffect::_GainEnergyMax { amount } => self.player.energy_max += amount,
            EventEffect::_GainCard { card: _ } => todo!(),
            EventEffect::_LoseCard { card: _ } => todo!(),
            EventEffect::_GainCurse { card: _ } => todo!(),
            EventEffect::None => todo!(),
        }
    }

    fn trigger_random_event(&mut self) {
        if let Some(event) = self.event_list.choose(&mut thread_rng()).cloned() {
            self.current_event = Some(event);
            self.state = GameState::Event;
        }
    }

    /// Resolves an event from start to finish. Combat will always be triggered at the end of an event.
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

    pub fn create_test_state(test_player: Player, test_enemy: Enemy) -> State {
        State {
            state: GameState::Combat,
            player: test_player,
            enemy: test_enemy,
            turn: Some(Turn::PlayerTurn),
            event_list: vec![
                Event::sketchy_janitor(),
                Event::quarterly_raise(),
                Event::skip_lunch(),
            ],
            current_event: None,
            event_in_progress: false,
            turn_started: false,
        }
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.state {
            GameState::Combat => {
                if self.turn == Some(Turn::PlayerTurn) {
                    self.player_turn(ctx)?;
                } else {
                    self.enemy_turn();
                    self.turn = Some(Turn::PlayerTurn);
                }
                Ok(())
            }
            GameState::Event => self.resolve_event(ctx),
            GameState::GameOver => Ok(()),
        }
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_start_combat_success() {
        let mut state = State::create_test_state(Player::create_test_player(), Enemy::hr_rep());
        state.start_combat();
        assert_eq!(state.state, GameState::Combat);
        assert_eq!(state.turn, Some(Turn::PlayerTurn));
    }

    #[test]
    fn test_enemy_turn_deal_damage_success() {
        let mut state = State::create_test_state(Player::create_test_player(), Enemy::hr_rep());
        state.enemy.actions = vec![Intent::Damage { amount: 2 }];
        state.enemy_turn();
        assert_eq!(state.player.hp, 8);
        assert_eq!(state.turn, Some(Turn::PlayerTurn));
    }

    #[test]
    fn test_enemy_turn_heal_success() {
        let mut state = State::create_test_state(Player::create_test_player(), Enemy::hr_rep());
        state.enemy.actions = vec![Intent::Healing { amount: 1 }];
        state.enemy.take_damage(2);
        state.enemy_turn();
        assert_eq!(state.enemy.hp, 3);
    }
}
