#![no_std]

use gstd::{exec, msg, prelude::*};
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
extern fn init() {
    let init_params: PebblesInit = msg::load().expect("Failed to load init parameters");

    // Check the validity of the initialization parameters
    if init_params.pebbles_count == 0 || init_params.max_pebbles_per_turn == 0 || init_params.max_pebbles_per_turn > init_params.pebbles_count {
        panic!("Invalid initialization parameters.");
    }

    let first_player = if get_random_u32() % 2 == 0 { Player::User } else { Player::Program };

    let state = GameState {
        pebbles_count: init_params.pebbles_count,
        max_pebbles_per_turn: init_params.max_pebbles_per_turn,
        pebbles_remaining: init_params.pebbles_count,
        difficulty: init_params.difficulty,
        first_player,
        winner: None,
    };

    unsafe {
        PEBBLES_GAME = Some(state);
    }

    // Program takes the first turn if it's the starting player
    if first_player == Player::Program {
        program_turn();
    }
}

#[no_mangle]
extern fn handle() {
    let action: PebblesAction = msg::load().expect("Failed to load action.");

    unsafe {
        if let Some(ref mut game) = PEBBLES_GAME {
            match action {
                PebblesAction::Turn(count) => {
                    if count == 0 || count > game.max_pebbles_per_turn || count > game.pebbles_remaining {
                        panic!("Invalid move.");
                    }

                    game.pebbles_remaining -= count;
                    check_win_condition(Player::User);

                    // Program's turn
                    program_turn();
                },
                PebblesAction::GiveUp => {
                    game.winner = Some(Player::Program);
                    msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Failed to send win event");
                },
                PebblesAction::Restart { difficulty, pebbles_count, max_pebbles_per_turn } => {
                    *game = GameState {
                        pebbles_count,
                        max_pebbles_per_turn,
                        pebbles_remaining: pebbles_count,
                        difficulty,
                        first_player: if get_random_u32() % 2 == 0 { Player::User } else { Player::Program },
                        winner: None,
                    };
                    // Program takes the first turn if it's the starting player
                    if game.first_player == Player::Program {
                        program_turn();
                    }
                }
            }
        }
    }
}

#[no_mangle]
extern fn state() {
    unsafe {
        if let Some(ref game) = PEBBLES_GAME {
            msg::reply(game.clone(), 0).expect("Failed to reply with game state");
        }
    }
}

/// Helper function to get a random u32 value
fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

/// Function to check win condition after each turn
fn check_win_condition(player: Player) {
    unsafe {
        if let Some(ref mut game) = PEBBLES_GAME {
            if game.pebbles_remaining == 0 {
                game.winner = Some(player.clone());
                msg::reply(PebblesEvent::Won(player), 0).expect("Failed to send win event");
            }
        }
    }
}

/// Simulates the program's turn in the game
fn program_turn() {
    unsafe {
        if let Some(ref mut game) = PEBBLES_GAME {
            // Determine number of pebbles to take; simple strategy for now
            let take = 1.min(game.pebbles_remaining);

            game.pebbles_remaining -= take;
            msg::reply(PebblesEvent::CounterTurn(take), 0).expect("Failed to send counter turn event");

            // Check win condition for Program
            check_win_condition(Player::Program);
        }
    }
}
