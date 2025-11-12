use rand::rng;
use rand::seq::IndexedRandom as _;

pub struct RandomPlayer {
    _player: crate::board::Player,
}

impl crate::ai::AIPlayer for RandomPlayer {
    fn new(player: crate::board::Player) -> Self {
        RandomPlayer { _player: player }
    }

    fn choose_move(&self, board: &crate::board::Board) -> Option<crate::board::Move> {
        let available_moves = board.get_available_moves();
        let mut rng = rng();
        available_moves.choose(&mut rng).cloned()
    }
}
