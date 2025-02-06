#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod clock;
mod led;

use arduino_hal::{hal::port::Dynamic, port::{mode::Output, Pin}};
use led::LedTask;
use panic_halt as _;

use fugit::{ Instant, Duration };

type TickInstant = Instant<u64, 1, 1000>;
type TickDuration = Duration<u64, 1, 1000>;

#[arduino_hal::entry]
fn main() -> ! {
    hal_main();
}

//fn bare_main() -> ! {
//    // Almost bare: Inline assembly needs additional compiler configuration or something like that,
//    // so building a delay using no-ops is too much of a hastle. Using arduino-hal's delay
//    // implementation, which is just a wrapper around properly configured no-ops.
//    const INTERVAL: u16 = 1000;
//
//    const DDRB: *mut u8 = 0x24 as *mut u8;
//    const PORTB: *mut u8 = 0x25 as *mut u8;
//
//    unsafe {
//        write_volatile(DDRB, 1 << 5);
//    }
//
//    loop { 
//        unsafe {
//            let write: u8 = read_volatile(PORTB as *const u8) ^ (1 << 5);
//            write_volatile(PORTB, write);
//        }
//        delay_ms(INTERVAL);
//        unsafe {
//            let write: u8 = read_volatile(PORTB as *const u8) & !(1 << 5);
//            write_volatile(PORTB, write);
//        }
//        delay_ms(INTERVAL);
//    }
//}
//
//


struct Ticker {
}

impl Ticker {
    fn new() -> Self {
        Self {}
    }

    //fn now(&self) -> TickInstant {
    fn now(&self) -> u64 {
        //TickInstant::from_ticks(clock::Clock::get_millis())
        clock::Clock::get_millis()
    }
}

struct LedArray {
    _array: [Pin<Output, Dynamic>; 4],
    _active: i8,
    _clockwise: bool
}

impl LedArray {
    fn new(mut array: [Pin<Output, Dynamic>; 4]) -> Self {
        array[0].toggle();
        Self { _array: array, _active: 0, _clockwise: true }
    }

    fn _length(&self) -> usize {
        self._array.len()
    }

    fn walk(&mut self) -> () {
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

    fn switch(&mut self) -> () {
        self._clockwise = !self._clockwise;
    }
}

//use core::fmt::Write;
//use ufmt::uwriteln;
//
//struct Serial {
//    usart0: USART0
//}
//
//impl Serial {
//    fn new(usart0: USART0) -> Self {
//        let ubrr: u16 = 103;  // Formula: F_CPU / (16 * BAUD) - 1 (for 16MHz clock)
//        usart0.ubrr0.write(|w| w.bits(bits));
//        usart0.ubrr0l.write(|w| w.bits((ubrr & 0xFF) as u8));
//        usart0.ubrr0h.write(|w| w.bits((ubrr >> 8) as u8));
//
//        // Enable TX
//        usart0.ucsr0b.modify(|_, w| w.txen0().set_bit());
//
//        Self { usart0 }
//    }
//}
//
//impl ufmt::uWrite for Serial {
//
//    type Error = core::convert::Infallible;
//
//    fn write_str(&mut self, s: &str) -> Result<(), core::convert::Infallible> {
//        for byte in s.bytes() {
//            unsafe {
//                self.usart0.usart0_status().read().udre0().bit_is_clear(); // Wait until ready
//                self.usart0.usart0_data().write(|w| w.bits(byte));
//            }
//        }
//        Ok(())
//    }
//}

struct Timer<'a> {
    //end_time: TickInstant,
    end_time: u64,
    ticker: &'a Ticker
}

impl<'a> Timer<'a> {
    //fn new(duration: TickDuration, ticker: &'a Ticker) -> Self {
    fn new(duration: u64, ticker: &'a Ticker) -> Self {
        Timer {
            ticker,
            end_time: ticker.now() + duration
        }
    }

    fn is_ready(&self) -> bool {
        self.ticker.now() >= self.end_time
    }
}

fn hal_main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);
    ufmt::uwriteln!(&mut serial, "Initialised Serial...").unwrap();

    clock::Clock::init(clock::TimerCounter::Tc0(dp.TC0));
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

