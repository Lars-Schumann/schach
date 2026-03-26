use crate::coord::Offset;
use crate::coord::Row;
use crate::coord::Square as S;
use crate::coord::Square;
use crate::game::CastlingSide as CS;

#[derive_const(PartialEq, Eq)]
#[derive(Debug, Copy, Clone, Default)]
pub enum PlayerKind {
    #[default]
    White,
    Black,
}
impl PlayerKind {
    pub const ALL: [Self; 2] = [Self::White, Self::Black];

    #[must_use]
    pub const fn opponent(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    #[must_use]
    pub(crate) const fn pawn_starting_row(self) -> Row {
        match self {
            Self::White => Row::_2,
            Self::Black => Row::_7,
        }
    }

    #[must_use]
    pub(crate) const fn pawn_promotion_row(self) -> Row {
        match self {
            Self::White => Row::_8,
            Self::Black => Row::_1,
        }
    }

    #[must_use]
    pub(crate) const fn castling_non_check_needed_squares(self, castling_side: CS) -> [Square; 3] {
        match (self, castling_side) {
            (Self::White, CS::Kingside) => [S::E1, S::F1, S::G1],
            (Self::White, CS::Queenside) => [S::E1, S::D1, S::C1],
            (Self::Black, CS::Kingside) => [S::E8, S::F8, S::G8],
            (Self::Black, CS::Queenside) => [S::E8, S::D8, S::C8],
        }
    }

    #[must_use]
    pub(crate) const fn castling_free_needed_squares(self, castling_side: CS) -> &'static [Square] {
        match (self, castling_side) {
            (Self::White, CS::Kingside) => &[S::F1, S::G1],
            (Self::White, CS::Queenside) => &[S::D1, S::C1, S::B1],
            (Self::Black, CS::Kingside) => &[S::F8, S::G8],
            (Self::Black, CS::Queenside) => &[S::D8, S::C8, S::B8],
        }
    }

    #[must_use]
    pub(crate) const fn king_start(self) -> Square {
        match self {
            Self::White => S::E1,
            Self::Black => S::E8,
        }
    }

    #[must_use]
    pub(crate) const fn king_castling_target(self, castling_side: CS) -> Square {
        match (self, castling_side) {
            (Self::White, CS::Kingside) => S::G1,
            (Self::White, CS::Queenside) => S::C1,
            (Self::Black, CS::Kingside) => S::G8,
            (Self::Black, CS::Queenside) => S::C8,
        }
    }

    #[must_use]
    pub(crate) const fn rook_start(self, castling_side: CS) -> Square {
        match (self, castling_side) {
            (Self::White, CS::Kingside) => S::H1,
            (Self::White, CS::Queenside) => S::A1,
            (Self::Black, CS::Kingside) => S::H8,
            (Self::Black, CS::Queenside) => S::A8,
        }
    }

    #[must_use]
    pub const fn rook_castling_target(self, castling_side: CS) -> Square {
        match (self, castling_side) {
            (Self::White, CS::Kingside) => S::F1,
            (Self::White, CS::Queenside) => S::D1,
            (Self::Black, CS::Kingside) => S::F8,
            (Self::Black, CS::Queenside) => S::D8,
        }
    }

    #[must_use]
    pub(crate) const fn forwards_one_row(self) -> Offset {
        match self {
            Self::White => Offset::U,
            Self::Black => Offset::D,
        }
    }

    #[must_use]
    pub(crate) const fn backwards_one_row(self) -> Offset {
        self.forwards_one_row() * -1
    }
}
