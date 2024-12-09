#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;
static HELLO: &[u8] = b"Hello BootLoader!!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 43, 1.1337).unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

