#![no_std]
#![no_main]

use defmt::info;
use {defmt_rtt as _, panic_probe as _};
use cortex_m_rt::entry;
use stm32f4xx_hal::{gpio::GpioExt, hal::delay::DelayNs, pac, rcc::RccExt, timer::SysTimerExt};

#[entry]
fn main() -> ! {

    // Get peripherals
    let peripherals = pac::Peripherals::take().unwrap();
    // Get RCC peripheral
    let mut rcc = peripherals.RCC.constrain();

    // Get STM32 core peripherals
    let core = cortex_m::Peripherals::take().unwrap();

    // Get SysDelay
    let mut delay = core.SYST.delay(&rcc.clocks);
    // Get GPIOD to access LED
    let gpiod = peripherals.GPIOD.split(&mut rcc);

    // Set PD12 pin as PUSH PULL OUTPUT, default will be LOW
    let mut pd_12 = gpiod.pd12.into_push_pull_output();
    let mut pd_14 = gpiod.pd14.into_push_pull_output();
    info!("Blinking Led program start");

    loop {
        pd_12.toggle();
        delay.delay_ms(50);
        pd_14.toggle();
        delay.delay_ms(50);
        
    }
}
