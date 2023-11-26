#![no_std]
#![no_main]

use panic_halt as _;

use air001_hal::{pac, prelude::*};
use cortex_m;
use fugit::HertzU32 as Hertz;

#[cortex_m_rt::entry]
fn start() -> ! {
    let mut p = pac::Peripherals::take().unwrap();

    let mut rcc = p
        .RCC
        .configure()
        .sysclk(Hertz::MHz(48))
        .freeze(&mut p.FLASH);

    let gpiob = p.GPIOB.split(&mut rcc);

    // Configure output mode
    let (mut red, mut green, mut blue) = cortex_m::interrupt::free(|cs| {
        (
            gpiob.pb1.into_push_pull_output(cs),
            gpiob.pb0.into_push_pull_output(cs),
            gpiob.pb3.into_push_pull_output(cs),
        )
    });

    loop {
        red.set_high().ok();
        green.set_low().ok();
        blue.set_low().ok();
        delay();

        red.set_low().ok();
        green.set_high().ok();
        blue.set_low().ok();
        delay();

        red.set_low().ok();
        green.set_low().ok();
        blue.set_high().ok();
        delay();
    }
}

fn delay() {
    for _i in 1..200000 {
        cortex_m::asm::nop();
    }
}
