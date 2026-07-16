#![no_std]
#![no_main]

mod interrupts;
mod traffic_light;
mod peripherals;
use crate::interrupts::set_user_button_interrupt;
use crate::{ peripherals::Stm32f407gUtility};
use crate::traffic_light::*;

use crate::interrupts::BUTTON_PRESSED;
use core::sync::atomic::Ordering;

use cortex_m_rt::entry;
use defmt::info;
use stm32f4xx_hal::{pac, prelude::*};
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    info!("Program is initializing");
    let peripherals = pac::Peripherals::take().unwrap();
    let mut utility = Stm32f407gUtility::new(peripherals.RCC);

    let mut current_traffic_light = Semaphore::new(TrafficLights::Red);

    // Use non-blocking delay
    let mut semaphore_timer = peripherals.TIM3.counter_ms(&mut utility.rcc);
    let mut debounce_timer = peripherals.TIM2.counter_ms(&mut utility.rcc);
    let mut button_armed = true;

    // Get PIN for Button
    let gpioa = utility.get_gpio_x(peripherals.GPIOA);
    let gpiod = utility.get_gpio_x(peripherals.GPIOD);
    // Set button mode
    let mut button = gpioa.pa0.into_pull_down_input();

    let mut led_red = gpiod.pd14.into_push_pull_output();
    let mut led_orange = gpiod.pd13.into_push_pull_output();
    let mut led_green = gpiod.pd12.into_push_pull_output();
    // Set interrupt for USER button
    set_user_button_interrupt(
        &mut button,
        peripherals.SYSCFG,
        peripherals.EXTI,
        &mut utility.rcc,
    );

    info!("Program is running");
    semaphore_timer.start(20000.millis()).unwrap();
    led_red.set_high();

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

            current_traffic_light.pedestrian_pressed_button(&mut semaphore_timer);
        }

        if semaphore_timer.wait().is_ok() {
            current_traffic_light.next_light();
            
            let duration = match current_traffic_light.get_current_light() {
                TrafficLights::Red => 20000.millis(),
                TrafficLights::Orange => 3500.millis(),
                TrafficLights::Green => 30000.millis(),
            };

            semaphore_timer.start(duration).unwrap();
        }

        match current_traffic_light.get_current_light() {
            TrafficLights::Red => {
                led_red.set_high();
                led_green.set_low();
                led_orange.set_low();
            },
            TrafficLights::Orange => {
                led_red.set_low();
                led_green.set_low();
                led_orange.set_high();

            },
            TrafficLights::Green => {
                led_red.set_low();
                led_green.set_high();
                led_orange.set_low();
            }
        }

        cortex_m::asm::nop();
    }
}
