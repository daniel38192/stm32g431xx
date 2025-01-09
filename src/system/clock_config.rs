
pub const PCLK1_FREQ: u32 = 64_000_000;

pub fn system_clock_config(){

    /* Settings */
    /* SYSCLK: 64MHz
     * HCLK: 64MHz
     * ABP1, APB2, APB3: 64MHz
     * Sys Clk Source: PLLCLK
     * HSE: 8MHz
     */

    unsafe {
        clocks::enable_hse();
        flash::configure();
        pll1::configure();
        pll1::enable_p();
        pll1::enable_main();
        pll1::switch_sys_clock_to()
    }
}

mod clocks {
    use stm32h5::stm32h562 as stm32;

    pub unsafe fn enable_hse(){
        let rcc = stm32::RCC::steal();

        // 1.Enable HSE Oscilator
        unsafe{
            *rcc.cr().as_ptr() |= 1 << 16;
        }

        // 2.Wait for HSE Oscilator ready state
        while rcc.cr().read().hserdy().bit_is_clear() {
        }
    }
}

mod flash {
    use stm32h5::stm32h562 as stm32;

    pub unsafe fn configure(){
        let flash = stm32::FLASH::steal();
        //Set flash read lattency, FLASH_ACR, Bits 3:0 LATENCY[3:0]
        *flash.acr().as_ptr() &= !(0xF << 0);
        *flash.acr().as_ptr() |= 0x3 << 0;
    }
}

mod pll1 {
    use stm32h5::stm32h562 as stm32;

    pub unsafe fn configure(){
        let rcc = stm32::RCC::steal();

        *rcc.pll1cfgr().as_ptr() = 0;

        //Set Clock source for PLL1 (HSE), RCC_PLL1CFGR, bits 1:0 PLL1SRC[1:0]
        /*
        00: no clock send to PLL1M divider and PLLs (default after reset).
        01: HSI selected as PLL clock (hsi_ck)
        10: CSI selected as PLL clock (csi_ck)
        11: HSE selected as PLL clock (hse_ck)
         */
        *rcc.pll1cfgr().as_ptr() |= 0x3 << 0;

        //Enable fractional latch for PLL1, RCC_PLL1CFGR, bit 4 PLL1FRACEN
        *rcc.pll1cfgr().as_ptr() |= 1 << 4;

        //Set Input frequency range for PLL1, RCC_PLL1CFGR, bits 3:2 PLL1RGE[1:0]
        /*
        Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator.
        To latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0,
        then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator.
         */
        *rcc.pll1cfgr().as_ptr() |= 0x3 << 2;


        //Set prescaler PLL1M, RCC_PLL1CFGR, bits 13:8 PLL1M[5:0]
        //000001: division by 1 (bypass)
        *rcc.pll1cfgr().as_ptr() |= 1 << 8;

        //Set Multiplication Factor for PLL1 (PLL1N), RCC_PLL1DIVR, bits 8:0 PLL1N[8:0]
        *rcc.pll1divr().as_ptr() &= !(0x1FF << 0);
        *rcc.pll1divr().as_ptr() |= 0xF << 0;

        //Set PLL1P divisor factor, RCC_PLL1DIVR, bits 15:9 PLL1P[6:0]
        //0000001: pll1_p_ck = vco1_ck / 2 (default after reset)
        *rcc.pll1divr().as_ptr() &= !(0x7F << 9);
        *rcc.pll1divr().as_ptr() |= 1 << 9;

    }

    pub fn enable_main(){
        let rcc;
        unsafe {rcc = stm32::RCC::steal()}
        //Enable PLL1, RCC_CR, bit 24 PLL1ON
        rcc.cr().modify(|_, w| w.pll1on().set_bit());
        //Wait for PLL1 clock ready flag, RCC_CR, Bit 25 PLL1RDY
        while rcc.cr().read().pll1rdy().bit_is_clear() {}
    }

    pub fn enable_p(){
        let rcc;
        unsafe {rcc = stm32::RCC::steal()}
        //Enable PLL1P output, RCC_PLL1CFGR, bit 16 PLL1PEN
        /*
        Set and reset by software to enable the pll1_p_ck output of the PLL1.
        This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
        To save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be
        disabled.
        0: pll1_p_ck output disabled (default after reset)
        1: pll1_p_ck output enabled
         */
        rcc.pll1cfgr().modify(|_, w| w.pll1pen().set_bit());
    }

    pub fn enable_q(){
        let rcc;
        unsafe {rcc = stm32::RCC::steal()}
        rcc.pll1cfgr().modify(|_, w| w.pll1qen().set_bit());
    }

    pub fn switch_sys_clock_to(){
        let rcc;
        unsafe {rcc = stm32::RCC::steal()}
        //Switch clock source, RCC_CFGR1, bits 1:0 SW[1:0]
        /*
        Set and reset by software to select system clock and trace clock sources (sys_ck).
        Set by hardware to force the selection of the HSI or CSI (depending on STOPWUCK
        selection) when leaving a system Stop mode, and to force the selection of the HSI in case of
        failure of the HSE when used directly or indirectly as system clock
        00: HSI selected as system clock (hsi_ck) (default after reset)
        01: CSI selected as system clock (csi_ck)
        10: HSE selected as system clock (hse_ck)
        11: PLL1 selected as system clock (pll1_p_ck for sys_ck)
        Others: reserved
         */
        rcc.cfgr1().modify(|_, w| w.sw().pll1());

        //Wait for system clock to be selected, RCC_CFGR1, bits 4:3 SWS[1:0]
        /*
        Set and reset by hardware to indicate which clock source is used as system clock. 000: HSI
        used as system clock (hsi_ck) (default after reset).
        01: CSI used as system clock (csi_ck)
        10: HSE used as system clock (hse_ck)
        11: PLL1 used as system clock (pll1_p_ck)
        Others: reserved
         */
        while !(rcc.cr().read().bits() & (0x3 << 3) > 0) { }
    }
}


