#![no_std]
#![no_main]

use panic_halt as _; // used to allow rust to run so it knows what to do if program crashes

use cortex_m_rt::entry; // gives access to main function

use MSPM0L1306_HAL::gpio;

#[entry]
fn main() -> ! {
    // enables configuriation of gpio registers
    gpio::enable();

    //configures gpio registers for PA0 to be an output of 1 to the led
    gpio::PA0.set_high();

    loop {
        //checks to see if PA18 is High/ SW1 is being pushed
        if gpio::PA18.get_input() {
            //if High then PA26 an led is triggered
            gpio::PA26.set_high();
        } else {
            //if not lef is set Low
            gpio::PA26.set_low();
        }
    }
}
