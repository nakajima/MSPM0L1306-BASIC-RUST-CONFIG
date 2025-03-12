#![no_std]
#![no_main]

use panic_halt as _; // used to allow rust to run so it knows what to do if program crashes

use cortex_m_rt::entry; // gives access to main function

use MSPM0L1306_HAL::{gpio::PA0, timg::TIMG0}; // gives access to main function

#[entry]
fn main() -> ! {
    //enables TIMG0 registers
    TIMG0::enable();
    //sets up a basic config to use PWM for the TIMG0 registers
    TIMG0::basic_config();

    //Configures PA0 to take PWM inputs
    PA0.configure_pwm();

    //Sets the output for PA0 by giving the duty cycle out of 100
    PA0.setpwm(20);

    loop {}
}
