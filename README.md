# st7305
A `no_std` Rust driver for the ST7305 monochrome reflective LCD controller.

This crate provides an interface to ST7305-based displays, built on top of `embedded-hal` 1.0.0. It utilizes a 15KB local framebuffer for tear-free screen updates over SPI.

## Features
 - `no_std`
 - `embedded_hal`
 - async support through `async` feature
 - `embedded-graphics` support through `graphics` feature

## Examples

### Quickstart
```rs
use st7305::{St7305, Orientation, BinaryColor};
use embedded_hal::delay::DelayNs;

// Assuming you have your SPI and Reset pins initialised from your HAL
// let spi = ...;
// let rst = ...;
// let mut delay = ...;

let mut display = St7305::new(spi, rst);

display.init(&mut delay).unwrao();

display.set_orientation(Orientation::Landscape);

display.color_clear(BinaryColor::On);

display.flush().unwrap();
```

### Drawing with `embedded-graphics`:
Enable the `graphics` feature in your `Cargo.toml`.
```rs
use embedded_graphics::{
	pixelcolor::BinaryColor,
	primitives::{Circle, PrimitiveStyle},
	prelude::*
};

let mut display = St7305::new(spi, rst);
display.init(&mut delay).unwrao();
display.set_orientation(Orientation::Landscape);
display.color_clear(BinaryColor::Off);

// draw a circle to the internal buffer
Circle::new(Point::new(150, 100), 100)
	.into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 2))
	.draw(&mut display)
	.unwrap();

// send the updated buffer to the screen
display.flush().unwrap();
```
