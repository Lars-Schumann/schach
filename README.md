Schach
===
A chess simulator, legal move generator, and notation parser with a focus on simplicity and correctness.

`#![no_std]` compatible and `#![forbid_unsafe}]` completely safe.

This Crate relies on several unstable features, mainly for const Traits and Generators.

Example
===

```rust
use schach::game::{GameState, StepResult};
use schach::notation::algebraic::san;

let game = GameState::INITIAL;
// or from a FEN String
// let game = GameState::try_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

let legal_moves = game.core.legal_moves();

for mv in legal_moves {
    match game.clone().step(mv) {
        StepResult::Continue(next_game) => {
          println!("Move: {:?} results in {next_game:?}", san(mv, game.clone()));
        }
        StepResult::Break(_game_result) => {}
    }
}
```
