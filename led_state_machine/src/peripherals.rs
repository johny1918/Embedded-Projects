use core::cell::Cell;

use stm32f4xx_hal::{
    gpio::GpioExt,
    hal::delay::DelayNs,
    pac,
    rcc::{Rcc, RccExt},
    syscfg::SysCfgExt,
    timer::SysTimerExt,
};

pub struct Stm32f407gUtility {
    pub rcc: Rcc,
}

impl Stm32f407gUtility {
    pub fn new(rcc: pac::RCC) -> Self {
        Self {
            rcc: rcc.constrain(),
        }
    }

    pub fn get_gpio_x<G: GpioExt>(&mut self, gpio: G) -> G::Parts {
        gpio.split(&mut self.rcc)
    }
}
