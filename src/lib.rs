#![feature(asm)]
#![feature(const_fn_union)]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

mod bcjr;
mod simd;
mod llr;
mod dword;
mod turbo;
mod umts;
pub mod interleavers;

pub use self::{
    bcjr::BcjrDecoder,
    llr::Llr,
    turbo::TurboDecoder,
};
