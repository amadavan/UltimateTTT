use ultimate_ttt::board::{BoardStatus, Player, Board};

mod random_ai;
use random_ai::RandomPlayer;

// Define the AIPlayer trait here or in a lib.rs
pub trait AIPlayer {
    fn new(player: Player) -> Self
    where
        Self: Sized;

    fn choose_move(&self, board: &Board) -> Option<ultimate_ttt::board::Move>;
}

pub fn play_game(
    ai_player1: &Box<dyn AIPlayer>,
    ai_player2: &Box<dyn AIPlayer>,
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
                println!("Error playing move: {}", play_result.err().unwrap());
                continue;
            }
        } else {
            panic!("AI could not choose a move");
        }

        current_player = if current_player == Player::X {
            Player::O
        } else {
            Player::X
        };
    }

    board.get_status().clone()
}

fn main() {
    let ai_player1: Box<dyn AIPlayer> = Box::new(RandomPlayer::new(Player::X));
    let ai_player2: Box<dyn AIPlayer> = Box::new(RandomPlayer::new(Player::O));

    let result = play_game(&ai_player1, &ai_player2);

    match result {
        BoardStatus::Won(player) => println!("Player {:?} wins!", player),
        BoardStatus::Draw => println!("The game is a draw!"),
        BoardStatus::InProgress => println!("The game is still in progress!"),
    }
}