///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Timerx Control Register
    pub timdcr: TIMDCR,
    ///0x04 - Timerx Interrupt Status Register
    pub timdisr: TIMDISR,
    ///0x08 - Timerx Interrupt Clear Register
    pub timdicr: TIMDICR,
    ///0x0c - TIMxDIER
    pub timddier: TIMDDIER,
    ///0x10 - Timerx Counter Register
    pub cntdr: CNTDR,
    ///0x14 - Timerx Period Register
    pub perdr: PERDR,
    ///0x18 - Timerx Repetition Register
    pub repdr: REPDR,
    ///0x1c - Timerx Compare 1 Register
    pub cmp1dr: CMP1DR,
    ///0x20 - Timerx Compare 1 Compound Register
    pub cmp1cdr: CMP1CDR,
    ///0x24 - Timerx Compare 2 Register
    pub cmp2dr: CMP2DR,
    ///0x28 - Timerx Compare 3 Register
    pub cmp3dr: CMP3DR,
    ///0x2c - Timerx Compare 4 Register
    pub cmp4dr: CMP4DR,
    ///0x30 - Timerx Capture 1 Register
    pub cpt1dr: CPT1DR,
    ///0x34 - Timerx Capture 2 Register
    pub cpt2dr: CPT2DR,
    ///0x38 - Timerx Deadtime Register
    pub dtdr: DTDR,
    ///0x3c - Timerx Output1 Set Register
    pub setd1r: SETD1R,
    ///0x40 - Timerx Output1 Reset Register
    pub rstd1r: RSTD1R,
    ///0x44 - Timerx Output2 Set Register
    pub setd2r: SETD2R,
    ///0x48 - Timerx Output2 Reset Register
    pub rstd2r: RSTD2R,
    ///0x4c - Timerx External Event Filtering Register 1
    pub eefdr1: EEFDR1,
    ///0x50 - Timerx External Event Filtering Register 2
    pub eefdr2: EEFDR2,
    ///0x54 - TimerA Reset Register
    pub rstdr: RSTDR,
    ///0x58 - Timerx Chopper Register
    pub chpdr: CHPDR,
    ///0x5c - Timerx Capture 2 Control Register
    pub cpt1dcr: CPT1DCR,
    ///0x60 - CPT2xCR
    pub cpt2dcr: CPT2DCR,
    ///0x64 - Timerx Output Register
    pub outdr: OUTDR,
    ///0x68 - Timerx Fault Register
    pub fltdr: FLTDR,
    ///0x6c - HRTIM Timerx Control Register 2
    pub timdcr2: TIMDCR2,
    ///0x70 - HRTIM Timerx External Event Filtering Register 3
    pub deefr3: DEEFR3,
}
///Timerx Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timdcr](timdcr) module
pub type TIMDCR = crate::Reg<u32, _TIMDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMDCR;
///`read()` method returns [timdcr::R](timdcr::R) reader structure
impl crate::Readable for TIMDCR {}
///`write(|w| ..)` method takes [timdcr::W](timdcr::W) writer structure
impl crate::Writable for TIMDCR {}
///Timerx Control Register
pub mod timdcr;
///Timerx Interrupt Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timdisr](timdisr) module
pub type TIMDISR = crate::Reg<u32, _TIMDISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMDISR;
///`read()` method returns [timdisr::R](timdisr::R) reader structure
impl crate::Readable for TIMDISR {}
///Timerx Interrupt Status Register
pub mod timdisr;
///Timerx Interrupt Clear Register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timdicr](timdicr) module
pub type TIMDICR = crate::Reg<u32, _TIMDICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMDICR;
///`write(|w| ..)` method takes [timdicr::W](timdicr::W) writer structure
impl crate::Writable for TIMDICR {}
///Timerx Interrupt Clear Register
pub mod timdicr;
///TIMxDIER
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timddier](timddier) module
pub type TIMDDIER = crate::Reg<u32, _TIMDDIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMDDIER;
///`read()` method returns [timddier::R](timddier::R) reader structure
impl crate::Readable for TIMDDIER {}
///`write(|w| ..)` method takes [timddier::W](timddier::W) writer structure
impl crate::Writable for TIMDDIER {}
///TIMxDIER
pub mod timddier;
///Timerx Counter Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cntdr](cntdr) module
pub type CNTDR = crate::Reg<u32, _CNTDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTDR;
///`read()` method returns [cntdr::R](cntdr::R) reader structure
impl crate::Readable for CNTDR {}
///`write(|w| ..)` method takes [cntdr::W](cntdr::W) writer structure
impl crate::Writable for CNTDR {}
///Timerx Counter Register
pub mod cntdr;
///Timerx Period Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [perdr](perdr) module
pub type PERDR = crate::Reg<u32, _PERDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERDR;
///`read()` method returns [perdr::R](perdr::R) reader structure
impl crate::Readable for PERDR {}
///`write(|w| ..)` method takes [perdr::W](perdr::W) writer structure
impl crate::Writable for PERDR {}
///Timerx Period Register
pub mod perdr;
///Timerx Repetition Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [repdr](repdr) module
pub type REPDR = crate::Reg<u32, _REPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REPDR;
///`read()` method returns [repdr::R](repdr::R) reader structure
impl crate::Readable for REPDR {}
///`write(|w| ..)` method takes [repdr::W](repdr::W) writer structure
impl crate::Writable for REPDR {}
///Timerx Repetition Register
pub mod repdr;
///Timerx Compare 1 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp1dr](cmp1dr) module
pub type CMP1DR = crate::Reg<u32, _CMP1DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1DR;
///`read()` method returns [cmp1dr::R](cmp1dr::R) reader structure
impl crate::Readable for CMP1DR {}
///`write(|w| ..)` method takes [cmp1dr::W](cmp1dr::W) writer structure
impl crate::Writable for CMP1DR {}
///Timerx Compare 1 Register
pub mod cmp1dr;
///Timerx Compare 1 Compound Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp1cdr](cmp1cdr) module
pub type CMP1CDR = crate::Reg<u32, _CMP1CDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1CDR;
///`read()` method returns [cmp1cdr::R](cmp1cdr::R) reader structure
impl crate::Readable for CMP1CDR {}
///`write(|w| ..)` method takes [cmp1cdr::W](cmp1cdr::W) writer structure
impl crate::Writable for CMP1CDR {}
///Timerx Compare 1 Compound Register
pub mod cmp1cdr;
///Timerx Compare 2 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp2dr](cmp2dr) module
pub type CMP2DR = crate::Reg<u32, _CMP2DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP2DR;
///`read()` method returns [cmp2dr::R](cmp2dr::R) reader structure
impl crate::Readable for CMP2DR {}
///`write(|w| ..)` method takes [cmp2dr::W](cmp2dr::W) writer structure
impl crate::Writable for CMP2DR {}
///Timerx Compare 2 Register
pub mod cmp2dr;
///Timerx Compare 3 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp3dr](cmp3dr) module
pub type CMP3DR = crate::Reg<u32, _CMP3DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP3DR;
///`read()` method returns [cmp3dr::R](cmp3dr::R) reader structure
impl crate::Readable for CMP3DR {}
///`write(|w| ..)` method takes [cmp3dr::W](cmp3dr::W) writer structure
impl crate::Writable for CMP3DR {}
///Timerx Compare 3 Register
pub mod cmp3dr;
///Timerx Compare 4 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp4dr](cmp4dr) module
pub type CMP4DR = crate::Reg<u32, _CMP4DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP4DR;
///`read()` method returns [cmp4dr::R](cmp4dr::R) reader structure
impl crate::Readable for CMP4DR {}
///`write(|w| ..)` method takes [cmp4dr::W](cmp4dr::W) writer structure
impl crate::Writable for CMP4DR {}
///Timerx Compare 4 Register
pub mod cmp4dr;
///Timerx Capture 1 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt1dr](cpt1dr) module
pub type CPT1DR = crate::Reg<u32, _CPT1DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1DR;
///`read()` method returns [cpt1dr::R](cpt1dr::R) reader structure
impl crate::Readable for CPT1DR {}
///Timerx Capture 1 Register
pub mod cpt1dr;
///Timerx Capture 2 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt2dr](cpt2dr) module
pub type CPT2DR = crate::Reg<u32, _CPT2DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2DR;
///`read()` method returns [cpt2dr::R](cpt2dr::R) reader structure
impl crate::Readable for CPT2DR {}
///Timerx Capture 2 Register
pub mod cpt2dr;
///Timerx Deadtime Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dtdr](dtdr) module
pub type DTDR = crate::Reg<u32, _DTDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTDR;
///`read()` method returns [dtdr::R](dtdr::R) reader structure
impl crate::Readable for DTDR {}
///`write(|w| ..)` method takes [dtdr::W](dtdr::W) writer structure
impl crate::Writable for DTDR {}
///Timerx Deadtime Register
pub mod dtdr;
///Timerx Output1 Set Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [setd1r](setd1r) module
pub type SETD1R = crate::Reg<u32, _SETD1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETD1R;
///`read()` method returns [setd1r::R](setd1r::R) reader structure
impl crate::Readable for SETD1R {}
///`write(|w| ..)` method takes [setd1r::W](setd1r::W) writer structure
impl crate::Writable for SETD1R {}
///Timerx Output1 Set Register
pub mod setd1r;
///Timerx Output1 Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rstd1r](rstd1r) module
pub type RSTD1R = crate::Reg<u32, _RSTD1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTD1R;
///`read()` method returns [rstd1r::R](rstd1r::R) reader structure
impl crate::Readable for RSTD1R {}
///`write(|w| ..)` method takes [rstd1r::W](rstd1r::W) writer structure
impl crate::Writable for RSTD1R {}
///Timerx Output1 Reset Register
pub mod rstd1r;
///Timerx Output2 Set Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [setd2r](setd2r) module
pub type SETD2R = crate::Reg<u32, _SETD2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETD2R;
///`read()` method returns [setd2r::R](setd2r::R) reader structure
impl crate::Readable for SETD2R {}
///`write(|w| ..)` method takes [setd2r::W](setd2r::W) writer structure
impl crate::Writable for SETD2R {}
///Timerx Output2 Set Register
pub mod setd2r;
///Timerx Output2 Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rstd2r](rstd2r) module
pub type RSTD2R = crate::Reg<u32, _RSTD2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTD2R;
///`read()` method returns [rstd2r::R](rstd2r::R) reader structure
impl crate::Readable for RSTD2R {}
///`write(|w| ..)` method takes [rstd2r::W](rstd2r::W) writer structure
impl crate::Writable for RSTD2R {}
///Timerx Output2 Reset Register
pub mod rstd2r;
///Timerx External Event Filtering Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eefdr1](eefdr1) module
pub type EEFDR1 = crate::Reg<u32, _EEFDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFDR1;
///`read()` method returns [eefdr1::R](eefdr1::R) reader structure
impl crate::Readable for EEFDR1 {}
///`write(|w| ..)` method takes [eefdr1::W](eefdr1::W) writer structure
impl crate::Writable for EEFDR1 {}
///Timerx External Event Filtering Register 1
pub mod eefdr1;
///Timerx External Event Filtering Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eefdr2](eefdr2) module
pub type EEFDR2 = crate::Reg<u32, _EEFDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFDR2;
///`read()` method returns [eefdr2::R](eefdr2::R) reader structure
impl crate::Readable for EEFDR2 {}
///`write(|w| ..)` method takes [eefdr2::W](eefdr2::W) writer structure
impl crate::Writable for EEFDR2 {}
///Timerx External Event Filtering Register 2
pub mod eefdr2;
///TimerA Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rstdr](rstdr) module
pub type RSTDR = crate::Reg<u32, _RSTDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTDR;
///`read()` method returns [rstdr::R](rstdr::R) reader structure
impl crate::Readable for RSTDR {}
///`write(|w| ..)` method takes [rstdr::W](rstdr::W) writer structure
impl crate::Writable for RSTDR {}
///TimerA Reset Register
pub mod rstdr;
///Timerx Chopper Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chpdr](chpdr) module
pub type CHPDR = crate::Reg<u32, _CHPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPDR;
///`read()` method returns [chpdr::R](chpdr::R) reader structure
impl crate::Readable for CHPDR {}
///`write(|w| ..)` method takes [chpdr::W](chpdr::W) writer structure
impl crate::Writable for CHPDR {}
///Timerx Chopper Register
pub mod chpdr;
///Timerx Capture 2 Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt1dcr](cpt1dcr) module
pub type CPT1DCR = crate::Reg<u32, _CPT1DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1DCR;
///`read()` method returns [cpt1dcr::R](cpt1dcr::R) reader structure
impl crate::Readable for CPT1DCR {}
///`write(|w| ..)` method takes [cpt1dcr::W](cpt1dcr::W) writer structure
impl crate::Writable for CPT1DCR {}
///Timerx Capture 2 Control Register
pub mod cpt1dcr;
///CPT2xCR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt2dcr](cpt2dcr) module
pub type CPT2DCR = crate::Reg<u32, _CPT2DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2DCR;
///`read()` method returns [cpt2dcr::R](cpt2dcr::R) reader structure
impl crate::Readable for CPT2DCR {}
///`write(|w| ..)` method takes [cpt2dcr::W](cpt2dcr::W) writer structure
impl crate::Writable for CPT2DCR {}
///CPT2xCR
pub mod cpt2dcr;
///Timerx Output Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [outdr](outdr) module
pub type OUTDR = crate::Reg<u32, _OUTDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTDR;
///`read()` method returns [outdr::R](outdr::R) reader structure
impl crate::Readable for OUTDR {}
///`write(|w| ..)` method takes [outdr::W](outdr::W) writer structure
impl crate::Writable for OUTDR {}
///Timerx Output Register
pub mod outdr;
///Timerx Fault Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fltdr](fltdr) module
pub type FLTDR = crate::Reg<u32, _FLTDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTDR;
///`read()` method returns [fltdr::R](fltdr::R) reader structure
impl crate::Readable for FLTDR {}
///`write(|w| ..)` method takes [fltdr::W](fltdr::W) writer structure
impl crate::Writable for FLTDR {}
///Timerx Fault Register
pub mod fltdr;
///HRTIM Timerx Control Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timdcr2](timdcr2) module
pub type TIMDCR2 = crate::Reg<u32, _TIMDCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMDCR2;
///`read()` method returns [timdcr2::R](timdcr2::R) reader structure
impl crate::Readable for TIMDCR2 {}
///`write(|w| ..)` method takes [timdcr2::W](timdcr2::W) writer structure
impl crate::Writable for TIMDCR2 {}
///HRTIM Timerx Control Register 2
pub mod timdcr2;
///HRTIM Timerx External Event Filtering Register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [deefr3](deefr3) module
pub type DEEFR3 = crate::Reg<u32, _DEEFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEEFR3;
///`read()` method returns [deefr3::R](deefr3::R) reader structure
impl crate::Readable for DEEFR3 {}
///`write(|w| ..)` method takes [deefr3::W](deefr3::W) writer structure
impl crate::Writable for DEEFR3 {}
///HRTIM Timerx External Event Filtering Register 3
pub mod deefr3;
