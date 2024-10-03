# README

# Hardware

Waveshare-rp2040-lcd-0.96

RPi Debug https://www.raspberrypi.com/products/debug-probe/

# Pre-Requisites

PS> winget install Rustlang.Rustup
PS> C:\test\Lukas\hackergarten\pico
PS> git clone https://github.com/rp-rs/rp-hal-boards
PS> cd PS C:\test\Lukas\hackergarten\pico\rp-hal-boards\boards\waveshare-rp2040-lcd-0-96>

PS> irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex

PS> rustup target add thumbv6m-none-eabi
PS> cargo install elf2uf2-rs

PS> cargo run --release --example waveshare_rp2040_lcd_demo

PS> git clone git@github.com:maebli/waveshare-rp2040-lcd-0-96-template.git

PS> cd waveshare-rp2040-lcd-0-96-template

PS> cargo run --release
