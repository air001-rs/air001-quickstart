#![no_std]
#![no_main]


use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use air001_pac as pac;

#[entry]
fn main() -> ! {
    asm::nop();
    loop {
        // your code goes here
    }
}
