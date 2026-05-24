# Schach

A chess simulator, legal move generator, and notation parser with a focus on simplicity and correctness.
This Crate relies on several unstable features so that it can make use of Ascii Chars, Generators, and const generic Enums.

# Example

```rust
use schach::game::Game;
use schach::game::MoveResult;

fn main() {
    let game = Game::new();
    // or from a FEN String
    // let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    for mv in game.legal_moves() {
        println!("Move: {:?}", mv.clone().san());
        match mv.make() {
            MoveResult::Continue(next_game) => {
                println!("Results in board: {:?}\n", next_game.core().board);
            }
            MoveResult::Break(game_result) => {
                println!("Ends the Game with Result: {:?}", game_result.kind)
            }
        }
    }
}
```

# Guarantees

- `#![no_std]` compatible
- `#![forbid_unsafe]` completely safe
- 0 hard dependencies
- every `Ongoing` `Game` has at least one legal move 
- starting from a valid `Game`, one can only reach other valid `Game`s, notably this is implemented without having to re-check the legality of a `Move` before it is made
