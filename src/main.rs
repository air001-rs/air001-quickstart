#![no_std]
#![no_main]

use panic_halt as _;

use air001_hal::{pac, prelude::*};
use cortex_m::asm;
use cortex_m_rt::entry;
use fugit::HertzU32 as Hertz;

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();

    let mut rcc = p
        .RCC
        .configure()
        .sysclk(Hertz::MHz(48))
        .freeze(&mut p.FLASH);

    asm::nop();
    loop {
        // your code goes here
    }
}
