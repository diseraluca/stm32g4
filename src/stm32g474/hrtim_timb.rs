///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Timerx Control Register
    pub timbcr: TIMBCR,
    ///0x04 - Timerx Interrupt Status Register
    pub timbisr: TIMBISR,
    ///0x08 - Timerx Interrupt Clear Register
    pub timbicr: TIMBICR,
    ///0x0c - TIMxDIER
    pub timbdier: TIMBDIER,
    ///0x10 - Timerx Counter Register
    pub cntr: CNTR,
    ///0x14 - Timerx Period Register
    pub perbr: PERBR,
    ///0x18 - Timerx Repetition Register
    pub repbr: REPBR,
    ///0x1c - Timerx Compare 1 Register
    pub cmp1br: CMP1BR,
    ///0x20 - Timerx Compare 1 Compound Register
    pub cmp1cbr: CMP1CBR,
    ///0x24 - Timerx Compare 2 Register
    pub cmp2br: CMP2BR,
    ///0x28 - Timerx Compare 3 Register
    pub cmp3br: CMP3BR,
    ///0x2c - Timerx Compare 4 Register
    pub cmp4br: CMP4BR,
    ///0x30 - Timerx Capture 1 Register
    pub cpt1br: CPT1BR,
    ///0x34 - Timerx Capture 2 Register
    pub cpt2br: CPT2BR,
    ///0x38 - Timerx Deadtime Register
    pub dtbr: DTBR,
    ///0x3c - Timerx Output1 Set Register
    pub setb1r: SETB1R,
    ///0x40 - Timerx Output1 Reset Register
    pub rstb1r: RSTB1R,
    ///0x44 - Timerx Output2 Set Register
    pub setb2r: SETB2R,
    ///0x48 - Timerx Output2 Reset Register
    pub rstb2r: RSTB2R,
    ///0x4c - Timerx External Event Filtering Register 1
    pub eefbr1: EEFBR1,
    ///0x50 - Timerx External Event Filtering Register 2
    pub eefbr2: EEFBR2,
    ///0x54 - TimerA Reset Register
    pub rstbr: RSTBR,
    ///0x58 - Timerx Chopper Register
    pub chpbr: CHPBR,
    ///0x5c - Timerx Capture 2 Control Register
    pub cpt1bcr: CPT1BCR,
    ///0x60 - CPT2xCR
    pub cpt2bcr: CPT2BCR,
    ///0x64 - Timerx Output Register
    pub outbr: OUTBR,
    ///0x68 - Timerx Fault Register
    pub fltbr: FLTBR,
    ///0x6c - HRTIM Timerx Control Register 2
    pub timbcr2: TIMBCR2,
    ///0x70 - HRTIM Timerx External Event Filtering Register 3
    pub beefr3: BEEFR3,
}
///Timerx Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timbcr](timbcr) module
pub type TIMBCR = crate::Reg<u32, _TIMBCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMBCR;
///`read()` method returns [timbcr::R](timbcr::R) reader structure
impl crate::Readable for TIMBCR {}
///`write(|w| ..)` method takes [timbcr::W](timbcr::W) writer structure
impl crate::Writable for TIMBCR {}
///Timerx Control Register
pub mod timbcr;
///Timerx Interrupt Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timbisr](timbisr) module
pub type TIMBISR = crate::Reg<u32, _TIMBISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMBISR;
///`read()` method returns [timbisr::R](timbisr::R) reader structure
impl crate::Readable for TIMBISR {}
///Timerx Interrupt Status Register
pub mod timbisr;
///Timerx Interrupt Clear Register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timbicr](timbicr) module
pub type TIMBICR = crate::Reg<u32, _TIMBICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMBICR;
///`write(|w| ..)` method takes [timbicr::W](timbicr::W) writer structure
impl crate::Writable for TIMBICR {}
///Timerx Interrupt Clear Register
pub mod timbicr;
///TIMxDIER
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timbdier](timbdier) module
pub type TIMBDIER = crate::Reg<u32, _TIMBDIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMBDIER;
///`read()` method returns [timbdier::R](timbdier::R) reader structure
impl crate::Readable for TIMBDIER {}
///`write(|w| ..)` method takes [timbdier::W](timbdier::W) writer structure
impl crate::Writable for TIMBDIER {}
///TIMxDIER
pub mod timbdier;
///Timerx Counter Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cntr](cntr) module
pub type CNTR = crate::Reg<u32, _CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTR;
///`read()` method returns [cntr::R](cntr::R) reader structure
impl crate::Readable for CNTR {}
///`write(|w| ..)` method takes [cntr::W](cntr::W) writer structure
impl crate::Writable for CNTR {}
///Timerx Counter Register
pub mod cntr;
///Timerx Period Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [perbr](perbr) module
pub type PERBR = crate::Reg<u32, _PERBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERBR;
///`read()` method returns [perbr::R](perbr::R) reader structure
impl crate::Readable for PERBR {}
///`write(|w| ..)` method takes [perbr::W](perbr::W) writer structure
impl crate::Writable for PERBR {}
///Timerx Period Register
pub mod perbr;
///Timerx Repetition Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [repbr](repbr) module
pub type REPBR = crate::Reg<u32, _REPBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REPBR;
///`read()` method returns [repbr::R](repbr::R) reader structure
impl crate::Readable for REPBR {}
///`write(|w| ..)` method takes [repbr::W](repbr::W) writer structure
impl crate::Writable for REPBR {}
///Timerx Repetition Register
pub mod repbr;
///Timerx Compare 1 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp1br](cmp1br) module
pub type CMP1BR = crate::Reg<u32, _CMP1BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1BR;
///`read()` method returns [cmp1br::R](cmp1br::R) reader structure
impl crate::Readable for CMP1BR {}
///`write(|w| ..)` method takes [cmp1br::W](cmp1br::W) writer structure
impl crate::Writable for CMP1BR {}
///Timerx Compare 1 Register
pub mod cmp1br;
///Timerx Compare 1 Compound Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp1cbr](cmp1cbr) module
pub type CMP1CBR = crate::Reg<u32, _CMP1CBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1CBR;
///`read()` method returns [cmp1cbr::R](cmp1cbr::R) reader structure
impl crate::Readable for CMP1CBR {}
///`write(|w| ..)` method takes [cmp1cbr::W](cmp1cbr::W) writer structure
impl crate::Writable for CMP1CBR {}
///Timerx Compare 1 Compound Register
pub mod cmp1cbr;
///Timerx Compare 2 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp2br](cmp2br) module
pub type CMP2BR = crate::Reg<u32, _CMP2BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP2BR;
///`read()` method returns [cmp2br::R](cmp2br::R) reader structure
impl crate::Readable for CMP2BR {}
///`write(|w| ..)` method takes [cmp2br::W](cmp2br::W) writer structure
impl crate::Writable for CMP2BR {}
///Timerx Compare 2 Register
pub mod cmp2br;
///Timerx Compare 3 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp3br](cmp3br) module
pub type CMP3BR = crate::Reg<u32, _CMP3BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP3BR;
///`read()` method returns [cmp3br::R](cmp3br::R) reader structure
impl crate::Readable for CMP3BR {}
///`write(|w| ..)` method takes [cmp3br::W](cmp3br::W) writer structure
impl crate::Writable for CMP3BR {}
///Timerx Compare 3 Register
pub mod cmp3br;
///Timerx Compare 4 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp4br](cmp4br) module
pub type CMP4BR = crate::Reg<u32, _CMP4BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP4BR;
///`read()` method returns [cmp4br::R](cmp4br::R) reader structure
impl crate::Readable for CMP4BR {}
///`write(|w| ..)` method takes [cmp4br::W](cmp4br::W) writer structure
impl crate::Writable for CMP4BR {}
///Timerx Compare 4 Register
pub mod cmp4br;
///Timerx Capture 1 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt1br](cpt1br) module
pub type CPT1BR = crate::Reg<u32, _CPT1BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1BR;
///`read()` method returns [cpt1br::R](cpt1br::R) reader structure
impl crate::Readable for CPT1BR {}
///Timerx Capture 1 Register
pub mod cpt1br;
///Timerx Capture 2 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt2br](cpt2br) module
pub type CPT2BR = crate::Reg<u32, _CPT2BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2BR;
///`read()` method returns [cpt2br::R](cpt2br::R) reader structure
impl crate::Readable for CPT2BR {}
///Timerx Capture 2 Register
pub mod cpt2br;
///Timerx Deadtime Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dtbr](dtbr) module
pub type DTBR = crate::Reg<u32, _DTBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTBR;
///`read()` method returns [dtbr::R](dtbr::R) reader structure
impl crate::Readable for DTBR {}
///`write(|w| ..)` method takes [dtbr::W](dtbr::W) writer structure
impl crate::Writable for DTBR {}
///Timerx Deadtime Register
pub mod dtbr;
///Timerx Output1 Set Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [setb1r](setb1r) module
pub type SETB1R = crate::Reg<u32, _SETB1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETB1R;
///`read()` method returns [setb1r::R](setb1r::R) reader structure
impl crate::Readable for SETB1R {}
///`write(|w| ..)` method takes [setb1r::W](setb1r::W) writer structure
impl crate::Writable for SETB1R {}
///Timerx Output1 Set Register
pub mod setb1r;
///Timerx Output1 Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rstb1r](rstb1r) module
pub type RSTB1R = crate::Reg<u32, _RSTB1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTB1R;
///`read()` method returns [rstb1r::R](rstb1r::R) reader structure
impl crate::Readable for RSTB1R {}
///`write(|w| ..)` method takes [rstb1r::W](rstb1r::W) writer structure
impl crate::Writable for RSTB1R {}
///Timerx Output1 Reset Register
pub mod rstb1r;
///Timerx Output2 Set Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [setb2r](setb2r) module
pub type SETB2R = crate::Reg<u32, _SETB2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETB2R;
///`read()` method returns [setb2r::R](setb2r::R) reader structure
impl crate::Readable for SETB2R {}
///`write(|w| ..)` method takes [setb2r::W](setb2r::W) writer structure
impl crate::Writable for SETB2R {}
///Timerx Output2 Set Register
pub mod setb2r;
///Timerx Output2 Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rstb2r](rstb2r) module
pub type RSTB2R = crate::Reg<u32, _RSTB2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTB2R;
///`read()` method returns [rstb2r::R](rstb2r::R) reader structure
impl crate::Readable for RSTB2R {}
///`write(|w| ..)` method takes [rstb2r::W](rstb2r::W) writer structure
impl crate::Writable for RSTB2R {}
///Timerx Output2 Reset Register
pub mod rstb2r;
///Timerx External Event Filtering Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eefbr1](eefbr1) module
pub type EEFBR1 = crate::Reg<u32, _EEFBR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFBR1;
///`read()` method returns [eefbr1::R](eefbr1::R) reader structure
impl crate::Readable for EEFBR1 {}
///`write(|w| ..)` method takes [eefbr1::W](eefbr1::W) writer structure
impl crate::Writable for EEFBR1 {}
///Timerx External Event Filtering Register 1
pub mod eefbr1;
///Timerx External Event Filtering Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eefbr2](eefbr2) module
pub type EEFBR2 = crate::Reg<u32, _EEFBR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFBR2;
///`read()` method returns [eefbr2::R](eefbr2::R) reader structure
impl crate::Readable for EEFBR2 {}
///`write(|w| ..)` method takes [eefbr2::W](eefbr2::W) writer structure
impl crate::Writable for EEFBR2 {}
///Timerx External Event Filtering Register 2
pub mod eefbr2;
///TimerA Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rstbr](rstbr) module
pub type RSTBR = crate::Reg<u32, _RSTBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTBR;
///`read()` method returns [rstbr::R](rstbr::R) reader structure
impl crate::Readable for RSTBR {}
///`write(|w| ..)` method takes [rstbr::W](rstbr::W) writer structure
impl crate::Writable for RSTBR {}
///TimerA Reset Register
pub mod rstbr;
///Timerx Chopper Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chpbr](chpbr) module
pub type CHPBR = crate::Reg<u32, _CHPBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPBR;
///`read()` method returns [chpbr::R](chpbr::R) reader structure
impl crate::Readable for CHPBR {}
///`write(|w| ..)` method takes [chpbr::W](chpbr::W) writer structure
impl crate::Writable for CHPBR {}
///Timerx Chopper Register
pub mod chpbr;
///Timerx Capture 2 Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt1bcr](cpt1bcr) module
pub type CPT1BCR = crate::Reg<u32, _CPT1BCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1BCR;
///`read()` method returns [cpt1bcr::R](cpt1bcr::R) reader structure
impl crate::Readable for CPT1BCR {}
///`write(|w| ..)` method takes [cpt1bcr::W](cpt1bcr::W) writer structure
impl crate::Writable for CPT1BCR {}
///Timerx Capture 2 Control Register
pub mod cpt1bcr;
///CPT2xCR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt2bcr](cpt2bcr) module
pub type CPT2BCR = crate::Reg<u32, _CPT2BCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2BCR;
///`read()` method returns [cpt2bcr::R](cpt2bcr::R) reader structure
impl crate::Readable for CPT2BCR {}
///`write(|w| ..)` method takes [cpt2bcr::W](cpt2bcr::W) writer structure
impl crate::Writable for CPT2BCR {}
///CPT2xCR
pub mod cpt2bcr;
///Timerx Output Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [outbr](outbr) module
pub type OUTBR = crate::Reg<u32, _OUTBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTBR;
///`read()` method returns [outbr::R](outbr::R) reader structure
impl crate::Readable for OUTBR {}
///`write(|w| ..)` method takes [outbr::W](outbr::W) writer structure
impl crate::Writable for OUTBR {}
///Timerx Output Register
pub mod outbr;
///Timerx Fault Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fltbr](fltbr) module
pub type FLTBR = crate::Reg<u32, _FLTBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTBR;
///`read()` method returns [fltbr::R](fltbr::R) reader structure
impl crate::Readable for FLTBR {}
///`write(|w| ..)` method takes [fltbr::W](fltbr::W) writer structure
impl crate::Writable for FLTBR {}
///Timerx Fault Register
pub mod fltbr;
///HRTIM Timerx Control Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timbcr2](timbcr2) module
pub type TIMBCR2 = crate::Reg<u32, _TIMBCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMBCR2;
///`read()` method returns [timbcr2::R](timbcr2::R) reader structure
impl crate::Readable for TIMBCR2 {}
///`write(|w| ..)` method takes [timbcr2::W](timbcr2::W) writer structure
impl crate::Writable for TIMBCR2 {}
///HRTIM Timerx Control Register 2
pub mod timbcr2;
///HRTIM Timerx External Event Filtering Register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [beefr3](beefr3) module
pub type BEEFR3 = crate::Reg<u32, _BEEFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BEEFR3;
///`read()` method returns [beefr3::R](beefr3::R) reader structure
impl crate::Readable for BEEFR3 {}
///`write(|w| ..)` method takes [beefr3::W](beefr3::W) writer structure
impl crate::Writable for BEEFR3 {}
///HRTIM Timerx External Event Filtering Register 3
pub mod beefr3;
