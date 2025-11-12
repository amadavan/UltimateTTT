use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

use ai::{Agent, RandomPlayer};
use ultimate_ttt::board::{Board, BoardStatus, Player};

pub fn main() {
    let mut current_player = Player::X;

    let ai_player1: Box<dyn Agent> = Box::new(RandomPlayer::new(Player::X));
    let ai_player2: Box<dyn Agent> = Box::new(RandomPlayer::new(Player::O));

    // Get board state
    // Load into a dictionary
    // If a board state is seen, we can exit the game
    // Otherwise we keep playing and evaluating board states
    let mut board_cache = HashSet::new();

    for i in 0..100000 {
        let mut board = Board::new();
        while board.get_status() == &BoardStatus::InProgress {
            let initial_board = board.to_hash();

            let chosen_move = if current_player == Player::X {
                ai_player1.choose_move(&board)
            } else {
                ai_player2.choose_move(&board)
            };

            if let Some(mv) = chosen_move {
                let play_result = board.play(mv, current_player);
                if play_result.is_err() {
                    println!("Error playing move: {}", play_result.err().unwrap());
                    continue;
                }
            } else {
                panic!("AI could not choose a move");
            }

            let new_board = board.to_hash();
            if board_cache.contains(&(initial_board, new_board)) {
                break;
            } else {
                board_cache.insert((initial_board, new_board));
            }

            current_player = if current_player == Player::X {
                Player::O
            } else {
                Player::X
            };
        }
    }
    // Save board cache to file
    let serialized = bincode::encode_to_vec(&board_cache, bincode::config::standard()).unwrap();
    let mut file = File::create("board_cache.bin").unwrap();
    file.write_all(&serialized).unwrap();
}
