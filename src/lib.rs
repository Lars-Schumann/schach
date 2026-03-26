#![deny(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]
#![feature(
    const_trait_impl,
    const_ops,
    const_convert,
    const_try,
    const_index,
    const_cmp,
    const_default,
    const_clone,
    derive_const,
    ascii_char,
    ascii_char_variants,
    result_option_map_or_default,
    stmt_expr_attributes,
    coroutines,
    gen_blocks,
    import_trait_associated_functions
)]
#![forbid(unsafe_code)]
#![no_std]
extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

pub mod board;
pub mod coord;
pub mod game;
pub mod move_gen;
pub mod mv;
pub mod notation;
pub mod piece;
pub mod player;

#[cfg(test)]
mod testing;
