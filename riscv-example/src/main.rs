#![no_std]
#![no_main]

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[export_name = "main"]
pub fn main() -> ! {
    let a: u32 = 0x12345678;
    let b: u32 = 0x90ABCDEF;
    if a + b > 0x87654321 {
        loop {}
    }
    loop {}
}