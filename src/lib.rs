#![deny(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]
#![feature(
    adt_const_params,
    ascii_char,
    ascii_char_variants,
    const_trait_impl,
    const_ops,
    coroutines,
    fn_traits,
    gen_blocks,
    import_trait_associated_functions,
    result_option_map_or_default,
    stmt_expr_attributes,
    unboxed_closures
)]
#![forbid(unsafe_code)]
#![no_std]
extern crate alloc;

#[cfg(test)]
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
