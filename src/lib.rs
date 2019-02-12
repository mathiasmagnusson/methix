#![cfg_attr(not(test), no_std)]

pub mod panic;
pub mod serial;
pub mod vga;

pub mod prelude {
	pub use crate::{println, print};
}
