#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod time;
use crate::time::{Clock, TimerCounter, Ticker};

mod led;
use crate::led::{LedArray, LedTask};

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);
    ufmt::uwriteln!(&mut serial, "Initialised Serial...").unwrap();

    Clock::init(TimerCounter::Tc0(dp.TC0));
    let ticker = Ticker::new();

    let _button = pins.d2.into_pull_up_input();

    let array = [
        pins.d3.into_output().downgrade(),
        pins.d4.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
        pins.d6.into_output().downgrade()
    ];
    let led_array = LedArray::new(array);
    let mut led_task = LedTask::new(&ticker, led_array);

    loop {
        led_task.poll();
    }
}


