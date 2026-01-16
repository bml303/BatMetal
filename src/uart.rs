//! UART0 Module for RP1
//! Target Baud: 115200

use crate::rp1::{UART0_BASE, UART_DR, UART_FR, FR_TXFF, memory_barrier};
use core::ptr::{read_volatile, write_volatile};


pub fn init() {
    // We assume Firmware has already set up:
    // - GPIO 14/15 ALT Function (UART0)
    // - Clocks
    // - Baud Rate (115200)
    // We just start writing!
}

pub fn putc(c: char) {
    unsafe {
        // Wait while TX FIFO is Full
        let fr_reg = (UART0_BASE + UART_FR) as *const u32;
        while (read_volatile(fr_reg) & FR_TXFF) != 0 {
            core::hint::spin_loop();
        }

        // Write Character to Data Register
        let dr_reg = (UART0_BASE + UART_DR) as *mut u32;
        write_volatile(dr_reg, c as u32);
        
        memory_barrier();
    }
}

pub fn puts(s: &str) {
    for c in s.chars() {
        putc(c);
        if c == '\n' {
            putc('\r');
        }
    }
}
