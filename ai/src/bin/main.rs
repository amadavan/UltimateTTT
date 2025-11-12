use ultimate_ttt::board::{BoardStatus, Player};
use ai::{RandomPlayer, AIPlayer, play_game};

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