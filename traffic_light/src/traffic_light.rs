use stm32f4xx_hal::{pac::TIM3, timer::{ CounterMs}};
use stm32f4xx_hal::prelude::*;


#[derive(defmt::Format, Copy, Clone)]
pub enum TrafficLights {
    Red,
    Orange,
    Green,
}

#[derive(Clone, Copy)]
pub struct Semaphore {
    light: TrafficLights,
    transition_shorthen: bool,
}

impl TrafficLights {
    pub fn next(self) -> Self {
        match self {
            Self::Red => Self::Orange,
            Self::Orange => Self::Green,
            Self::Green => Self::Red,
        }
    }
}

impl Semaphore {
    pub fn new(light: TrafficLights) -> Self {
        Self { light, transition_shorthen: false }
    }

    pub fn next_light(&mut self) {
        self.light = self.light.next();
        self.transition_shorthen = false;
    }

    pub fn get_current_light(&self) -> TrafficLights {
        self.light
    }

    pub fn pedestrian_pressed_button(
        &mut self,
        timer: &mut CounterMs<TIM3>,
    ) {
        if self.transition_shorthen {
            return;
        }

        let (normal_duration, fast_furation): (u32, u32) = match self.light {
            TrafficLights::Red => (20_000, 10_000),
            TrafficLights::Orange => (3_500, 1_000),
            TrafficLights::Green => return,
        };

        let elapsed_ms = timer.now().duration_since_epoch().ticks();
        let remaining_ms = normal_duration.saturating_sub(elapsed_ms);

        if remaining_ms > fast_furation {
            timer.start(fast_furation.millis()).unwrap();
            self.transition_shorthen = true;
        }
    }
}
