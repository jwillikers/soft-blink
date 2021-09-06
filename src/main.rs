#![no_std]
#![no_main]

use panic_halt as _;

extern crate feather_m0 as bsp;
use bsp::entry;
use bsp::hal;
use bsp::Pins;
use core::fmt::Debug;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::CorePeripherals;
use hal::pac::Peripherals;
use hal::prelude::*;
use core::f32::consts::PI;
use micromath::F32Ext;
use bsp::hal::pwm::{Pwm2, Channel};

/// Oscillates up to a max range and back again as a sine wave.
#[derive(Clone, Copy, Debug, PartialEq)]
struct BrightnessRange {
    angle: u32,
    max: u32,
    value: u32,
}

impl BrightnessRange {
    fn new(max: u32) -> BrightnessRange {
        BrightnessRange {
            angle: 0u32,
            max,
            value: 0u32
        }
    }
}

impl Iterator for BrightnessRange {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let radians: f32 = self.angle as f32 * PI / 180_f32;
        self.value = ((((self.max / 2u32) + (self.max / 2u32)) as f32) * radians.sin()) as u32;
        if self.angle >= 360u32 {
            self.angle = 0u32;
        } else {
            self.angle += 1u32;
        }
        return Some(self.value);
    }
}

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);
    let gclk0 = clocks.gclk0();
    let pins = Pins::new(peripherals.PORT);

    // The red led on the Feather M0, Pin D13, maps to PA17 on the ATSAMD21.
    // The Alternate Peripheral Function E on pin PA17 uses the Timer Clock Control 2.
    let mut _red_led = pins.d13.into_mode::<hal::gpio::v2::AlternateE>();
    let pwm_frequency = 1.khz();
    let mut pwm2 = Pwm2::new(
        &clocks.tcc2_tc3(&gclk0).unwrap(),
        pwm_frequency,
        peripherals.TCC2,
        &mut peripherals.PM,
    );
    let channel = Channel::_1;
    let max_duty = pwm2.get_max_duty();
    let delay_ms = 4u8;

    loop {
        for j in BrightnessRange::new(max_duty) {
            pwm2.set_duty(channel, j);
            delay.delay_ms(delay_ms);
        }
    }
}
