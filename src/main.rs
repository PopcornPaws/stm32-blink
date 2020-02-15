#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;
extern crate stm32f3;

use cortex_m_rt::entry;
use stm32f3::stm32f303;

// Use main as the entry point for this application
#[entry]
fn main() -> ! {
    // Get handles to the hardware
    let peripherals = stm32f303::Peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA; // Push button is accessed via A0
    let gpioe = &peripherals.GPIOE; // Led is accessed via E15
    let rcc = &peripherals.RCC;

    // Enable GPIO clock for IO ports A and E on ABP2 Clock
    rcc.abp2enr.modify(|_, w| {
        w.iopaen().enabled();
        w.iopeen().enabled();
    });
    
}
