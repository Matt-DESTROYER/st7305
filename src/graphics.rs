use display_interface::WriteOnlyDataCommand;
use embedded_hal::digital::OutputPin;
use embedded_graphics_core::{
	draw_target::DrawTarget,
	geometry::{
		OriginDimensions,
		Size
	},
	pixelcolor::BinaryColor,
	Pixel
};

use crate::{
	Orientation,
	St7305
};

impl<DI, RST, PinError> OriginDimensions for St7305<DI, RST>
where
	DI: WriteOnlyDataCommand,
	RST: OutputPin<Error = PinError>
{
	fn size(&self) -> Size {
		match self.orientation {
			Orientation::Landscape => Size::new(400, 300),
			Orientation::Portrait => Size::new(300, 400)
		}
	}
}

impl<DI, RST, PinError> DrawTarget for St7305<DI, RST>
where
	DI: WriteOnlyDataCommand,
	RST: OutputPin<Error = PinError>
{
	type Color = BinaryColor;
	type Error = core::convert::Infallible;

	fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
	where
		I: IntoIterator<Item = Pixel<Self::Color>>
	{
		for Pixel(point, color) in pixels {
			let x = point.x as u16;
			let y = point.y as u16;

			match self.orientation {
				// adapted from a Waveshare demo
				Orientation::Landscape => {
					if x >= 400 || y >= 300 {
						continue;
					}

					let inv_y = 300 - 1 - y;
					let byte_x = x / 2;
					let block_y = inv_y / 4;
					let index = (byte_x * (300 / 4) + block_y) as usize;
					let local_x = x % 2;
					let local_y = inv_y % 4;
					let bit = 7 - (local_y * 2 + local_x);

					if color.is_on() {
						self.buffer[index] |= 1 << bit;
					} else {
						self.buffer[index] &= !(1 << bit);
					}
				},
				Orientation::Portrait => {
					if x >= 300 || y >= 400 {
						continue;
					}

					let byte_y = y / 2;
					let byte_x = x / 4;
					let index = (byte_y * (300 / 4) + byte_x) as usize;
					let local_x = x % 4;
					let local_y = y % 2;
					let bit = 7 - (local_x * 2 + local_y);

					if color.is_on() {
						self.buffer[index] |= 1 << bit;
					} else {
						self.buffer[index] &= !(1 << bit);
					}
				}
			}
		}

		Ok(())
	}
}
