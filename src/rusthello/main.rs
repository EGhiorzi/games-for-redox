//! `RUSThello` (v. 2.1.0)
//! A simple Reversi game written in Rust with love.
//! Based on `reversi` library (by the same author).
//! Released under MIT license.
//! by Enrico Ghiorzi

#![crate_name = "rusthello"]
#![crate_type = "bin"]

// External crates
extern crate rand;
extern crate rayon;
extern crate reversi;
extern crate termion;

// Modules
mod interface;
mod human_player;
mod ai_player;

use reversi::{ReversiError, Side};
use reversi::game::{PlayerAction, IsPlayer, Game};
use interface::UserCommand;
use std::result;
use std::cmp::Ordering;

pub enum OtherAction {
    Help,
    Quit,
}

pub type Action = PlayerAction<OtherAction>;
pub type Result<T> = result::Result<T, ReversiError>;

fn main() {
    // Main intro
    interface::intro();

    loop {
        interface::main_menu();

        match interface::input_main_menu() {
            // Runs the game
            UserCommand::NewGame => {
                if play_game().is_err() {
                    panic!("Match ended with an error!");
                }
            }
            // Prints help message
            UserCommand::Help => interface::help(),
            // Print credits
            UserCommand::Credits => interface::credits(),
            // Quit RUSThello
            UserCommand::Quit => {
                interface::quitting_message(None);
                break;
            }
            _ => panic!("Main got a user command it shouldn't have got!"),
        }
    }
}

fn play_game() -> Result<()> {

    // Get the two players
    interface::new_player_menu();
    let mut dark_human = false;
    let dark = match interface::choose_new_player(Side::Dark) {
        UserCommand::Quit => return Ok(()),
        UserCommand::HumanPlayer => {
            dark_human = true;
            Box::new(human_player::HumanPlayer) as Box<IsPlayer<OtherAction>>
        }
        UserCommand::AiWeak => Box::new(ai_player::AiPlayer::Weak) as Box<IsPlayer<OtherAction>>,
        UserCommand::AiMedium => Box::new(ai_player::AiPlayer::Medium) as Box<IsPlayer<OtherAction>>,
        UserCommand::AiStrong => Box::new(ai_player::AiPlayer::Strong) as Box<IsPlayer<OtherAction>>,
        _ => panic!("Returned an invalid player choice"),
    };
    let mut light_human = false;
    let light = match interface::choose_new_player(Side::Light) {
        UserCommand::Quit => return Ok(()),
        UserCommand::HumanPlayer => {
            light_human = true;
            Box::new(human_player::HumanPlayer) as Box<IsPlayer<OtherAction>>
        }
        UserCommand::AiWeak => Box::new(ai_player::AiPlayer::Weak) as Box<IsPlayer<OtherAction>>,
        UserCommand::AiMedium => Box::new(ai_player::AiPlayer::Medium) as Box<IsPlayer<OtherAction>>,
        UserCommand::AiStrong => Box::new(ai_player::AiPlayer::Strong) as Box<IsPlayer<OtherAction>>,
        _ => panic!("Returned an invalid player choice"),
    };

    // Print commands info
    interface::commands_info();

    // Create a new game
    let mut game = Game::new(&*dark, &*light);

    // Draw the current board and game info
    interface::draw_board(game.get_current_turn());

    // Proceed with turn after turn till the game ends
    while !game.is_endgame() {
        let state_side = game.get_current_state().unwrap();
        match game.play_turn() {
            Ok(action) => {
                match action {
                    PlayerAction::Move(coord) => {
                        match state_side {
                            Side::Dark => {
                                if !dark_human {
                                    interface::move_message(state_side, coord);
                                }
                            }
                            Side::Light => {
                                if !light_human {
                                    interface::move_message(state_side, coord);
                                }
                            }
                        }
                        interface::draw_board(game.get_current_turn());
                    }
                    PlayerAction::Undo => interface::draw_board(game.get_current_turn()),
                    PlayerAction::Other(OtherAction::Help) => {
                        interface::help();
                        interface::draw_board(game.get_current_turn());
                    }
                    PlayerAction::Other(OtherAction::Quit) => {
                        interface::quitting_message(game.get_current_state());
                        return Ok(());
                    }
                }
            }
            Err(err) => {
                match err {
                    ReversiError::NoUndo => interface::no_undo_message(game.get_current_turn().get_state().unwrap()),
                    _ => return Err(err),
                }
            }
        }
    }

    let (score_dark, score_light) = game.get_current_turn().get_score();
    interface::endgame_message(match score_dark.cmp(&score_light) {
                                   Ordering::Greater => Some(Side::Dark),
                                   Ordering::Less => Some(Side::Light),
                                   Ordering::Equal => None,
                               });

    Ok(())
}
