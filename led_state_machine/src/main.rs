#![no_std]
#![no_main]

mod peripherals;
mod interrupts;
mod led_state_machine;
use crate::{led_state_machine::LedState, peripherals::Stm32f407gUtility};
use crate::interrupts::set_user_button_interrupt;
use crate::led_state_machine::LedStateMachine;

use core::sync::atomic::Ordering;
use crate::interrupts::BUTTON_PRESSED;

use defmt::info;
use stm32f4xx_hal::hal::delay::DelayNs;
use stm32f4xx_hal::timer::SysTimerExt;
use {defmt_rtt as _, panic_probe as _};
use cortex_m_rt::entry;
use stm32f4xx_hal::pac;



#[entry]
fn main() -> ! {
    info!("Program is initializing");
    let peripherals = pac::Peripherals::take().unwrap();
    let core_functions = cortex_m::Peripherals::take().unwrap();
    let mut utility = Stm32f407gUtility::new(peripherals.RCC);
    let mut current_state = LedStateMachine::new(LedState::Off);

    let mut delay = core_functions.SYST.delay(&utility.rcc.clocks);
    // Get PIN for Button
    let gpioa = utility.get_gpio_x(peripherals.GPIOA);
    let gpiod = utility.get_gpio_x(peripherals.GPIOD);
    // Set button mode
    let button = gpioa.pa0.into_pull_down_input();

    let mut led_red = gpiod.pd14.into_push_pull_output();
    // Set interrupt for USER button
    set_user_button_interrupt(button, peripherals.SYSCFG, peripherals.EXTI, &mut utility.rcc);
    
  loop 
  {
    if BUTTON_PRESSED.load(Ordering::Acquire) 
    {
        // Șterge și evenimentele suplimentare produse de bounce.
        BUTTON_PRESSED.store(false, Ordering::Release);

        current_state.next_state();

        info!("LED state changed");
    }

    match current_state.get_state() 
    {
        LedState::Off => {
            led_red.set_low();
            info!("Current state is OFF");
        }
        LedState::On => {
            led_red.set_high();
            info!("Current state is ON");
        }
        LedState::FastBlink => {
            led_red.set_high();
            delay.delay_ms(50);
            led_red.set_low();
            delay.delay_ms(50);
            info!("Current state is FastBlink 50ms");
        }
        LedState::SlowBlink => {
            led_red.set_high();
            delay.delay_ms(500);
            led_red.set_low();
            delay.delay_ms(500);
            info!("Current state is SlowBlink 500ms");
        }
    }
 }
}
