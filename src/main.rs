// src/main.rs

// std and main are not available for bare metal software
#![no_std]
#![no_main]

// pick a panicking behavior
// #[allow(unused_imports)]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use fugit::{Duration, ExtU32};
use hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(_cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let gpioa = dp.GPIOA.split();
        let button = gpioa.pa0;
        // Set up the LED. On the Mini-F4 it's connected to pin PC13.
        let gpioc = dp.GPIOC.split();
        let mut led = gpioc.pc13.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(48.MHz()).freeze();

        // Create a delay abstraction based on general-pupose 32-bit timer TIM5
        let mut delay = dp.TIM5.delay_us(&clocks);

        let mut counter = dp.TIM1.counter_ms(&clocks);

        let mut del_var: Duration<u32, 1, 1000> = 2001.millis();

        led.set_low();

        loop {
            // // On for 1s, off for 3s.
            led.set_high();
            // // Use `embedded_hal::DelayMs` trait
            delay.delay_ms(1000_u32);
            led.set_low();
            // // or use `fugit::ExtU32` trait
            // delay.delay(1.secs());
            // led.toggle();
            delay.delay(2.secs());
        }
    }
    loop {
        continue;
    }
}
