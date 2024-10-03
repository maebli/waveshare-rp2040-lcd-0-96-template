//! Example of graphics on the LCD of the Waveshare RP2040-LCD-0.96
//!
//! Draws a red and green line with a blue rectangle.
//! After that it fills the screen line for line, at the end it starts over with
//! another colour, RED, GREEN and BLUE.
#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use embedded_graphics::primitives::Line;
use panic_halt as _;
use rp2040_hal::fugit::RateExtU32;

use waveshare_rp2040_lcd_0_96::entry;
use waveshare_rp2040_lcd_0_96::{
    hal::{
        self,
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pio::PIOExt,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::Text,
};
use st7735_lcd::{Orientation, ST7735};

const LCD_WIDTH: u32 = 160;
const LCD_HEIGHT: u32 = 80;

#[entry]
fn main() -> ! {
    loop {}
}
