use crate::video::Colour;
use crate::video::colour::VGAColour;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VGAChar {
	char: u8,
	colour: VGAColour,
}

impl VGAChar {

	#[inline]
	pub fn new(
		char: u8,
		colour: VGAColour
	) -> Self {
		VGAChar {
			char,
			colour,
		}
	}
}