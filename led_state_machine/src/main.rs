#![no_std]
#![no_main]

mod interrupts;
mod led_state_machine;
mod peripherals;
use crate::interrupts::set_user_button_interrupt;
use crate::led_state_machine::LedStateMachine;
use crate::{led_state_machine::LedState, peripherals::Stm32f407gUtility};

use crate::interrupts::BUTTON_PRESSED;
use core::sync::atomic::Ordering;

use cortex_m_rt::entry;
use defmt::info;
use stm32f4xx_hal::{pac, prelude::*};
use stm32f4xx_hal::timer::SysTimerExt;
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    info!("Program is initializing");
    let peripherals = pac::Peripherals::take().unwrap();
    let core_functions = cortex_m::Peripherals::take().unwrap();
    let mut utility = Stm32f407gUtility::new(peripherals.RCC);
    let mut current_state = LedStateMachine::new(LedState::Off);

    // Use non-blocking delay
    let mut blink_timer = core_functions.SYST.counter_us(&utility.rcc.clocks);
    let mut debounce_timer = peripherals.TIM2.counter_ms(&mut utility.rcc);
    let mut button_armed = true;

    // Get PIN for Button
    let gpioa = utility.get_gpio_x(peripherals.GPIOA);
    let gpiod = utility.get_gpio_x(peripherals.GPIOD);
    // Set button mode
    let mut button = gpioa.pa0.into_pull_down_input();

    let mut led_red = gpiod.pd14.into_push_pull_output();
    // Set interrupt for USER button
    set_user_button_interrupt(
        &mut button,
        peripherals.SYSCFG,
        peripherals.EXTI,
        &mut utility.rcc,
    );

    led_red.set_low();

    loop {
        // Logic to handle the button to be pressed only 1 time
       if !button_armed && debounce_timer.wait().is_ok() {
            if button.is_low() {
                BUTTON_PRESSED.store(false, Ordering::Release);
                button_armed = true;
            } else {
                debounce_timer.start(20.millis()).unwrap();
            }
        }
        
        if button_armed && BUTTON_PRESSED.swap(false, Ordering::AcqRel) {
            button_armed = false;
            debounce_timer.start(50.millis()).unwrap();

            current_state.next_state();

            match current_state.get_state() {
                LedState::Off => {
                    blink_timer.cancel().ok();
                    led_red.set_low();
                }

                LedState::On => {
                    blink_timer.cancel().ok();
                    led_red.set_high();
                }

                LedState::FastBlink => {
                    led_red.set_high();
                    blink_timer.start(50.millis()).unwrap();
                }

                LedState::SlowBlink => {
                    led_red.set_high();
                    blink_timer.start(500.millis()).unwrap();
                }
            }

            info!("LED state changed to {}", current_state.get_state());
        }

        match current_state.get_state() {
            LedState::FastBlink | LedState::SlowBlink => {
                if blink_timer.wait().is_ok() {
                    led_red.toggle();
                }
            }

            LedState::Off | LedState::On => {}
        }

    }
}
