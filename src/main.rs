#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod time;
mod led;
mod button;
mod channel;

use crate::time::{Clock, TimerCounter, Ticker};
use crate::led::{LedArray, LedTask};
use crate::button::{ButtonTask, ButtonDirection};
use crate::channel::Channel;

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);
    ufmt::uwriteln!(&mut serial, "Initialised Serial...").unwrap();

    Clock::init(TimerCounter::Tc0(dp.TC0));
    let ticker = Ticker::new();


    let array = [
        pins.d3.into_output().downgrade(),
        pins.d4.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
        pins.d6.into_output().downgrade()
    ];
    let led_array = LedArray::new(array);

    let channel: Channel<ButtonDirection> = Channel::new();

    let mut button_task = ButtonTask::new(
        pins.d2.into_pull_up_input().downgrade(), 
        &ticker, 
        channel.get_sender()
    );
    let mut led_task = LedTask::new(
        &ticker, 
        led_array,
        channel.get_reciever()
    );

    loop {
        button_task.poll();
        led_task.poll();
    }
}


