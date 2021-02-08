///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Timerx Control Register
    pub timccr: TIMCCR,
    ///0x04 - Timerx Interrupt Status Register
    pub timcisr: TIMCISR,
    ///0x08 - Timerx Interrupt Clear Register
    pub timcicr: TIMCICR,
    ///0x0c - TIMxDIER
    pub timcdier: TIMCDIER,
    ///0x10 - Timerx Counter Register
    pub cntcr: CNTCR,
    ///0x14 - Timerx Period Register
    pub percr: PERCR,
    ///0x18 - Timerx Repetition Register
    pub repcr: REPCR,
    ///0x1c - Timerx Compare 1 Register
    pub cmp1cr: CMP1CR,
    ///0x20 - Timerx Compare 1 Compound Register
    pub cmp1ccr: CMP1CCR,
    ///0x24 - Timerx Compare 2 Register
    pub cmp2cr: CMP2CR,
    ///0x28 - Timerx Compare 3 Register
    pub cmp3cr: CMP3CR,
    ///0x2c - Timerx Compare 4 Register
    pub cmp4cr: CMP4CR,
    ///0x30 - Timerx Capture 1 Register
    pub cpt1cr: CPT1CR,
    ///0x34 - Timerx Capture 2 Register
    pub cpt2cr: CPT2CR,
    ///0x38 - Timerx Deadtime Register
    pub dtcr: DTCR,
    ///0x3c - Timerx Output1 Set Register
    pub setc1r: SETC1R,
    ///0x40 - Timerx Output1 Reset Register
    pub rstc1r: RSTC1R,
    ///0x44 - Timerx Output2 Set Register
    pub setc2r: SETC2R,
    ///0x48 - Timerx Output2 Reset Register
    pub rstc2r: RSTC2R,
    ///0x4c - Timerx External Event Filtering Register 1
    pub eefcr1: EEFCR1,
    ///0x50 - Timerx External Event Filtering Register 2
    pub eefcr2: EEFCR2,
    ///0x54 - TimerA Reset Register
    pub rstcr: RSTCR,
    ///0x58 - Timerx Chopper Register
    pub chpcr: CHPCR,
    ///0x5c - Timerx Capture 2 Control Register
    pub cpt1ccr: CPT1CCR,
    ///0x60 - CPT2xCR
    pub cpt2ccr: CPT2CCR,
    ///0x64 - Timerx Output Register
    pub outcr: OUTCR,
    ///0x68 - Timerx Fault Register
    pub fltcr: FLTCR,
    ///0x6c - HRTIM Timerx Control Register 2
    pub timccr2: TIMCCR2,
    ///0x70 - HRTIM Timerx External Event Filtering Register 3
    pub ceefr3: CEEFR3,
}
///Timerx Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timccr](timccr) module
pub type TIMCCR = crate::Reg<u32, _TIMCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCCR;
///`read()` method returns [timccr::R](timccr::R) reader structure
impl crate::Readable for TIMCCR {}
///`write(|w| ..)` method takes [timccr::W](timccr::W) writer structure
impl crate::Writable for TIMCCR {}
///Timerx Control Register
pub mod timccr;
///Timerx Interrupt Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timcisr](timcisr) module
pub type TIMCISR = crate::Reg<u32, _TIMCISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCISR;
///`read()` method returns [timcisr::R](timcisr::R) reader structure
impl crate::Readable for TIMCISR {}
///Timerx Interrupt Status Register
pub mod timcisr;
///Timerx Interrupt Clear Register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timcicr](timcicr) module
pub type TIMCICR = crate::Reg<u32, _TIMCICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCICR;
///`write(|w| ..)` method takes [timcicr::W](timcicr::W) writer structure
impl crate::Writable for TIMCICR {}
///Timerx Interrupt Clear Register
pub mod timcicr;
///TIMxDIER
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timcdier](timcdier) module
pub type TIMCDIER = crate::Reg<u32, _TIMCDIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCDIER;
///`read()` method returns [timcdier::R](timcdier::R) reader structure
impl crate::Readable for TIMCDIER {}
///`write(|w| ..)` method takes [timcdier::W](timcdier::W) writer structure
impl crate::Writable for TIMCDIER {}
///TIMxDIER
pub mod timcdier;
///Timerx Counter Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cntcr](cntcr) module
pub type CNTCR = crate::Reg<u32, _CNTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTCR;
///`read()` method returns [cntcr::R](cntcr::R) reader structure
impl crate::Readable for CNTCR {}
///`write(|w| ..)` method takes [cntcr::W](cntcr::W) writer structure
impl crate::Writable for CNTCR {}
///Timerx Counter Register
pub mod cntcr;
///Timerx Period Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [percr](percr) module
pub type PERCR = crate::Reg<u32, _PERCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERCR;
///`read()` method returns [percr::R](percr::R) reader structure
impl crate::Readable for PERCR {}
///`write(|w| ..)` method takes [percr::W](percr::W) writer structure
impl crate::Writable for PERCR {}
///Timerx Period Register
pub mod percr;
///Timerx Repetition Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [repcr](repcr) module
pub type REPCR = crate::Reg<u32, _REPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REPCR;
///`read()` method returns [repcr::R](repcr::R) reader structure
impl crate::Readable for REPCR {}
///`write(|w| ..)` method takes [repcr::W](repcr::W) writer structure
impl crate::Writable for REPCR {}
///Timerx Repetition Register
pub mod repcr;
///Timerx Compare 1 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp1cr](cmp1cr) module
pub type CMP1CR = crate::Reg<u32, _CMP1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1CR;
///`read()` method returns [cmp1cr::R](cmp1cr::R) reader structure
impl crate::Readable for CMP1CR {}
///`write(|w| ..)` method takes [cmp1cr::W](cmp1cr::W) writer structure
impl crate::Writable for CMP1CR {}
///Timerx Compare 1 Register
pub mod cmp1cr;
///Timerx Compare 1 Compound Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp1ccr](cmp1ccr) module
pub type CMP1CCR = crate::Reg<u32, _CMP1CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1CCR;
///`read()` method returns [cmp1ccr::R](cmp1ccr::R) reader structure
impl crate::Readable for CMP1CCR {}
///`write(|w| ..)` method takes [cmp1ccr::W](cmp1ccr::W) writer structure
impl crate::Writable for CMP1CCR {}
///Timerx Compare 1 Compound Register
pub mod cmp1ccr;
///Timerx Compare 2 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp2cr](cmp2cr) module
pub type CMP2CR = crate::Reg<u32, _CMP2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP2CR;
///`read()` method returns [cmp2cr::R](cmp2cr::R) reader structure
impl crate::Readable for CMP2CR {}
///`write(|w| ..)` method takes [cmp2cr::W](cmp2cr::W) writer structure
impl crate::Writable for CMP2CR {}
///Timerx Compare 2 Register
pub mod cmp2cr;
///Timerx Compare 3 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp3cr](cmp3cr) module
pub type CMP3CR = crate::Reg<u32, _CMP3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP3CR;
///`read()` method returns [cmp3cr::R](cmp3cr::R) reader structure
impl crate::Readable for CMP3CR {}
///`write(|w| ..)` method takes [cmp3cr::W](cmp3cr::W) writer structure
impl crate::Writable for CMP3CR {}
///Timerx Compare 3 Register
pub mod cmp3cr;
///Timerx Compare 4 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp4cr](cmp4cr) module
pub type CMP4CR = crate::Reg<u32, _CMP4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP4CR;
///`read()` method returns [cmp4cr::R](cmp4cr::R) reader structure
impl crate::Readable for CMP4CR {}
///`write(|w| ..)` method takes [cmp4cr::W](cmp4cr::W) writer structure
impl crate::Writable for CMP4CR {}
///Timerx Compare 4 Register
pub mod cmp4cr;
///Timerx Capture 1 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt1cr](cpt1cr) module
pub type CPT1CR = crate::Reg<u32, _CPT1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1CR;
///`read()` method returns [cpt1cr::R](cpt1cr::R) reader structure
impl crate::Readable for CPT1CR {}
///Timerx Capture 1 Register
pub mod cpt1cr;
///Timerx Capture 2 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt2cr](cpt2cr) module
pub type CPT2CR = crate::Reg<u32, _CPT2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2CR;
///`read()` method returns [cpt2cr::R](cpt2cr::R) reader structure
impl crate::Readable for CPT2CR {}
///Timerx Capture 2 Register
pub mod cpt2cr;
///Timerx Deadtime Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dtcr](dtcr) module
pub type DTCR = crate::Reg<u32, _DTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTCR;
///`read()` method returns [dtcr::R](dtcr::R) reader structure
impl crate::Readable for DTCR {}
///`write(|w| ..)` method takes [dtcr::W](dtcr::W) writer structure
impl crate::Writable for DTCR {}
///Timerx Deadtime Register
pub mod dtcr;
///Timerx Output1 Set Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [setc1r](setc1r) module
pub type SETC1R = crate::Reg<u32, _SETC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETC1R;
///`read()` method returns [setc1r::R](setc1r::R) reader structure
impl crate::Readable for SETC1R {}
///`write(|w| ..)` method takes [setc1r::W](setc1r::W) writer structure
impl crate::Writable for SETC1R {}
///Timerx Output1 Set Register
pub mod setc1r;
///Timerx Output1 Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rstc1r](rstc1r) module
pub type RSTC1R = crate::Reg<u32, _RSTC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTC1R;
///`read()` method returns [rstc1r::R](rstc1r::R) reader structure
impl crate::Readable for RSTC1R {}
///`write(|w| ..)` method takes [rstc1r::W](rstc1r::W) writer structure
impl crate::Writable for RSTC1R {}
///Timerx Output1 Reset Register
pub mod rstc1r;
///Timerx Output2 Set Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [setc2r](setc2r) module
pub type SETC2R = crate::Reg<u32, _SETC2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETC2R;
///`read()` method returns [setc2r::R](setc2r::R) reader structure
impl crate::Readable for SETC2R {}
///`write(|w| ..)` method takes [setc2r::W](setc2r::W) writer structure
impl crate::Writable for SETC2R {}
///Timerx Output2 Set Register
pub mod setc2r;
///Timerx Output2 Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rstc2r](rstc2r) module
pub type RSTC2R = crate::Reg<u32, _RSTC2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTC2R;
///`read()` method returns [rstc2r::R](rstc2r::R) reader structure
impl crate::Readable for RSTC2R {}
///`write(|w| ..)` method takes [rstc2r::W](rstc2r::W) writer structure
impl crate::Writable for RSTC2R {}
///Timerx Output2 Reset Register
pub mod rstc2r;
///Timerx External Event Filtering Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eefcr1](eefcr1) module
pub type EEFCR1 = crate::Reg<u32, _EEFCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFCR1;
///`read()` method returns [eefcr1::R](eefcr1::R) reader structure
impl crate::Readable for EEFCR1 {}
///`write(|w| ..)` method takes [eefcr1::W](eefcr1::W) writer structure
impl crate::Writable for EEFCR1 {}
///Timerx External Event Filtering Register 1
pub mod eefcr1;
///Timerx External Event Filtering Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eefcr2](eefcr2) module
pub type EEFCR2 = crate::Reg<u32, _EEFCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFCR2;
///`read()` method returns [eefcr2::R](eefcr2::R) reader structure
impl crate::Readable for EEFCR2 {}
///`write(|w| ..)` method takes [eefcr2::W](eefcr2::W) writer structure
impl crate::Writable for EEFCR2 {}
///Timerx External Event Filtering Register 2
pub mod eefcr2;
///TimerA Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rstcr](rstcr) module
pub type RSTCR = crate::Reg<u32, _RSTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCR;
///`read()` method returns [rstcr::R](rstcr::R) reader structure
impl crate::Readable for RSTCR {}
///`write(|w| ..)` method takes [rstcr::W](rstcr::W) writer structure
impl crate::Writable for RSTCR {}
///TimerA Reset Register
pub mod rstcr;
///Timerx Chopper Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chpcr](chpcr) module
pub type CHPCR = crate::Reg<u32, _CHPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPCR;
///`read()` method returns [chpcr::R](chpcr::R) reader structure
impl crate::Readable for CHPCR {}
///`write(|w| ..)` method takes [chpcr::W](chpcr::W) writer structure
impl crate::Writable for CHPCR {}
///Timerx Chopper Register
pub mod chpcr;
///Timerx Capture 2 Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt1ccr](cpt1ccr) module
pub type CPT1CCR = crate::Reg<u32, _CPT1CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1CCR;
///`read()` method returns [cpt1ccr::R](cpt1ccr::R) reader structure
impl crate::Readable for CPT1CCR {}
///`write(|w| ..)` method takes [cpt1ccr::W](cpt1ccr::W) writer structure
impl crate::Writable for CPT1CCR {}
///Timerx Capture 2 Control Register
pub mod cpt1ccr;
///CPT2xCR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt2ccr](cpt2ccr) module
pub type CPT2CCR = crate::Reg<u32, _CPT2CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2CCR;
///`read()` method returns [cpt2ccr::R](cpt2ccr::R) reader structure
impl crate::Readable for CPT2CCR {}
///`write(|w| ..)` method takes [cpt2ccr::W](cpt2ccr::W) writer structure
impl crate::Writable for CPT2CCR {}
///CPT2xCR
pub mod cpt2ccr;
///Timerx Output Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [outcr](outcr) module
pub type OUTCR = crate::Reg<u32, _OUTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTCR;
///`read()` method returns [outcr::R](outcr::R) reader structure
impl crate::Readable for OUTCR {}
///`write(|w| ..)` method takes [outcr::W](outcr::W) writer structure
impl crate::Writable for OUTCR {}
///Timerx Output Register
pub mod outcr;
///Timerx Fault Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fltcr](fltcr) module
pub type FLTCR = crate::Reg<u32, _FLTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTCR;
///`read()` method returns [fltcr::R](fltcr::R) reader structure
impl crate::Readable for FLTCR {}
///`write(|w| ..)` method takes [fltcr::W](fltcr::W) writer structure
impl crate::Writable for FLTCR {}
///Timerx Fault Register
pub mod fltcr;
///HRTIM Timerx Control Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timccr2](timccr2) module
pub type TIMCCR2 = crate::Reg<u32, _TIMCCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCCR2;
///`read()` method returns [timccr2::R](timccr2::R) reader structure
impl crate::Readable for TIMCCR2 {}
///`write(|w| ..)` method takes [timccr2::W](timccr2::W) writer structure
impl crate::Writable for TIMCCR2 {}
///HRTIM Timerx Control Register 2
pub mod timccr2;
///HRTIM Timerx External Event Filtering Register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ceefr3](ceefr3) module
pub type CEEFR3 = crate::Reg<u32, _CEEFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEEFR3;
///`read()` method returns [ceefr3::R](ceefr3::R) reader structure
impl crate::Readable for CEEFR3 {}
///`write(|w| ..)` method takes [ceefr3::W](ceefr3::W) writer structure
impl crate::Writable for CEEFR3 {}
///HRTIM Timerx External Event Filtering Register 3
pub mod ceefr3;
