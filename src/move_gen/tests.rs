use alloc::vec;
use alloc::vec::Vec;
use std::println;

use crate::game::Game;
use crate::game::GameCore;
use crate::game::GameResult;
use crate::game::GameResultKind;
use crate::game::MoveResult;
use crate::game::Phase::Ongoing;
use crate::game::Phase::Terminated;
use crate::mv::Move;
use crate::testing::skip_if_no_expensive_test_opt_in;

#[expect(clippy::struct_field_names)]
#[derive(Debug, Default)]
struct SearchStats {
    continued_games: usize,
    checkmated_games: usize,
    drawn_games: usize,
}

#[must_use]
fn search(
    game: Game<{ Ongoing }>,
    max_depth: u32,
    checker: impl Fn(&Game<{ Ongoing }>),
) -> SearchStats {
    let mut terminated_games_checkmate: Vec<Game<{ Terminated }>> = vec![];
    let mut terminated_games_draw: Vec<Game<{ Terminated }>> = vec![];
    let mut continued_games: Vec<Game<{ Ongoing }>> = vec![game];
    let mut new_continued_games: Vec<Game<{ Ongoing }>> = vec![];

    for _ in 0..=max_depth {
        for game in continued_games.clone() {
            checker(&game);
            let legal_moves: Vec<Move> = game.legal_moves().collect();

            for mv in legal_moves {
                match mv.make() {
                    MoveResult::Break(GameResult {
                        kind: GameResultKind::Win,
                        final_game_state,
                    }) => terminated_games_checkmate.push(final_game_state),
                    MoveResult::Break(GameResult {
                        kind: GameResultKind::Draw(_),
                        final_game_state,
                    }) => terminated_games_draw.push(final_game_state),
                    MoveResult::Continue(game_state) => {
                        new_continued_games.push(game_state);
                    }
                }
            }
        }

        core::mem::swap(&mut continued_games, &mut new_continued_games);
        new_continued_games.clear();
    }

    SearchStats {
        continued_games: continued_games.len(),
        checkmated_games: terminated_games_checkmate.len(),
        drawn_games: terminated_games_draw.len(),
    }
}

fn random_walk(
    mut game: Game<{ Ongoing }>,
    max_depth: u32,
    checker: impl Fn(&Game<{ Ongoing }>),
) -> MoveResult {
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    use oorandom::Rand64;

    let mut rand = Rand64::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    ); // cryptographically secure :P

    for _ in 0..max_depth {
        checker(&game);
        let legal_moves: Vec<Move> = game.legal_moves().collect();

        #[expect(clippy::cast_possible_truncation)]
        let random_move =
            legal_moves[rand.rand_range(0..legal_moves.len() as u64) as usize].clone();

        match random_move.make() {
            MoveResult::Continue(game_state) => {
                game = game_state;
            }
            terminated @ MoveResult::Break(_) => {
                return terminated;
            }
        }
    }
    MoveResult::Continue(game)
}

#[test]
fn perft() {
    skip_if_no_expensive_test_opt_in!();

    let depth = 3;
    let game = Game::perft();

    let before = std::time::Instant::now();

    let stats = search(game, depth, |_| ());

    println!("---------------------------");
    println!("depth: {depth}");
    println!("{stats:?}");
    println!("elapsed: {:?}", before.elapsed());
    println!("---------------------------");
}

#[test]
fn many_random_walks() {
    skip_if_no_expensive_test_opt_in!();

    let max_depth = 1_000;
    let walk_count = 25;
    let game = Game::new();

    for i in 0..walk_count {
        match random_walk(game.clone(), max_depth, owl_checker_depth_1) {
            MoveResult::Continue(Game { core, .. })
            | MoveResult::Break(GameResult {
                final_game_state: Game { core, .. },
                ..
            }) => println!("{i}: {:?}", core.full_move_count),
        }
    }
}

#[allow(dead_code)]
fn owl_checker_move_count(core: &GameCore) {
    let schach_move_count = core.legal_inner_moves().count();
    let owl_move_count = owlchess::movegen::legal::gen_all(
        &owlchess::Board::from_fen(core.to_fen().as_str()).unwrap(),
    )
    .len();
    assert_eq!(schach_move_count, owl_move_count);
}

#[allow(dead_code)]
fn owl_checker_depth_1(game: &Game<{ Ongoing }>) {
    let schach_all_legals = game.legal_moves().collect::<Vec<_>>();
    for mv in schach_all_legals {
        let schach_move_san = mv.clone().san();
        let owl_board = owlchess::Board::from_fen(game.core.to_fen().as_str()).unwrap();
        let owl_move = owlchess::Move::from_san(schach_move_san.as_str(), &owl_board).unwrap();

        let new_owl_board = owl_board.make_move(owl_move).unwrap();
        let MoveResult::Continue(new_schach_board) = mv.make() else {
            continue;
        };

        let new_owl_moves = owlchess::movegen::legal::gen_all(&new_owl_board);
        let new_owl_move_count = new_owl_moves.iter().count();

        let new_schach_moves = new_schach_board.legal_moves().collect::<Vec<_>>();
        let new_schach_move_count = new_schach_moves.len();

        if new_schach_move_count != new_owl_move_count {
            println!("old schach: {}", game.core.to_fen().as_str());
            println!("made move {}", schach_move_san.as_str());
            println!();
            println!(
                "resulted in (schach opinion): {}",
                new_schach_board.core.to_fen().as_str()
            );
            println!("resulted in (owlchs opinion): {}", new_owl_board.as_fen());
            println!();
            println!("schach moves: {new_schach_move_count}");
            for mv in new_schach_moves {
                println!("{}", mv.san().as_str());
            }
            println!("owlchs moves: {new_owl_move_count}");
            for mv in &new_owl_moves {
                println!("{mv}");
            }
            assert_eq!(1, 0);
            panic!();
        }
        assert_eq!(
            new_schach_move_count,
            new_owl_move_count,
            "schach_fen: {}",
            new_schach_board.core.to_fen().as_str()
        );
    }
}
