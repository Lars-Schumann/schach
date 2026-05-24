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
