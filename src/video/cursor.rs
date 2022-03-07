use core::fmt::{Arguments, Write};
use crate::VGABuffer;
use crate::video::Colour;
use crate::video::colour::VGAColour;
use crate::video::text::VGAChar;

pub struct Cursor {
	colour: VGAColour,

	pos_x: usize,
	pos_y: usize,

	buffer: &'static mut VGABuffer
}

impl Default for Cursor {
	fn default() -> Self {
		Cursor {
			pos_x: 0,
			pos_y: 0,

			colour: VGAColour::new(Colour::Green, Colour::Black),

			buffer: unsafe { &mut *(0xb8000 as *mut VGABuffer) },
		}
	}
}

impl Write for Cursor {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		Ok(())
	}

	fn write_char(&mut self, c: char) -> core::fmt::Result {
		let char = c as u8;

		let (x,y) = (self.pos_x,self.pos_y);

		self.buffer.chars[y][x] = VGAChar::new(char,self.colour);

		self.pos_x += 1;
		if self.pos_x == 80 {
			self.pos_x = 0;
			self.pos_y += 1;
			if self.pos_y == 25 {
				self.pos_y = 0;
			}
		}

		Ok(())
	}
}