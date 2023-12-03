#![no_std]
#![no_main]

use panic_halt as _;

use air001_hal::{pac, prelude::*, serial::Serial};
use cortex_m;
use cortex_m_semihosting::hprintln;
use nb::block;

#[cortex_m_rt::entry]
unsafe fn start() -> ! {
    let mut p = pac::Peripherals::take().unwrap();

    let mut rcc = p.RCC.configure().sysclk(24.mhz()).freeze(&mut p.FLASH);

    // let rcc = &p.RCC;
    let gpioa = &p.GPIOA;
    let usart = &p.USART1;

    // enable GPIOA
    rcc.regs.iopenr.write(|w| w.gpioaen().set_bit());
    // enable usart1 clock
    rcc.regs.apbenr2.write(|w| w.usart1en().set_bit());

    // configure GPIO
    {
        // pull up (0b01)
        gpioa.pupdr.write(|w| w.pupd2().bits(0b01));

        // push pull (0b00)
        gpioa.otyper.write(|w| w.ot2().clear_bit());

        // high speed
        gpioa.ospeedr.write(|w| w.ospeed2().bits(0b11));

        // select GPIO alternative function (af1)
        gpioa.afrl.write(|w| w.afsel2().bits(0b0001));

        // alternative function mode (0b10)
        gpioa.moder.write(|w| w.mode2().bits(0b10));
    }

    // config usart
    {
        // disable
        // usart.cr1.write(|w| w.ue().clear_bit());

        // baud rate
        let brr = rcc.clocks.pclk().0 / 9600;
        usart.brr.write(|w| w.bits(brr));

        // no advance feature
        usart.cr2.reset();
        usart.cr3.reset();

        // tx mode and enable
        usart.cr1.modify(|_, w| w.te().set_bit().ue().set_bit());

        // enable
        // usart.cr1.write(|w| w.ue().set_bit());

        hprintln!("sysclk {}", rcc.clocks.sysclk().0);
        hprintln!("hclk {}", rcc.clocks.hclk().0);
        hprintln!("pclk {}", rcc.clocks.pclk().0);
        hprintln!("brr {}", brr);
    }

    loop {
        for i in 'a'..='z' {
            while !usart.sr.read().txe().bit_is_clear() {
                hprintln!("wait");
            }
            hprintln!("write {}", i);
            usart.dr.write(|w| w.dr().bits(i as u16));
            // usart.dr.write(|w| w.bits(i as u32));
            // while !usart.sr.read().tc().bit_is_clear() {
            //     hprintln!("not complete ");
            // }
        }
    }
}
