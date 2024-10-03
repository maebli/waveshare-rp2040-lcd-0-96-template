//! Example of graphics on the LCD of the Waveshare RP2040-LCD-0.96
//!
//! Draws a red and green line with a blue rectangle.
//! After that it fills the screen line for line, at the end it starts over with
//! another colour, RED, GREEN and BLUE.
#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use embedded_graphics::primitives::Line;
use fugit::RateExtU32;
use panic_halt as _;
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
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set up the delay for the first core.
    let sys_freq = clocks.system_clock.freq().to_Hz();
    let mut delay = Delay::new(core.SYST, sys_freq);

    let (mut _pio, _sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    // https://www.waveshare.com/wiki/RP2040-LCD-0.96
    // ST7735S LCD
    let lcd_dc = pins.gp8.into_push_pull_output();
    let lcd_clk = pins.gp10.into_function::<hal::gpio::FunctionSpi>();
    let lcd_mosi = pins.gp11.into_function::<hal::gpio::FunctionSpi>();
    let lcd_rst = pins
        .gp12
        .into_push_pull_output_in_state(hal::gpio::PinState::High);
    let mut _lcd_bl = pins
        .gp25
        .into_push_pull_output_in_state(hal::gpio::PinState::High);

    let spi = hal::Spi::<_, _, _, 8>::new(pac.SPI1, (lcd_mosi, lcd_clk));

    // Exchange the uninitialised SPI driver for an initialised one
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        10.MHz(),
        embedded_hal::spi::MODE_0,
    );

    // LCD is a 65K IPS LCD 160x80, color order is BGR and a offset 1,26 pixel.
    // LCD controller can correct this by settings the order bit (bit 3) in MADCTL register.
    // Also the colours are inverted, LCD controller can also correct this by writing to INVON register with no paramters.
    // All this is handled by the ST7735 crate.
    let mut display = ST7735::new(spi, lcd_dc, lcd_rst, false, true, LCD_WIDTH, LCD_HEIGHT);

    display.init(&mut delay).unwrap();
    display.set_orientation(&Orientation::Landscape).unwrap();

    display.set_offset(1, 26);

    let lcd_zero = Point::zero();
    let lcd_max_corner = Point::new((LCD_WIDTH - 1) as i32, (LCD_HEIGHT - 1) as i32);

    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLUE)
        .build();

    Rectangle::with_corners(lcd_zero, lcd_max_corner)
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();

    
    

    // Infinite colour wheel loop
    let mut l: i32 = 0;
    let mut c = Rgb565::RED;

    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
    loop {
        Text::new("Happy Birthday!", Point::new(2, 21), style)
            .draw(&mut display)
            .unwrap();
            Text::new("HACKERGARTEN", Point::new(2, 41), style)
            .draw(&mut display)
            .unwrap();
    }
}
