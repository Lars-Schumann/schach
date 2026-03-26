use alloc::vec;
use alloc::vec::Vec;
use core::ops::Not;

use crate::board::Board;
use crate::coord::Square;
use crate::game::CastlingSide;
use crate::game::GameResult;
use crate::game::GameResultKind;
use crate::game::GameState;
use crate::game::GameStateCore;
use crate::game::Ongoing;
use crate::game::StepResult;
use crate::game::Terminated;
use crate::mv::KingMove;
use crate::mv::Move;
use crate::mv::MoveKind;
use crate::mv::PawnMove;
use crate::mv::Threat;
use crate::piece::PieceKind;

impl GameState<Ongoing> {
    #[must_use]
    pub fn search(self, max_depth: u32, checker: impl Fn(&Self)) -> SearchStats {
        let mut terminated_games_checkmate: Vec<GameState<Terminated>> = vec![];
        let mut terminated_games_draw: Vec<GameState<Terminated>> = vec![];
        let mut continued_games: Vec<Self> = vec![self];
        let mut new_continued_games: Vec<Self> = vec![];

        for _ in 0..=max_depth {
            continued_games.clone().into_iter().for_each(|game| {
                checker(&game);
                let legal_moves: Vec<Move> = game.core.legal_moves().collect();

                for mv in legal_moves {
                    match game.clone().step(mv) {
                        StepResult::Break(GameResult {
                            kind: GameResultKind::Win,
                            final_game_state,
                        }) => terminated_games_checkmate.push(final_game_state),
                        StepResult::Break(GameResult {
                            kind: GameResultKind::Draw(_),
                            final_game_state,
                        }) => terminated_games_draw.push(final_game_state),
                        StepResult::Continue(game_state) => {
                            new_continued_games.push(game_state);
                        }
                    }
                }
            });

            core::mem::swap(&mut continued_games, &mut new_continued_games);
            new_continued_games.clear();
        }

        SearchStats {
            continued_games: continued_games.len(),
            checkmated_games: terminated_games_checkmate.len(),
            drawn_games: terminated_games_draw.len(),
        }
    }

    #[cfg(feature = "rand")]
    pub fn random_walk(self, max_depth: u32, checker: impl Fn(&Self)) -> StepResult {
        use crate::alloc::borrow::ToOwned as _;

        let mut rng = rand::rng();
        let mut game = self;

        for _ in 0..max_depth {
            use rand::seq::IndexedRandom;

            checker(&game);
            let legal_moves: Vec<Move> = game.core.legal_moves().collect();

            let random_move = legal_moves
                .choose(&mut rng)
                .expect("a GameState<Ongoing> to always have legal moves")
                .to_owned();

            match game.clone().step(random_move) {
                StepResult::Continue(game_state) => {
                    game = game_state;
                }
                terminated @ StepResult::Break(_) => {
                    return terminated;
                }
            }
        }
        StepResult::Continue(game)
    }
}

impl GameStateCore {
    pub fn legal_moves(&self) -> impl Iterator<Item = Move> {
        self.threatening_move_candidates()
            .chain(self.pawn_step_candidates())
            .chain(self.castle_candidates())
            .filter(move |mov| {
                self.board
                    .with_move_applied(*mov)
                    .is_king_checked(self.active_player)
                    .not()
            })
    }

    gen fn castle_candidates(&self) -> Move {
        for castling_side in CastlingSide::ALL {
            if self.has_castling_right(castling_side).not() {
                continue;
            }

            if self
                .are_castle_squares_free_from_checks_and_pieces(castling_side)
                .not()
            {
                continue;
            }

            yield Move {
                kind: MoveKind::King(KingMove::Castle {
                    rook_start: self.active_player.rook_start(castling_side),
                    rook_target: self.active_player.rook_castling_target(castling_side),
                    castling_side,
                }),
                origin: self.active_player.king_start(),
                destination: self.active_player.king_castling_target(castling_side),
            }
        }
    }

    fn threatening_move_candidates(&self) -> impl Iterator<Item = Move> {
        self.board
            .threatening_moves_by(self.active_player)
            .flat_map(|threat| self.threat_to_move_candidates(threat))
    }

