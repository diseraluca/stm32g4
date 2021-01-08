#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - slave mode control register"]
    pub smcr: SMCR,
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dier: DIER,
    #[doc = "0x10 - status register"]
    pub sr: SR,
    #[doc = "0x14 - event generation register"]
    pub egr: EGR,
    _reserved_6_ccmr1: [u8; 4usize],
    _reserved_7_ccmr2: [u8; 4usize],
    #[doc = "0x20 - capture/compare enable register"]
    pub ccer: CCER,
    #[doc = "0x24 - counter"]
    pub cnt: CNT,
    #[doc = "0x28 - prescaler"]
    pub psc: PSC,
    #[doc = "0x2c - auto-reload register"]
    pub arr: ARR,
    #[doc = "0x30 - repetition counter register"]
    pub rcr: RCR,
    #[doc = "0x34 - capture/compare register 1"]
    pub ccr1: CCR1,
    #[doc = "0x38 - capture/compare register 2"]
    pub ccr2: CCR2,
    #[doc = "0x3c - capture/compare register 3"]
    pub ccr3: CCR3,
    #[doc = "0x40 - capture/compare register 4"]
    pub ccr4: CCR4,
    #[doc = "0x44 - break and dead-time register"]
    pub bdtr: BDTR,
    #[doc = "0x48 - capture/compare register 4"]
    pub ccr5: CCR5,
    #[doc = "0x4c - capture/compare register 4"]
    pub ccr6: CCR6,
    #[doc = "0x50 - capture/compare mode register 2 (output mode)"]
    pub ccmr3_output: CCMR3_OUTPUT,
    #[doc = "0x54 - timer Deadtime Register 2"]
    pub dtr2: DTR2,
    #[doc = "0x58 - DMA control register"]
    pub ecr: ECR,
    #[doc = "0x5c - TIM timer input selection register"]
    pub tisel: TISEL,
    #[doc = "0x60 - TIM alternate function option register 1"]
    pub af1: AF1,
    #[doc = "0x64 - TIM alternate function option register 2"]
    pub af2: AF2,
    _reserved26: [u8; 884usize],
    #[doc = "0x3dc - control register"]
    pub dcr: DCR,
    #[doc = "0x3e0 - DMA address for full transfer"]
    pub dmar: DMAR,
}
impl RegisterBlock {
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CCMR1_INPUT) }
    }
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub fn ccmr1_input_mut(&self) -> &mut CCMR1_INPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CCMR1_INPUT) }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CCMR1_OUTPUT) }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn ccmr1_output_mut(&self) -> &mut CCMR1_OUTPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CCMR1_OUTPUT) }
    }
    #[doc = "0x1c - capture/compare mode register 2 (input mode)"]
    #[inline(always)]
    pub fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const CCMR2_INPUT) }
    }
    #[doc = "0x1c - capture/compare mode register 2 (input mode)"]
    #[inline(always)]
    pub fn ccmr2_input_mut(&self) -> &mut CCMR2_INPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut CCMR2_INPUT) }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const CCMR2_OUTPUT) }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub fn ccmr2_output_mut(&self) -> &mut CCMR2_OUTPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut CCMR2_OUTPUT) }
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "control register 1"]
pub mod cr1;
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure"]
impl crate::Writable for CR2 {}
#[doc = "control register 2"]
pub mod cr2;
#[doc = "slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcr](smcr) module"]
pub type SMCR = crate::Reg<u32, _SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCR;
#[doc = "`read()` method returns [smcr::R](smcr::R) reader structure"]
impl crate::Readable for SMCR {}
#[doc = "`write(|w| ..)` method takes [smcr::W](smcr::W) writer structure"]
impl crate::Writable for SMCR {}
#[doc = "slave mode control register"]
pub mod smcr;
#[doc = "DMA/Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dier](dier) module"]
pub type DIER = crate::Reg<u32, _DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIER;
#[doc = "`read()` method returns [dier::R](dier::R) reader structure"]
impl crate::Readable for DIER {}
#[doc = "`write(|w| ..)` method takes [dier::W](dier::W) writer structure"]
impl crate::Writable for DIER {}
#[doc = "DMA/Interrupt enable register"]
pub mod dier;
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "status register"]
pub mod sr;
#[doc = "event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [egr](egr) module"]
pub type EGR = crate::Reg<u32, _EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EGR;
#[doc = "`write(|w| ..)` method takes [egr::W](egr::W) writer structure"]
impl crate::Writable for EGR {}
#[doc = "event generation register"]
pub mod egr;
#[doc = "capture/compare mode register 1 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr1_output](ccmr1_output) module"]
pub type CCMR1_OUTPUT = crate::Reg<u32, _CCMR1_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR1_OUTPUT;
#[doc = "`read()` method returns [ccmr1_output::R](ccmr1_output::R) reader structure"]
impl crate::Readable for CCMR1_OUTPUT {}
#[doc = "`write(|w| ..)` method takes [ccmr1_output::W](ccmr1_output::W) writer structure"]
impl crate::Writable for CCMR1_OUTPUT {}
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod ccmr1_output;
#[doc = "capture/compare mode register 1 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr1_input](ccmr1_input) module"]
pub type CCMR1_INPUT = crate::Reg<u32, _CCMR1_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR1_INPUT;
#[doc = "`read()` method returns [ccmr1_input::R](ccmr1_input::R) reader structure"]
impl crate::Readable for CCMR1_INPUT {}
#[doc = "`write(|w| ..)` method takes [ccmr1_input::W](ccmr1_input::W) writer structure"]
impl crate::Writable for CCMR1_INPUT {}
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod ccmr1_input;
#[doc = "capture/compare mode register 2 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr2_output](ccmr2_output) module"]
pub type CCMR2_OUTPUT = crate::Reg<u32, _CCMR2_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR2_OUTPUT;
#[doc = "`read()` method returns [ccmr2_output::R](ccmr2_output::R) reader structure"]
impl crate::Readable for CCMR2_OUTPUT {}
#[doc = "`write(|w| ..)` method takes [ccmr2_output::W](ccmr2_output::W) writer structure"]
impl crate::Writable for CCMR2_OUTPUT {}
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr2_output;
#[doc = "capture/compare mode register 2 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr2_input](ccmr2_input) module"]
pub type CCMR2_INPUT = crate::Reg<u32, _CCMR2_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR2_INPUT;
#[doc = "`read()` method returns [ccmr2_input::R](ccmr2_input::R) reader structure"]
impl crate::Readable for CCMR2_INPUT {}
#[doc = "`write(|w| ..)` method takes [ccmr2_input::W](ccmr2_input::W) writer structure"]
impl crate::Writable for CCMR2_INPUT {}
#[doc = "capture/compare mode register 2 (input mode)"]
pub mod ccmr2_input;
#[doc = "capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccer](ccer) module"]
pub type CCER = crate::Reg<u32, _CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCER;
#[doc = "`read()` method returns [ccer::R](ccer::R) reader structure"]
impl crate::Readable for CCER {}
#[doc = "`write(|w| ..)` method takes [ccer::W](ccer::W) writer structure"]
impl crate::Writable for CCER {}
#[doc = "capture/compare enable register"]
pub mod ccer;
#[doc = "counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "counter"]
pub mod cnt;
#[doc = "prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psc](psc) module"]
pub type PSC = crate::Reg<u32, _PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSC;
#[doc = "`read()` method returns [psc::R](psc::R) reader structure"]
impl crate::Readable for PSC {}
#[doc = "`write(|w| ..)` method takes [psc::W](psc::W) writer structure"]
impl crate::Writable for PSC {}
#[doc = "prescaler"]
pub mod psc;
#[doc = "auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arr](arr) module"]
pub type ARR = crate::Reg<u32, _ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARR;
#[doc = "`read()` method returns [arr::R](arr::R) reader structure"]
impl crate::Readable for ARR {}
#[doc = "`write(|w| ..)` method takes [arr::W](arr::W) writer structure"]
impl crate::Writable for ARR {}
#[doc = "auto-reload register"]
pub mod arr;
#[doc = "repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr](rcr) module"]
pub type RCR = crate::Reg<u32, _RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR;
#[doc = "`read()` method returns [rcr::R](rcr::R) reader structure"]
impl crate::Readable for RCR {}
#[doc = "`write(|w| ..)` method takes [rcr::W](rcr::W) writer structure"]
impl crate::Writable for RCR {}
#[doc = "repetition counter register"]
pub mod rcr;
#[doc = "capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr1](ccr1) module"]
pub type CCR1 = crate::Reg<u32, _CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR1;
#[doc = "`read()` method returns [ccr1::R](ccr1::R) reader structure"]
impl crate::Readable for CCR1 {}
#[doc = "`write(|w| ..)` method takes [ccr1::W](ccr1::W) writer structure"]
impl crate::Writable for CCR1 {}
#[doc = "capture/compare register 1"]
pub mod ccr1;
#[doc = "capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr2](ccr2) module"]
pub type CCR2 = crate::Reg<u32, _CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR2;
#[doc = "`read()` method returns [ccr2::R](ccr2::R) reader structure"]
impl crate::Readable for CCR2 {}
#[doc = "`write(|w| ..)` method takes [ccr2::W](ccr2::W) writer structure"]
impl crate::Writable for CCR2 {}
#[doc = "capture/compare register 2"]
pub mod ccr2;
#[doc = "capture/compare register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr3](ccr3) module"]
pub type CCR3 = crate::Reg<u32, _CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR3;
#[doc = "`read()` method returns [ccr3::R](ccr3::R) reader structure"]
impl crate::Readable for CCR3 {}
#[doc = "`write(|w| ..)` method takes [ccr3::W](ccr3::W) writer structure"]
impl crate::Writable for CCR3 {}
#[doc = "capture/compare register 3"]
pub mod ccr3;
#[doc = "capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr4](ccr4) module"]
pub type CCR4 = crate::Reg<u32, _CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR4;
#[doc = "`read()` method returns [ccr4::R](ccr4::R) reader structure"]
impl crate::Readable for CCR4 {}
#[doc = "`write(|w| ..)` method takes [ccr4::W](ccr4::W) writer structure"]
impl crate::Writable for CCR4 {}
#[doc = "capture/compare register 4"]
pub mod ccr4;
#[doc = "break and dead-time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdtr](bdtr) module"]
pub type BDTR = crate::Reg<u32, _BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTR;
#[doc = "`read()` method returns [bdtr::R](bdtr::R) reader structure"]
impl crate::Readable for BDTR {}
#[doc = "`write(|w| ..)` method takes [bdtr::W](bdtr::W) writer structure"]
impl crate::Writable for BDTR {}
#[doc = "break and dead-time register"]
pub mod bdtr;
#[doc = "capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr5](ccr5) module"]
pub type CCR5 = crate::Reg<u32, _CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR5;
#[doc = "`read()` method returns [ccr5::R](ccr5::R) reader structure"]
impl crate::Readable for CCR5 {}
#[doc = "`write(|w| ..)` method takes [ccr5::W](ccr5::W) writer structure"]
impl crate::Writable for CCR5 {}
#[doc = "capture/compare register 4"]
pub mod ccr5;
#[doc = "capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr6](ccr6) module"]
pub type CCR6 = crate::Reg<u32, _CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR6;
#[doc = "`read()` method returns [ccr6::R](ccr6::R) reader structure"]
impl crate::Readable for CCR6 {}
#[doc = "`write(|w| ..)` method takes [ccr6::W](ccr6::W) writer structure"]
impl crate::Writable for CCR6 {}
#[doc = "capture/compare register 4"]
pub mod ccr6;
#[doc = "capture/compare mode register 2 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr3_output](ccmr3_output) module"]
pub type CCMR3_OUTPUT = crate::Reg<u32, _CCMR3_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR3_OUTPUT;
#[doc = "`read()` method returns [ccmr3_output::R](ccmr3_output::R) reader structure"]
impl crate::Readable for CCMR3_OUTPUT {}
#[doc = "`write(|w| ..)` method takes [ccmr3_output::W](ccmr3_output::W) writer structure"]
impl crate::Writable for CCMR3_OUTPUT {}
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr3_output;
#[doc = "timer Deadtime Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtr2](dtr2) module"]
pub type DTR2 = crate::Reg<u32, _DTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTR2;
#[doc = "`read()` method returns [dtr2::R](dtr2::R) reader structure"]
impl crate::Readable for DTR2 {}
#[doc = "`write(|w| ..)` method takes [dtr2::W](dtr2::W) writer structure"]
impl crate::Writable for DTR2 {}
#[doc = "timer Deadtime Register 2"]
pub mod dtr2;
#[doc = "DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](ecr) module"]
pub type ECR = crate::Reg<u32, _ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECR;
#[doc = "`read()` method returns [ecr::R](ecr::R) reader structure"]
impl crate::Readable for ECR {}
#[doc = "`write(|w| ..)` method takes [ecr::W](ecr::W) writer structure"]
impl crate::Writable for ECR {}
#[doc = "DMA control register"]
pub mod ecr;
#[doc = "TIM timer input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tisel](tisel) module"]
pub type TISEL = crate::Reg<u32, _TISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TISEL;
#[doc = "`read()` method returns [tisel::R](tisel::R) reader structure"]
impl crate::Readable for TISEL {}
#[doc = "`write(|w| ..)` method takes [tisel::W](tisel::W) writer structure"]
impl crate::Writable for TISEL {}
#[doc = "TIM timer input selection register"]
pub mod tisel;
#[doc = "TIM alternate function option register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [af1](af1) module"]
pub type AF1 = crate::Reg<u32, _AF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AF1;
#[doc = "`read()` method returns [af1::R](af1::R) reader structure"]
impl crate::Readable for AF1 {}
#[doc = "`write(|w| ..)` method takes [af1::W](af1::W) writer structure"]
impl crate::Writable for AF1 {}
#[doc = "TIM alternate function option register 1"]
pub mod af1;
#[doc = "TIM alternate function option register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [af2](af2) module"]
pub type AF2 = crate::Reg<u32, _AF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AF2;
#[doc = "`read()` method returns [af2::R](af2::R) reader structure"]
impl crate::Readable for AF2 {}
#[doc = "`write(|w| ..)` method takes [af2::W](af2::W) writer structure"]
impl crate::Writable for AF2 {}
#[doc = "TIM alternate function option register 2"]
pub mod af2;
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr](dcr) module"]
pub type DCR = crate::Reg<u32, _DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR;
#[doc = "`read()` method returns [dcr::R](dcr::R) reader structure"]
impl crate::Readable for DCR {}
#[doc = "`write(|w| ..)` method takes [dcr::W](dcr::W) writer structure"]
impl crate::Writable for DCR {}
#[doc = "control register"]
pub mod dcr;
#[doc = "DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmar](dmar) module"]
pub type DMAR = crate::Reg<u32, _DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAR;
#[doc = "`read()` method returns [dmar::R](dmar::R) reader structure"]
impl crate::Readable for DMAR {}
#[doc = "`write(|w| ..)` method takes [dmar::W](dmar::W) writer structure"]
impl crate::Writable for DMAR {}
#[doc = "DMA address for full transfer"]
pub mod dmar;
