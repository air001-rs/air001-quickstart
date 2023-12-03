#![no_std]
#![no_main]

use panic_halt as _;

use air001_hal::{pac, prelude::*};
use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();

    let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut p.FLASH);

    asm::nop();
    loop {
        // your code goes here
    }
}
