#![no_std]
#![no_main]

mod video;

use core::fmt::Write;
use core::panic::PanicInfo;
use crate::video::{Cursor, VGABuffer};

static HELLO: &[u8] = b"---| BOOT |---";

#[no_mangle]
pub extern "C" fn _start() -> ! {

	let mut cursor = Cursor::default();

	for i in HELLO {
		let ch = *i as char;
		cursor.write_char(ch);
	}

	loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}