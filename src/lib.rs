#![feature(asm)]
#![feature(const_fn_union)]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

mod bcjr;
pub mod dword;
pub mod interleavers;
mod llr;
pub mod trellises;

#[cfg(target_tests)]
pub mod simd;
#[cfg(not(target_tests))]
mod simd;

#[cfg(target_tests)]
pub mod turbo;
#[cfg(not(target_tests))]
mod turbo;

pub use self::{bcjr::BcjrDecoder, llr::Llr, turbo::TurboDecoder};
