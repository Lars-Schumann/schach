use alloc::vec::Vec;
use core::num::NonZeroU64;
use core::ops::Index;
use core::ops::IndexMut;
use core::ops::Not;

use crate::board::Board;
use crate::coord::Square;
use crate::mv::Move;
use crate::mv::MoveKind;
use crate::mv::Threat;
use crate::piece::Piece;
use crate::player::PlayerKind;

pub(crate) static REPETITIONS_TO_FORCED_DRAW_COUNT: usize = 5;
pub(crate) static FIFTY_MOVE_RULE_COUNT: FiftyMoveRuleClock = FiftyMoveRuleClock(100);

#[derive_const(PartialEq, Eq)]
#[derive(Debug, Copy, Clone)]
pub enum CastlingSide {
    Kingside,
    Queenside,
}
impl CastlingSide {
    pub const ALL: [Self; 2] = [Self::Kingside, Self::Queenside];
}

#[derive_const(Clone, PartialEq, Eq)]
#[derive(Debug, Copy, Hash)]
pub enum DrawKind {
    Stalemate,
    ThreefoldRepetition,
    FiftyMove,
    InsufficientMaterial,
}

#[derive_const(Clone, PartialEq, Eq)]
#[derive(Debug, Copy, Hash)]
pub enum GameResultKind {
    Draw(DrawKind),
    Win,
}
impl GameResultKind {
    #[must_use]
    pub const fn is_win(&self) -> bool {
        self == &Self::Win
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameResult {
    pub kind: GameResultKind,
    pub final_game_state: GameState<Terminated>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepResult {
    Ongoing(GameState<Ongoing>),
    Terminated(GameResult),
}

#[derive_const(Default, Clone, PartialEq, Eq)]
#[derive(Debug, Copy, Hash)]
pub(crate) struct PieceCounts {
    pub(crate) white_pawn: u8,
    pub(crate) white_knight: u8,
    pub(crate) white_bishop: u8,
    pub(crate) white_rook: u8,
    pub(crate) white_queen: u8,
    pub(crate) white_king: u8,
    pub(crate) black_pawn: u8,
    pub(crate) black_knight: u8,
    pub(crate) black_bishop: u8,
    pub(crate) black_rook: u8,
    pub(crate) black_queen: u8,
    pub(crate) black_king: u8,
}
impl PieceCounts {
    const KINGS_ONLY: Self = Self {
        white_king: 1,
        black_king: 1,
        ..Default::default()
    };

    const WHITE_KING_AND_TWO_KNIGHTS: Self = Self {
        white_king: 1,
        white_knight: 2,
        black_king: 1,
        ..Default::default()
    };

    const BLACK_KING_AND_TWO_KNIGHTS: Self = Self {
        black_king: 1,
        black_knight: 2,
        white_king: 1,
        ..Default::default()
    };
}
impl Index<Piece> for PieceCounts {
    type Output = u8;

    fn index(&self, index: Piece) -> &Self::Output {
        #[rustfmt::skip]
        match index {
            Piece::WHITE_PAWN   => &self.white_pawn,
            Piece::WHITE_KNIGHT => &self.white_knight,
            Piece::WHITE_BISHOP => &self.white_bishop,
            Piece::WHITE_ROOK   => &self.white_rook,
            Piece::WHITE_QUEEN  => &self.white_queen,
            Piece::WHITE_KING   => &self.white_king,

            Piece::BLACK_PAWN   => &self.black_pawn,
            Piece::BLACK_KNIGHT => &self.black_knight,
            Piece::BLACK_BISHOP => &self.black_bishop,
            Piece::BLACK_ROOK   => &self.black_rook,
            Piece::BLACK_QUEEN  => &self.black_queen,
            Piece::BLACK_KING   => &self.black_king,
        }
    }
}
impl IndexMut<Piece> for PieceCounts {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        #[rustfmt::skip]
        match index {
            Piece::WHITE_PAWN   => &mut self.white_pawn,
            Piece::WHITE_KNIGHT => &mut self.white_knight,
            Piece::WHITE_BISHOP => &mut self.white_bishop,
            Piece::WHITE_ROOK   => &mut self.white_rook,
            Piece::WHITE_QUEEN  => &mut self.white_queen,
            Piece::WHITE_KING   => &mut self.white_king,

            Piece::BLACK_PAWN   => &mut self.black_pawn,
            Piece::BLACK_KNIGHT => &mut self.black_knight,
            Piece::BLACK_BISHOP => &mut self.black_bishop,
            Piece::BLACK_ROOK   => &mut self.black_rook,
            Piece::BLACK_QUEEN  => &mut self.black_queen,
            Piece::BLACK_KING   => &mut self.black_king,
        }
    }
}

#[derive_const(PartialEq, Eq)]
#[derive(Debug, Clone, Copy, Hash)]
pub struct FullMoveCount(pub NonZeroU64); // non-zero & unsigned because this always starts at 1 and cant decrease 
impl const Default for FullMoveCount {
    fn default() -> Self {
        Self::new()
    }
}
impl FullMoveCount {
    #[must_use]
    pub const fn new() -> Self {
        const { Self(NonZeroU64::new(1).expect("1 to not be 0")) }
    }
    pub const fn increase(&mut self) {
        self.0 = self
            .0
            .checked_add(1)
            .expect("a game to not take more than u64::MAX turns");
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FiftyMoveRuleClock(pub u64);
impl FiftyMoveRuleClock {
    #[must_use]
    pub const fn new(initial: u64) -> Self {
        Self(initial)
    }
    pub const fn increase(&mut self) {
        self.0 += 1;
    }
    pub const fn reset(&mut self) {
        self.0 = 0;
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleSet {
    #[default]
    Standard,
    Perft,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GameStateCore {
    pub board: Board,
    pub fifty_move_rule_clock: FiftyMoveRuleClock,
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<Square>,
    pub active_player: PlayerKind,
    pub full_move_count: FullMoveCount,
}
impl GameStateCore {
    #[must_use]
    pub const fn with_opponent_active(mut self) -> Self {
        self.active_player = self.active_player.opponent();
        self
    }

    #[must_use]
    pub(crate) const fn has_castling_right(&self, castling_side: CastlingSide) -> bool {
        match (self.active_player, castling_side) {
            (PlayerKind::White, CastlingSide::Kingside) => self.castling_rights.white_kingside,
            (PlayerKind::White, CastlingSide::Queenside) => self.castling_rights.white_queenside,
            (PlayerKind::Black, CastlingSide::Kingside) => self.castling_rights.black_kingside,
            (PlayerKind::Black, CastlingSide::Queenside) => self.castling_rights.black_queenside,
        }
    }

    pub(crate) const fn deny_castling(
        &mut self,
        for_player: PlayerKind,
        castling_side: CastlingSide,
    ) {
        match (for_player, castling_side) {
            (PlayerKind::White, CastlingSide::Kingside) => {
                self.castling_rights.white_kingside = false;
            }
            (PlayerKind::White, CastlingSide::Queenside) => {
                self.castling_rights.white_queenside = false;
            }
            (PlayerKind::Black, CastlingSide::Kingside) => {
                self.castling_rights.black_kingside = false;
            }
            (PlayerKind::Black, CastlingSide::Queenside) => {
                self.castling_rights.black_queenside = false;
            }
        }
    }

    //TODO: better name
    pub(crate) fn are_castle_squares_free_from_checks_and_pieces(
        &self,
        castling_side: CastlingSide,
    ) -> bool {
        let threatened_squares = self
            .board
            .threatened_squares_by(self.active_player.opponent())
            .collect::<Vec<_>>();

        let are_castle_squares_free_from_checks = self
            .active_player
            .castling_non_check_needed_squares(castling_side)
            .iter()
            .all(|castle_square| {
                threatened_squares
                    .iter()
                    .all(|threatened_square| threatened_square != castle_square)
            });

        let are_castle_squares_free_from_pieces = self
            .active_player
            .castling_free_needed_squares(castling_side)
            .iter()
            .all(|square| self.board[*square].is_none());

        are_castle_squares_free_from_checks && are_castle_squares_free_from_pieces
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Ongoing;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Terminated;
pub trait Phase {}

impl Phase for Ongoing {}
impl Phase for Terminated {}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GameState<P: Phase> {
    pub core: GameStateCore,
    pub position_history: Vec<Position>,
    pub rule_set: RuleSet,
    phase: core::marker::PhantomData<P>,
}

impl GameState<Ongoing> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn terminated(self) -> GameState<Terminated> {
        GameState::<Terminated> {
            core: self.core,
            position_history: self.position_history,
            rule_set: self.rule_set,
            phase: core::marker::PhantomData::<Terminated>,
        }
    }

    #[must_use]
    pub fn with_core(core: GameStateCore) -> Self {
        Self {
            core,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn perft() -> Self {
        Self {
            rule_set: RuleSet::Perft,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn step(mut self, mv: Move) -> StepResult {
        self.core.board.apply_move(mv);
        let mut game = self;

        // handle our own castling rights
        match mv.kind {
            MoveKind::King(_) => {
                game.core
                    .deny_castling(game.core.active_player, CastlingSide::Kingside);
                game.core
                    .deny_castling(game.core.active_player, CastlingSide::Queenside);
            }
            MoveKind::Rook { .. } => {
                for castling_side in [CastlingSide::Kingside, CastlingSide::Queenside] {
                    if mv.origin == game.core.active_player.rook_start(castling_side) {
                        game.core
                            .deny_castling(game.core.active_player, castling_side);
                    }
                }
            }
            _ => { /*nothing */ }
        }

        // handle opponents castling rights
        if mv.is_capture() && mv.kind.is_pawn_en_passant().not() {
            for castling_side in CastlingSide::ALL {
                if mv.destination == game.core.active_player.opponent().rook_start(castling_side) {
                    game.core
                        .deny_castling(game.core.active_player.opponent(), castling_side);
                }
            }
        }

        // handle en passant target, and only set the square if taking will actually be an option!
        game.core.en_passant_target = if mv.kind.is_pawn_double_step() {
            let possible_en_passant_target = (mv.destination
                + game.core.active_player.backwards_one_row())
            .expect("this to always be on the board");

            let mut future = game.core.with_opponent_active();
            future.en_passant_target = Some(possible_en_passant_target);

            future
                .legal_moves()
                .any(|mv| mv.kind.is_pawn_en_passant())
                .then_some(possible_en_passant_target)
        } else {
            None
        };

        let current_position = Position {
            board: game.core.board,
            castling_rights: game.core.castling_rights,
            en_passant_target: game.core.en_passant_target,
        };

        if game.rule_set != RuleSet::Perft {
            game.position_history.push(current_position.clone());

            // handle fifty move rule counter
            if mv.is_pawn_or_capture() {
                game.core.fifty_move_rule_clock.reset();
            } else {
                game.core.fifty_move_rule_clock.increase();
            }
        }

        let future = game.core.with_opponent_active();
        if future.legal_moves().count() == 0 {
            return if future.board.is_king_checked(future.active_player) {
                StepResult::Terminated(GameResult {
                    kind: GameResultKind::Win,
                    final_game_state: game.terminated(),
                })
            } else {
                StepResult::Terminated(GameResult {
                    kind: GameResultKind::Draw(DrawKind::Stalemate),
                    final_game_state: game.terminated(),
                })
            };
        }

        if game.rule_set != RuleSet::Perft {
            if game
                .position_history
                .iter()
                .filter(|&position| *position == current_position)
                .count()
                == REPETITIONS_TO_FORCED_DRAW_COUNT
            {
                return StepResult::Terminated(GameResult {
                    kind: GameResultKind::Draw(DrawKind::ThreefoldRepetition),
                    final_game_state: game.terminated(),
                });
            }

            if game.core.fifty_move_rule_clock == FIFTY_MOVE_RULE_COUNT {
                return StepResult::Terminated(GameResult {
                    kind: GameResultKind::Draw(DrawKind::FiftyMove),
                    final_game_state: game.terminated(),
                });
            }
        }

        let piece_counts = game.core.board.piece_counts();

        if piece_counts == PieceCounts::KINGS_ONLY {
            return StepResult::Terminated(GameResult {
                kind: GameResultKind::Draw(DrawKind::InsufficientMaterial),
                final_game_state: game.terminated(),
            });
        }

        if game.core.active_player == PlayerKind::Black {
            game.core.full_move_count.increase();
        }

        game.core.active_player = game.core.active_player.opponent();
        StepResult::Ongoing(game)
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}
impl CastlingRights {
    #[must_use]
    #[allow(clippy::fn_params_excessive_bools)]
    pub const fn new(
        white_kingside: bool,
        white_queenside: bool,
        black_kingside: bool,
        black_queenside: bool,
    ) -> Self {
        Self {
            white_kingside,
            white_queenside,
            black_kingside,
            black_queenside,
        }
    }
    #[must_use]
    pub const fn all_available() -> Self {
        Self::new(true, true, true, true)
    }

    #[must_use]
    pub const fn none_available() -> Self {
        Self::new(false, false, false, false)
    }

    pub const ALL: [Self; 16] = [
        Self::new(O, O, O, O),
        Self::new(O, O, O, X),
        Self::new(O, O, X, O),
        Self::new(O, O, X, X),
        Self::new(O, X, O, O),
        Self::new(O, X, O, X),
        Self::new(O, X, X, O),
        Self::new(O, X, X, X),
        Self::new(X, O, O, O),
        Self::new(X, O, O, X),
        Self::new(X, O, X, O),
        Self::new(X, O, X, X),
        Self::new(X, X, O, O),
        Self::new(X, X, O, X),
        Self::new(X, X, X, O),
        Self::new(X, X, X, X),
    ];
}
const X: bool = true;
const O: bool = false;

impl Default for CastlingRights {
    fn default() -> Self {
        Self::all_available()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub board: Board,
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<Square>,
}

pub(crate) gen fn attacked_squares(
    board: &Board,
    origin: Square,
    active_player: PlayerKind,
) -> Threat {
    let Some(piece) = board[origin] else {
        return;
    };
    if piece.owner != active_player {
        return;
    }

    let (directions, range_upper_bound) = piece.threat_directions();
    let range_upper_bound = i32::from(range_upper_bound);

    let rays = directions.iter().map(move |&direction| {
        (1..=range_upper_bound)
            .map(move |range| origin + (direction * range)) // once this is Err(_) once, it'll _always_ be out of bounds
            .map_while(Result::ok)
    });

    for ray in rays {
        for destination in ray {
            match board[destination] {
                None => {
                    yield Threat {
                        piece,
                        origin,
                        destination,
                    }
                }
                Some(attacked_piece) if attacked_piece.owner == active_player => {
                    break;
                }
                Some(attacked_piece) if attacked_piece.owner != active_player => {
                    yield Threat {
                        piece,
                        origin,
                        destination,
                    };
                    break;
                }
                _ => unreachable!(),
            }
        }
    }
}
