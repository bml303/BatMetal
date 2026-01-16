//! RP1 Initialization Module (PCIe I/O Controller on Pi 5)
//! 
//! Contains base addresses and low-level functions to access RP1 via PCIe BAR0.

use core::arch::asm;

/// RP1 Base Address mapped via PCIe BAR0
/// Note: 0x1c00000000 is the correct address after firmware PCIe mapping
pub const RP1_BASE: usize = 0x1c00_0000_00;

/// IO Bank 0 Controller Base (GPIO config)
pub const IO_BANK0_BASE: usize = RP1_BASE + 0x000d_0000;

/// RIO Controller Base (Register I/O for GPIO)
pub const RIO_BASE: usize = RP1_BASE + 0x000e_0000;

/// PADs Bank 0 Base (Electrical configuration)
pub const PADS_BANK0_BASE: usize = RP1_BASE + 0x000f_0000;

// ============================================================
// RIO OFFSETS (Datasheet RP1 Section 5.3.2)
// ============================================================
pub const RIO_OUT: usize = 0x00;
pub const RIO_OE: usize = 0x04;
pub const RIO_OUT_SET: usize = 0x2000 + RIO_OUT;
pub const RIO_OUT_CLR: usize = 0x3000 + RIO_OUT;
pub const RIO_OE_SET: usize = 0x2000 + RIO_OE;

// ============================================================
// UART0 OFFSETS (PL011)
// ============================================================
pub const UART0_OFFSET: usize = 0x30000;
pub const UART0_BASE: usize = RP1_BASE + UART0_OFFSET;

pub const UART_DR: usize = 0x00;
pub const UART_FR: usize = 0x18;

pub const FR_TXFF: u32 = 1 << 5; // Transmit FIFO Full


/// Memory barrier to synchronize PCIe accesses
/// Must be called after every volatile write to RP1 to ensure
/// completion before proceeding.
#[inline(always)]
pub fn memory_barrier() {
    unsafe {
        asm!("dsb sy; isb", options(nostack, preserves_flags));
    }
}
