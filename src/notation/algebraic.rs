use alloc::vec;
use alloc::vec::Vec;
use core::ascii::Char as AsciiChar;
use core::ops::Not;
use core::ops::Not::not;

use crate::game::CastlingSide;
use crate::game::GameResult;
use crate::game::GameResultKind;
use crate::game::StepResult;
use crate::mv::KingMove;
use crate::mv::Move;
use crate::mv::MoveKind;
use crate::mv::PawnMove;
use crate::piece::PieceKind;

const O_O: [AsciiChar; 3] = [
    AsciiChar::CapitalO,
    AsciiChar::HyphenMinus,
    AsciiChar::CapitalO,
];

const O_O_O: [AsciiChar; 5] = [
    AsciiChar::CapitalO,
    AsciiChar::HyphenMinus,
    AsciiChar::CapitalO,
    AsciiChar::HyphenMinus,
    AsciiChar::CapitalO,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OriginAmbiguationLevel {
    Empty,
    FileOnly,
    RankOnly,
    Full,
}

#[derive(Debug, Clone, Copy)]
struct CaptureRepresentation {
    capture: Option<AsciiChar>,
    no_capture: Option<AsciiChar>,
}

#[must_use]
fn notation_creator(
    mv: Move,
    ambiguation_level: OriginAmbiguationLevel,
    capture_representation: CaptureRepresentation,
) -> Vec<AsciiChar> {
    let core_move_notation = match mv.inner.kind {
        MoveKind::King(KingMove::Castle {
            castling_side: CastlingSide::Kingside,
            ..
        }) => O_O.to_vec(),
        MoveKind::King(KingMove::Castle {
            castling_side: CastlingSide::Queenside,
            ..
        }) => O_O_O.to_vec(),
        MoveKind::Pawn(_)
        | MoveKind::Knight { .. }
        | MoveKind::Bishop { .. }
        | MoveKind::Rook { .. }
        | MoveKind::Queen { .. }
        | MoveKind::King(KingMove::Normal { .. }) => {
            let piece_repr = (mv.inner.kind.is_pawn()).not().then_some([mv
                .inner
                .kind
                .piece_kind()
                .to_white_piece()
                .to_fen_repr()]);

            let start_square_repr = match ambiguation_level {
                OriginAmbiguationLevel::Empty => [].to_vec(),
                OriginAmbiguationLevel::FileOnly => [mv.inner.origin.col.to_fen_repr()].to_vec(),
                OriginAmbiguationLevel::RankOnly => [mv.inner.origin.row.to_fen_repr()].to_vec(),
                OriginAmbiguationLevel::Full => mv.inner.origin.to_fen_repr().to_vec(),
            };

            let capture_symbol = if mv.inner.is_capture() {
                capture_representation.capture.map(|c| [c])
            } else {
                capture_representation.no_capture.map(|c| [c])
            };

            let target = mv.inner.destination.to_fen_repr();

            let promotion_replacement = match mv.inner.kind {
                MoveKind::Pawn(
                    PawnMove::SingleStep {
                        promotion_replacement: Some(replacement),
                    }
                    | PawnMove::Capture {
                        promotion_replacement: Some(replacement),
                    },
                ) => Some([replacement.kind.to_white_piece().to_fen_repr()]),
                _ => None,
            };

            #[rustfmt::skip]
            [
                piece_repr.as_ref().map_or_default(<[_; 1]>::as_slice),
                start_square_repr.as_slice(),
                capture_symbol.as_ref().map_or_default(<[_; 1]>::as_slice),
                target.as_slice(),
                promotion_replacement.as_ref().map_or_default(<[_; 1]>::as_slice),
            ]
            .concat()
        }
    };

    let outcome = mv.make();

    let mut append = vec![];
    match outcome {
        | StepResult::Continue(future) => {
            if future.core.board.is_king_checked(future.core.active_player) {
                append.push(AsciiChar::PlusSign);
            }
        }
        | StepResult::Break(GameResult {
            kind: GameResultKind::Win,
            ..
        }) => {
            append.push(AsciiChar::NumberSign);
        }
        | StepResult::Break(GameResult {
            kind: GameResultKind::Draw(_),
            ..
        }) => { /* TODO: nothing yet, this isn't Ascii :[ 1/2 / 1/2 or smt */ }
    }

    [core_move_notation, append].concat()
}

impl Move {
    #[must_use]
    pub fn lan(self) -> Vec<AsciiChar> {
        notation_creator(
            self,
            OriginAmbiguationLevel::Full,
            CaptureRepresentation {
                capture: Some(AsciiChar::SmallX),
                no_capture: Some(AsciiChar::HyphenMinus),
            },
        )
    }

    #[must_use]
    pub fn san(self) -> Vec<AsciiChar> {
        let capture_repr = CaptureRepresentation {
            capture: Some(AsciiChar::SmallX),
            no_capture: None,
        };
        let mut legal_moves = self.game.core.legal_inner_moves().collect::<Vec<_>>();

        let mov_index = legal_moves
            .iter()
            .position(|m| *m == self.inner)
            .expect("passed illegal move");

        legal_moves.swap_remove(mov_index);

        let interfering_moves = legal_moves
            .iter()
            .filter(|legal| legal.kind.piece_kind() == self.inner.kind.piece_kind())
            .filter(|legal| legal.destination == self.inner.destination)
            .filter(|legal| {
                // for the Promotion case, remove Duplicate Promotions to just different pieces.
                if self.inner.kind.is_promotion()
                    && legal.origin == self.inner.origin
                    && legal.destination == self.inner.destination
                {
                    return false;
                }
                true
            })
            .collect::<Vec<_>>();

        if interfering_moves.is_empty() {
            if self.inner.kind.piece_kind() == PieceKind::Pawn && self.inner.is_capture() {
                //Pawns always have the File when capturing!
                return notation_creator(self, OriginAmbiguationLevel::FileOnly, capture_repr);
            }
            return notation_creator(self, OriginAmbiguationLevel::Empty, capture_repr);
        }

        if not(interfering_moves
            .iter()
            .any(|inter| inter.origin.row == self.inner.origin.row))
        {
            return notation_creator(self, OriginAmbiguationLevel::RankOnly, capture_repr);
        }

        if not(interfering_moves
            .iter()
            .any(|inter| inter.origin.col == self.inner.origin.col))
        {
            return notation_creator(self, OriginAmbiguationLevel::FileOnly, capture_repr);
        }

        notation_creator(self, OriginAmbiguationLevel::Full, capture_repr)
    }
}

#[cfg(test)]
mod tests {
    use std::println;

    use super::*;
}
