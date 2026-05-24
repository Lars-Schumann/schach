use alloc::vec;
use alloc::vec::Vec;
use core::ops::Not;

use crate::board::Board;
use crate::common::not;
use crate::coord::Square;
use crate::game::CastlingRight;
use crate::game::CastlingSide;
use crate::game::Game;
use crate::game::GameCore;
use crate::game::Phase::Ongoing;
use crate::mv::InnerMove;
use crate::mv::KingMove;
use crate::mv::Move;
use crate::mv::MoveKind;
use crate::mv::PawnMove;
use crate::mv::Threat;
use crate::piece::PieceKind;

#[cfg(test)]
mod tests;

impl Game<{ Ongoing }> {
    pub fn legal_moves(&self) -> impl Iterator<Item = Move> {
        self.core
            .legal_inner_moves()
            .map(|inner| Move::create_unchecked(inner, self.clone()))
    }
}

impl GameCore {
    pub(crate) fn legal_inner_moves(&self) -> impl Iterator<Item = InnerMove> {
        self.threatening_move_candidates()
            .chain(self.pawn_step_candidates())
            .chain(self.castle_candidates())
            .filter(move |mv| {
                self.board
                    .with_move_applied(*mv)
                    .is_king_checked(self.active_player)
                    .not()
            })
    }

    gen fn castle_candidates(&self) -> InnerMove {
        for castling_side in CastlingSide::ALL {
            if self.castling_rights[(self.active_player, castling_side)]
                == CastlingRight::Unavailable
            {
                continue;
            }

            if not(self.are_castle_squares_free_from_checks_and_pieces(castling_side)) {
                continue;
            }

            yield InnerMove {
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

    fn threatening_move_candidates(&self) -> impl Iterator<Item = InnerMove> {
        self.board
            .threatening_moves_by(self.active_player)
            .flat_map(|threat| self.threat_to_move_candidates(threat))
    }

    gen fn pawn_step_candidates(&self) -> InnerMove {
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
                    yield InnerMove {
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
                yield InnerMove {
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

            yield InnerMove {
                kind: MoveKind::Pawn(PawnMove::DoubleStep),
                origin: square,
                destination: two_in_front,
            }
        }
    }

    #[must_use]
    fn threat_to_move_candidates(&self, threat: Threat) -> Vec<InnerMove> {
        let is_capture = self.board[threat.destination].is_some();
        let origin = threat.origin;
        let destination = threat.destination;
        match threat.piece.kind {
            PieceKind::Knight => vec![InnerMove {
                kind: MoveKind::Knight { is_capture },
                origin,
                destination,
            }],
            PieceKind::Bishop => vec![InnerMove {
                kind: MoveKind::Bishop { is_capture },
                origin,
                destination,
            }],
            PieceKind::Rook => vec![InnerMove {
                kind: MoveKind::Rook { is_capture },
                origin,
                destination,
            }],
            PieceKind::Queen => vec![InnerMove {
                kind: MoveKind::Queen { is_capture },
                origin,
                destination,
            }],
            PieceKind::King => vec![InnerMove {
                kind: MoveKind::King(KingMove::Normal { is_capture }),
                origin,
                destination,
            }],
            PieceKind::Pawn if is_capture => {
                if threat.destination.row == self.active_player.pawn_promotion_row() {
                    PieceKind::PROMOTION_OPTIONS
                        .iter()
                        .map(|promotion_option| InnerMove {
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
                    vec![InnerMove {
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
                    vec![InnerMove {
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
    pub fn apply_move(&mut self, mv: InnerMove) {
        *self = self.with_move_applied(mv);
    }

    #[must_use]
    pub fn with_move_applied(mut self, mv: InnerMove) -> Self {
        self.mv(mv.origin, mv.destination);
        match mv.kind {
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
                self[mv.destination] = Some(replacement);
            }

            MoveKind::King(KingMove::Castle {
                rook_start,
                rook_target,
                ..
            }) => {
                self.mv(rook_start, rook_target);
            }
        }
        self
    }
}