///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control Register 1
    pub cr1: CR1,
    ///0x04 - Control Register 2
    pub cr2: CR2,
    ///0x08 - Interrupt Status Register
    pub isr: ISR,
    ///0x0c - Interrupt Clear Register
    pub icr: ICR,
    ///0x10 - Interrupt Enable Register
    pub ier: IER,
    ///0x14 - Output Enable Register
    pub oenr: OENR,
    ///0x18 - ODISR
    pub odisr: ODISR,
    ///0x1c - Output Disable Status Register
    pub odsr: ODSR,
    ///0x20 - Burst Mode Control Register
    pub bmcr: BMCR,
    ///0x24 - BMTRG
    pub bmtrg: BMTRG,
    ///0x28 - BMCMPR
    pub bmcmpr: BMCMPR,
    ///0x2c - Burst Mode Period Register
    pub bmper: BMPER,
    ///0x30 - Timer External Event Control Register 1
    pub eecr1: EECR1,
    ///0x34 - Timer External Event Control Register 2
    pub eecr2: EECR2,
    ///0x38 - Timer External Event Control Register 3
    pub eecr3: EECR3,
    ///0x3c - ADC Trigger 1 Register
    pub adc1r: ADC1R,
    ///0x40 - ADC Trigger 2 Register
    pub adc2r: ADC2R,
    ///0x44 - ADC Trigger 3 Register
    pub adc3r: ADC3R,
    ///0x48 - ADC Trigger 4 Register
    pub adc4r: ADC4R,
    ///0x4c - DLL Control Register
    pub dllcr: DLLCR,
    ///0x50 - HRTIM Fault Input Register 1
    pub fltinr1: FLTINR1,
    ///0x54 - HRTIM Fault Input Register 2
    pub fltinr2: FLTINR2,
    ///0x58 - BDMUPDR
    pub bdmupdr: BDMUPDR,
    ///0x5c - Burst DMA Timerx update Register
    pub bdtaupr: BDTAUPR,
    ///0x60 - Burst DMA Timerx update Register
    pub bdtbupr: BDTBUPR,
    ///0x64 - Burst DMA Timerx update Register
    pub bdtcupr: BDTCUPR,
    ///0x68 - Burst DMA Timerx update Register
    pub bdtdupr: BDTDUPR,
    ///0x6c - Burst DMA Timerx update Register
    pub bdteupr: BDTEUPR,
    ///0x70 - Burst DMA Data Register
    pub bdmadr: BDMADR,
    ///0x74 - Burst DMA Timerx update Register
    pub bdtfupr: BDTFUPR,
    ///0x78 - HRTIM ADC Extended Trigger Register
    pub adcer: ADCER,
    ///0x7c - HRTIM ADC Trigger Update Register
    pub adcur: ADCUR,
    ///0x80 - HRTIM ADC Post Scaler Register 1
    pub adcps1: ADCPS1,
    ///0x84 - HRTIM ADC Post Scaler Register 2
    pub adcps2: ADCPS2,
    ///0x88 - HRTIM Fault Input Register 3
    pub fltinr3: FLTINR3,
    ///0x8c - HRTIM Fault Input Register 4
    pub fltinr4: FLTINR4,
}
///Control Register 1
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
///Control Register 1
pub mod cr1;
///Control Register 2
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
///Control Register 2
pub mod cr2;
///Interrupt Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr](isr) module
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
///`read()` method returns [isr::R](isr::R) reader structure
impl crate::Readable for ISR {}
///Interrupt Status Register
pub mod isr;
///Interrupt Clear Register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](icr) module
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
///`write(|w| ..)` method takes [icr::W](icr::W) writer structure
impl crate::Writable for ICR {}
///Interrupt Clear Register
pub mod icr;
///Interrupt Enable Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](ier) module
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
///`read()` method returns [ier::R](ier::R) reader structure
impl crate::Readable for IER {}
///`write(|w| ..)` method takes [ier::W](ier::W) writer structure
impl crate::Writable for IER {}
///Interrupt Enable Register
pub mod ier;
///Output Enable Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [oenr](oenr) module
pub type OENR = crate::Reg<u32, _OENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OENR;
///`read()` method returns [oenr::R](oenr::R) reader structure
impl crate::Readable for OENR {}
///`write(|w| ..)` method takes [oenr::W](oenr::W) writer structure
impl crate::Writable for OENR {}
///Output Enable Register
pub mod oenr;
///ODISR
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [odisr](odisr) module
pub type ODISR = crate::Reg<u32, _ODISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODISR;
///`write(|w| ..)` method takes [odisr::W](odisr::W) writer structure
impl crate::Writable for ODISR {}
///ODISR
pub mod odisr;
///Output Disable Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [odsr](odsr) module
pub type ODSR = crate::Reg<u32, _ODSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODSR;
///`read()` method returns [odsr::R](odsr::R) reader structure
impl crate::Readable for ODSR {}
///Output Disable Status Register
pub mod odsr;
///Burst Mode Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bmcr](bmcr) module
pub type BMCR = crate::Reg<u32, _BMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMCR;
///`read()` method returns [bmcr::R](bmcr::R) reader structure
impl crate::Readable for BMCR {}
///`write(|w| ..)` method takes [bmcr::W](bmcr::W) writer structure
impl crate::Writable for BMCR {}
///Burst Mode Control Register
pub mod bmcr;
///BMTRG
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bmtrg](bmtrg) module
pub type BMTRG = crate::Reg<u32, _BMTRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMTRG;
///`read()` method returns [bmtrg::R](bmtrg::R) reader structure
impl crate::Readable for BMTRG {}
///`write(|w| ..)` method takes [bmtrg::W](bmtrg::W) writer structure
impl crate::Writable for BMTRG {}
///BMTRG
pub mod bmtrg;
///BMCMPR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bmcmpr](bmcmpr) module
pub type BMCMPR = crate::Reg<u32, _BMCMPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMCMPR;
///`read()` method returns [bmcmpr::R](bmcmpr::R) reader structure
impl crate::Readable for BMCMPR {}
///`write(|w| ..)` method takes [bmcmpr::W](bmcmpr::W) writer structure
impl crate::Writable for BMCMPR {}
///BMCMPR
pub mod bmcmpr;
///Burst Mode Period Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bmper](bmper) module
pub type BMPER = crate::Reg<u32, _BMPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMPER;
///`read()` method returns [bmper::R](bmper::R) reader structure
impl crate::Readable for BMPER {}
///`write(|w| ..)` method takes [bmper::W](bmper::W) writer structure
impl crate::Writable for BMPER {}
///Burst Mode Period Register
pub mod bmper;
///Timer External Event Control Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eecr1](eecr1) module
pub type EECR1 = crate::Reg<u32, _EECR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EECR1;
///`read()` method returns [eecr1::R](eecr1::R) reader structure
impl crate::Readable for EECR1 {}
///`write(|w| ..)` method takes [eecr1::W](eecr1::W) writer structure
impl crate::Writable for EECR1 {}
///Timer External Event Control Register 1
pub mod eecr1;
///Timer External Event Control Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eecr2](eecr2) module
pub type EECR2 = crate::Reg<u32, _EECR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EECR2;
///`read()` method returns [eecr2::R](eecr2::R) reader structure
impl crate::Readable for EECR2 {}
///`write(|w| ..)` method takes [eecr2::W](eecr2::W) writer structure
impl crate::Writable for EECR2 {}
///Timer External Event Control Register 2
pub mod eecr2;
///Timer External Event Control Register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eecr3](eecr3) module
pub type EECR3 = crate::Reg<u32, _EECR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EECR3;
///`read()` method returns [eecr3::R](eecr3::R) reader structure
impl crate::Readable for EECR3 {}
///`write(|w| ..)` method takes [eecr3::W](eecr3::W) writer structure
impl crate::Writable for EECR3 {}
///Timer External Event Control Register 3
pub mod eecr3;
///ADC Trigger 1 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc1r](adc1r) module
pub type ADC1R = crate::Reg<u32, _ADC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1R;
///`read()` method returns [adc1r::R](adc1r::R) reader structure
impl crate::Readable for ADC1R {}
///`write(|w| ..)` method takes [adc1r::W](adc1r::W) writer structure
impl crate::Writable for ADC1R {}
///ADC Trigger 1 Register
pub mod adc1r;
///ADC Trigger 2 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc2r](adc2r) module
pub type ADC2R = crate::Reg<u32, _ADC2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC2R;
///`read()` method returns [adc2r::R](adc2r::R) reader structure
impl crate::Readable for ADC2R {}
///`write(|w| ..)` method takes [adc2r::W](adc2r::W) writer structure
impl crate::Writable for ADC2R {}
///ADC Trigger 2 Register
pub mod adc2r;
///ADC Trigger 3 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc3r](adc3r) module
pub type ADC3R = crate::Reg<u32, _ADC3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC3R;
///`read()` method returns [adc3r::R](adc3r::R) reader structure
impl crate::Readable for ADC3R {}
///`write(|w| ..)` method takes [adc3r::W](adc3r::W) writer structure
impl crate::Writable for ADC3R {}
///ADC Trigger 3 Register
pub mod adc3r;
///ADC Trigger 4 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc4r](adc4r) module
pub type ADC4R = crate::Reg<u32, _ADC4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC4R;
///`read()` method returns [adc4r::R](adc4r::R) reader structure
impl crate::Readable for ADC4R {}
///`write(|w| ..)` method takes [adc4r::W](adc4r::W) writer structure
impl crate::Writable for ADC4R {}
///ADC Trigger 4 Register
pub mod adc4r;
///DLL Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dllcr](dllcr) module
pub type DLLCR = crate::Reg<u32, _DLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLLCR;
///`read()` method returns [dllcr::R](dllcr::R) reader structure
impl crate::Readable for DLLCR {}
///`write(|w| ..)` method takes [dllcr::W](dllcr::W) writer structure
impl crate::Writable for DLLCR {}
///DLL Control Register
pub mod dllcr;
///HRTIM Fault Input Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fltinr1](fltinr1) module
pub type FLTINR1 = crate::Reg<u32, _FLTINR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTINR1;
///`read()` method returns [fltinr1::R](fltinr1::R) reader structure
impl crate::Readable for FLTINR1 {}
///`write(|w| ..)` method takes [fltinr1::W](fltinr1::W) writer structure
impl crate::Writable for FLTINR1 {}
///HRTIM Fault Input Register 1
pub mod fltinr1;
///HRTIM Fault Input Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fltinr2](fltinr2) module
pub type FLTINR2 = crate::Reg<u32, _FLTINR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTINR2;
///`read()` method returns [fltinr2::R](fltinr2::R) reader structure
impl crate::Readable for FLTINR2 {}
///`write(|w| ..)` method takes [fltinr2::W](fltinr2::W) writer structure
impl crate::Writable for FLTINR2 {}
///HRTIM Fault Input Register 2
pub mod fltinr2;
///BDMUPDR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdmupdr](bdmupdr) module
pub type BDMUPDR = crate::Reg<u32, _BDMUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMUPDR;
///`read()` method returns [bdmupdr::R](bdmupdr::R) reader structure
impl crate::Readable for BDMUPDR {}
///`write(|w| ..)` method takes [bdmupdr::W](bdmupdr::W) writer structure
impl crate::Writable for BDMUPDR {}
///BDMUPDR
pub mod bdmupdr;
///Burst DMA Timerx update Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdtaupr](bdtaupr) module
pub type BDTAUPR = crate::Reg<u32, _BDTAUPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTAUPR;
///`read()` method returns [bdtaupr::R](bdtaupr::R) reader structure
impl crate::Readable for BDTAUPR {}
///`write(|w| ..)` method takes [bdtaupr::W](bdtaupr::W) writer structure
impl crate::Writable for BDTAUPR {}
///Burst DMA Timerx update Register
pub mod bdtaupr;
///Burst DMA Timerx update Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdtbupr](bdtbupr) module
pub type BDTBUPR = crate::Reg<u32, _BDTBUPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTBUPR;
///`read()` method returns [bdtbupr::R](bdtbupr::R) reader structure
impl crate::Readable for BDTBUPR {}
///`write(|w| ..)` method takes [bdtbupr::W](bdtbupr::W) writer structure
impl crate::Writable for BDTBUPR {}
///Burst DMA Timerx update Register
pub mod bdtbupr;
///Burst DMA Timerx update Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdtcupr](bdtcupr) module
pub type BDTCUPR = crate::Reg<u32, _BDTCUPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTCUPR;
///`read()` method returns [bdtcupr::R](bdtcupr::R) reader structure
impl crate::Readable for BDTCUPR {}
///`write(|w| ..)` method takes [bdtcupr::W](bdtcupr::W) writer structure
impl crate::Writable for BDTCUPR {}
///Burst DMA Timerx update Register
pub mod bdtcupr;
///Burst DMA Timerx update Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdtdupr](bdtdupr) module
pub type BDTDUPR = crate::Reg<u32, _BDTDUPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTDUPR;
///`read()` method returns [bdtdupr::R](bdtdupr::R) reader structure
impl crate::Readable for BDTDUPR {}
///`write(|w| ..)` method takes [bdtdupr::W](bdtdupr::W) writer structure
impl crate::Writable for BDTDUPR {}
///Burst DMA Timerx update Register
pub mod bdtdupr;
///Burst DMA Timerx update Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdteupr](bdteupr) module
pub type BDTEUPR = crate::Reg<u32, _BDTEUPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTEUPR;
///`read()` method returns [bdteupr::R](bdteupr::R) reader structure
impl crate::Readable for BDTEUPR {}
///`write(|w| ..)` method takes [bdteupr::W](bdteupr::W) writer structure
impl crate::Writable for BDTEUPR {}
///Burst DMA Timerx update Register
pub mod bdteupr;
///Burst DMA Timerx update Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdtfupr](bdtfupr) module
pub type BDTFUPR = crate::Reg<u32, _BDTFUPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTFUPR;
///`read()` method returns [bdtfupr::R](bdtfupr::R) reader structure
impl crate::Readable for BDTFUPR {}
///`write(|w| ..)` method takes [bdtfupr::W](bdtfupr::W) writer structure
impl crate::Writable for BDTFUPR {}
///Burst DMA Timerx update Register
pub mod bdtfupr;
///Burst DMA Data Register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdmadr](bdmadr) module
pub type BDMADR = crate::Reg<u32, _BDMADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMADR;
///`write(|w| ..)` method takes [bdmadr::W](bdmadr::W) writer structure
impl crate::Writable for BDMADR {}
///Burst DMA Data Register
pub mod bdmadr;
///HRTIM ADC Extended Trigger Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adcer](adcer) module
pub type ADCER = crate::Reg<u32, _ADCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCER;
///`read()` method returns [adcer::R](adcer::R) reader structure
impl crate::Readable for ADCER {}
///`write(|w| ..)` method takes [adcer::W](adcer::W) writer structure
impl crate::Writable for ADCER {}
///HRTIM ADC Extended Trigger Register
pub mod adcer;
///HRTIM ADC Trigger Update Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adcur](adcur) module
pub type ADCUR = crate::Reg<u32, _ADCUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCUR;
///`read()` method returns [adcur::R](adcur::R) reader structure
impl crate::Readable for ADCUR {}
///`write(|w| ..)` method takes [adcur::W](adcur::W) writer structure
impl crate::Writable for ADCUR {}
///HRTIM ADC Trigger Update Register
pub mod adcur;
///HRTIM ADC Post Scaler Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adcps1](adcps1) module
pub type ADCPS1 = crate::Reg<u32, _ADCPS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCPS1;
///`read()` method returns [adcps1::R](adcps1::R) reader structure
impl crate::Readable for ADCPS1 {}
///`write(|w| ..)` method takes [adcps1::W](adcps1::W) writer structure
impl crate::Writable for ADCPS1 {}
///HRTIM ADC Post Scaler Register 1
pub mod adcps1;
///HRTIM ADC Post Scaler Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adcps2](adcps2) module
pub type ADCPS2 = crate::Reg<u32, _ADCPS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCPS2;
///`read()` method returns [adcps2::R](adcps2::R) reader structure
impl crate::Readable for ADCPS2 {}
///`write(|w| ..)` method takes [adcps2::W](adcps2::W) writer structure
impl crate::Writable for ADCPS2 {}
///HRTIM ADC Post Scaler Register 2
pub mod adcps2;
///HRTIM Fault Input Register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fltinr3](fltinr3) module
pub type FLTINR3 = crate::Reg<u32, _FLTINR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTINR3;
///`read()` method returns [fltinr3::R](fltinr3::R) reader structure
impl crate::Readable for FLTINR3 {}
///`write(|w| ..)` method takes [fltinr3::W](fltinr3::W) writer structure
impl crate::Writable for FLTINR3 {}
///HRTIM Fault Input Register 3
pub mod fltinr3;
///HRTIM Fault Input Register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fltinr4](fltinr4) module
pub type FLTINR4 = crate::Reg<u32, _FLTINR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTINR4;
///`read()` method returns [fltinr4::R](fltinr4::R) reader structure
impl crate::Readable for FLTINR4 {}
///`write(|w| ..)` method takes [fltinr4::W](fltinr4::W) writer structure
impl crate::Writable for FLTINR4 {}
///HRTIM Fault Input Register 4
pub mod fltinr4;
