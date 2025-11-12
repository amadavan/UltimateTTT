# Ultimate Tic-Tac-Toe

A Rust implementation of Ultimate Tic-Tac-Toe with AI players.

## What is Ultimate Tic-Tac-Toe?

Ultimate Tic-Tac-Toe is a strategic variant of the classic tic-tac-toe game. The game consists of a 3×3 grid of smaller tic-tac-toe boards (called microboards). Players must win individual microboards to claim positions in the larger board, with the ultimate goal of winning the overall game.

### Rules
- The game is played on a 3×3 grid of 3×3 tic-tac-toe boards
- Players alternate turns between X and O
- The first move can be played anywhere on any microboard
- After each move, the opponent must play in the microboard that corresponds to the position of the previous move
- If a microboard is won or full, the opponent can play on any available microboard
- Win a microboard by getting three in a row within that board
- Win the game by getting three microboards in a row on the main board

## Features

- ✅ Complete game logic implementation
- ✅ Board state management
- ✅ Move validation
- ✅ Win condition detection
- ✅ Interactive gameplay
- ✅ AI player framework

## Project Structure

```
src/
├── board.rs          # Core game logic and board representation
├── lib.rs            # Library entry point
├── ai/
│   ├── mod.rs        # AI player trait and game simulation
│   └── random.rs     # Random move AI implementation
└── bin/
    ├── main.rs       # Main game executable
    └── test_random.rs # Random AI testing utility
```

## Usage

### Running the Main Game
```bash
cargo run --bin main
```

### Building the Project
```bash
cargo build
```

## AI Development

The project includes a trait-based AI system that makes it easy to implement different AI strategies:

```rust
pub trait AIPlayer {
    fn new(player: crate::board::Player) -> Self where Self: Sized;
    fn choose_move(&self, board: &crate::board::Board) -> Option<crate::board::Move>;
}
```

### AI Players
- ❌ Random AI

### Future AI Ideas
- Minimax AI with alpha-beta pruning
- Monte Carlo Tree Search (MCTS)
- Neural network-based AI
- Heuristic-based AI

## Dependencies

- `rand = "0.9.2"` - For random number generation in AI players

## Development

This project uses Rust 2024 edition. To contribute:

1. Clone the repository
2. Make your changes
3. Run `cargo test` to ensure tests pass
4. Run `cargo fmt` to format code
5. Run `cargo clippy` for linting

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Feel free to:
- Add new AI implementations
- Improve the game interface
- Add more comprehensive tests
- Optimize performance
- Enhance documentation