    gen fn pawn_step_candidates(&self) -> Move {
        for square in Square::ALL {
            if self.board[square] != Some(PieceKind::Pawn.to_piece(self.active_player)) {
                continue;
            }

            let one_in_front = (square + self.active_player.forwards_one_row())
                .expect("a pawn to never be on the last row");

            if self.board[one_in_front].is_some() {
                continue; // pawns cant capture moving forward!
            }

            if one_in_front.row == self.active_player.pawn_promotion_row() {
                for promotion_option in PieceKind::PROMOTION_OPTIONS {
                    yield Move {
                        kind: MoveKind::Pawn(PawnMove::SingleStep {
                            promotion_replacement: Some(
                                promotion_option.to_piece(self.active_player),
                            ),
                        }),
                        origin: square,
                        destination: one_in_front,
                    }
                }
            } else {
                yield Move {
                    kind: MoveKind::Pawn(PawnMove::SingleStep {
                        promotion_replacement: None,
                    }),
                    origin: square,
                    destination: one_in_front,
                }
            }

            let Ok(two_in_front) = square + self.active_player.forwards_one_row() * 2 else {
                continue; // this one can def be out of range.
            };

            if square.row != self.active_player.pawn_starting_row() {
                continue; // pawns can only double-move when they haven't moved yet!
            }

            if self.board[two_in_front].is_some() {
                continue; // pawns cant capture moving forward!
            }

            yield Move {
                kind: MoveKind::Pawn(PawnMove::DoubleStep),
                origin: square,
                destination: two_in_front,
            }
        }
    }

    #[must_use]
    fn threat_to_move_candidates(&self, threat: Threat) -> Vec<Move> {
        let is_capture = self.board[threat.destination].is_some();
        let origin = threat.origin;
        let destination = threat.destination;
        match threat.piece.kind {
            PieceKind::Knight => vec![Move {
                kind: MoveKind::Knight { is_capture },
                origin,
                destination,
            }],
            PieceKind::Bishop => vec![Move {
                kind: MoveKind::Bishop { is_capture },
                origin,
                destination,
            }],
            PieceKind::Rook => vec![Move {
                kind: MoveKind::Rook { is_capture },
                origin,
                destination,
            }],
            PieceKind::Queen => vec![Move {
                kind: MoveKind::Queen { is_capture },
                origin,
                destination,
            }],
            PieceKind::King => vec![Move {
                kind: MoveKind::King(KingMove::Normal { is_capture }),
                origin,
                destination,
            }],
            PieceKind::Pawn if is_capture => {
                if threat.destination.row == self.active_player.pawn_promotion_row() {
                    PieceKind::PROMOTION_OPTIONS
                        .iter()
                        .map(|promotion_option| Move {
                            kind: MoveKind::Pawn(PawnMove::Capture {
                                promotion_replacement: Some(
                                    promotion_option.to_piece(self.active_player),
                                ),
                            }),
                            origin,
                            destination,
                        })
                        .collect()
                } else {
                    vec![Move {
                        kind: MoveKind::Pawn(PawnMove::Capture {
                            promotion_replacement: None,
                        }),
                        origin,
                        destination,
                    }]
                }
            }
            PieceKind::Pawn => {
                //en passant case, this is never gonna lead to promotion
                if Some(destination) == self.en_passant_target {
                    vec![Move {
                        kind: MoveKind::Pawn(PawnMove::EnPassant {
                            affected: (threat.destination + self.active_player.backwards_one_row())
                                .expect("this to be on the board"),
                        }),
                        origin,
                        destination,
                    }]
                } else {
                    vec![]
                }
            }
        }
    }
}

impl Board {
    pub const fn apply_move(&mut self, m: Move) {
        *self = self.with_move_applied(m);
    }

