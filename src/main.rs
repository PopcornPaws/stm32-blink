#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f3::stm32f303;

// Use main as the entry point for this application
#[entry]
fn main() -> ! {
    // Get handles to the hardware
    let peripherals = stm32f303::Peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA; // Push button is accessed via A0
    let gpioe = &peripherals.GPIOE; // LED is accessed via E15
    let rcc = &peripherals.RCC;

    // Enable GPIO clock for IO ports A and E on ABP2 Clock
    rcc.ahbenr.modify(|_, w| {
        w.iopaen().enabled();
        w.iopeen().enabled()
    });

    // Set up button (A0) as input
    gpioa.moder.modify(|_, w| w.moder0().input());

    // Set up button (A0) as pull-down
    unsafe {
        gpioa.pupdr.modify(|_, w| w.pupdr0().bits(0b00));
    }

    // Set up LED (E15) as output
    gpioe.moder.modify(|_, w| w.moder15().output());

    // Main loop
    loop {
        // If button is pressed, switch LED on.
        if gpioa.idr.read().bits() & 1 == 1 {
            gpioe.bsrr.write(|w| w.bs15().set_bit())
        }

        // If button is not pressed, switch LED off.
        if gpioa.idr.read().bits() & 1 == 0 {
            gpioe.brr.write(|w| w.br15().set_bit())
        }
    }
}
