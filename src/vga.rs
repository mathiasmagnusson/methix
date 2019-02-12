use core::{fmt, ptr};
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
	pub fn new(foreground: Color, background: Color) -> ColorCode {
		ColorCode((background as u8) << 4 | (foreground as u8))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
	ascii_char: u8,
	color_code: ColorCode,
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[repr(transparent)]
struct Buffer {
	chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

lazy_static::lazy_static! {
	static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

pub struct Writer {
	current_column: usize,
	current_row: usize,
	color_code: ColorCode,
	buffer: &'static mut Buffer,
}

impl Writer {
	fn new() -> Self {
		Self {
			current_column: 0,
			current_row: 0,
			color_code: ColorCode::new(Color::White, Color::Black),
			buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
		}
	}

	pub fn set_color(color_code: ColorCode) {
		let mut writer = WRITER.lock();
		writer.color_code = color_code;
	}

	fn new_line(&mut self) {
		self.current_column = 0;
		if self.current_row < BUFFER_HEIGHT - 1 {
			self.current_row += 1;
		} else {
			for i in 1..BUFFER_HEIGHT {
				unsafe {
					ptr::write_volatile(&mut self.buffer.chars[i - 1], self.buffer.chars[i]);
				}
			}
			unsafe {
				ptr::write_volatile(
					&mut self.buffer.chars[BUFFER_HEIGHT - 1],
					[
						ScreenChar { ascii_char: b' ', color_code: self.color_code };
						BUFFER_WIDTH
					]
				);
			}
		}
	}

	fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			8 => {
				if self.current_column > 0 {
					self.current_column -= 1;
					let col = self.current_column;
					let row = self.current_row;

					unsafe {
						ptr::write_volatile(
							&mut self.buffer.chars[row][col],
							ScreenChar {
								ascii_char: b' ',
								color_code: self.color_code,
							}
						);
					}
				} else {
					if self.current_row > 0 {
						self.current_row -= 1;
						self.current_column = BUFFER_WIDTH - 1;

						let col = self.current_column;
						let row = self.current_row;

						unsafe {
							ptr::write_volatile(
								&mut self.buffer.chars[row][col],
								ScreenChar {
									ascii_char: b' ',
									color_code: self.color_code,
								},
							);
						}
					}
				}
			}
			byte => {
				let col = self.current_column;
				let row = self.current_row;

				unsafe {
					ptr::write_volatile(
						&mut self.buffer.chars[row][col],
						ScreenChar {
							ascii_char: byte,
							color_code: self.color_code,
						},
					);
				}

				self.current_column += 1;

				if self.current_column == BUFFER_WIDTH {
					self.new_line();
				}
			}
		}
	}

	fn write(&mut self, s: &str) {
		for byte in s.bytes() {
			match byte {
				0x20...0x7e | b'\n' => self.write_byte(byte),
				_ => self.write_byte(0xfe),
			}
		}
	}
}

impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write(s);
		Ok(())
	}
}

#[macro_export]
macro_rules! println {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;
	let mut writer = WRITER.lock();
	
	writer.write_fmt(args).unwrap();
}
