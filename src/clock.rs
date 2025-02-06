use core::cell;

static MILLIS: avr_device::interrupt::Mutex<cell::Cell<u64>> = avr_device::interrupt::Mutex::new(cell::Cell::new(0));

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    // Interupt method that is triggered when Timer/Counter0 value is equal to the value in Output
    //    Compare Register A
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + 1);
    })
}

pub fn init_tc0(tc0: &arduino_hal::pac::TC0) -> () {
    // Timer Design v2.0: (Thanks Rahix)
    // CTC mode, reset clock on OCR0A match (11 on TCCR0A bits 0:1, keep 0 on TCCR0B bit 3)
    // Set OCRA to 249 (1111_1001 on OCR0A)
    // Prescalar at x64 (011 on TCCR0B bits 0:2)
    // Enable interupts on OCF0A (1 on TIMSK0 bit 1)
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| w.bits(249 as u8));
    tc0.tccr0b.write(|w| w.cs0().prescale_64());
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    // Handle interupt (vector no. 15, 0x001C, TIMER0 COMPA)
}

pub enum TimerCounter {
    Tc0(arduino_hal::pac::TC0),
    Tc1(arduino_hal::pac::TC1),
    Tc2(arduino_hal::pac::TC2)
}
pub struct Clock {
    tc: TimerCounter
}

impl Clock {
    // SCRAP ORIGINAL PLAN in favour of pretty accurate custom millisecond counter:
    //    https://blog.rahix.de/005-avr-hal-millis
    //
    // 16 MHz clock speed, prescalar at x64, counting up to 250 (set TCNT0 max value to 249) will
    //    result in an "exactly" 1 millisecond interupt pulse 
    //
    // ORIGINAL PLAN
    // timer/clock 0
    // interupts: TOV0, OCF0A, OCF0B
    //
    // Timer/Counter 0 -> TCTN0
    // Timer/Counter Control Register -> TCCR0A, TCCR0B
    // Output Compare Register -> OCR0A, OCR0B
    // Output Compare Pins -> OC0A, OC0B
    // Interupt Flag Regiser -> TIFR0 (show all Interupt Request Signals)
    // Interupts Mask Register -> TIMSK0
    //
    // Use clock selection logic to choose from:
    // inactive (no clock)
    // internal
    // pre-scaled
    // external (pin T0)
    //  => controlled by clock select bits (CS02:0) located in TCCR0B register
    //
    // counting sequence is determined by WGM00/01/02 bits (0,1 in TCCR0A, 2 in TCCR0B)
    //
    // if OCRnx value matches TCNTn value, a signal is sent to the corresponding interupt (A/B)
    //
    // The design:
    // Set clock bits to some internal function (simplest: 001 on bits 0:2 in TCCR0B @ 0x45)
    //  - 001 for at-clock speed, 101 for prescalar slowdown up to 1024x (2x per bit)
    // Keep WGM0n at normal operation (00 on bits 0:1 TCCR0A @ 0x44, 0 on bit 3 in TCCR0B @ 0x45)
    // Enable interupts on TCNT overflow OVF0 (1 on bit 0 in TIMSK0 @ 0x6E)
    //  - Now interupt is triggered once every TCNT0 cycle 256 steps at X MHz
    // Catch the interupt to increment internal counter (Vector No. 17, 0x0020, TIMER0 OVF)
    // Or, use the overflow interupt (TIMSK0 bit 0)
    //
    // The Design v2.0: (Thanks Rahix)
    // Prescalar at x64 (011 on TCCR0B bits 0:2)
    // CTC mode, reset clock on OCR0A match (11 on TCCR0A bits 0:1, keep 0 on TCCR0B bit 3)
    // Set OCRA to 249 (1111_1001 on OCR0A)
    // Enable interupts on OCF0A (1 on TIMSK0 bit 1)
    // Handle interupt (vector no. 15, 0x001C, TIMER0 COMPA)

    pub fn init(timer: TimerCounter) -> Self {
        match timer {
            TimerCounter::Tc0(ref tc) => {
                init_tc0(&tc);
            },
            _ => panic!("not implemented")
        };

        unsafe { avr_device::interrupt::enable() };

        Clock { tc: timer }
    }

    pub fn get_millis() -> u64 {
        avr_device::interrupt::free(|cs| {
            MILLIS.borrow(cs).get()
        })
    }
}
