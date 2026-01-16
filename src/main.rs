#![no_std]
#![no_main]

core::arch::global_asm!(include_str!("../boot.S"));

mod rp1;
mod gpio;
mod led;
mod uart;

const ERROR_LED_GPIO : u32 = 5;
const ALIVE_LED_GPIO : u32 = 6;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    led::init_act_led();
    
    for _ in 0..5 {
        // booting 
        led::set_act_led(true);
        delay(1_500_000);
        led::set_act_led(false);
        delay(1_500_000);
    }

    uart::init();
    
    uart::puts("\n");
    uart::puts("========================================\n");
    uart::puts("   RASPBERRY PI 5 - BARE METAL KERNEL   \n");
    uart::puts("========================================\n");

    uart::puts("[OK] hardware UART Initialized\n");

    led::init_external_led(ALIVE_LED_GPIO);
    led::set_external_led(ALIVE_LED_GPIO, true);
    uart::puts("[OK] LED GPIO 6 Ready\n");
    
    led::init_external_led(ERROR_LED_GPIO);
    led::set_external_led(ERROR_LED_GPIO, false);
    uart::puts("[OK] LED GPIO 5 Ready (OFF)\n");
    
    uart::puts("\n[INFO] System Running...\n");

    let mut counter = 0;
    loop {
        led::set_external_led(ALIVE_LED_GPIO, true);
        delay(1_000_000);
        
        led::set_external_led(ALIVE_LED_GPIO, false);
        delay(1_000_000);
        
        counter += 1;
        if counter % 10 == 0 {
             uart::puts("[ALIVE] System is running.\n");
        }
    }
}

fn delay(n: usize) {
    for i in 0..n {
        core::hint::black_box(i);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    uart::puts("\n\n!!! PANIC !!!\n");

    loop {
        led::set_act_led(true);
        delay(1_000_000);
        led::set_act_led(false);
        delay(1_000_000);
    }
}