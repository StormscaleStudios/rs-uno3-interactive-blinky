use crate::time::{Timer, Ticker};

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
    state: LedState<'a>
}

impl<'a> LedTask<'a> {
    pub fn new(ticker: &'a Ticker, array: LedArray) -> Self {
        LedTask {
            array,
            ticker,
            state: LedState::Toggle
        }
    }

    pub fn poll(&mut self) -> () {
        match &self.state {
            LedState::Toggle => {
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
    _array: [Pin<Output, Dynamic>; 4],
    _active: i8,
    _clockwise: bool
}

impl LedArray {
    pub fn new(mut array: [Pin<Output, Dynamic>; 4]) -> Self {
        array[0].toggle();
        Self { _array: array, _active: 0, _clockwise: true }
    }

    fn _length(&self) -> usize {
        self._array.len()
    }

    pub fn walk(&mut self) -> () {
        self._array[self._active as usize].toggle();
        let modifier: i8 = match self._clockwise {
            true => 1,
            false => -1
        };

        // scuffed custom modulo operator because Rust only has remainder, which operates in 
        // reverse for negative numbers
        self._active = (((self._active + modifier) % 4) + 4) % 4;
        self._array[self._active as usize].toggle();
    }

    pub fn switch(&mut self) -> () {
        self._clockwise = !self._clockwise;
    }
}
