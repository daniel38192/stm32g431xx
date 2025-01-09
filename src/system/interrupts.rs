use stm32h5::stm32h562::Interrupt::USART1;

pub fn enable_interrupts(){
    unsafe {
        let mut cortex_p = cortex_m::Peripherals::take().unwrap();
        cortex_m::peripheral::NVIC::unmask(USART1);
        cortex_p.NVIC.set_priority(USART1, 1);
    }
}