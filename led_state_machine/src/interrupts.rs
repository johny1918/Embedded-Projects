use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use defmt::info;
use stm32f4xx_hal::gpio::ExtiPin;
use stm32f4xx_hal::{
    gpio::{Edge, Input, PA0},
    interrupt, pac,
    rcc::Rcc,
    syscfg::SysCfgExt,
};
pub static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

#[interrupt]
fn EXTI0() {
    unsafe {
        (*pac::EXTI::ptr())
            .pr()
            .write(|w| w.pr0().clear_bit_by_one());
    }

    BUTTON_PRESSED.store(true, Ordering::Release);
}

pub fn set_user_button_interrupt(
    mut button: PA0<Input>,
    syscfg: pac::SYSCFG,
    mut exti: pac::EXTI,
    rcc: &mut Rcc,
) {
    let mut syscfg = syscfg.constrain(rcc);

    button.make_interrupt_source(&mut syscfg);
    button.trigger_on_edge(&mut exti, Edge::Rising);
    button.clear_interrupt_pending_bit();
    button.enable_interrupt(&mut exti);

    pac::NVIC::unpend(pac::Interrupt::EXTI0);

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI0);
    }
}
