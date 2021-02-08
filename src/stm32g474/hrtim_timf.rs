///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Timerx Control Register
    pub timfcr: TIMFCR,
    ///0x04 - Timerx Interrupt Status Register
    pub timfisr: TIMFISR,
    ///0x08 - Timerx Interrupt Clear Register
    pub timficr: TIMFICR,
    ///0x0c - TIMxDIER
    pub timfdier: TIMFDIER,
    ///0x10 - Timerx Counter Register
    pub cntfr: CNTFR,
    ///0x14 - Timerx Period Register
    pub perfr: PERFR,
    ///0x18 - Timerx Repetition Register
    pub repfr: REPFR,
    ///0x1c - Timerx Compare 1 Register
    pub cmp1fr: CMP1FR,
    ///0x20 - Timerx Compare 1 Compound Register
    pub cmp1cfr: CMP1CFR,
    ///0x24 - Timerx Compare 2 Register
    pub cmp2fr: CMP2FR,
    ///0x28 - Timerx Compare 3 Register
    pub cmp3fr: CMP3FR,
    ///0x2c - Timerx Compare 4 Register
    pub cmp4fr: CMP4FR,
    ///0x30 - Timerx Capture 1 Register
    pub cpt1fr: CPT1FR,
    ///0x34 - Timerx Capture 2 Register
    pub cpt2fr: CPT2FR,
    ///0x38 - Timerx Deadtime Register
    pub dtfr: DTFR,
    ///0x3c - Timerx Output1 Set Register
    pub setf1r: SETF1R,
    ///0x40 - Timerx Output1 Reset Register
    pub rste1r: RSTE1R,
    ///0x44 - Timerx Output2 Set Register
    pub setf2r: SETF2R,
    ///0x48 - Timerx Output2 Reset Register
    pub rstf2r: RSTF2R,
    ///0x4c - Timerx External Event Filtering Register 1
    pub eeffr1: EEFFR1,
    ///0x50 - Timerx External Event Filtering Register 2
    pub eeffr2: EEFFR2,
    ///0x54 - TimerA Reset Register
    pub rstfr: RSTFR,
    ///0x58 - Timerx Chopper Register
    pub chpfr: CHPFR,
    ///0x5c - Timerx Capture 2 Control Register
    pub cpt1fcr: CPT1FCR,
    ///0x60 - CPT2xCR
    pub cpt2fcr: CPT2FCR,
    ///0x64 - Timerx Output Register
    pub outfr: OUTFR,
    ///0x68 - Timerx Fault Register
    pub fltfr: FLTFR,
    ///0x6c - HRTIM Timerx Control Register 2
    pub timfcr2: TIMFCR2,
    ///0x70 - HRTIM Timerx External Event Filtering Register 3
    pub feefr3: FEEFR3,
}
///Timerx Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timfcr](timfcr) module
pub type TIMFCR = crate::Reg<u32, _TIMFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMFCR;
///`read()` method returns [timfcr::R](timfcr::R) reader structure
impl crate::Readable for TIMFCR {}
///`write(|w| ..)` method takes [timfcr::W](timfcr::W) writer structure
impl crate::Writable for TIMFCR {}
///Timerx Control Register
pub mod timfcr;
///Timerx Interrupt Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timfisr](timfisr) module
pub type TIMFISR = crate::Reg<u32, _TIMFISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMFISR;
///`read()` method returns [timfisr::R](timfisr::R) reader structure
impl crate::Readable for TIMFISR {}
///Timerx Interrupt Status Register
pub mod timfisr;
///Timerx Interrupt Clear Register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timficr](timficr) module
pub type TIMFICR = crate::Reg<u32, _TIMFICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMFICR;
///`write(|w| ..)` method takes [timficr::W](timficr::W) writer structure
impl crate::Writable for TIMFICR {}
///Timerx Interrupt Clear Register
pub mod timficr;
///TIMxDIER
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timfdier](timfdier) module
pub type TIMFDIER = crate::Reg<u32, _TIMFDIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMFDIER;
///`read()` method returns [timfdier::R](timfdier::R) reader structure
impl crate::Readable for TIMFDIER {}
///`write(|w| ..)` method takes [timfdier::W](timfdier::W) writer structure
impl crate::Writable for TIMFDIER {}
///TIMxDIER
pub mod timfdier;
///Timerx Counter Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cntfr](cntfr) module
pub type CNTFR = crate::Reg<u32, _CNTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTFR;
///`read()` method returns [cntfr::R](cntfr::R) reader structure
impl crate::Readable for CNTFR {}
///`write(|w| ..)` method takes [cntfr::W](cntfr::W) writer structure
impl crate::Writable for CNTFR {}
///Timerx Counter Register
pub mod cntfr;
///Timerx Period Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [perfr](perfr) module
pub type PERFR = crate::Reg<u32, _PERFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERFR;
///`read()` method returns [perfr::R](perfr::R) reader structure
impl crate::Readable for PERFR {}
///`write(|w| ..)` method takes [perfr::W](perfr::W) writer structure
impl crate::Writable for PERFR {}
///Timerx Period Register
pub mod perfr;
///Timerx Repetition Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [repfr](repfr) module
pub type REPFR = crate::Reg<u32, _REPFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REPFR;
///`read()` method returns [repfr::R](repfr::R) reader structure
impl crate::Readable for REPFR {}
///`write(|w| ..)` method takes [repfr::W](repfr::W) writer structure
impl crate::Writable for REPFR {}
///Timerx Repetition Register
pub mod repfr;
///Timerx Compare 1 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp1fr](cmp1fr) module
pub type CMP1FR = crate::Reg<u32, _CMP1FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1FR;
///`read()` method returns [cmp1fr::R](cmp1fr::R) reader structure
impl crate::Readable for CMP1FR {}
///`write(|w| ..)` method takes [cmp1fr::W](cmp1fr::W) writer structure
impl crate::Writable for CMP1FR {}
///Timerx Compare 1 Register
pub mod cmp1fr;
///Timerx Compare 1 Compound Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp1cfr](cmp1cfr) module
pub type CMP1CFR = crate::Reg<u32, _CMP1CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1CFR;
///`read()` method returns [cmp1cfr::R](cmp1cfr::R) reader structure
impl crate::Readable for CMP1CFR {}
///`write(|w| ..)` method takes [cmp1cfr::W](cmp1cfr::W) writer structure
impl crate::Writable for CMP1CFR {}
///Timerx Compare 1 Compound Register
pub mod cmp1cfr;
///Timerx Compare 2 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp2fr](cmp2fr) module
pub type CMP2FR = crate::Reg<u32, _CMP2FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP2FR;
///`read()` method returns [cmp2fr::R](cmp2fr::R) reader structure
impl crate::Readable for CMP2FR {}
///`write(|w| ..)` method takes [cmp2fr::W](cmp2fr::W) writer structure
impl crate::Writable for CMP2FR {}
///Timerx Compare 2 Register
pub mod cmp2fr;
///Timerx Compare 3 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp3fr](cmp3fr) module
pub type CMP3FR = crate::Reg<u32, _CMP3FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP3FR;
///`read()` method returns [cmp3fr::R](cmp3fr::R) reader structure
impl crate::Readable for CMP3FR {}
///`write(|w| ..)` method takes [cmp3fr::W](cmp3fr::W) writer structure
impl crate::Writable for CMP3FR {}
///Timerx Compare 3 Register
pub mod cmp3fr;
///Timerx Compare 4 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp4fr](cmp4fr) module
pub type CMP4FR = crate::Reg<u32, _CMP4FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP4FR;
///`read()` method returns [cmp4fr::R](cmp4fr::R) reader structure
impl crate::Readable for CMP4FR {}
///`write(|w| ..)` method takes [cmp4fr::W](cmp4fr::W) writer structure
impl crate::Writable for CMP4FR {}
///Timerx Compare 4 Register
pub mod cmp4fr;
///Timerx Capture 1 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt1fr](cpt1fr) module
pub type CPT1FR = crate::Reg<u32, _CPT1FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1FR;
///`read()` method returns [cpt1fr::R](cpt1fr::R) reader structure
impl crate::Readable for CPT1FR {}
///Timerx Capture 1 Register
pub mod cpt1fr;
///Timerx Capture 2 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt2fr](cpt2fr) module
pub type CPT2FR = crate::Reg<u32, _CPT2FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2FR;
///`read()` method returns [cpt2fr::R](cpt2fr::R) reader structure
impl crate::Readable for CPT2FR {}
///Timerx Capture 2 Register
pub mod cpt2fr;
///Timerx Deadtime Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dtfr](dtfr) module
pub type DTFR = crate::Reg<u32, _DTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTFR;
///`read()` method returns [dtfr::R](dtfr::R) reader structure
impl crate::Readable for DTFR {}
///`write(|w| ..)` method takes [dtfr::W](dtfr::W) writer structure
impl crate::Writable for DTFR {}
///Timerx Deadtime Register
pub mod dtfr;
///Timerx Output1 Set Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [setf1r](setf1r) module
pub type SETF1R = crate::Reg<u32, _SETF1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETF1R;
///`read()` method returns [setf1r::R](setf1r::R) reader structure
impl crate::Readable for SETF1R {}
///`write(|w| ..)` method takes [setf1r::W](setf1r::W) writer structure
impl crate::Writable for SETF1R {}
///Timerx Output1 Set Register
pub mod setf1r;
///Timerx Output1 Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rste1r](rste1r) module
pub type RSTE1R = crate::Reg<u32, _RSTE1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTE1R;
///`read()` method returns [rste1r::R](rste1r::R) reader structure
impl crate::Readable for RSTE1R {}
///`write(|w| ..)` method takes [rste1r::W](rste1r::W) writer structure
impl crate::Writable for RSTE1R {}
///Timerx Output1 Reset Register
pub mod rste1r;
///Timerx Output2 Set Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [setf2r](setf2r) module
pub type SETF2R = crate::Reg<u32, _SETF2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETF2R;
///`read()` method returns [setf2r::R](setf2r::R) reader structure
impl crate::Readable for SETF2R {}
///`write(|w| ..)` method takes [setf2r::W](setf2r::W) writer structure
impl crate::Writable for SETF2R {}
///Timerx Output2 Set Register
pub mod setf2r;
///Timerx Output2 Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rstf2r](rstf2r) module
pub type RSTF2R = crate::Reg<u32, _RSTF2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTF2R;
///`read()` method returns [rstf2r::R](rstf2r::R) reader structure
impl crate::Readable for RSTF2R {}
///`write(|w| ..)` method takes [rstf2r::W](rstf2r::W) writer structure
impl crate::Writable for RSTF2R {}
///Timerx Output2 Reset Register
pub mod rstf2r;
///Timerx External Event Filtering Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eeffr1](eeffr1) module
pub type EEFFR1 = crate::Reg<u32, _EEFFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFFR1;
///`read()` method returns [eeffr1::R](eeffr1::R) reader structure
impl crate::Readable for EEFFR1 {}
///`write(|w| ..)` method takes [eeffr1::W](eeffr1::W) writer structure
impl crate::Writable for EEFFR1 {}
///Timerx External Event Filtering Register 1
pub mod eeffr1;
///Timerx External Event Filtering Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eeffr2](eeffr2) module
pub type EEFFR2 = crate::Reg<u32, _EEFFR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFFR2;
///`read()` method returns [eeffr2::R](eeffr2::R) reader structure
impl crate::Readable for EEFFR2 {}
///`write(|w| ..)` method takes [eeffr2::W](eeffr2::W) writer structure
impl crate::Writable for EEFFR2 {}
///Timerx External Event Filtering Register 2
pub mod eeffr2;
///TimerA Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rstfr](rstfr) module
pub type RSTFR = crate::Reg<u32, _RSTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTFR;
///`read()` method returns [rstfr::R](rstfr::R) reader structure
impl crate::Readable for RSTFR {}
///`write(|w| ..)` method takes [rstfr::W](rstfr::W) writer structure
impl crate::Writable for RSTFR {}
///TimerA Reset Register
pub mod rstfr;
///Timerx Chopper Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chpfr](chpfr) module
pub type CHPFR = crate::Reg<u32, _CHPFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPFR;
///`read()` method returns [chpfr::R](chpfr::R) reader structure
impl crate::Readable for CHPFR {}
///`write(|w| ..)` method takes [chpfr::W](chpfr::W) writer structure
impl crate::Writable for CHPFR {}
///Timerx Chopper Register
pub mod chpfr;
///Timerx Capture 2 Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt1fcr](cpt1fcr) module
pub type CPT1FCR = crate::Reg<u32, _CPT1FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1FCR;
///`read()` method returns [cpt1fcr::R](cpt1fcr::R) reader structure
impl crate::Readable for CPT1FCR {}
///`write(|w| ..)` method takes [cpt1fcr::W](cpt1fcr::W) writer structure
impl crate::Writable for CPT1FCR {}
///Timerx Capture 2 Control Register
pub mod cpt1fcr;
///CPT2xCR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt2fcr](cpt2fcr) module
pub type CPT2FCR = crate::Reg<u32, _CPT2FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2FCR;
///`read()` method returns [cpt2fcr::R](cpt2fcr::R) reader structure
impl crate::Readable for CPT2FCR {}
///`write(|w| ..)` method takes [cpt2fcr::W](cpt2fcr::W) writer structure
impl crate::Writable for CPT2FCR {}
///CPT2xCR
pub mod cpt2fcr;
///Timerx Output Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [outfr](outfr) module
pub type OUTFR = crate::Reg<u32, _OUTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTFR;
///`read()` method returns [outfr::R](outfr::R) reader structure
impl crate::Readable for OUTFR {}
///`write(|w| ..)` method takes [outfr::W](outfr::W) writer structure
impl crate::Writable for OUTFR {}
///Timerx Output Register
pub mod outfr;
///Timerx Fault Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fltfr](fltfr) module
pub type FLTFR = crate::Reg<u32, _FLTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTFR;
///`read()` method returns [fltfr::R](fltfr::R) reader structure
impl crate::Readable for FLTFR {}
///`write(|w| ..)` method takes [fltfr::W](fltfr::W) writer structure
impl crate::Writable for FLTFR {}
///Timerx Fault Register
pub mod fltfr;
///HRTIM Timerx Control Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timfcr2](timfcr2) module
pub type TIMFCR2 = crate::Reg<u32, _TIMFCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMFCR2;
///`read()` method returns [timfcr2::R](timfcr2::R) reader structure
impl crate::Readable for TIMFCR2 {}
///`write(|w| ..)` method takes [timfcr2::W](timfcr2::W) writer structure
impl crate::Writable for TIMFCR2 {}
///HRTIM Timerx Control Register 2
pub mod timfcr2;
///HRTIM Timerx External Event Filtering Register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [feefr3](feefr3) module
pub type FEEFR3 = crate::Reg<u32, _FEEFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FEEFR3;
///`read()` method returns [feefr3::R](feefr3::R) reader structure
impl crate::Readable for FEEFR3 {}
///`write(|w| ..)` method takes [feefr3::W](feefr3::W) writer structure
impl crate::Writable for FEEFR3 {}
///HRTIM Timerx External Event Filtering Register 3
pub mod feefr3;
