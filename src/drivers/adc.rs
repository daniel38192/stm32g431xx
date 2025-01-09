// ADC implementation

//Generic

/*
    ADC on-off control (ADEN, ADDIS, ADRDY)
    First of all, follow the procedure explained in Section26.4.6: ADC Deep-power-down mode (DEEPPWD) and ADC voltage regulator (ADVREGEN)).
    Once DEEPPWD=0 and ADVREGEN=1, the ADC can be enabled and the ADC needs a stabilization time of tSTAB before it starts converting accurately, as shown in Figure200. Two
    control bits enable or disable the ADC:
        • ADEN =1 enables the ADC. The flag ADRDY is set once the ADC is ready for operation.
        • ADDIS =1 disables the ADC. ADEN and ADDIS are then automatically cleared by hardware as soon as the analog ADC is effectively disabled.
    Regular conversion can then start either by setting ADSTART=1 (refer to Section26.4.18: Conversion on external trigger and trigger polarity (EXTSEL, EXTEN,JEXTSEL, JEXTEN))
    or when an external trigger event occurs, if triggers are enabled.
    Injected conversions start by setting JADSTART=1 or when an external injected trigger
    event occurs, if injected triggers are enabled.
 */
use core::arch::asm;
use stm32h5::stm32h562 as stm32;

use crate::drivers::gpio;
use crate::drivers::gpio::config::{MODER, OSPEEDR, OTYPER, PUPDR};
use crate::drivers::gpio::GPIOPORT;


use crate::drivers::uart::Serial;

pub fn adc1_test_configure(){

    fn port_gpio_configure(){

        //configure adc port test gpio
        //PB1 ADC12_INP5
        let adc_gpio = gpio::Gpio::new(GPIOPORT::GPIOB, 1);

        const ADC_CFG: gpio::GpioConfig = gpio::GpioConfig {
            moder: MODER::Analog,
            otyper: OTYPER::PushPull,
            ospeedr: OSPEEDR::MediumSpeed,
            pupdr: PUPDR::None,
            alt_func_select: None
        };

        adc_gpio.configure(ADC_CFG);
    }


    fn port_adc_enable_rcc_clk(){
        let rcc = unsafe {stm32::RCC::steal()};

        rcc.ccipr5().modify(|_, w| w.adcdacsel().hse());

        rcc.ahb2enr().modify(|_,w| w.adc12en().set_bit());
    }

    fn port_adc_register_configure(){
        // internal temperature sensor test
        //ADC1 VINP[16]
        //put ADC1_INPUT16 in the first conversion in regular sequence, ADC regular sequence register 1 (ADC_SQR1)
        let adc = adc1();

        adc.sqr1().modify(|_, w| unsafe {w.sq1().bits(5)});

        //Enable internal temperature sensor
        //adc.ccr().modify(|_, w| w.tsen().set_bit());

        // set the lenght of conversions by 1
        //adc.sqr1().modify(|_, w| unsafe {w.l().bits(1)});

        /*

        //enable discontinuous mode, ADC configuration register (ADC_CFGR)
        adc.cfgr().modify(|_, w| w.discen().set_bit());

        //just one channel
        adc.cfgr().modify(|_, w | unsafe {w.discnum().bits(1)});

         */

        /*
        //select sampling for channel 5
        adc.smpr1().modify(|_, w| unsafe {w.smp5().bits(2)});
         */

        adc.smpr1().modify(|_, w| unsafe {w.smp5().bits(4)})



    }

    fn adc_calibrate(){
        let adc = adc1();
        /*
        Software procedure to calibrate the ADC
        1. Ensure DEEPPWD=0, ADVREGEN=1 and that ADC voltage regulator startup time
        has elapsed.
        2. Ensure that ADEN=0.
        3. Select the input mode for this calibration by setting ADCALDIF=0 (single-ended input) or ADCALDIF=1 (differential input).
        4. Set ADCAL=1.
        5. Wait until ADCAL=0.
        6. The calibration factor can be read from ADC_CALFACT register
        */
        adc.cr().modify(|_, w | w.adcal().set_bit());

        while adc.cr().read().adcal().bit_is_set() { }
    }

    fn adc_enable(){
        let adc = adc1();
        //Enable adc peripheral, ADEN bit in ADC control register (ADC_CR)
        adc.cr().modify(|_, w| w.aden().set_bit());
        //Wait for ADRDY bit in ADC interrupt and status register (ADC_ISR)
        while adc.isr().read().adrdy().bit_is_clear() { }
    }

    fn adc_enable_voltage_regulator(){
        let adc = adc1();
        adc.cr().modify(|_, w| w.advregen().set_bit());
        unsafe {
            for _ in 1..60 {
                asm!("nop")
            }
        }
    }

    fn adc_exit_deep_down_mode(){
        let adc = adc1();
        //Disable deep power down
        adc.cr().modify(|_, w| w.deeppwd().clear_bit());
    }

    port_gpio_configure();
    Serial::println("adc gpio configured");
    port_adc_enable_rcc_clk();
    Serial::println("adc rcc clock enabled");
    adc_exit_deep_down_mode();
    adc_enable_voltage_regulator();
    Serial::println("adc voltage regulator enabled");
    port_adc_register_configure();
    Serial::println("adc peripheral configured");
    adc_calibrate();
    Serial::println("adc calibrated");
    adc_enable();
    Serial::println("adc ready!");
}

pub fn adc1_get_value() -> u32 {
    let adc = adc1();
    //set ADSTART bit in ADC control register (ADC_CR)
    adc.cr().modify(|_, w| w.adstart().set_bit());
    //wait till adc conversion finishes, EOC: End of conversion flag in ADC interrupt and status register (ADC_ISR)
    while adc.isr().read().eoc().bit_is_clear() { };

    adc.dr().read().bits()
}

fn adc1() -> stm32::ADC1 {
    unsafe {stm32::ADC1::steal()}
}
