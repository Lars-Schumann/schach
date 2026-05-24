use crate::common::no_fmt;
use crate::coord::Offset;
use crate::player::PlayerKind;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
impl PieceKind {
    pub const ALL: [Self; 6] = [
        Self::Pawn,
        Self::Knight,
        Self::Bishop,
        Self::Rook,
        Self::Queen,
        Self::King,
    ];

    pub const PROMOTION_OPTIONS: [Self; 4] = [Self::Knight, Self::Bishop, Self::Rook, Self::Queen];

    #[must_use]
    pub const fn to_black_piece(self) -> Piece {
        Piece::new(PlayerKind::Black, self)
    }

    #[must_use]
    pub const fn to_white_piece(self) -> Piece {
        Piece::new(PlayerKind::White, self)
    }

    #[must_use]
    pub const fn to_piece(self, owner: PlayerKind) -> Piece {
        Piece::new(owner, self)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub struct Piece {
    pub owner: PlayerKind,
    pub kind: PieceKind,
}
impl Piece {
    #[must_use]
    pub const fn new(owner: PlayerKind, kind: PieceKind) -> Self {
        Self { owner, kind }
    }

    no_fmt! {
    pub const WHITE_PAWN:   Self = Self::new(PlayerKind::White, PieceKind::Pawn);
    pub const WHITE_KNIGHT: Self = Self::new(PlayerKind::White, PieceKind::Knight);
    pub const WHITE_BISHOP: Self = Self::new(PlayerKind::White, PieceKind::Bishop);
    pub const WHITE_ROOK:   Self = Self::new(PlayerKind::White, PieceKind::Rook);
    pub const WHITE_QUEEN:  Self = Self::new(PlayerKind::White, PieceKind::Queen);
    pub const WHITE_KING:   Self = Self::new(PlayerKind::White, PieceKind::King);

    pub const BLACK_PAWN:   Self = Self::new(PlayerKind::Black, PieceKind::Pawn);
    pub const BLACK_KNIGHT: Self = Self::new(PlayerKind::Black, PieceKind::Knight);
    pub const BLACK_BISHOP: Self = Self::new(PlayerKind::Black, PieceKind::Bishop);
    pub const BLACK_ROOK:   Self = Self::new(PlayerKind::Black, PieceKind::Rook);
    pub const BLACK_QUEEN:  Self = Self::new(PlayerKind::Black, PieceKind::Queen);
    pub const BLACK_KING:   Self = Self::new(PlayerKind::Black, PieceKind::King);
    }

    pub const ALL: [Self; 12] = [
        Self::WHITE_PAWN,
        Self::WHITE_KING,
        Self::WHITE_BISHOP,
        Self::WHITE_ROOK,
        Self::WHITE_QUEEN,
        Self::WHITE_KING,
        Self::BLACK_PAWN,
        Self::BLACK_KNIGHT,
        Self::BLACK_BISHOP,
        Self::BLACK_ROOK,
        Self::BLACK_QUEEN,
        Self::BLACK_KING,
    ];

    #[must_use]
    pub(crate) const fn threat_directions(self) -> (&'static [Offset], Range) {
        match (self.kind, self.owner) {
            (PieceKind::Pawn, PlayerKind::White) => (&Offset::PAWN_UP_DIAGONAL, Range::One),
            (PieceKind::Pawn, PlayerKind::Black) => (&Offset::PAWN_DOWN_DIAGONAL, Range::One),
            (PieceKind::Knight, _) => (&Offset::KNIGHT, Range::One),
            (PieceKind::Bishop, _) => (&Offset::BISHOP, Range::Unlimited),
            (PieceKind::Rook, _) => (&Offset::ROOK, Range::Unlimited),
            (PieceKind::Queen, _) => (&Offset::QUEEN, Range::Unlimited),
            (PieceKind::King, _) => (&Offset::KING_DIRECT, Range::One),
        }
    }
}

impl core::fmt::Display for Piece {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let symbol = match *self {
            Self::WHITE_PAWN => "♙",
            Self::WHITE_KNIGHT => "♘",
            Self::WHITE_BISHOP => "♗",
            Self::WHITE_ROOK => "♖",
            Self::WHITE_QUEEN => "♕",
            Self::WHITE_KING => "♔",

            Self::BLACK_PAWN => "♟",
            Self::BLACK_KNIGHT => "♞",
            Self::BLACK_BISHOP => "♝",
            Self::BLACK_ROOK => "♜",
            Self::BLACK_QUEEN => "♛",
            Self::BLACK_KING => "♚",
        };
        write!(f, "{symbol}")
    }
}

pub(crate) enum Range {
    One,
    Unlimited,
}
impl From<Range> for i32 {
    fn from(value: Range) -> Self {
        match value {
            Range::One => 1,
            Range::Unlimited => Self::MAX,
        }
    }
}
