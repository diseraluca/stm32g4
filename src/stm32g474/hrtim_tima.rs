///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Timerx Control Register
    pub timacr: TIMACR,
    ///0x04 - Timerx Interrupt Status Register
    pub timaisr: TIMAISR,
    ///0x08 - Timerx Interrupt Clear Register
    pub timaicr: TIMAICR,
    ///0x0c - TIMxDIER
    pub timadier: TIMADIER,
    ///0x10 - Timerx Counter Register
    pub cntar: CNTAR,
    ///0x14 - Timerx Period Register
    pub perar: PERAR,
    ///0x18 - Timerx Repetition Register
    pub repar: REPAR,
    ///0x1c - Timerx Compare 1 Register
    pub cmp1ar: CMP1AR,
    ///0x20 - Timerx Compare 1 Compound Register
    pub cmp1car: CMP1CAR,
    ///0x24 - Timerx Compare 2 Register
    pub cmp2ar: CMP2AR,
    ///0x28 - Timerx Compare 3 Register
    pub cmp3ar: CMP3AR,
    ///0x2c - Timerx Compare 4 Register
    pub cmp4ar: CMP4AR,
    ///0x30 - Timerx Capture 1 Register
    pub cpt1ar: CPT1AR,
    ///0x34 - Timerx Capture 2 Register
    pub cpt2ar: CPT2AR,
    ///0x38 - Timerx Deadtime Register
    pub dtar: DTAR,
    ///0x3c - Timerx Output1 Set Register
    pub seta1r: SETA1R,
    ///0x40 - Timerx Output1 Reset Register
    pub rsta1r: RSTA1R,
    ///0x44 - Timerx Output2 Set Register
    pub seta2r: SETA2R,
    ///0x48 - Timerx Output2 Reset Register
    pub rsta2r: RSTA2R,
    ///0x4c - Timerx External Event Filtering Register 1
    pub eefar1: EEFAR1,
    ///0x50 - Timerx External Event Filtering Register 2
    pub eefar2: EEFAR2,
    ///0x54 - TimerA Reset Register
    pub rstar: RSTAR,
    ///0x58 - Timerx Chopper Register
    pub chpar: CHPAR,
    ///0x5c - Timerx Capture 2 Control Register
    pub cpt1acr: CPT1ACR,
    ///0x60 - CPT2xCR
    pub cpt2acr: CPT2ACR,
    ///0x64 - Timerx Output Register
    pub outar: OUTAR,
    ///0x68 - Timerx Fault Register
    pub fltar: FLTAR,
    ///0x6c - HRTIM Timerx Control Register 2
    pub timacr2: TIMACR2,
    ///0x70 - HRTIM Timerx External Event Filtering Register 3
    pub aeefr3: AEEFR3,
}
///Timerx Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timacr](timacr) module
pub type TIMACR = crate::Reg<u32, _TIMACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMACR;
///`read()` method returns [timacr::R](timacr::R) reader structure
impl crate::Readable for TIMACR {}
///`write(|w| ..)` method takes [timacr::W](timacr::W) writer structure
impl crate::Writable for TIMACR {}
///Timerx Control Register
pub mod timacr;
///Timerx Interrupt Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timaisr](timaisr) module
pub type TIMAISR = crate::Reg<u32, _TIMAISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMAISR;
///`read()` method returns [timaisr::R](timaisr::R) reader structure
impl crate::Readable for TIMAISR {}
///Timerx Interrupt Status Register
pub mod timaisr;
///Timerx Interrupt Clear Register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timaicr](timaicr) module
pub type TIMAICR = crate::Reg<u32, _TIMAICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMAICR;
///`write(|w| ..)` method takes [timaicr::W](timaicr::W) writer structure
impl crate::Writable for TIMAICR {}
///Timerx Interrupt Clear Register
pub mod timaicr;
///TIMxDIER
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timadier](timadier) module
pub type TIMADIER = crate::Reg<u32, _TIMADIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMADIER;
///`read()` method returns [timadier::R](timadier::R) reader structure
impl crate::Readable for TIMADIER {}
///`write(|w| ..)` method takes [timadier::W](timadier::W) writer structure
impl crate::Writable for TIMADIER {}
///TIMxDIER
pub mod timadier;
///Timerx Counter Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cntar](cntar) module
pub type CNTAR = crate::Reg<u32, _CNTAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTAR;
///`read()` method returns [cntar::R](cntar::R) reader structure
impl crate::Readable for CNTAR {}
///`write(|w| ..)` method takes [cntar::W](cntar::W) writer structure
impl crate::Writable for CNTAR {}
///Timerx Counter Register
pub mod cntar;
///Timerx Period Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [perar](perar) module
pub type PERAR = crate::Reg<u32, _PERAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERAR;
///`read()` method returns [perar::R](perar::R) reader structure
impl crate::Readable for PERAR {}
///`write(|w| ..)` method takes [perar::W](perar::W) writer structure
impl crate::Writable for PERAR {}
///Timerx Period Register
pub mod perar;
///Timerx Repetition Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [repar](repar) module
pub type REPAR = crate::Reg<u32, _REPAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REPAR;
///`read()` method returns [repar::R](repar::R) reader structure
impl crate::Readable for REPAR {}
///`write(|w| ..)` method takes [repar::W](repar::W) writer structure
impl crate::Writable for REPAR {}
///Timerx Repetition Register
pub mod repar;
///Timerx Compare 1 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp1ar](cmp1ar) module
pub type CMP1AR = crate::Reg<u32, _CMP1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1AR;
///`read()` method returns [cmp1ar::R](cmp1ar::R) reader structure
impl crate::Readable for CMP1AR {}
///`write(|w| ..)` method takes [cmp1ar::W](cmp1ar::W) writer structure
impl crate::Writable for CMP1AR {}
///Timerx Compare 1 Register
pub mod cmp1ar;
///Timerx Compare 1 Compound Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp1car](cmp1car) module
pub type CMP1CAR = crate::Reg<u32, _CMP1CAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1CAR;
///`read()` method returns [cmp1car::R](cmp1car::R) reader structure
impl crate::Readable for CMP1CAR {}
///`write(|w| ..)` method takes [cmp1car::W](cmp1car::W) writer structure
impl crate::Writable for CMP1CAR {}
///Timerx Compare 1 Compound Register
pub mod cmp1car;
///Timerx Compare 2 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp2ar](cmp2ar) module
pub type CMP2AR = crate::Reg<u32, _CMP2AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP2AR;
///`read()` method returns [cmp2ar::R](cmp2ar::R) reader structure
impl crate::Readable for CMP2AR {}
///`write(|w| ..)` method takes [cmp2ar::W](cmp2ar::W) writer structure
impl crate::Writable for CMP2AR {}
///Timerx Compare 2 Register
pub mod cmp2ar;
///Timerx Compare 3 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp3ar](cmp3ar) module
pub type CMP3AR = crate::Reg<u32, _CMP3AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP3AR;
///`read()` method returns [cmp3ar::R](cmp3ar::R) reader structure
impl crate::Readable for CMP3AR {}
///`write(|w| ..)` method takes [cmp3ar::W](cmp3ar::W) writer structure
impl crate::Writable for CMP3AR {}
///Timerx Compare 3 Register
pub mod cmp3ar;
///Timerx Compare 4 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmp4ar](cmp4ar) module
pub type CMP4AR = crate::Reg<u32, _CMP4AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP4AR;
///`read()` method returns [cmp4ar::R](cmp4ar::R) reader structure
impl crate::Readable for CMP4AR {}
///`write(|w| ..)` method takes [cmp4ar::W](cmp4ar::W) writer structure
impl crate::Writable for CMP4AR {}
///Timerx Compare 4 Register
pub mod cmp4ar;
///Timerx Capture 1 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt1ar](cpt1ar) module
pub type CPT1AR = crate::Reg<u32, _CPT1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1AR;
///`read()` method returns [cpt1ar::R](cpt1ar::R) reader structure
impl crate::Readable for CPT1AR {}
///Timerx Capture 1 Register
pub mod cpt1ar;
///Timerx Capture 2 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt2ar](cpt2ar) module
pub type CPT2AR = crate::Reg<u32, _CPT2AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2AR;
///`read()` method returns [cpt2ar::R](cpt2ar::R) reader structure
impl crate::Readable for CPT2AR {}
///Timerx Capture 2 Register
pub mod cpt2ar;
///Timerx Deadtime Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dtar](dtar) module
pub type DTAR = crate::Reg<u32, _DTAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTAR;
///`read()` method returns [dtar::R](dtar::R) reader structure
impl crate::Readable for DTAR {}
///`write(|w| ..)` method takes [dtar::W](dtar::W) writer structure
impl crate::Writable for DTAR {}
///Timerx Deadtime Register
pub mod dtar;
///Timerx Output1 Set Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [seta1r](seta1r) module
pub type SETA1R = crate::Reg<u32, _SETA1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETA1R;
///`read()` method returns [seta1r::R](seta1r::R) reader structure
impl crate::Readable for SETA1R {}
///`write(|w| ..)` method takes [seta1r::W](seta1r::W) writer structure
impl crate::Writable for SETA1R {}
///Timerx Output1 Set Register
pub mod seta1r;
///Timerx Output1 Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rsta1r](rsta1r) module
pub type RSTA1R = crate::Reg<u32, _RSTA1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTA1R;
///`read()` method returns [rsta1r::R](rsta1r::R) reader structure
impl crate::Readable for RSTA1R {}
///`write(|w| ..)` method takes [rsta1r::W](rsta1r::W) writer structure
impl crate::Writable for RSTA1R {}
///Timerx Output1 Reset Register
pub mod rsta1r;
///Timerx Output2 Set Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [seta2r](seta2r) module
pub type SETA2R = crate::Reg<u32, _SETA2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETA2R;
///`read()` method returns [seta2r::R](seta2r::R) reader structure
impl crate::Readable for SETA2R {}
///`write(|w| ..)` method takes [seta2r::W](seta2r::W) writer structure
impl crate::Writable for SETA2R {}
///Timerx Output2 Set Register
pub mod seta2r;
///Timerx Output2 Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rsta2r](rsta2r) module
pub type RSTA2R = crate::Reg<u32, _RSTA2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTA2R;
///`read()` method returns [rsta2r::R](rsta2r::R) reader structure
impl crate::Readable for RSTA2R {}
///`write(|w| ..)` method takes [rsta2r::W](rsta2r::W) writer structure
impl crate::Writable for RSTA2R {}
///Timerx Output2 Reset Register
pub mod rsta2r;
///Timerx External Event Filtering Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eefar1](eefar1) module
pub type EEFAR1 = crate::Reg<u32, _EEFAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFAR1;
///`read()` method returns [eefar1::R](eefar1::R) reader structure
impl crate::Readable for EEFAR1 {}
///`write(|w| ..)` method takes [eefar1::W](eefar1::W) writer structure
impl crate::Writable for EEFAR1 {}
///Timerx External Event Filtering Register 1
pub mod eefar1;
///Timerx External Event Filtering Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eefar2](eefar2) module
pub type EEFAR2 = crate::Reg<u32, _EEFAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFAR2;
///`read()` method returns [eefar2::R](eefar2::R) reader structure
impl crate::Readable for EEFAR2 {}
///`write(|w| ..)` method takes [eefar2::W](eefar2::W) writer structure
impl crate::Writable for EEFAR2 {}
///Timerx External Event Filtering Register 2
pub mod eefar2;
///TimerA Reset Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rstar](rstar) module
pub type RSTAR = crate::Reg<u32, _RSTAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTAR;
///`read()` method returns [rstar::R](rstar::R) reader structure
impl crate::Readable for RSTAR {}
///`write(|w| ..)` method takes [rstar::W](rstar::W) writer structure
impl crate::Writable for RSTAR {}
///TimerA Reset Register
pub mod rstar;
///Timerx Chopper Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chpar](chpar) module
pub type CHPAR = crate::Reg<u32, _CHPAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPAR;
///`read()` method returns [chpar::R](chpar::R) reader structure
impl crate::Readable for CHPAR {}
///`write(|w| ..)` method takes [chpar::W](chpar::W) writer structure
impl crate::Writable for CHPAR {}
///Timerx Chopper Register
pub mod chpar;
///Timerx Capture 2 Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt1acr](cpt1acr) module
pub type CPT1ACR = crate::Reg<u32, _CPT1ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1ACR;
///`read()` method returns [cpt1acr::R](cpt1acr::R) reader structure
impl crate::Readable for CPT1ACR {}
///`write(|w| ..)` method takes [cpt1acr::W](cpt1acr::W) writer structure
impl crate::Writable for CPT1ACR {}
///Timerx Capture 2 Control Register
pub mod cpt1acr;
///CPT2xCR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt2acr](cpt2acr) module
pub type CPT2ACR = crate::Reg<u32, _CPT2ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2ACR;
///`read()` method returns [cpt2acr::R](cpt2acr::R) reader structure
impl crate::Readable for CPT2ACR {}
///`write(|w| ..)` method takes [cpt2acr::W](cpt2acr::W) writer structure
impl crate::Writable for CPT2ACR {}
///CPT2xCR
pub mod cpt2acr;
///Timerx Output Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [outar](outar) module
pub type OUTAR = crate::Reg<u32, _OUTAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTAR;
///`read()` method returns [outar::R](outar::R) reader structure
impl crate::Readable for OUTAR {}
///`write(|w| ..)` method takes [outar::W](outar::W) writer structure
impl crate::Writable for OUTAR {}
///Timerx Output Register
pub mod outar;
///Timerx Fault Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fltar](fltar) module
pub type FLTAR = crate::Reg<u32, _FLTAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTAR;
///`read()` method returns [fltar::R](fltar::R) reader structure
impl crate::Readable for FLTAR {}
///`write(|w| ..)` method takes [fltar::W](fltar::W) writer structure
impl crate::Writable for FLTAR {}
///Timerx Fault Register
pub mod fltar;
///HRTIM Timerx Control Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timacr2](timacr2) module
pub type TIMACR2 = crate::Reg<u32, _TIMACR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMACR2;
///`read()` method returns [timacr2::R](timacr2::R) reader structure
impl crate::Readable for TIMACR2 {}
///`write(|w| ..)` method takes [timacr2::W](timacr2::W) writer structure
impl crate::Writable for TIMACR2 {}
///HRTIM Timerx Control Register 2
pub mod timacr2;
///HRTIM Timerx External Event Filtering Register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aeefr3](aeefr3) module
pub type AEEFR3 = crate::Reg<u32, _AEEFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AEEFR3;
///`read()` method returns [aeefr3::R](aeefr3::R) reader structure
impl crate::Readable for AEEFR3 {}
///`write(|w| ..)` method takes [aeefr3::W](aeefr3::W) writer structure
impl crate::Writable for AEEFR3 {}
///HRTIM Timerx External Event Filtering Register 3
pub mod aeefr3;
