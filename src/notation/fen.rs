use alloc::borrow::ToOwned;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::ascii::Char as AsciiChar;

use crate::board::Board;
use crate::coord::Col;
use crate::coord::ColIndexOutOfRange;
use crate::coord::Row;
use crate::coord::RowIndexOutOfRange;
use crate::coord::Square;
use crate::coord::SquareOutOfRange;
use crate::game::CastlingRight;
use crate::game::CastlingRights;
use crate::game::FiftyMoveRuleClock;
use crate::game::FullMoveCount;
use crate::game::GameStateCore;
use crate::piece::Piece;
use crate::piece::PieceKind;
use crate::player::PlayerKind;

struct FenStrings {
    piece_placements: Vec<AsciiChar>,
    active_player: Vec<AsciiChar>,
    castling_availability: Vec<AsciiChar>,
    en_passant_target_square: Vec<AsciiChar>,
    half_move_clock: Vec<AsciiChar>,
    full_move_number: Vec<AsciiChar>,
}

#[derive(Debug)]
pub enum GameFromFenError {
    NotAscii,
    WrongFieldCount,
    MalformedBoard(BoardFromFenError),
    MalformedPlayer(InvalidPlayer),
    MalformedEnPassantTarget(SquareFromFenError),
    MalformedFiftyRuleClock(core::num::ParseIntError),
    MalformedMoveCount(core::num::ParseIntError),
}
impl GameStateCore {
    pub fn try_from_fen(fen: &str) -> Result<Self, GameFromFenError> {
        let fen_parts: [Vec<AsciiChar>; 6] = fen
            .split_ascii_whitespace()
            .map(|str| {
                str.bytes()
                    .map(AsciiChar::from_u8)
                    .collect::<Option<Vec<AsciiChar>>>()
            })
            .collect::<Option<Vec<Vec<AsciiChar>>>>()
            .ok_or(GameFromFenError::NotAscii)?
            .try_into()
            .map_err(|_| GameFromFenError::WrongFieldCount)?;

        let fen = FenStrings {
            piece_placements: fen_parts[0].clone(),
            active_player: fen_parts[1].clone(),
            castling_availability: fen_parts[2].clone(),
            en_passant_target_square: fen_parts[3].clone(),
            half_move_clock: fen_parts[4].clone(),
            full_move_number: fen_parts[5].clone(),
        };

        let board = Board::try_from_fen_repr(fen.piece_placements.as_slice())
            .map_err(GameFromFenError::MalformedBoard)?;

        let fifty_move_rule_clock =
            FiftyMoveRuleClock::try_from_fen_repr(fen.half_move_clock.as_slice())
                .map_err(GameFromFenError::MalformedFiftyRuleClock)?;

        let castling_rights = CastlingRights::from_fen_repr(&fen.castling_availability);

        let active_player = PlayerKind::try_from_fen_repr(fen.active_player.as_slice())
            .map_err(GameFromFenError::MalformedPlayer)?;

        let en_passant_target = Square::try_from_fen_repr(fen.en_passant_target_square.as_slice())
            .map_err(GameFromFenError::MalformedEnPassantTarget)?;

        let full_move_count = FullMoveCount::try_from_fen_repr(fen.full_move_number.as_slice())
            .map_err(GameFromFenError::MalformedMoveCount)?;

        Ok(Self {
            board,
            fifty_move_rule_clock,
            castling_rights,
            en_passant_target,
            active_player,
            full_move_count,
        })
    }

    #[must_use]
    pub fn to_fen(&self) -> Vec<AsciiChar> {
        let Self {
            board,
            fifty_move_rule_clock,
            castling_rights,
            en_passant_target,
            active_player,
            full_move_count,
        } = self;

        let fen = FenStrings {
            piece_placements: Board::to_fen_repr(board),
            active_player: vec![PlayerKind::to_fen_repr(*active_player)],
            castling_availability: CastlingRights::to_fen_repr(*castling_rights),
            en_passant_target_square: Square::option_to_fen_repr(*en_passant_target),
            half_move_clock: FiftyMoveRuleClock::to_fen_repr(*fifty_move_rule_clock),
            full_move_number: FullMoveCount::to_fen_repr(*full_move_count),
        };

        let space = vec![AsciiChar::Space];

        [
            fen.piece_placements,
            space.clone(),
            fen.active_player,
            space.clone(),
            fen.castling_availability,
            space.clone(),
            fen.en_passant_target_square,
            space.clone(),
            fen.half_move_clock,
            space,
            fen.full_move_number,
        ]
        .concat()
    }
}

