#![feature(asm)]
#![feature(const_fn_union)]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

mod bcjr;
pub mod dword;
pub mod interleavers;
mod llr;
mod simd;
pub mod trellises;
mod turbo;

pub use self::{bcjr::BcjrDecoder, llr::Llr, turbo::TurboDecoder};
