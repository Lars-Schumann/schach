use crate::coord::Square;
use crate::game::CastlingSide;
use crate::piece::Piece;
use crate::piece::PieceKind;

#[derive(Clone, Copy)]
pub(crate) struct Threat {
    pub(crate) piece: Piece,
    pub(crate) origin: Square,
    pub(crate) destination: Square,
}

impl core::fmt::Debug for Threat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}: {:?} => {:?}",
            self.piece, self.origin, self.destination
        )
    }
}

#[derive_const(PartialEq, Eq)]
#[derive(Debug, Copy, Clone)]
pub enum MoveKind {
    Pawn(PawnMove),
    Knight { is_capture: bool },
    Bishop { is_capture: bool },
    Rook { is_capture: bool },
    Queen { is_capture: bool },
    King(KingMove),
}

impl MoveKind {
    #[must_use]
    pub const fn piece_kind(&self) -> PieceKind {
        match self {
            | Self::Pawn(_) => PieceKind::Pawn,
            | Self::Knight { .. } => PieceKind::Knight,
            | Self::Bishop { .. } => PieceKind::Bishop,
            | Self::Rook { .. } => PieceKind::Rook,
            | Self::Queen { .. } => PieceKind::Queen,
            | Self::King(_) => PieceKind::King,
        }
    }

    #[must_use]
    pub const fn is_pawn(&self) -> bool {
        matches!(self, Self::Pawn(_))
    }

    #[must_use]
    pub const fn is_pawn_double_step(&self) -> bool {
        self == &Self::Pawn(PawnMove::DoubleStep)
    }

    #[must_use]
    pub const fn is_pawn_en_passant(&self) -> bool {
        matches!(self, Self::Pawn(PawnMove::EnPassant { .. }))
    }

    #[must_use]
    pub const fn is_promotion(&self) -> bool {
        matches!(
            self,
            Self::Pawn(
                PawnMove::SingleStep {
                    promotion_replacement: Some(_)
                } | PawnMove::Capture {
                    promotion_replacement: Some(_)
                }
            )
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub kind: MoveKind,
    pub origin: Square,
    pub destination: Square,
}
impl Move {
    #[must_use]
    pub const fn is_capture(&self) -> bool {
        match self.kind {
            | MoveKind::Knight { is_capture, .. }
            | MoveKind::Bishop { is_capture, .. }
            | MoveKind::Rook { is_capture, .. }
            | MoveKind::Queen { is_capture, .. }
            | MoveKind::King(KingMove::Normal { is_capture, .. }) => is_capture,
            | MoveKind::Pawn(PawnMove::Capture { .. } | PawnMove::EnPassant { .. }) => true,
            | MoveKind::Pawn(PawnMove::SingleStep { .. } | PawnMove::DoubleStep)
            | MoveKind::King(KingMove::Castle { .. }) => false,
        }
    }

    #[must_use]
    pub const fn is_pawn_or_capture(&self) -> bool {
        self.kind.piece_kind() == PieceKind::Pawn || self.is_capture()
    }
}

#[derive_const(PartialEq, Eq)]
#[derive(Debug, Copy, Clone)]
pub enum PawnMove {
    SingleStep {
        promotion_replacement: Option<Piece>,
    },
    DoubleStep,
    Capture {
        promotion_replacement: Option<Piece>,
    },
    EnPassant {
        affected: Square,
    },
}

#[derive_const(PartialEq, Eq)]
#[derive(Debug, Copy, Clone)]
pub enum KingMove {
    Normal {
        is_capture: bool,
    },
    Castle {
        rook_start: Square,
        rook_target: Square,
        castling_side: CastlingSide,
    },
}
