//! Servo Motor Driver (Software PWM)
//! Controls a standard 9g Servo on GPIO 18.
//! Frequency: 50Hz (20ms Period)

use crate::gpio::{self, func, PAD_OUTPUT_12MA};

const LOOPS_PER_MICROSECOND: usize = 3;

pub fn init(pin: u32) {
    gpio::set_gpio_function(pin, func::PROC_RIO);
    gpio::configure_pad(pin, PAD_OUTPUT_12MA);
    gpio::set_as_output(pin);
    gpio::set_output_low(pin);
}

fn delay_us(us: usize) {
    for i in 0..(us * LOOPS_PER_MICROSECOND) {
        core::hint::black_box(i);
    }
}

pub fn pulse(pin: u32, high_time_us: usize) {
    // Period is 20ms (20,000us)
    gpio::set_output_high(pin);
    delay_us(high_time_us);
    gpio::set_output_low(pin);
    if high_time_us < 20_000 {
        delay_us(20_000 - high_time_us);
    }
}

/// Move Servo to Angle (0-180 degrees)
pub fn write_angle(pin: u32, angle: usize) {
    let clamp_angle = if angle > 180 { 180 } else { angle };
    let high_time = 1000 + ((clamp_angle * 1000) / 180);
    pulse(pin, high_time);
}
