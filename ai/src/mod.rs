pub mod random;

pub trait AIPlayer {
    fn new(player: crate::board::Player) -> Self
    where
        Self: Sized;

    fn choose_move(&self, board: &crate::board::Board) -> Option<crate::board::Move>;
}

pub fn play_game<AI1, AI2>(
    ai_player1: &Box<dyn AIPlayer>,
    ai_player2: &Box<dyn AIPlayer>,
) -> crate::board::BoardStatus {
    let mut board = crate::board::Board::new();
    let mut current_player = crate::board::Player::X;

    while board.get_status() == &crate::board::BoardStatus::InProgress {
        let chosen_move = if current_player == crate::board::Player::X {
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

        current_player = if current_player == crate::board::Player::X {
            crate::board::Player::O
        } else {
            crate::board::Player::X
        };
    }

    board.get_status().clone()
}
