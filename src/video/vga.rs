use crate::video::text::VGAChar;

const BUFFER_HEIGHT: u8 = 25;
const BUFFER_WIDTH: u8 = 80;

#[repr(transparent)]
pub struct VGABuffer {
	pub chars: [[VGAChar; BUFFER_WIDTH as usize]; BUFFER_HEIGHT as usize]
}