use core::panic::PanicInfo;
use crate::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
	println!("Kernel Panic: {}", panic_info);
	loop {}
}
