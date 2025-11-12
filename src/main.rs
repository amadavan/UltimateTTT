use std::io;

use ultimate_ttt::board;

fn main() {
    let mut board = board::Board::new();
    println!("{:?}", board);

    println!("Available moves: {:?}", board.get_available_moves());

    let mut current_player = board::Player::X;
    while board.get_status() == &board::BoardStatus::InProgress {
        println!("{:?}", board);
        println!("Player {:?} turn...", current_player);
        println!("Available microboards to play in: {:?}", board.get_available_moves());
        println!("Enter your move as: <microboard_row> <microboard_col> <cell_row> <cell_col>");
        
        // Game logic would go here
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let coords: Vec<usize> = input
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if coords.len() == 4 {
            println!("Playing move: {:?}", coords);
            let mv = (coords[0], coords[1], coords[2], coords[3]).into();

            let play = board.play(mv, current_player);
            if let Err(err) = play {
                println!("Error: {}", err);
                continue;
            }
        }
        else {
            println!("Invalid input. Please enter 4 numbers separated by spaces.");
            continue;
        }

        if current_player == board::Player::X {
            current_player = board::Player::O;
        } else {
            current_player = board::Player::X;
        }
    }
}