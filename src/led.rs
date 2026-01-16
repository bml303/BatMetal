//! LED Control Module
//! Manages ACT LED (BCM2712 onboard) and External LEDs (RP1)


use core::ptr::{write_volatile, read_volatile};
use crate::rp1::{RIO_BASE, RIO_OUT_SET, RIO_OUT_CLR, memory_barrier};
use crate::gpio::{self, func, PAD_OUTPUT_12MA};

const GIO_AON_DATA: *mut u32 = 0x107d517c04 as *mut u32;

/// GPIO AON Direction Register (0 = output, 1 = input)
const GIO_AON_DIR: *mut u32 = 0x107d517c08 as *mut u32;

const ACT_LED_BIT: u32 = 9;


pub fn init_act_led() {
    unsafe {
        let mut dir = read_volatile(GIO_AON_DIR);
        dir &= !(1 << ACT_LED_BIT);
        write_volatile(GIO_AON_DIR, dir);
        memory_barrier();
        
        let mut data = read_volatile(GIO_AON_DATA);
        data |= 1 << ACT_LED_BIT;
        write_volatile(GIO_AON_DATA, data);
        memory_barrier();
    }
}

// NOTE: Pi 5 ACT LED is ACTIVE-LOW:
// - Write 0 = LED ON
// - Write 1 = LED OFF
pub fn set_act_led(on: bool) {
    unsafe {
        let mut data = read_volatile(GIO_AON_DATA);
        if on {
            data &= !(1 << ACT_LED_BIT);
        } else {
            data |= 1 << ACT_LED_BIT;
        }
        write_volatile(GIO_AON_DATA, data);
        memory_barrier();
    }
}


/// Configures GPIO as Output with PROC_RIO function
pub fn init_external_led(pin: u32) {
    gpio::set_gpio_function(pin, func::PROC_RIO);
    
    gpio::configure_pad(pin, PAD_OUTPUT_12MA);
    
    gpio::set_as_output(pin);
    
    gpio::set_output_low(pin);
}

pub fn set_external_led(pin: u32, on: bool) {
    unsafe {
        let offset = if on { RIO_OUT_SET } else { RIO_OUT_CLR };
        write_volatile((RIO_BASE + offset) as *mut u32, 1 << pin);
        memory_barrier();
    }
}