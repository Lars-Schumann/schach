use alloc::vec::Vec;

use crate::coord::Col;
use crate::coord::Square;
use crate::game::PieceCounts;
use crate::game::attacked_squares;
use crate::mv::Threat;
use crate::piece::Piece;
use crate::piece::PieceKind;
use crate::player::PlayerKind;

pub const COL_COUNT: usize = 8;
pub const ROW_COUNT: usize = 8;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Board(pub [[Option<Piece>; ROW_COUNT]; COL_COUNT]);
impl Board {
    #[must_use]
    pub const fn empty() -> Self {
        Self([[None; ROW_COUNT]; COL_COUNT])
    }

    pub(crate) fn threatening_moves_by(
        &self,
        threatened_by: PlayerKind,
    ) -> impl Iterator<Item = Threat> {
        Square::ALL
            .iter()
            .flat_map(move |square| crate::game::attacked_squares(self, *square, threatened_by))
    }

    pub fn threatened_squares_by(&self, threatened_by: PlayerKind) -> impl Iterator<Item = Square> {
        Square::ALL.iter().flat_map(move |square| {
            attacked_squares(self, *square, threatened_by).map(|threat| threat.destination)
        })
    }

    #[must_use]
    pub fn king_position(&self, king_owner: PlayerKind) -> Square {
        *Square::ALL
            .iter()
            .find(|square| {
                self[**square]
                    == Some(Piece {
                        kind: PieceKind::King,
                        owner: king_owner,
                    })
            })
            .expect("where did the king go?")
    }

    #[must_use]
    pub fn is_king_checked(&self, king_owner: PlayerKind) -> bool {
        self.threatened_squares_by(king_owner.opponent())
            .any(|square| square == self.king_position(king_owner))
    }

    pub const fn mov(&mut self, start: Square, target: Square) {
        self[target] = self[start];
        self[start] = None;
    }

    #[must_use]
    pub const fn new() -> Self {
        #[allow(clippy::wildcard_imports)]
        use crate::coord::Square as S;
        use crate::piece::Piece as P;

        let mut board = Self::empty();

        board[S::A1] = Some(P::WHITE_ROOK);
        board[S::B1] = Some(P::WHITE_KNIGHT);
        board[S::C1] = Some(P::WHITE_BISHOP);
        board[S::D1] = Some(P::WHITE_QUEEN);
        board[S::E1] = Some(P::WHITE_KING);
        board[S::F1] = Some(P::WHITE_BISHOP);
        board[S::G1] = Some(P::WHITE_KNIGHT);
        board[S::H1] = Some(P::WHITE_ROOK);

        board[S::A8] = Some(P::BLACK_ROOK);
        board[S::B8] = Some(P::BLACK_KNIGHT);
        board[S::C8] = Some(P::BLACK_BISHOP);
        board[S::D8] = Some(P::BLACK_QUEEN);
        board[S::E8] = Some(P::BLACK_KING);
        board[S::F8] = Some(P::BLACK_BISHOP);
        board[S::G8] = Some(P::BLACK_KNIGHT);
        board[S::H8] = Some(P::BLACK_ROOK);

        board[S::A2] = Some(P::WHITE_PAWN);
        board[S::B2] = Some(P::WHITE_PAWN);
        board[S::C2] = Some(P::WHITE_PAWN);
        board[S::D2] = Some(P::WHITE_PAWN);
        board[S::E2] = Some(P::WHITE_PAWN);
        board[S::F2] = Some(P::WHITE_PAWN);
        board[S::G2] = Some(P::WHITE_PAWN);
        board[S::H2] = Some(P::WHITE_PAWN);

        board[S::A7] = Some(P::BLACK_PAWN);
        board[S::B7] = Some(P::BLACK_PAWN);
        board[S::C7] = Some(P::BLACK_PAWN);
        board[S::D7] = Some(P::BLACK_PAWN);
        board[S::E7] = Some(P::BLACK_PAWN);
        board[S::F7] = Some(P::BLACK_PAWN);
        board[S::G7] = Some(P::BLACK_PAWN);
        board[S::H7] = Some(P::BLACK_PAWN);

        board
    }

    #[must_use]
    pub(crate) fn piece_counts(&self) -> PieceCounts {
        let mut piece_counts = PieceCounts::default();
        for square in Square::ALL {
            if let Some(piece) = self[square] {
                piece_counts[piece] += 1;
            }
        }
        piece_counts
    }
}
impl const Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
impl const core::ops::Index<Square> for Board {
    type Output = Option<Piece>;
    fn index(&self, index: Square) -> &Self::Output {
        let col = usize::from(u8::from(index.col) - 1);
        let row = usize::from(u8::from(index.row) - 1);
        &self.0[col][row]
    }
}
impl const core::ops::IndexMut<Square> for Board {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        let col = usize::from(u8::from(index.col) - 1);
        let row = usize::from(u8::from(index.row) - 1);
        &mut self.0[col][row]
    }
}
impl core::fmt::Debug for Board {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f)?;
        for square in Square::ALL {
            if square.col == Col::_1 {
                write!(f, "{}", i8::from(square.row))?;
            }
            match self[square] {
                Some(piece) => write!(f, "{piece} ")?,
                None => {
                    if square.is_black() {
                        write!(f, "□ ")?;
                    } else {
                        write!(f, "■ ")?;
                    }
                }
            }
            if square.col == Col::_8 {
                writeln!(f)?;
            }
        }
        write!(f, "  ")?;
        for col in Col::ALL {
            write!(f, "{} ", i8::from(col))?;
        }
        Ok(())
    }
}

#[allow(unused)]
pub(crate) struct DebugBoard {
    pub inner: Board,
    pub highlighted_squares: Vec<Square>,
}
impl core::fmt::Debug for DebugBoard {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f)?;
        for square in Square::ALL {
            if square.col == Col::_1 {
                write!(f, "{}", i8::from(square.row))?;
            }
            if self.highlighted_squares.contains(&square) {
                write!(f, "\x1B[31m")?;
            }
            match self.inner[square] {
                Some(piece) => write!(f, "{piece} ")?,
                None => {
                    if square.is_black() {
                        write!(f, "□ ")?;
                    } else {
                        write!(f, "■ ")?;
                    }
                }
            }
            if self.highlighted_squares.contains(&square) {
                write!(f, "\x1B[0m")?;
            }
            if square.col == Col::_8 {
                writeln!(f)?;
            }
        }
        write!(f, "  ")?;
        for col in Col::ALL {
            write!(f, "{} ", i8::from(col))?;
        }
        Ok(())
    }
}