impl CastlingRights {
    #[must_use]
    fn from_fen_repr(value: &[AsciiChar]) -> Self {
        if value == [AsciiChar::HyphenMinus] {
            return Self::none_available();
        }
        Self {
            white_kingside: value.contains(&AsciiChar::CapitalK).into(), // `K`
            white_queenside: value.contains(&AsciiChar::CapitalQ).into(), // `Q`
            black_kingside: value.contains(&AsciiChar::SmallK).into(),   // `k`
            black_queenside: value.contains(&AsciiChar::SmallQ).into(),  // `q`
        }
    }

    #[must_use]
    fn to_fen_repr(self) -> Vec<AsciiChar> {
        if self == Self::none_available() {
            return vec![AsciiChar::HyphenMinus];
        }
        let mut out = vec![];
        if self.white_kingside == CastlingRight::Available {
            out.push(AsciiChar::CapitalK);
        }
        if self.white_queenside == CastlingRight::Available {
            out.push(AsciiChar::CapitalQ);
        }
        if self.black_kingside == CastlingRight::Available {
            out.push(AsciiChar::SmallK);
        }
        if self.black_queenside == CastlingRight::Available {
            out.push(AsciiChar::SmallQ);
        }
        assert!(!out.is_empty());
        out
    }
}

impl FullMoveCount {
    fn try_from_fen_repr(value: &[AsciiChar]) -> Result<Self, core::num::ParseIntError> {
        value.as_str().parse().map(Self)
    }

    fn to_fen_repr(self) -> Vec<AsciiChar> {
        self.0
            .to_string()
            .as_ascii()
            .expect("digits to be ascii")
            .to_owned()
    }
}

impl FiftyMoveRuleClock {
    fn try_from_fen_repr(value: &[AsciiChar]) -> Result<Self, core::num::ParseIntError> {
        Ok(Self::new(value.as_str().parse()?))
    }

    fn to_fen_repr(self) -> Vec<AsciiChar> {
        self.0
            .to_string()
            .as_ascii()
            .expect("digits to be ascii")
            .to_owned()
    }
}

impl PieceKind {
    #[must_use]
    pub const fn to_ascii_lower(self) -> AsciiChar {
        match self {
            Self::Pawn => AsciiChar::SmallP,
            Self::Knight => AsciiChar::SmallN,
            Self::Bishop => AsciiChar::SmallB,
            Self::Rook => AsciiChar::SmallR,
            Self::Queen => AsciiChar::SmallQ,
            Self::King => AsciiChar::SmallK,
        }
    }

    #[must_use]
    pub const fn to_ascii_upper(self) -> AsciiChar {
        match self {
            Self::Pawn => AsciiChar::CapitalP,
            Self::Knight => AsciiChar::CapitalN,
            Self::Bishop => AsciiChar::CapitalB,
            Self::Rook => AsciiChar::CapitalR,
            Self::Queen => AsciiChar::CapitalQ,
            Self::King => AsciiChar::CapitalK,
        }
    }
}
#[derive(Debug)]
pub struct MalformedPieceError;

impl Piece {
    pub(super) const fn try_from_fen_repr(value: AsciiChar) -> Result<Self, MalformedPieceError> {
        match value as u8 {
            b'P' => Ok(Self::WHITE_PAWN),
            b'N' => Ok(Self::WHITE_KNIGHT),
            b'B' => Ok(Self::WHITE_BISHOP),
            b'R' => Ok(Self::WHITE_ROOK),
            b'Q' => Ok(Self::WHITE_QUEEN),
            b'K' => Ok(Self::WHITE_KING),

            b'p' => Ok(Self::BLACK_PAWN),
            b'n' => Ok(Self::BLACK_KNIGHT),
            b'b' => Ok(Self::BLACK_BISHOP),
            b'r' => Ok(Self::BLACK_ROOK),
            b'q' => Ok(Self::BLACK_QUEEN),
            b'k' => Ok(Self::BLACK_KING),

            _ => Err(MalformedPieceError),
        }
    }

    #[must_use]
    pub(super) const fn to_fen_repr(self) -> AsciiChar {
        match self.owner {
            PlayerKind::White => self.kind.to_ascii_upper(),
            PlayerKind::Black => self.kind.to_ascii_lower(),
        }
    }
}

