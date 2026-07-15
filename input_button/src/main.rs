#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m_rt::entry;
use defmt::info;
use {defmt_rtt as _, panic_probe as _};

use stm32f4xx_hal::{
    gpio::{Edge, ExtiPin, GpioExt}, interrupt, pac, rcc::RccExt, syscfg::SysCfgExt, hal::delay::DelayNs, timer::SysTimerExt,
};

static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

#[interrupt]
fn EXTI0() {
    // EXTI0 este de tip "write 1 to clear".
    unsafe {
        (*pac::EXTI::ptr())
            .pr()
            .write(|w| w.pr0().clear_bit_by_one());
    }

    // Button pressed detected
    BUTTON_PRESSED.store(true, Ordering::Release);
}

#[entry]
fn main() -> ! {

    let mut start_lights = false;

    // Get peripherals
    let peripherals = pac::Peripherals::take().unwrap();
    let mut rcc = peripherals.RCC.constrain();
    let core = cortex_m::Peripherals::take().unwrap();

    // Get sys timer
    let mut delay_sys = core.SYST.delay(&rcc.clocks);

    // Get GPIO's
    let gpioa = peripherals.GPIOA.split(&mut rcc);
    let gpiod = peripherals.GPIOD.split(&mut rcc);
    let mut syscfg = peripherals.SYSCFG.constrain(&mut rcc);
    let mut exti = peripherals.EXTI;

    // User button from board
    let mut button = gpioa.pa0.into_input();
    // PD 15, blue led
    let mut led_blue = gpiod.pd15.into_push_pull_output();

    // PD 14, red led
    let mut led_red = gpiod.pd14.into_push_pull_output();

    
    // Set PA0 to EXTI0
    button.make_interrupt_source(&mut syscfg);

    // Set button detection on rising/falling edge
    button.trigger_on_edge(&mut exti, Edge::Rising);

    // Clear pin for interrupt
    button.clear_interrupt_pending_bit();

    // Enable EXTI10
    button.enable_interrupt(&mut exti);

    
    // Clear pending event
    pac::NVIC::unpend(pac::Interrupt::EXTI0);

    // Enable interrupt for EXTI0
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI0);
    }


    info!("Program started");

    loop {
        
        if BUTTON_PRESSED.swap(false, Ordering::AcqRel) {
            info!("USER button pressed");
            if start_lights == false {
                start_lights = true;
            }
            
            
        }
        if start_lights == true {
            // Simulate police lights blue and red
            led_blue.toggle();
            delay_sys.delay_ms(50);
            led_red.toggle();
            delay_sys.delay_ms(50);
        }


        cortex_m::asm::nop();
    }
}
