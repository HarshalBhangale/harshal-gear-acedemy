#![cfg(test)]

use gtest::{Program, System};
use pebbles_game_io::*;

// Assuming 'get_random_u32()' can be mocked or redirected for testing purposes.
// For simplicity, I will proceed without explicitly handling it here.

static WASM_BINARY: &[u8] = include_bytes!("../target/pebbles_game_bg.wasm");

fn setup() -> (System, Program) {
    let system = System::new();
    let program = system.upload_program(WASM_BINARY);
    (system, program)
}

#[test]
fn test_initialization() {
    let (mut system, program) = setup();
    let init = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    system.init_program(&program, init);

    let state: GameState = system.get_program_state(&program).unwrap();
    assert_eq!(state.pebbles_count, 15);
    assert_eq!(state.max_pebbles_per_turn, 3);
    assert!(state.winner.is_none());
    assert_eq!(state.pebbles_remaining, 15);
}

#[test]
fn test_valid_turn() {
    let (mut system, program) = setup();
    let init = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    system.init_program(&program, init);

    // User takes a turn
    system.send_program_action(&program, PebblesAction::Turn(3));

    let state: GameState = system.get_program_state(&program).unwrap();
    assert_eq!(state.pebbles_remaining, 12);
}

#[test]
fn test_invalid_turn() {
    let (mut system, program) = setup();
    let init = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    system.init_program(&program, init);

    // User tries to take more pebbles than allowed per turn
    system.send_program_action(&program, PebblesAction::Turn(5));

    let state: GameState = system.get_program_state(&program).unwrap();
    assert_eq!(state.pebbles_remaining, 15); // Assert no change if invalid
}

#[test]
fn test_restart_game() {
    let (mut system, program) = setup();
    let init = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    system.init_program(&program, init);

    // Restart the game with new parameters
    let restart_action = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 10,
        max_pebbles_per_turn: 2,
    };
    system.send_program_action(&program, restart_action);

    let state: GameState = system.get_program_state(&program).unwrap();
    assert_eq!(state.pebbles_count, 10);
    assert_eq!(state.max_pebbles_per_turn, 2);
    assert_eq!(state.pebbles_remaining, 10);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
}

#[test]
fn test_game_over_conditions() {
    let (mut system, program) = setup();
    let init = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 1,
        max_pebbles_per_turn: 1,
    };

    system.init_program(&program, init);

    // User takes the last pebble
    system.send_program_action(&program, PebblesAction::Turn(1));

    let state: GameState = system.get_program_state(&program).unwrap();
    assert_eq!(state.pebbles_remaining, 0);
    assert!(matches!(state.winner, Some(Player::User)));
}

// Additional negative scenarios and invalid input tests can be added similarly
