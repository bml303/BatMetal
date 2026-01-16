//! GPIO Configuration Module for RP1

use core::ptr::write_volatile;
use crate::rp1::{
    IO_BANK0_BASE, PADS_BANK0_BASE, RIO_BASE,
    RIO_OE_SET, RIO_OUT_CLR, memory_barrier,
};


pub mod func {
    /// Processor Control (PROC_RIO)
    pub const PROC_RIO: u32 = 0x05;
}


pub fn set_gpio_function(pin: u32, function: u32) {
    unsafe {
        let ctrl_reg = (IO_BANK0_BASE + 0x04 + (8 * pin as usize)) as *mut u32;
        write_volatile(ctrl_reg, function);
        memory_barrier();
    }
}


/// IE=1, Drive=12mA, OD=0
pub const PAD_OUTPUT_12MA: u32 = 0x70;

pub fn configure_pad(pin: u32, config: u32) {
    unsafe {
        let pad_reg = (PADS_BANK0_BASE + 0x04 + (4 * pin as usize)) as *mut u32;
        write_volatile(pad_reg, config);
        memory_barrier();
    }
}

pub fn set_as_output(pin: u32) {
    unsafe {
        write_volatile((RIO_BASE + RIO_OE_SET) as *mut u32, 1 << pin);
        memory_barrier();
    }
}

/// Set GPIO Low (Off)
pub fn set_output_low(pin: u32) {
    unsafe {
        write_volatile((RIO_BASE + RIO_OUT_CLR) as *mut u32, 1 << pin);
        memory_barrier();
    }
}
