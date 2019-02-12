#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use methix::prelude::*;
use methix::vga::{Writer, ColorCode, Color};

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	Writer::set_color(ColorCode::new(Color::Yellow, Color::Black));

	println!("Hello{1} World{0}", "!", ",");

	loop {}
}
