#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use riscv_rt::interrupt;

use gd32vf103xx_hal as hal;

use hal::prelude::*;
use hal::pac as pac;

#[entry]
fn main() -> ! {
    
    let x = pac::Peripherals::take().unwrap();

    loop {
        // your code goes here
    }
}
