///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: CR1,
    ///0x04 - control register 2
    pub cr2: CR2,
    ///0x08 - slave mode control register
    pub smcr: SMCR,
    ///0x0c - DMA/Interrupt enable register
    pub dier: DIER,
    ///0x10 - status register
    pub sr: SR,
    ///0x14 - event generation register
    pub egr: EGR,
    _reserved_6_ccmr1: [u8; 4usize],
    _reserved7: [u8; 4usize],
    ///0x20 - capture/compare enable register
    pub ccer: CCER,
    ///0x24 - counter
    pub cnt: CNT,
    ///0x28 - prescaler
    pub psc: PSC,
    ///0x2c - auto-reload register
    pub arr: ARR,
    ///0x30 - repetition counter register
    pub rcr: RCR,
    ///0x34 - capture/compare register 1
    pub ccr1: CCR1,
    ///0x38 - capture/compare register 2
    pub ccr2: CCR2,
    _reserved14: [u8; 8usize],
    ///0x44 - break and dead-time register
    pub bdtr: BDTR,
    _reserved15: [u8; 12usize],
    ///0x54 - timer Deadtime Register 2
    pub dtr2: DTR2,
    _reserved16: [u8; 4usize],
    ///0x5c - TIM timer input selection register
    pub tisel: TISEL,
    ///0x60 - TIM alternate function option register 1
    pub af1: AF1,
    ///0x64 - TIM alternate function option register 2
    pub af2: AF2,
    _reserved19: [u8; 884usize],
    ///0x3dc - DMA control register
    pub dcr: DCR,
    ///0x3e0 - DMA address for full transfer
    pub dmar: DMAR,
}
impl RegisterBlock {
    ///0x18 - capture/compare mode register 1 (input mode)
    #[inline(always)]
    pub fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CCMR1_INPUT) }
    }
    ///0x18 - capture/compare mode register 1 (input mode)
    #[inline(always)]
    pub fn ccmr1_input_mut(&self) -> &mut CCMR1_INPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CCMR1_INPUT) }
    }
    ///0x18 - capture/compare mode register (output mode)
    #[inline(always)]
    pub fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CCMR1_OUTPUT) }
    }
    ///0x18 - capture/compare mode register (output mode)
    #[inline(always)]
    pub fn ccmr1_output_mut(&self) -> &mut CCMR1_OUTPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CCMR1_OUTPUT) }
    }
}
///control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](cr1) module
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
///`read()` method returns [cr1::R](cr1::R) reader structure
impl crate::Readable for CR1 {}
///`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure
impl crate::Writable for CR1 {}
///control register 1
pub mod cr1;
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](cr2) module
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
///`read()` method returns [cr2::R](cr2::R) reader structure
impl crate::Readable for CR2 {}
///`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure
impl crate::Writable for CR2 {}
///control register 2
pub mod cr2;
///slave mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smcr](smcr) module
pub type SMCR = crate::Reg<u32, _SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCR;
///`read()` method returns [smcr::R](smcr::R) reader structure
impl crate::Readable for SMCR {}
///`write(|w| ..)` method takes [smcr::W](smcr::W) writer structure
impl crate::Writable for SMCR {}
///slave mode control register
pub mod smcr;
///DMA/Interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dier](dier) module
pub type DIER = crate::Reg<u32, _DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIER;
///`read()` method returns [dier::R](dier::R) reader structure
impl crate::Readable for DIER {}
///`write(|w| ..)` method takes [dier::W](dier::W) writer structure
impl crate::Writable for DIER {}
///DMA/Interrupt enable register
pub mod dier;
///status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](sr) module
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
///`read()` method returns [sr::R](sr::R) reader structure
impl crate::Readable for SR {}
///`write(|w| ..)` method takes [sr::W](sr::W) writer structure
impl crate::Writable for SR {}
///status register
pub mod sr;
///event generation register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [egr](egr) module
pub type EGR = crate::Reg<u32, _EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EGR;
///`write(|w| ..)` method takes [egr::W](egr::W) writer structure
impl crate::Writable for EGR {}
///event generation register
pub mod egr;
///capture/compare mode register (output mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr1_output](ccmr1_output) module
pub type CCMR1_OUTPUT = crate::Reg<u32, _CCMR1_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR1_OUTPUT;
///`read()` method returns [ccmr1_output::R](ccmr1_output::R) reader structure
impl crate::Readable for CCMR1_OUTPUT {}
///`write(|w| ..)` method takes [ccmr1_output::W](ccmr1_output::W) writer structure
impl crate::Writable for CCMR1_OUTPUT {}
///capture/compare mode register (output mode)
pub mod ccmr1_output;
///capture/compare mode register 1 (input mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr1_input](ccmr1_input) module
pub type CCMR1_INPUT = crate::Reg<u32, _CCMR1_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR1_INPUT;
///`read()` method returns [ccmr1_input::R](ccmr1_input::R) reader structure
impl crate::Readable for CCMR1_INPUT {}
///`write(|w| ..)` method takes [ccmr1_input::W](ccmr1_input::W) writer structure
impl crate::Writable for CCMR1_INPUT {}
///capture/compare mode register 1 (input mode)
pub mod ccmr1_input;
///capture/compare enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccer](ccer) module
pub type CCER = crate::Reg<u32, _CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCER;
///`read()` method returns [ccer::R](ccer::R) reader structure
impl crate::Readable for CCER {}
///`write(|w| ..)` method takes [ccer::W](ccer::W) writer structure
impl crate::Writable for CCER {}
///capture/compare enable register
pub mod ccer;
///counter
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cnt](cnt) module
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
///`read()` method returns [cnt::R](cnt::R) reader structure
impl crate::Readable for CNT {}
///`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure
impl crate::Writable for CNT {}
///counter
pub mod cnt;
///prescaler
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [psc](psc) module
pub type PSC = crate::Reg<u32, _PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSC;
///`read()` method returns [psc::R](psc::R) reader structure
impl crate::Readable for PSC {}
///`write(|w| ..)` method takes [psc::W](psc::W) writer structure
impl crate::Writable for PSC {}
///prescaler
pub mod psc;
///auto-reload register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [arr](arr) module
pub type ARR = crate::Reg<u32, _ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARR;
///`read()` method returns [arr::R](arr::R) reader structure
impl crate::Readable for ARR {}
///`write(|w| ..)` method takes [arr::W](arr::W) writer structure
impl crate::Writable for ARR {}
///auto-reload register
pub mod arr;
///repetition counter register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcr](rcr) module
pub type RCR = crate::Reg<u32, _RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR;
///`read()` method returns [rcr::R](rcr::R) reader structure
impl crate::Readable for RCR {}
///`write(|w| ..)` method takes [rcr::W](rcr::W) writer structure
impl crate::Writable for RCR {}
///repetition counter register
pub mod rcr;
///capture/compare register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr1](ccr1) module
pub type CCR1 = crate::Reg<u32, _CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR1;
///`read()` method returns [ccr1::R](ccr1::R) reader structure
impl crate::Readable for CCR1 {}
///`write(|w| ..)` method takes [ccr1::W](ccr1::W) writer structure
impl crate::Writable for CCR1 {}
///capture/compare register 1
pub mod ccr1;
///capture/compare register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr2](ccr2) module
pub type CCR2 = crate::Reg<u32, _CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR2;
///`read()` method returns [ccr2::R](ccr2::R) reader structure
impl crate::Readable for CCR2 {}
///`write(|w| ..)` method takes [ccr2::W](ccr2::W) writer structure
impl crate::Writable for CCR2 {}
///capture/compare register 2
pub mod ccr2;
///break and dead-time register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdtr](bdtr) module
pub type BDTR = crate::Reg<u32, _BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTR;
///`read()` method returns [bdtr::R](bdtr::R) reader structure
impl crate::Readable for BDTR {}
///`write(|w| ..)` method takes [bdtr::W](bdtr::W) writer structure
impl crate::Writable for BDTR {}
///break and dead-time register
pub mod bdtr;
///timer Deadtime Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dtr2](dtr2) module
pub type DTR2 = crate::Reg<u32, _DTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTR2;
///`read()` method returns [dtr2::R](dtr2::R) reader structure
impl crate::Readable for DTR2 {}
///`write(|w| ..)` method takes [dtr2::W](dtr2::W) writer structure
impl crate::Writable for DTR2 {}
///timer Deadtime Register 2
pub mod dtr2;
///TIM timer input selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tisel](tisel) module
pub type TISEL = crate::Reg<u32, _TISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TISEL;
///`read()` method returns [tisel::R](tisel::R) reader structure
impl crate::Readable for TISEL {}
///`write(|w| ..)` method takes [tisel::W](tisel::W) writer structure
impl crate::Writable for TISEL {}
///TIM timer input selection register
pub mod tisel;
///TIM alternate function option register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [af1](af1) module
pub type AF1 = crate::Reg<u32, _AF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AF1;
///`read()` method returns [af1::R](af1::R) reader structure
impl crate::Readable for AF1 {}
///`write(|w| ..)` method takes [af1::W](af1::W) writer structure
impl crate::Writable for AF1 {}
///TIM alternate function option register 1
pub mod af1;
///TIM alternate function option register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [af2](af2) module
pub type AF2 = crate::Reg<u32, _AF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AF2;
///`read()` method returns [af2::R](af2::R) reader structure
impl crate::Readable for AF2 {}
///`write(|w| ..)` method takes [af2::W](af2::W) writer structure
impl crate::Writable for AF2 {}
///TIM alternate function option register 2
pub mod af2;
///DMA control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcr](dcr) module
pub type DCR = crate::Reg<u32, _DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR;
///`read()` method returns [dcr::R](dcr::R) reader structure
impl crate::Readable for DCR {}
///`write(|w| ..)` method takes [dcr::W](dcr::W) writer structure
impl crate::Writable for DCR {}
///DMA control register
pub mod dcr;
///DMA address for full transfer
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmar](dmar) module
pub type DMAR = crate::Reg<u32, _DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAR;
///`read()` method returns [dmar::R](dmar::R) reader structure
impl crate::Readable for DMAR {}
///`write(|w| ..)` method takes [dmar::W](dmar::W) writer structure
impl crate::Writable for DMAR {}
///DMA address for full transfer
pub mod dmar;
