use rand::seq::IndexedRandom;
use ultimate_ttt::board::{Player, Board, Move};

pub struct RandomPlayer {
    _player: Player,
}

impl crate::AIPlayer for RandomPlayer {
    fn new(player: Player) -> Self {
        RandomPlayer { _player: player }
    }

    fn choose_move(&self, board: &Board) -> Option<Move> {
        let available_moves = board.get_available_moves();
        let mut rng = rand::rng();
        available_moves.choose(&mut rng).copied()
    }
}
