#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
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
pub struct VGAColour(u8);

impl VGAColour {

	#[inline]
	pub fn new(fore: Colour, back: Colour) -> Self {
		VGAColour((back as u8) << 4 | (fore as u8))
	}
}

impl Default for VGAColour {
	fn default() -> Self {
		VGAColour {
			0: 0x0F as u8
		}
	}
}