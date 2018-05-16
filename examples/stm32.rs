//! An example of how to use light-cli on an STM32F103 chip.
//! 
//! To run call `cargo run --example stm32 --target thumbv7m-none-eabi --release`.
//! 
//! A typical command line communication for the example could look like:
//! ```
//! >> EHLO
//! << EHLO Name=
//! >> HELLO Name=Johnson
//! << Name set
//! >> EHLO
//! << EHLO Name=Johnson
//! ```
//! 

#![no_std]

extern crate cortex_m;
extern crate panic_abort;
extern crate embedded_hal as hal;
extern crate stm32f103xx_hal as dev_hal;
extern crate heapless;

#[macro_use]
extern crate light_cli;

use core::fmt::Write;   
use dev_hal::serial::Serial;
use dev_hal::prelude::*;
use light_cli::{LightCliInput, LightCliOutput};
use heapless::consts::*;
use heapless::String;


fn main() {
    let dp = dev_hal::stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // USART1
    let tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let rx = gpiob.pb7;

    let serial = Serial::usart1(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        9_600.bps(),
        clocks,
        &mut rcc.apb2,
    );

    let (mut tx, mut rx) = serial.split();

    let mut name : String<U32> = String::new();

    let mut cl_in : LightCliInput<U32> = LightCliInput::new();
    let mut cl_out = LightCliOutput::new(&mut tx);

    writeln!(cl_out, "Commands:").unwrap();
    writeln!(cl_out, "   - HELLO Name=<Name>: Set the name").unwrap();
    writeln!(cl_out, "   - EHLO: Print the name").unwrap();

    loop {
        let _ = cl_out.flush();
        let _ = cl_in.fill(&mut rx);

        lightcli!(cl_in, cl_out, cmd, key, val, [
                "HELLO" => [
                    "Name" => name = String::from(val)
                ] => { writeln!(cl_out, "Name set").unwrap(); };
                "EHLO" => [
                ] => { writeln!(cl_out, "EHLO Name={}", name.as_str()).unwrap(); }
            ]
        );
    }
}