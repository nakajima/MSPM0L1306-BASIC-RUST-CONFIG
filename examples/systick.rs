#![no_std]
#![no_main]
#![allow(static_mut_refs)]

use panic_halt as _; // used to allow rust to run so it knows what to do if program crashes

use cortex_m_rt::{entry, exception}; // gives access to main function

use MSPM0L1306_HAL::{self, exceptions, gpio};

#[exception]
fn SysTick() {
    // interrupt function which runs whenever a SysTick exception occurs

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

    // configures SysTick exception to run once a second
    exceptions::interruptsetupsysttick();

    loop {}
}
