#![feature(asm)]
#![feature(const_fn_union)]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

mod bcjr;
mod simd;
mod llr;
pub mod dword;
mod turbo;
pub mod interleavers;
pub mod trellises;

pub use self::{
    bcjr::BcjrDecoder,
    llr::Llr,
    turbo::TurboDecoder,
};
