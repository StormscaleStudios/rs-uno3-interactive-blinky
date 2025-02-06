use crate::{LedArray, TickDuration, Ticker, Timer};


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
                    Timer::new(400, &self.ticker)
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
