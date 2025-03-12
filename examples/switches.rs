#![no_std]
#![no_main]
#![allow(static_mut_refs)]

use panic_halt as _; // used to allow rust to run so it knows what to do if program crashes

use cortex_m_rt::entry; // gives access to main function

use MSPM0L1306_HAL::{self, exceptions, gpio};

use mspm0l130x::interrupt; // import needed to access interrupt type for MSPM0

#[interrupt]
fn INT_GROUP1() {
    // interrupt function which runs whenever an gpio interrupt occurs
    // creates a bool which can be used at any time in the function
    static mut VAL: bool = true;

    //gets bool and changes value and runs code based on value
    if *VAL {
        *VAL = false;
        // sets high every other time
        gpio::PA0.set_high();
    } else {
        *VAL = true;
        gpio::PA0.set_low();
    }
}

#[entry]
fn main() -> ! {
    // enables gpio
    gpio::enable();

    // configures the switch pins to receive interrupts and trigger them when pushed down or pulled up
    exceptions::interruptsetupgpioswitches();

    loop {}
}
