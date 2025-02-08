use crate::button::ButtonDirection;
use crate::channel::Reciever;
use crate::time::{Ticker, Timer};

use arduino_hal::{hal::port::Dynamic, port::{mode::Output, Pin}};
use fugit::Duration;

type TickDuration = Duration<u64, 1, 1000>;

pub enum LedState<'a> {
    Toggle,
    Wait(Timer<'a>)
}

pub struct LedTask<'a> {
    array: LedArray,
    ticker: &'a Ticker,
    state: LedState<'a>,
    channel: Reciever<'a, ButtonDirection>
}

impl<'a> LedTask<'a> {
    pub fn new(ticker: &'a Ticker, array: LedArray, channel: Reciever<'a, ButtonDirection>) -> Self {
        LedTask {
            array,
            ticker,
            state: LedState::Toggle,
            channel
        }
    }

    pub fn poll(&mut self) -> () {
        match &self.state {
            LedState::Toggle => {
                match self.channel.receive() {
                    Some(direction) => {
                        self.array.switch(direction);
                    }
                    None => {}
                }
                self.array.walk();
                self.state = LedState::Wait(
                    Timer::new(TickDuration::from_ticks(250), &self.ticker)
                )
            },
            LedState::Wait(timer) => {
                if timer.is_ready() {
                    self.state = LedState::Toggle
                }
            }
        }
    }

}

pub struct LedArray {
    array: [Pin<Output, Dynamic>; 4],
    active: i8,
    direction: ButtonDirection
}

impl LedArray {
    pub fn new(mut array: [Pin<Output, Dynamic>; 4]) -> Self {
        array[0].set_high();
        Self { array , active: 0, direction: ButtonDirection::Clockwise }
    }

    fn _length(&self) -> usize {
        self.array.len()
    }

    pub fn walk(&mut self) -> () {
        self.array[self.active as usize].toggle();
        let modifier: i8 = match self.direction {
            ButtonDirection::Clockwise => 1,
            ButtonDirection::CounterClockwise => -1
        };

        // scuffed custom modulo operator because Rust only has remainder, which operates in 
        // reverse for negative numbers
        self.active = (((self.active + modifier) % 4) + 4) % 4;
        self.array[self.active as usize].toggle();
    }

    pub fn switch(&mut self, direction: ButtonDirection) -> () {
        self.direction = direction
    }
}
