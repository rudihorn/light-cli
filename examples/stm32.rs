
#![no_std]

extern crate cortex_m;
extern crate panic_abort;
extern crate embedded_hal as hal;
extern crate stm32f103xx_hal as dev_hal;
extern crate heapless;

#[macro_use]
extern crate light_cli;

use dev_hal::serial::Serial;
use dev_hal::prelude::*;
use light_cli::LightCli;
use heapless::consts::*;
use heapless::String;

fn main() {
    let dp = dev_hal::stm32f103xx::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // USART3
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;

    let serial = Serial::usart3(
        dp.USART3,
        (tx, rx),
        &mut afio.mapr,
        9_600.bps(),
        clocks,
        &mut rcc.apb1,
    );

    let (mut tx, mut rx) = serial.split();

    let mut name : String<U32> = String::new();
    let mut cli : LightCli<U32> = LightCli::new();

    loop {
        cli.fill(&mut rx).unwrap();

        let _ = lightcli!(cli, cmd, key, val, [
            "HELLO" => [
                "Name" => name = String::from(val)
            ] => {};
            "EHLO" => [
            ] => {}
        ],
        {}, {}, {}
    );
    }
}