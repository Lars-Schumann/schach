use core::ops::ControlFlow;

use crate::common::not;
use crate::game::CastlingRight;
use crate::game::CastlingSide;
use crate::game::DrawKind;
use crate::game::FiftyMoveRuleClock;
use crate::game::Game;
use crate::game::GameResult;
use crate::game::GameResultKind;
use crate::game::Phase::Ongoing;
use crate::game::PieceCounts;
use crate::game::Position;
use crate::mv::InnerMove;
use crate::mv::MoveKind;
use crate::player::PlayerKind;

pub(crate) const REPETITIONS_TO_FORCED_DRAW_COUNT: usize = 5;
pub(crate) const FIFTY_MOVE_RULE_COUNT: FiftyMoveRuleClock = FiftyMoveRuleClock(100);

pub(super) fn set_active_player_castling_rights(mv: InnerMove, game: &mut Game<{ Ongoing }>) {
    match mv.kind {
        MoveKind::King(_) => {
            for castling_side in CastlingSide::ALL {
                game.core.castling_rights[(game.core.active_player, castling_side)] =
                    CastlingRight::Unavailable;
            }
        }
        MoveKind::Rook { .. } => {
            for castling_side in CastlingSide::ALL {
                if mv.origin == game.core.active_player.rook_start(castling_side) {
                    game.core.castling_rights[(game.core.active_player, castling_side)] =
                        CastlingRight::Unavailable;
                }
            }
        }
        _ => { /*nothing */ }
    }
}

pub(super) fn set_opponents_castling_rights(mv: InnerMove, game: &mut Game<{ Ongoing }>) {
    if mv.is_capture() && not(mv.kind.is_pawn_en_passant()) {
        for castling_side in CastlingSide::ALL {
            if mv.destination == game.core.active_player.opponent().rook_start(castling_side) {
                game.core.castling_rights[(game.core.active_player.opponent(), castling_side)] =
                    CastlingRight::Unavailable;
            }
        }
    }
}

pub(super) fn set_en_passant_target(mv: InnerMove, game: &mut Game<{ Ongoing }>) {
    game.core.en_passant_target = if mv.kind.is_pawn_double_step() {
        let possible_en_passant_target = (mv.destination
            + game.core.active_player.backwards_one_row())
        .expect("this to always be on the board");

        let mut future = game.core.with_opponent_active();
        future.en_passant_target = Some(possible_en_passant_target);

        future
            .legal_inner_moves()
            .any(|mv| mv.kind.is_pawn_en_passant())
            .then_some(possible_en_passant_target)
    } else {
        None
    };
}

pub(super) fn set_position_history(game: &mut Game<{ Ongoing }>) {
    let current_position = Position {
        board: game.core.board,
        castling_rights: game.core.castling_rights,
        en_passant_target: game.core.en_passant_target,
    };

    game.position_history.push(current_position);
}

pub(super) fn set_fifty_move_rule_clock(mv: InnerMove, game: &mut Game<{ Ongoing }>) {
    if mv.is_pawn_or_capture() {
        game.core.fifty_move_rule_clock.reset();
    } else {
        game.core.fifty_move_rule_clock.increase();
    }
}

pub(super) fn check_stalemate_or_checkmate(game: &Game<{ Ongoing }>) -> ControlFlow<GameResult> {
    let future = game.core.with_opponent_active();

    if future.legal_inner_moves().count() > 0 {
        return ControlFlow::Continue(());
    }

    if future.board.is_king_checked(future.active_player) {
        ControlFlow::Break(GameResult {
            kind: GameResultKind::Win,
            final_game_state: game.clone().terminated(),
        })
    } else {
        ControlFlow::Break(GameResult {
            kind: GameResultKind::Draw(DrawKind::Stalemate),
            final_game_state: game.clone().terminated(),
        })
    }
}

pub(super) fn check_threefold_repetition_draw(game: &Game<{ Ongoing }>) -> ControlFlow<GameResult> {
    let current_position = Position {
        board: game.core.board,
        castling_rights: game.core.castling_rights,
        en_passant_target: game.core.en_passant_target,
    };

    if game
        .position_history
        .iter()
        .filter(|&position| *position == current_position)
        .count()
        == REPETITIONS_TO_FORCED_DRAW_COUNT
    {
        ControlFlow::Break(GameResult {
            kind: GameResultKind::Draw(DrawKind::ThreefoldRepetition),
            final_game_state: game.clone().terminated(),
        })
    } else {
        ControlFlow::Continue(())
    }
}

pub(super) fn check_fifty_move_draw(game: &Game<{ Ongoing }>) -> ControlFlow<GameResult> {
    if game.core.fifty_move_rule_clock == FIFTY_MOVE_RULE_COUNT {
        ControlFlow::Break(GameResult {
            kind: GameResultKind::Draw(DrawKind::FiftyMove),
            final_game_state: game.clone().terminated(),
        })
    } else {
        ControlFlow::Continue(())
    }
}

pub(super) fn check_insufficient_material_draw(
    game: &Game<{ Ongoing }>,
) -> ControlFlow<GameResult> {
    let piece_counts = game.core.board.piece_counts();

    if piece_counts == PieceCounts::KINGS_ONLY {
        ControlFlow::Break(GameResult {
            kind: GameResultKind::Draw(DrawKind::InsufficientMaterial),
            final_game_state: game.clone().terminated(),
        })
    } else {
        ControlFlow::Continue(())
    }
}

pub(super) fn set_full_move_count(game: &mut Game<{ Ongoing }>) {
    if game.core.active_player == PlayerKind::Black {
        game.core.full_move_count.increase();
    }
}
