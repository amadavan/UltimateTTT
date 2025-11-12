pub mod random_agent;
pub mod minimax_ai;

// Re-export the RandomPlayer so it can be used directly
pub use random_agent::RandomPlayer;

use ultimate_ttt::board::{Board, BoardStatus, Player, Move};

pub trait Agent {
    fn new(player: Player) -> Self
    where
        Self: Sized;

    fn choose_move(&self, board: &Board) -> Option<Move>;
}

pub fn play_game(
    ai_player1: &Box<dyn Agent>,
    ai_player2: &Box<dyn Agent>,
) -> BoardStatus {
    let mut board = Board::new();
    let mut current_player = Player::X;

    while board.get_status() == &BoardStatus::InProgress {
        let chosen_move = if current_player == Player::X {
            ai_player1.choose_move(&board)
        } else {
            ai_player2.choose_move(&board)
        };

        if let Some(mv) = chosen_move {
            let play_result = board.play(mv, current_player);
            if play_result.is_err() {
                println!("{:?}", board);
                println!("Error playing move: {}", play_result.err().unwrap());
                println!("Invalid move attempted: {:?}", mv);
            }
        } else {
            panic!("AI could not choose a move");
        }

        // println!("Player {:?} played move: {:?}", current_player, chosen_move);

        current_player = if current_player == Player::X {
            Player::O
        } else {
            Player::X
        };
    }

    board.get_status().clone()
}
