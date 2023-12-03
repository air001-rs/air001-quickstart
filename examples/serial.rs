#![no_std]
#![no_main]

use core::fmt::Write;

use panic_halt as _;

use air001_hal::{pac, prelude::*, serial::Serial};
use cortex_m;
use nb::block;

#[cortex_m_rt::entry]
fn start() -> ! {
    let mut p = pac::Peripherals::take().unwrap();

    let mut rcc = p.RCC.configure().sysclk(24.mhz()).freeze(&mut p.FLASH);

    let gpioa = p.GPIOA.split(&mut rcc);

    let (tx, rx) = cortex_m::interrupt::free(|cs| {
        (
            gpioa.pa2.into_alternate_af1(cs),
            gpioa.pa3.into_alternate_af1(cs),
        )
    });

    let mut serial = Serial::usart1(p.USART1, (tx, rx), 115200.bps(), &mut rcc);

    serial.write_str("Hello, World!").unwrap();

    loop {
        let received = block!(serial.read()).unwrap();
        block!(serial.write(received)).ok();
    }
}

