# Schach

A chess simulator, legal move generator, and notation parser with a focus on simplicity and correctness.

`#![no_std]` compatible, `#![forbid_unsafe]` completely safe and 0 hard dependencies.

This Crate relies on several unstable features, mainly for const Traits and Generators.

# Example

```rust
use schach::game::Game;
use schach::game::StepResult;

fn main() {
    let game = Game::INITIAL;
    // or from a FEN String
    // let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    let legal_moves = game.legal_moves();

    for mv in legal_moves {
        match mv.clone().make() {
            StepResult::Continue(next_game) => {
                println!("Move: {:?} results in {next_game:?}", mv.san());
            }
            StepResult::Break(_game_result) => {}
        }
    }
}
```
