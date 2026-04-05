use schach::game::Game;
use schach::game::StepResult;

fn main() {
    let game = Game::INITIAL;
    // or from a FEN String
    // let game = GameState::try_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

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
