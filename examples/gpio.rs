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
        .sysclk(Hertz::MHz(28))
        .freeze(&mut p.FLASH);

    let gpioa = p.GPIOA.split(&mut rcc);
    let gpiob = p.GPIOB.split(&mut rcc);

    let pa1 = cortex_m::interrupt::free(|cs| gpioa.pa1.into_pull_up_input(cs));

    let mut red = cortex_m::interrupt::free(|cs| gpiob.pb1.into_push_pull_output(cs));
    let mut green = cortex_m::interrupt::free(|cs| gpiob.pb0.into_push_pull_output(cs));

    loop {
        if let Ok(true) = pa1.is_high() {
            red.set_high().ok();
            green.set_high().ok();
        } else {
            red.set_low().ok();
            green.set_low().ok();
        }
    }
}

fn delay() {
    for _i in 1..200000 {
        cortex_m::asm::nop();
    }
}
