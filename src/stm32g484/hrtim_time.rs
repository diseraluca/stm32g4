///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Timerx Control Register
    pub timecr: TIMECR,
    ///0x04 - Timerx Interrupt Status Register
    pub timeisr: TIMEISR,
    ///0x08 - Timerx Interrupt Clear Register
    pub timeicr: TIMEICR,
    ///0x0c - TIMxDIER
    pub timedier: TIMEDIER,
    ///0x10 - Timerx Counter Register
    pub cnter: CNTER,
    ///0x14 - Timerx Period Register
    pub perer: PERER,
    ///0x18 - Timerx Repetition Register
    pub reper: REPER,
    ///0x1c - Timerx Compare 1 Register
    pub cmp1er: CMP1ER,
    ///0x20 - Timerx Compare 1 Compound Register
    pub cmp1cer: CMP1CER,
    ///0x24 - Timerx Compare 2 Register
    pub cmp2er: CMP2ER,
    ///0x28 - Timerx Compare 3 Register
    pub cmp3er: CMP3ER,
    ///0x2c - Timerx Compare 4 Register
    pub cmp4er: CMP4ER,
    ///0x30 - Timerx Capture 1 Register
    pub cpt1er: CPT1ER,
    ///0x34 - Timerx Capture 2 Register
    pub cpt2er: CPT2ER,
    ///0x38 - Timerx Deadtime Register
    pub dter: DTER,
    ///0x3c - Timerx Output1 Set Register
    pub sete1r: SETE1R,
    ///0x40 - Timerx Output1 Reset Register
    pub rste1r: RSTE1R,
    ///0x44 - Timerx Output2 Set Register
    pub sete2r: SETE2R,
    ///0x48 - Timerx Output2 Reset Register
    pub rste2r: RSTE2R,
    ///0x4c - Timerx External Event Filtering Register 1
    pub eefer1: EEFER1,
    ///0x50 - Timerx External Event Filtering Register 2
    pub eefer2: EEFER2,
    ///0x54 - TimerA Reset Register
    pub rster: RSTER,
    ///0x58 - Timerx Chopper Register
    pub chper: CHPER,
    ///0x5c - Timerx Capture 2 Control Register
    pub cpt1ecr: CPT1ECR,
    ///0x60 - CPT2xCR
    pub cpt2ecr: CPT2ECR,
    ///0x64 - Timerx Output Register
    pub outer: OUTER,
    ///0x68 - Timerx Fault Register
    pub flter: FLTER,
    ///0x6c - HRTIM Timerx Control Register 2
    pub timecr2: TIMECR2,
    ///0x70 - HRTIM Timerx External Event Filtering Register 3
    pub eeefr3: EEEFR3,
}
///Timerx Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timecr](timecr) module
pub type TIMECR = crate::Reg<u32, _TIMECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMECR;
///`read()` method returns [timecr::R](timecr::R) reader structure
impl crate::Readable for TIMECR {}
///`write(|w| ..)` method takes [timecr::W](timecr::W) writer structure
impl crate::Writable for TIMECR {}
///Timerx Control Register
pub mod timecr;
///Timerx Interrupt Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timeisr](timeisr) module
pub type TIMEISR = crate::Reg<u32, _TIMEISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEISR;
///`read()` method returns [timeisr::R](timeisr::R) reader structure
impl crate::Readable for TIMEISR {}
///Timerx Interrupt Status Register
pub mod timeisr;
///Timerx Interrupt Clear Register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timeicr](timeicr) module
pub type TIMEICR = crate::Reg<u32, _TIMEICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEICR;
///`write(|w| ..)` method takes [timeicr::W](timeicr::W) writer structure
impl crate::Writable for TIMEICR {}
///Timerx Interrupt Clear Register
pub mod timeicr;
///TIMxDIER
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timedier](timedier) module
pub type TIMEDIER = crate::Reg<u32, _TIMEDIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEDIER;
///`read()` method returns [timedier::R](timedier::R) reader structure
impl crate::Readable for TIMEDIER {}
///`write(|w| ..)` method takes [timedier::W](timedier::W) writer structure
impl crate::Writable for TIMEDIER {}
///TIMxDIER
pub mod timedier;
///Timerx Counter Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cnter](cnter) module
pub type CNTER = crate::Reg<u32, _CNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTER;
///`read()` method returns [cnter::R](cnter::R) reader structure
impl crate::Readable for CNTER {}
///`write(|w| ..)` method takes [cnter::W](cnter::W) writer structure
impl crate::Writable for CNTER {}
///Timerx Counter Register
pub mod cnter;
///Timerx Period Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [perer](perer) module
pub type PERER = crate::Reg<u32, _PERER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERER;
///`read()` method returns [perer::R](perer::R) reader structure
impl crate::Readable for PERER {}
///`write(|w| ..)` method takes [perer::W](perer::W) writer structure
impl crate::Writable for PERER {}
///Timerx Period Register
pub mod perer;
///Timerx Repetition Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [reper](reper) module
pub type REPER = crate::Reg<u32, _REPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REPER;
///`read()` method returns [reper::R](reper::R) reader structure
impl crate::Readable for REPER {}
///`write(|w| ..)` method takes [reper::W](reper::W) writer structure
impl crate::Writable for REPER {}
///Timerx Repetition Register
pub mod reper;
///Timerx Compare 1 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp1er](cmp1er) module
pub type CMP1ER = crate::Reg<u32, _CMP1ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1ER;
///`read()` method returns [cmp1er::R](cmp1er::R) reader structure
impl crate::Readable for CMP1ER {}
///`write(|w| ..)` method takes [cmp1er::W](cmp1er::W) writer structure
impl crate::Writable for CMP1ER {}
///Timerx Compare 1 Register
pub mod cmp1er;
///Timerx Compare 1 Compound Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp1cer](cmp1cer) module
pub type CMP1CER = crate::Reg<u32, _CMP1CER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1CER;
///`read()` method returns [cmp1cer::R](cmp1cer::R) reader structure
impl crate::Readable for CMP1CER {}
///`write(|w| ..)` method takes [cmp1cer::W](cmp1cer::W) writer structure
impl crate::Writable for CMP1CER {}
///Timerx Compare 1 Compound Register
pub mod cmp1cer;
///Timerx Compare 2 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp2er](cmp2er) module
pub type CMP2ER = crate::Reg<u32, _CMP2ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP2ER;
///`read()` method returns [cmp2er::R](cmp2er::R) reader structure
impl crate::Readable for CMP2ER {}
///`write(|w| ..)` method takes [cmp2er::W](cmp2er::W) writer structure
impl crate::Writable for CMP2ER {}
///Timerx Compare 2 Register
pub mod cmp2er;
///Timerx Compare 3 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp3er](cmp3er) module
pub type CMP3ER = crate::Reg<u32, _CMP3ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP3ER;
///`read()` method returns [cmp3er::R](cmp3er::R) reader structure
impl crate::Readable for CMP3ER {}
///`write(|w| ..)` method takes [cmp3er::W](cmp3er::W) writer structure
impl crate::Writable for CMP3ER {}
///Timerx Compare 3 Register
pub mod cmp3er;
///Timerx Compare 4 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp4er](cmp4er) module
pub type CMP4ER = crate::Reg<u32, _CMP4ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP4ER;
///`read()` method returns [cmp4er::R](cmp4er::R) reader structure
impl crate::Readable for CMP4ER {}
///`write(|w| ..)` method takes [cmp4er::W](cmp4er::W) writer structure
impl crate::Writable for CMP4ER {}
///Timerx Compare 4 Register
pub mod cmp4er;
///Timerx Capture 1 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt1er](cpt1er) module
pub type CPT1ER = crate::Reg<u32, _CPT1ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1ER;
///`read()` method returns [cpt1er::R](cpt1er::R) reader structure
impl crate::Readable for CPT1ER {}
///Timerx Capture 1 Register
pub mod cpt1er;
///Timerx Capture 2 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt2er](cpt2er) module
pub type CPT2ER = crate::Reg<u32, _CPT2ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2ER;
///`read()` method returns [cpt2er::R](cpt2er::R) reader structure
impl crate::Readable for CPT2ER {}
///Timerx Capture 2 Register
pub mod cpt2er;
///Timerx Deadtime Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dter](dter) module
pub type DTER = crate::Reg<u32, _DTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTER;
///`read()` method returns [dter::R](dter::R) reader structure
impl crate::Readable for DTER {}
///`write(|w| ..)` method takes [dter::W](dter::W) writer structure
impl crate::Writable for DTER {}
///Timerx Deadtime Register
pub mod dter;
///Timerx Output1 Set Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sete1r](sete1r) module
pub type SETE1R = crate::Reg<u32, _SETE1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETE1R;
///`read()` method returns [sete1r::R](sete1r::R) reader structure
impl crate::Readable for SETE1R {}
///`write(|w| ..)` method takes [sete1r::W](sete1r::W) writer structure
impl crate::Writable for SETE1R {}
///Timerx Output1 Set Register
pub mod sete1r;
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
///For information about available fields see [sete2r](sete2r) module
pub type SETE2R = crate::Reg<u32, _SETE2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETE2R;
///`read()` method returns [sete2r::R](sete2r::R) reader structure
impl crate::Readable for SETE2R {}
///`write(|w| ..)` method takes [sete2r::W](sete2r::W) writer structure
impl crate::Writable for SETE2R {}
///Timerx Output2 Set Register
pub mod sete2r;
///Timerx Output2 Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rste2r](rste2r) module
pub type RSTE2R = crate::Reg<u32, _RSTE2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTE2R;
///`read()` method returns [rste2r::R](rste2r::R) reader structure
impl crate::Readable for RSTE2R {}
///`write(|w| ..)` method takes [rste2r::W](rste2r::W) writer structure
impl crate::Writable for RSTE2R {}
///Timerx Output2 Reset Register
pub mod rste2r;
///Timerx External Event Filtering Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eefer1](eefer1) module
pub type EEFER1 = crate::Reg<u32, _EEFER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFER1;
///`read()` method returns [eefer1::R](eefer1::R) reader structure
impl crate::Readable for EEFER1 {}
///`write(|w| ..)` method takes [eefer1::W](eefer1::W) writer structure
impl crate::Writable for EEFER1 {}
///Timerx External Event Filtering Register 1
pub mod eefer1;
///Timerx External Event Filtering Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eefer2](eefer2) module
pub type EEFER2 = crate::Reg<u32, _EEFER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFER2;
///`read()` method returns [eefer2::R](eefer2::R) reader structure
impl crate::Readable for EEFER2 {}
///`write(|w| ..)` method takes [eefer2::W](eefer2::W) writer structure
impl crate::Writable for EEFER2 {}
///Timerx External Event Filtering Register 2
pub mod eefer2;
///TimerA Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rster](rster) module
pub type RSTER = crate::Reg<u32, _RSTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTER;
///`read()` method returns [rster::R](rster::R) reader structure
impl crate::Readable for RSTER {}
///`write(|w| ..)` method takes [rster::W](rster::W) writer structure
impl crate::Writable for RSTER {}
///TimerA Reset Register
pub mod rster;
///Timerx Chopper Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chper](chper) module
pub type CHPER = crate::Reg<u32, _CHPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPER;
///`read()` method returns [chper::R](chper::R) reader structure
impl crate::Readable for CHPER {}
///`write(|w| ..)` method takes [chper::W](chper::W) writer structure
impl crate::Writable for CHPER {}
///Timerx Chopper Register
pub mod chper;
///Timerx Capture 2 Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt1ecr](cpt1ecr) module
pub type CPT1ECR = crate::Reg<u32, _CPT1ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1ECR;
///`read()` method returns [cpt1ecr::R](cpt1ecr::R) reader structure
impl crate::Readable for CPT1ECR {}
///`write(|w| ..)` method takes [cpt1ecr::W](cpt1ecr::W) writer structure
impl crate::Writable for CPT1ECR {}
///Timerx Capture 2 Control Register
pub mod cpt1ecr;
///CPT2xCR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt2ecr](cpt2ecr) module
pub type CPT2ECR = crate::Reg<u32, _CPT2ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2ECR;
///`read()` method returns [cpt2ecr::R](cpt2ecr::R) reader structure
impl crate::Readable for CPT2ECR {}
///`write(|w| ..)` method takes [cpt2ecr::W](cpt2ecr::W) writer structure
impl crate::Writable for CPT2ECR {}
///CPT2xCR
pub mod cpt2ecr;
///Timerx Output Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [outer](outer) module
pub type OUTER = crate::Reg<u32, _OUTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTER;
///`read()` method returns [outer::R](outer::R) reader structure
impl crate::Readable for OUTER {}
///`write(|w| ..)` method takes [outer::W](outer::W) writer structure
impl crate::Writable for OUTER {}
///Timerx Output Register
pub mod outer;
///Timerx Fault Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flter](flter) module
pub type FLTER = crate::Reg<u32, _FLTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTER;
///`read()` method returns [flter::R](flter::R) reader structure
impl crate::Readable for FLTER {}
///`write(|w| ..)` method takes [flter::W](flter::W) writer structure
impl crate::Writable for FLTER {}
///Timerx Fault Register
pub mod flter;
///HRTIM Timerx Control Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timecr2](timecr2) module
pub type TIMECR2 = crate::Reg<u32, _TIMECR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMECR2;
///`read()` method returns [timecr2::R](timecr2::R) reader structure
impl crate::Readable for TIMECR2 {}
///`write(|w| ..)` method takes [timecr2::W](timecr2::W) writer structure
impl crate::Writable for TIMECR2 {}
///HRTIM Timerx Control Register 2
pub mod timecr2;
///HRTIM Timerx External Event Filtering Register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eeefr3](eeefr3) module
pub type EEEFR3 = crate::Reg<u32, _EEEFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEEFR3;
///`read()` method returns [eeefr3::R](eeefr3::R) reader structure
impl crate::Readable for EEEFR3 {}
///`write(|w| ..)` method takes [eeefr3::W](eeefr3::W) writer structure
impl crate::Writable for EEEFR3 {}
///HRTIM Timerx External Event Filtering Register 3
pub mod eeefr3;
