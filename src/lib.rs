#![no_std]

pub mod instruction;
use crate::instruction::Instruction;

use display_interface::{
	DataFormat::U8,
	WriteOnlyDataCommand
};
use embedded_hal::{
	delay::DelayNs,
	digital::OutputPin
};
#[cfg(feature = "async")]
use embedded_hal_async::delay::DelayNs as AsyncDelayNs;

#[cfg(feature = "graphics")]
mod graphics;

#[derive(Debug)]
pub enum Error<PinError> {
	DisplayError,
	Pin(PinError)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BinaryColor {
	On = 0xFF,
	Off = 0x00
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Orientation {
	Portrait,
	Landscape
}

pub struct St7305<DI, RST> {
	di: DI,
	rst: RST,
	buffer: [u8; 15000],
	orientation: Orientation
}

impl<DI, RST, PinError> St7305<DI, RST>
where
	DI: WriteOnlyDataCommand,
	RST: OutputPin<Error = PinError>
{
	pub fn new(interface: DI, reset: RST) -> Self {
		Self {
			di: interface,
			rst: reset,
			buffer: [0; 15000],
			orientation: Orientation::Landscape
		}
	}

	pub fn init(&mut self, delay_source: &mut impl DelayNs) -> Result<(), Error<PinError>> {
		self.hard_reset(delay_source)?;

		self.write_command(Instruction::NVMLOADCTRL)?;
		self.write_data(&[0x17, 0x02])?;

		self.write_command(Instruction::BSTEN)?;
		self.write_data(&[0x01])?;

		self.write_command(Instruction::GCTRL)?;
		self.write_data(&[0x11, 0x04])?;

		self.write_command(Instruction::VSHPCTRL)?;
		self.write_data(&[0x69, 0x69, 0x69, 0x69])?;

		self.write_command(Instruction::VSLPCTRL)?;
		self.write_data(&[0x19, 0x19, 0x19, 0x19])?;

		self.write_command(Instruction::VSHNCTRL)?;
		self.write_data(&[0x4B, 0x4B, 0x4B, 0x4B])?;
		
		self.write_command(Instruction::VSLNCTRL)?;
		self.write_data(&[0x19, 0x19, 0x19, 0x19])?;

		self.write_command(Instruction::OSCSET)?;
		self.write_data(&[0x80, 0xE9])?;

		self.write_command(Instruction::FRCTRL)?;
		self.write_data(&[0x02])?;

		self.write_command(Instruction::GTUPEQH)?;
		self.write_data(&[0xE5, 0xF6, 0x05, 0x46, 0x77, 0x77, 0x77, 0x77, 0x76, 0x45])?;

		self.write_command(Instruction::GTUPEQL)?;
		self.write_data(&[0x05, 0x46, 0x77, 0x77, 0x77, 0x77, 0x76, 0x45])?;

		self.write_command(Instruction::SOUEQ)?;
		self.write_data(&[0x13])?;

		self.write_command(Instruction::GATESET)?;
		self.write_data(&[0x64])?;

		self.write_command(Instruction::SLPOUT)?;
		delay_source.delay_ms(200);
		self.write_command(Instruction::VSHLSEL)?;
		self.write_data(&[0x00])?;

		self.write_command(Instruction::MADCTL)?;
		self.write_data(&[0x48])?;

		self.write_command(Instruction::DTFORM)?;
		self.write_data(&[0x11])?;

		self.write_command(Instruction::GAMAMS)?;
		self.write_data(&[0x20])?;

		self.write_command(Instruction::PNLSET)?;
		self.write_data(&[0x29])?;

		self.write_command(Instruction::INVON)?;
		
		self.write_command(Instruction::CASET)?;
		self.write_data(&[0x12, 0x2A])?;

		self.write_command(Instruction::RASET)?;
		self.write_data(&[0x00, 0xC7])?;

		self.write_command(Instruction::TEON)?;
		self.write_data(&[0x00])?;

		self.write_command(Instruction::AUTOPWRCTRL)?;
		self.write_data(&[0xFF])?;

		self.write_command(Instruction::HPM)?;
		self.write_command(Instruction::DISPON)?;

		Ok(())
	}

	#[cfg(feature = "async")]
	pub async fn init_async(&mut self, delay_source: &mut impl AsyncDelayNs) -> Result<(), Error<PinError>> {
		self.hard_reset_async(delay_source).await?;

		self.write_command(Instruction::NVMLOADCTRL)?;
		self.write_data(&[0x17, 0x02])?;

		self.write_command(Instruction::BSTEN)?;
		self.write_data(&[0x01])?;

		self.write_command(Instruction::GCTRL)?;
		self.write_data(&[0x11, 0x04])?;

		self.write_command(Instruction::VSHPCTRL)?;
		self.write_data(&[0x69, 0x69, 0x69, 0x69])?;

		self.write_command(Instruction::VSLPCTRL)?;
		self.write_data(&[0x19, 0x19, 0x19, 0x19])?;

		self.write_command(Instruction::VSHNCTRL)?;
		self.write_data(&[0x4B, 0x4B, 0x4B, 0x4B])?;
		
		self.write_command(Instruction::VSLNCTRL)?;
		self.write_data(&[0x19, 0x19, 0x19, 0x19])?;

		self.write_command(Instruction::OSCSET)?;
		self.write_data(&[0x80, 0xE9])?;

		self.write_command(Instruction::FRCTRL)?;
		self.write_data(&[0x02])?;

		self.write_command(Instruction::GTUPEQH)?;
		self.write_data(&[0xE5, 0xF6, 0x05, 0x46, 0x77, 0x77, 0x77, 0x77, 0x76, 0x45])?;

		self.write_command(Instruction::GTUPEQL)?;
		self.write_data(&[0x05, 0x46, 0x77, 0x77, 0x77, 0x77, 0x76, 0x45])?;

		self.write_command(Instruction::SOUEQ)?;
		self.write_data(&[0x13])?;

		self.write_command(Instruction::GATESET)?;
		self.write_data(&[0x64])?;

		self.write_command(Instruction::SLPOUT)?;
		delay_source.delay_ms(200).await;
		self.write_command(Instruction::VSHLSEL)?;
		self.write_data(&[0x00])?;

		self.write_command(Instruction::MADCTL)?;
		self.write_data(&[0x48])?;

		self.write_command(Instruction::DTFORM)?;
		self.write_data(&[0x11])?;

		self.write_command(Instruction::GAMAMS)?;
		self.write_data(&[0x20])?;

		self.write_command(Instruction::PNLSET)?;
		self.write_data(&[0x29])?;

		self.write_command(Instruction::INVON)?;
		
		self.write_command(Instruction::CASET)?;
		self.write_data(&[0x12, 0x2A])?;

		self.write_command(Instruction::RASET)?;
		self.write_data(&[0x00, 0xC7])?;

		self.write_command(Instruction::TEON)?;
		self.write_data(&[0x00])?;

		self.write_command(Instruction::AUTOPWRCTRL)?;
		self.write_data(&[0xFF])?;

		self.write_command(Instruction::HPM)?;
		self.write_command(Instruction::DISPON)?;

		Ok(())
	}

	pub fn hard_reset(&mut self, delay_source: &mut impl DelayNs) -> Result<(), Error<PinError>> {
		self.rst.set_high()
			.map_err(Error::Pin)?;
		delay_source.delay_ms(50);

		self.rst.set_low()
			.map_err(Error::Pin)?;
		delay_source.delay_ms(20);

		self.rst.set_high()
			.map_err(Error::Pin)?;
		delay_source.delay_ms(50);

		Ok(())
	}

	pub fn color_clear(&mut self, color: u8) {
		self.buffer.fill(color);
	}

	pub fn flush(&mut self) -> Result<(), Error<PinError>> {
		self.write_command(Instruction::CASET)?;
		self.write_data(&[0x12, 0x2A])?;

		self.write_command(Instruction::RASET)?;
		self.write_data(&[0x00, 0xC7])?;

		self.write_command(Instruction::RAMWR)?;
		
		self.di
			.send_data(U8(&self.buffer))
			.map_err(|_| Error::DisplayError)?;

		Ok(())
	}

	#[cfg(feature = "async")]
	pub async fn hard_reset_async(&mut self, delay_source: &mut impl AsyncDelayNs) -> Result<(), Error<PinError>> {
		self.rst.set_high()
			.map_err(Error::Pin)?;
		delay_source.delay_ms(50).await;

		self.rst.set_low()
			.map_err(Error::Pin)?;
		delay_source.delay_ms(20).await;

		self.rst.set_high()
			.map_err(Error::Pin)?;
		delay_source.delay_ms(50).await;

		Ok(())
	}

	pub fn release(self) -> (DI, RST) {
		(self.di, self.rst)
	}

	pub fn set_display_on(&mut self, on:bool) -> Result<(), Error<PinError>> {
		if on {
			self.write_command(Instruction::DISPON)
		} else {
			self.write_command(Instruction::DISPOFF)
		}
	}

	pub fn set_invert(&mut self, invert: bool) -> Result<(), Error<PinError>> {
		if invert {
			self.write_command(Instruction::INVON)
		} else {
			self.write_command(Instruction::INVOFF)
		}
	}

	pub fn orientation(&self) -> Orientation {
		self.orientation
	}

	pub fn set_orientation(&mut self, orientation: Orientation) {
		self.orientation = orientation;
	}

	fn write_command(&mut self, command: Instruction) -> Result<(), Error<PinError>> {
		self.di
			.send_commands(U8(&[command as u8]))
			.map_err(|_| Error::DisplayError)?;

		Ok(())
	}

	fn write_data(&mut self, data: &[u8]) -> Result<(), Error<PinError>> {
		self.di
			.send_data(U8(data))
			.map_err(|_| Error::DisplayError)
	}
}
