#![feature(asm)]
#![feature(const_fn_union)]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

mod simd;
mod llr;
mod dword;
mod turbo;
mod symbol;
pub mod interleavers;

pub use llr::Llr;
