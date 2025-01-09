
mod interrupts;
mod clock_config;

pub use clock_config::PCLK1_FREQ;

pub fn system_init(){
    clock_config::system_clock_config();
    interrupts::enable_interrupts()
}