#[derive(Debug)]
pub enum InvalidPlayer {
    IllegalCharacter(AsciiChar),
    TooShort(usize),
    TooLong(usize),
}

impl PlayerKind {
    const fn try_from_fen_repr(value: &[AsciiChar]) -> Result<Self, InvalidPlayer> {
        match value {
            [AsciiChar::SmallW] => Ok(Self::White), // `w`
            [AsciiChar::SmallB] => Ok(Self::Black), // `b`
            [] => Err(InvalidPlayer::TooShort(value.len())),
            [illegal] => Err(InvalidPlayer::IllegalCharacter(*illegal)),
            [_, _, ..] => Err(InvalidPlayer::TooLong(value.len())),
        }
    }

    #[must_use]
    const fn to_fen_repr(self) -> AsciiChar {
        match self {
            Self::White => AsciiChar::SmallW,
            Self::Black => AsciiChar::SmallB,
        }
    }
}

#[derive(Debug)]
pub enum BoardFromFenError {
    IllegalCharacter(AsciiChar),
    IllegalRowDimensions,
    IllegalColDimensions,
}

impl Board {
    fn try_from_fen_repr(value: &[AsciiChar]) -> Result<Self, BoardFromFenError> {
        fn fen_row_to_board_row(
            row: &[AsciiChar],
        ) -> Result<[Option<Piece>; 8], BoardFromFenError> {
            let mut out_row: Vec<Option<Piece>> = vec![];

            for c in row {
                match *c as u8 {
                    b'1'..=b'8' => out_row.extend(vec![None; usize::from(u8::from(*c) - b'0')]),
                    b'P' | b'N' | b'B' | b'R' | b'Q' | b'K' | b'p' | b'n' | b'b' | b'r' | b'q'
                    | b'k' => {
                        out_row.push(Some(
                            Piece::try_from_fen_repr(*c)
                                .expect("these chars to be valid Piece reprs"),
                        ));
                    }

                    _ => return Err(BoardFromFenError::IllegalCharacter(*c)),
                }
            }

            out_row
                .try_into()
                .map_err(|_| BoardFromFenError::IllegalRowDimensions)
        }

        let piece_placements_chunked: Self = value
            .split(|c| *c == AsciiChar::Solidus)
            .map(fen_row_to_board_row)
            .collect::<Result<Vec<[Option<Piece>; 8]>, BoardFromFenError>>()?
            .try_into()
            .map(Board)
            .map_err(|_| BoardFromFenError::IllegalColDimensions)?;

        let mut new_board = Self::empty();

        //TODO: un-fuck this
        for square in Square::ALL {
            new_board[Square {
                col: Col::try_from(u8::from(square.row))
                    .expect("valid row numbers to be valid col numbers"),
                row: Row::try_from(9 - u8::from(square.col))
                    .expect("9 - 1..=8 to be in range 1..=8"),
            }] = piece_placements_chunked[square];
        }

        Ok(new_board)
    }

    #[must_use]
    fn to_fen_repr(&self) -> Vec<AsciiChar> {
        let mut running_square_count: u32 = 0;
        let mut out: Vec<AsciiChar> = vec![];
        for square in Square::ALL {
            match self[square] {
                None => running_square_count += 1,
                Some(piece) => {
                    if running_square_count != 0 {
                        out.extend(
                            running_square_count
                                .to_string()
                                .as_ascii()
                                .expect("digits to be ascii")
                                .to_owned(),
                        ); //as the running count should never exceed 8, this should always be a single digit
                    }
                    running_square_count = 0;
                    out.push(Piece::to_fen_repr(piece));
                }
            }
            if square.col == Col::_8 {
                if running_square_count != 0 {
                    out.extend(
                        running_square_count
                            .to_string()
                            .as_ascii()
                            .expect("digits to be ascii")
                            .to_owned(),
                    ); //as the running count should never exceed 8, this should always be a single digit
                }
                running_square_count = 0;
                if square != Square::H1 {
                    out.push(AsciiChar::Solidus);
                }
            }
        }
        out
    }
}

