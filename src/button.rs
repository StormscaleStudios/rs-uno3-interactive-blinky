use crate::time::{Timer, Ticker, TickDuration};
use crate::channel::Sender;

use arduino_hal::port::Pin;
use arduino_hal::port::mode::{Input, PullUp};
use arduino_hal::hal::port::Dynamic;

pub enum ButtonDirection {
    Clockwise,
    CounterClockwise
}

enum ButtonState<'a> {
    Listening,
    Debounce(Timer<'a>),
}

pub struct ButtonTask<'a> {
    pin: Pin<Input<PullUp>, Dynamic>,
    ticker: &'a Ticker,
    direction: ButtonDirection,
    state: ButtonState<'a>,
    channel: Sender<'a, ButtonDirection>
}

impl<'a> ButtonTask<'a> {
    pub fn new(button: Pin<Input<PullUp>, Dynamic>, ticker: &'a Ticker, channel: Sender<'a, ButtonDirection>) -> Self {
        ButtonTask {
            pin: button,
            ticker,
            direction: ButtonDirection::Clockwise,
            state: ButtonState::Listening,
            channel
        }
    }

    pub fn poll(&mut self) -> () {
        match self.state {
            ButtonState::Listening => {
                if self.pin.is_low() {
                    self.state = ButtonState::Debounce(Timer::new(TickDuration::from_ticks(100), self.ticker));
                    
                    match self.direction {
                        ButtonDirection::Clockwise => {
                            self.direction = ButtonDirection::CounterClockwise;
                            self.channel.send(ButtonDirection::CounterClockwise);
                        }
                        ButtonDirection::CounterClockwise => {
                            self.direction = ButtonDirection::Clockwise;
                            self.channel.send(ButtonDirection::Clockwise);
                        }
                    }
                }

            }
            ButtonState::Debounce(ref timer) => {
                if timer.is_ready() && self.pin.is_high() {
                    self.state = ButtonState::Listening
                }
            }
        }
    }
}

