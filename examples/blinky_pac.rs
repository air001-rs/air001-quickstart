#![no_std]
#![no_main]

use panic_halt as _;

use air001_hal::pac;
use cortex_m;

#[cortex_m_rt::entry]
unsafe fn start() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let rcc = &p.RCC;
    let gpiob = &p.GPIOB;

    // enable GPIOB
    rcc.iopenr.modify(|_, w| w.gpioben().set_bit());

    // output mode
    gpiob.moder.modify(|_, w| {
        w.mode1().bits(1);
        w.mode0().bits(1);
        w.mode3().bits(1)
    });

    // enable pull-up
    gpiob.pupdr.modify(|_, w| {
        w.pupd1().bits(1);
        w.pupd0().bits(1);
        w.pupd3().bits(1)
    });

    loop {
        gpiob.bsrr.write(|w| w.bs1().set_bit());
        gpiob.brr.write(|w| w.br3().set_bit());
        delay();

        gpiob.bsrr.write(|w| w.bs0().set_bit());
        gpiob.brr.write(|w| w.br1().set_bit());
        delay();

        gpiob.bsrr.write(|w| w.bs3().set_bit());
        gpiob.brr.write(|w| w.br0().set_bit());
        delay();
    }
}

fn delay() {
    for _i in 1..200000 {
        cortex_m::asm::nop();
    }
}
