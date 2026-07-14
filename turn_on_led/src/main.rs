#![no_std]
#![no_main]

use defmt::info;
use {defmt_rtt as _, panic_probe as _};
use cortex_m_rt::entry;
use stm32f4xx_hal::{gpio::GpioExt, pac, rcc::RccExt};


#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    // Stm32f4xx_hal handling configuration of registers
    let mut rcc = peripherals.RCC.constrain();
    let gpiod = peripherals.GPIOD.split(&mut rcc);

    let mut led = gpiod.pd12.into_push_pull_output();

    led.set_high();


    // Manual configurations of registers

    // peripherals.RCC.ahb1enr().modify(|r, w| unsafe {
    //     w.bits(r.bits() | (1 << 3))
    // });

    // peripherals.GPIOD.moder().modify(|r, w| unsafe {
    //     let mut bits = r.bits();
    //     bits &= !(0b11 << 24);
    //     bits |=0b01 << 24;
    //     w.bits(bits)
    // });

    // peripherals.GPIOD.otyper().modify(|r,w|unsafe {
    //     w.bits(r.bits() & !(1 << 12))
    // });

    // peripherals.GPIOD.ospeedr().modify(|r,w|unsafe {
    //     w.bits(r.bits() & !(0b11 << 24))
    // });

    // peripherals.GPIOD.pupdr().modify(|r,w|unsafe {
    //     w.bits(r.bits() & !(0b11 << 24))
    // });

    // peripherals.GPIOD.bsrr().write(|w|unsafe {
    //     w.bits(1 << 12)
    // });

    info!("Program initialized");

    loop {
        cortex_m::asm::nop();;
    }
}