    #[must_use]
    pub const fn with_move_applied(mut self, m: Move) -> Self {
        self.mov(m.origin, m.destination);
        match m.kind {
            | MoveKind::Pawn(
                PawnMove::SingleStep {
                    promotion_replacement: None,
                }
                | PawnMove::DoubleStep
                | PawnMove::Capture {
                    promotion_replacement: None,
                },
            )
            | MoveKind::Knight { .. }
            | MoveKind::Bishop { .. }
            | MoveKind::Rook { .. }
            | MoveKind::Queen { .. }
            | MoveKind::King(KingMove::Normal { .. }) => { /*nothing */ }

            MoveKind::Pawn(PawnMove::EnPassant { affected }) => {
                self[affected] = None;
            }

            MoveKind::Pawn(
                PawnMove::SingleStep {
                    promotion_replacement: Some(replacement),
                }
                | PawnMove::Capture {
                    promotion_replacement: Some(replacement),
                },
            ) => {
                self[m.destination] = Some(replacement);
            }

            MoveKind::King(KingMove::Castle {
                rook_start,
                rook_target,
                ..
            }) => {
                self.mov(rook_start, rook_target);
            }
        }
        self
    }
}

#[derive(Debug, Default)]
pub struct SearchStats {
    pub continued_games: usize,
    pub checkmated_games: usize,
    pub drawn_games: usize,
}

#[cfg(test)]
mod tests {
    use std::println;

    use super::*;
    use crate::notation::san::standard_algebraic_notation;
    use crate::testing::skip_if_no_expensive_test_opt_in;

    #[test]
    fn search() {
        skip_if_no_expensive_test_opt_in!();

        let depth = 3;
        let game = GameState::default();

        let before = std::time::Instant::now();

        let stats = game.search(depth, |_| ());

        println!("---------------------------");
        println!("depth: {depth}");
        println!("{stats:?}");
        println!("elapsed: {:?}", before.elapsed());
        println!("---------------------------");
    }

    #[test]
    fn perft() {
        skip_if_no_expensive_test_opt_in!();

        let depth = 3;
        let game = GameState::perft();

        let before = std::time::Instant::now();

        let stats = game.search(depth, |_| ());

        println!("---------------------------");
        println!("depth: {depth}");
        println!("{stats:?}");
        println!("elapsed: {:?}", before.elapsed());
        println!("---------------------------");
    }

    #[cfg(feature = "rand")]
    #[cfg(feature = "rayon")]
    #[test]
    fn many_random_walks() {
        use rayon::prelude::*;
        skip_if_no_expensive_test_opt_in!();

        let max_depth = 1_000;
        let walk_count = 1_000;
        let game = GameState::new();

        (0..walk_count).into_par_iter().panic_fuse().for_each(|i| {
            match game.clone().random_walk(max_depth, owl_checker_depth_1) {
                StepResult::Continue(GameState { core, .. })
                | StepResult::Break(GameResult {
                    final_game_state: GameState { core, .. },
                    ..
                }) => println!("{i}: {:?}", core.full_move_count),
            }
        });
    }

    #[allow(dead_code)]
    fn owl_checker_move_count(game: &GameState<Ongoing>) {
        let schach_move_count = game.core.legal_moves().count();
        let owl_move_count = owlchess::movegen::legal::gen_all(
            &owlchess::Board::from_fen(game.core.to_fen().as_str()).unwrap(),
        )
        .len();
        assert_eq!(schach_move_count, owl_move_count);
    }

    #[allow(dead_code)]
    fn owl_checker_depth_1(game: &GameState<Ongoing>) {
        let schach_all_legals = game.core.legal_moves().collect::<Vec<_>>();
        for mv in &schach_all_legals {
            let schach_move_san = standard_algebraic_notation(game.clone(), *mv);
            let owl_board = owlchess::Board::from_fen(game.core.to_fen().as_str()).unwrap();
            let owl_move = owlchess::Move::from_san(schach_move_san.as_str(), &owl_board).unwrap();

            let new_owl_board = owl_board.make_move(owl_move).unwrap();
            let StepResult::Continue(new_schach_board) = game.clone().step(*mv) else {
                continue;
            };

            let new_owl_moves = owlchess::movegen::legal::gen_all(&new_owl_board);
            let new_owl_move_count = new_owl_moves.iter().count();

            let new_schach_moves = new_schach_board.core.legal_moves().collect::<Vec<_>>();
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
                    println!(
                        "{}",
                        standard_algebraic_notation(new_schach_board.clone(), mv).as_str()
                    );
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
}
