#![no_std]
#![no_main]

mod bootloader;

use core::panic::PanicInfo;


pub fn start_phase0()
{
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