impl Col {
    const fn try_from_fen_repr(value: AsciiChar) -> Result<Self, ColIndexOutOfRange> {
        Self::try_from(u8::from(value) - b'a' + 1)
    }
    #[must_use]
    pub(crate) const fn to_fen_repr(self) -> AsciiChar {
        AsciiChar::from_u8(u8::from(self) + b'a' - 1).expect("a..=h to be ascii")
    }
}
impl Row {
    const fn try_from_fen_repr(value: AsciiChar) -> Result<Self, RowIndexOutOfRange> {
        Self::try_from(u8::from(value) - b'0')
    }
    #[must_use]
    pub(crate) const fn to_fen_repr(self) -> AsciiChar {
        AsciiChar::from_u8(u8::from(self) + b'0').expect("0..=8 to be ascii")
    }
}

#[derive(Debug)]
pub enum SquareFromFenError {
    OutOfRange(SquareOutOfRange),
    IllegalCharacter(AsciiChar),
    TooShort(usize),
    TooLong(usize),
}
impl From<SquareOutOfRange> for SquareFromFenError {
    fn from(value: SquareOutOfRange) -> Self {
        Self::OutOfRange(value)
    }
}
impl Square {
    fn try_from_fen_repr(value: &[AsciiChar]) -> Result<Option<Self>, SquareFromFenError> {
        // supposed to look like `-` || `a5` / `d2` / `f7` / ...
        match value {
            [AsciiChar::HyphenMinus] => Ok(None), //  `-`
            [col, row] => Ok(Some(Self {
                col: Col::try_from_fen_repr(*col).map_err(SquareOutOfRange::from)?,
                row: Row::try_from_fen_repr(*row).map_err(SquareOutOfRange::from)?,
            })),
            [] => Err(SquareFromFenError::TooShort(value.len())),
            [illegal] => Err(SquareFromFenError::IllegalCharacter(*illegal)),
            [_, _, _, ..] => Err(SquareFromFenError::TooLong(value.len())),
        }
    }

    #[must_use]
    pub(super) const fn to_fen_repr(self) -> [AsciiChar; 2] {
        [Col::to_fen_repr(self.col), Row::to_fen_repr(self.row)]
    }

    #[must_use]
    fn option_to_fen_repr(value: Option<Self>) -> Vec<AsciiChar> {
        #[allow(clippy::option_if_let_else)]
        match value {
            None => vec![AsciiChar::HyphenMinus], // `-`
            Some(square) => square.to_fen_repr().to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::println;

    use super::*;

    #[test]
    fn test_squares_fen_round_trip() {
        for square in Square::ALL {
            println!("{square:?}: {}", Square::to_fen_repr(square).as_str());
            assert_eq!(
                Some(square),
                Square::try_from_fen_repr(&Square::to_fen_repr(square)).unwrap()
            );
        }
    }

    #[test]
    fn test_columns_fen_round_trip() {
        for col in Col::ALL {
            println!("{col:?}: {}", Col::to_fen_repr(col).as_str());
            assert_eq!(col, Col::try_from_fen_repr(Col::to_fen_repr(col)).unwrap());
        }
    }

    #[test]
    fn test_rows_fen_round_trip() {
        for row in Row::ALL {
            println!("{row:?}: {}", Row::to_fen_repr(row).as_str());
            assert_eq!(row, Row::try_from_fen_repr(Row::to_fen_repr(row)).unwrap());
        }
    }

    #[test]
    fn test_pieces_fen_round_trip() {
        for piece in Piece::ALL {
            println!("{piece}: {}", Piece::to_fen_repr(piece).as_str());
            assert_eq!(
                piece,
                Piece::try_from_fen_repr(Piece::to_fen_repr(piece)).unwrap()
            );
        }
    }

    #[test]
    fn test_player_kind_fens_round_trip() {
        for player_kind in PlayerKind::ALL {
            println!(
                "{player_kind:?}: {}",
                PlayerKind::to_fen_repr(player_kind).as_str()
            );
            assert_eq!(
                player_kind,
                PlayerKind::try_from_fen_repr(
                    vec![PlayerKind::to_fen_repr(player_kind)].as_slice()
                )
                .unwrap()
            );
        }
    }

    #[test]
    fn test_castling_rights_fen_round_trip() {
        for castling_rights in CastlingRights::ALL {
            println!(
                "{castling_rights:?}: {}",
                CastlingRights::to_fen_repr(castling_rights).as_str()
            );
            assert_eq!(
                castling_rights,
                CastlingRights::from_fen_repr(
                    CastlingRights::to_fen_repr(castling_rights).as_slice()
                )
            );
        }
    }

    #[test]
    fn test_initial_game_state() {
        let starting_position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            starting_position_fen,
            GameStateCore::default().to_fen().as_str()
        );
    }
}
