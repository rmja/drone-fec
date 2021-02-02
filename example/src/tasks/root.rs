//! The root task.

use crate::{thr, thr::ThrsInit, Regs};
use drone_cortexm::{reg::prelude::*, thr::prelude::*};

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    println!("Hello, world!");
    
    drone_fec::simd::tests::saturate_bits_i32_impl();
    drone_fec::simd::tests::saturate_bits_u32_impl();
    drone_fec::simd::tests::saturating_add_impl();
    drone_fec::simd::tests::saturating_sub_impl();
    drone_fec::simd::tests::half_add_impl();
    drone_fec::simd::tests::half_sub_impl();
    drone_fec::simd::tests::max_impl();
    drone_fec::simd::tests::min_impl();

    drone_fec::turbo::tests::decode_excel_example_impl();

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}
