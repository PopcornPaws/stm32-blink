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
    let gpioa = &peripherals.GPIOA;
    let gpioe = &peripherals.GPIOE;
    let rcc = &peripherals.RCC;

    // Enable GPIO clock for IO ports A and E
    rcc.abp2enr.modify(|_, w| {
        w.iopaen().enabled();
        w.iopeen().enabled();
    });
    
}
