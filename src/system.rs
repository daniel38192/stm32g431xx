#![allow(dead_code)]

use alloc::string::ToString;
use alloc::vec::Vec;
use core::panic::PanicInfo;
use core::sync::atomic;
use core::sync::atomic::Ordering;
use cortex_m::peripheral::NVIC;
use stm32g4::stm32g431::Interrupt::USART1;
use crate::drivers::serial::Serial;

pub const SYSCLK_FREQ: u32 = 170000000;
pub const APB1_FREQ: u32 = 170000000;
pub const APB2_FREQ: u32 = 170000000;


pub fn system_init(){
    //Configure system clock
    system_clock_config();
    //Configure Interrupts
    enable_interrupts();
    //Configure Serial
    Serial::configure();
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {

    disable_interrupts_in_case_of_fault();

    let binding = info.to_string();
    let message: Vec<&str> = binding.split("\n").collect();

    for p in message {
        Serial::println(p);
    }

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

fn system_clock_config(){
    /*
     * System clock settings.
     * Input clock source: 8Mhz Crystal
     * System clock: PLL
     * PLL source: HSE
     * PLL input prescaler: divide by 2
     * PLL multiplication factor: X 85
     * Set flash latency to 4 cycles
     * SYSCLK: 170MHz
     * APB1: 170Mhz
     * APB2: 170Mhz
     */
    unsafe {
        enable_hsi();
        enable_hse();
        configure_pll();
        enable_pll();
        configure_flash();
        switch_clock_source_to_pll();
    }
}

fn enable_interrupts(){
    unsafe {
        let mut cortex_m_peripherals = cortex_m::Peripherals::take().unwrap();
        enable_usart1_interrupt(&mut cortex_m_peripherals.NVIC)
    }
}

fn disable_interrupts_in_case_of_fault(){
    disable_usart1_interrupt()
}

unsafe fn enable_hsi(){

    let rcc = &*stm32g4::stm32g431::RCC::ptr();
    // Enable HSI bit 8 in RCC_CR
    rcc.cr.as_ptr().write(rcc.cr.as_ptr().read() | (1 << 8));
    // Wait for HSIRDY bit 10 in RCC_CR
    while !((rcc.cr.as_ptr().read() & (1 << 10)) > 0 ){ }
}

unsafe fn enable_hse(){
    let rcc = &*stm32g4::stm32g431::RCC::ptr();
    // Enable HSE bit 16 in RCC_CR
    rcc.cr.as_ptr().write(rcc.cr.as_ptr().read() | (1 << 16));
    // Wait for HSERDY bit 17 in RCC_CR
    while !((rcc.cr.as_ptr().read() & (1 << 17)) > 0){ }
}

unsafe fn configure_pll(){
    /*
     * PLL source: HSE
     * PLL input prescaler: divide by 2
     * PLL multiplication factor: X 85
     */
    let rcc = &*stm32g4::stm32g431::RCC::ptr();
    // set PLL divisor factor bits 7:4 in RCC_PLLCFGR
    rcc.pllcfgr.as_ptr().write(rcc.pllcfgr.as_ptr().read() | (1 << 4));
    // clear redundant bit 0x1000 in RCC_PLLCGFR register
    //rcc.pllcfgr.as_ptr().write(rcc.pllcfgr.as_ptr().read() & !0x1000);
    // set PLL multiplication factor bits 14:8 in RCC_PLLCFGR
    rcc.pllcfgr.as_ptr().write(rcc.pllcfgr.as_ptr().read() | (85 << 8));
    // set HSE as PLL clock source Bits 1:0 in RCC_PLLCFGR
    rcc.pllcfgr.as_ptr().write(rcc.pllcfgr.as_ptr().read() | (3 << 0));
    // enable PLL R clock used as system clock
    rcc.pllcfgr.as_ptr().write(rcc.pllcfgr.as_ptr().read() | (1 << 24));
    rcc.pllcfgr.as_ptr().write(rcc.pllcfgr.as_ptr().read() | (1 << 28));
}

unsafe fn enable_pll(){
    let rcc = &*stm32g4::stm32g431::RCC::ptr();
    // enable PLL bit 24 in RCC_CR register
    rcc.cr.as_ptr().write(rcc.cr.as_ptr().read() | (1 << 24));
    // wait for PLL ready flag bit 25 in RCC_CR register
    while !((rcc.cr.as_ptr().read() & (1 << 25)) > 0) { }
}

unsafe fn configure_flash(){
    let flash = &*stm32g4::stm32g431::FLASH::ptr();
    // Set latency to 4 Bits 3:0 in FLASH_ACR register
    flash.acr.as_ptr().write(flash.acr.as_ptr().read() & !(0xF << 0));
    flash.acr.as_ptr().write(flash.acr.as_ptr().read() | (4 << 0));
}

unsafe fn switch_clock_source_to_pll(){
    let rcc = &*stm32g4::stm32g431::RCC::ptr();
    // Switch to PLL clock Bits 1:0 in RCC_CFGR register
    rcc.cfgr.as_ptr().write(rcc.cfgr.as_ptr().read() | (3 << 0));
    // Wait to PLL clock to be used Bits 3:2 in RCC_CFGR register
    while !((rcc.cfgr.as_ptr().read() & (3 << 2)) > 0) { }
}

unsafe fn enable_usart1_interrupt(nvic: &mut NVIC){
    NVIC::unmask(USART1);
    NVIC::set_priority(nvic, USART1, 1);
}

fn disable_usart1_interrupt(){
    NVIC::mask(USART1);
}