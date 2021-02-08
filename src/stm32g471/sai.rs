///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    ///0x04 - AConfiguration register 1
    pub acr1: ACR1,
    ///0x08 - AConfiguration register 2
    pub acr2: ACR2,
    ///0x0c - AFRCR
    pub afrcr: AFRCR,
    ///0x10 - ASlot register
    pub aslotr: ASLOTR,
    ///0x14 - AInterrupt mask register2
    pub aim: AIM,
    ///0x18 - AStatus register
    pub asr: ASR,
    ///0x1c - AClear flag register
    pub aclrfr: ACLRFR,
    ///0x20 - AData register
    pub adr: ADR,
    ///0x24 - BConfiguration register 1
    pub bcr1: BCR1,
    ///0x28 - BConfiguration register 2
    pub bcr2: BCR2,
    ///0x2c - BFRCR
    pub bfrcr: BFRCR,
    ///0x30 - BSlot register
    pub bslotr: BSLOTR,
    ///0x34 - BInterrupt mask register2
    pub bim: BIM,
    ///0x38 - BStatus register
    pub bsr: BSR,
    ///0x3c - BClear flag register
    pub bclrfr: BCLRFR,
    ///0x40 - BData register
    pub bdr: BDR,
    ///0x44 - PDM control register
    pub pdmcr: PDMCR,
    ///0x48 - PDM delay register
    pub pdmdly: PDMDLY,
}
///BConfiguration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bcr1](bcr1) module
pub type BCR1 = crate::Reg<u32, _BCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR1;
///`read()` method returns [bcr1::R](bcr1::R) reader structure
impl crate::Readable for BCR1 {}
///`write(|w| ..)` method takes [bcr1::W](bcr1::W) writer structure
impl crate::Writable for BCR1 {}
///BConfiguration register 1
pub mod bcr1;
///BConfiguration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bcr2](bcr2) module
pub type BCR2 = crate::Reg<u32, _BCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR2;
///`read()` method returns [bcr2::R](bcr2::R) reader structure
impl crate::Readable for BCR2 {}
///`write(|w| ..)` method takes [bcr2::W](bcr2::W) writer structure
impl crate::Writable for BCR2 {}
///BConfiguration register 2
pub mod bcr2;
///BFRCR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bfrcr](bfrcr) module
pub type BFRCR = crate::Reg<u32, _BFRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFRCR;
///`read()` method returns [bfrcr::R](bfrcr::R) reader structure
impl crate::Readable for BFRCR {}
///`write(|w| ..)` method takes [bfrcr::W](bfrcr::W) writer structure
impl crate::Writable for BFRCR {}
///BFRCR
pub mod bfrcr;
///BSlot register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bslotr](bslotr) module
pub type BSLOTR = crate::Reg<u32, _BSLOTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSLOTR;
///`read()` method returns [bslotr::R](bslotr::R) reader structure
impl crate::Readable for BSLOTR {}
///`write(|w| ..)` method takes [bslotr::W](bslotr::W) writer structure
impl crate::Writable for BSLOTR {}
///BSlot register
pub mod bslotr;
///BInterrupt mask register2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bim](bim) module
pub type BIM = crate::Reg<u32, _BIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIM;
///`read()` method returns [bim::R](bim::R) reader structure
impl crate::Readable for BIM {}
///`write(|w| ..)` method takes [bim::W](bim::W) writer structure
impl crate::Writable for BIM {}
///BInterrupt mask register2
pub mod bim;
///BStatus register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bsr](bsr) module
pub type BSR = crate::Reg<u32, _BSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSR;
///`read()` method returns [bsr::R](bsr::R) reader structure
impl crate::Readable for BSR {}
///BStatus register
pub mod bsr;
///BClear flag register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bclrfr](bclrfr) module
pub type BCLRFR = crate::Reg<u32, _BCLRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCLRFR;
///`write(|w| ..)` method takes [bclrfr::W](bclrfr::W) writer structure
impl crate::Writable for BCLRFR {}
///BClear flag register
pub mod bclrfr;
///BData register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdr](bdr) module
pub type BDR = crate::Reg<u32, _BDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDR;
///`read()` method returns [bdr::R](bdr::R) reader structure
impl crate::Readable for BDR {}
///`write(|w| ..)` method takes [bdr::W](bdr::W) writer structure
impl crate::Writable for BDR {}
///BData register
pub mod bdr;
///AConfiguration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [acr1](acr1) module
pub type ACR1 = crate::Reg<u32, _ACR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR1;
///`read()` method returns [acr1::R](acr1::R) reader structure
impl crate::Readable for ACR1 {}
///`write(|w| ..)` method takes [acr1::W](acr1::W) writer structure
impl crate::Writable for ACR1 {}
///AConfiguration register 1
pub mod acr1;
///AConfiguration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [acr2](acr2) module
pub type ACR2 = crate::Reg<u32, _ACR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR2;
///`read()` method returns [acr2::R](acr2::R) reader structure
impl crate::Readable for ACR2 {}
///`write(|w| ..)` method takes [acr2::W](acr2::W) writer structure
impl crate::Writable for ACR2 {}
///AConfiguration register 2
pub mod acr2;
///AFRCR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [afrcr](afrcr) module
pub type AFRCR = crate::Reg<u32, _AFRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFRCR;
///`read()` method returns [afrcr::R](afrcr::R) reader structure
impl crate::Readable for AFRCR {}
///`write(|w| ..)` method takes [afrcr::W](afrcr::W) writer structure
impl crate::Writable for AFRCR {}
///AFRCR
pub mod afrcr;
///ASlot register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aslotr](aslotr) module
pub type ASLOTR = crate::Reg<u32, _ASLOTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASLOTR;
///`read()` method returns [aslotr::R](aslotr::R) reader structure
impl crate::Readable for ASLOTR {}
///`write(|w| ..)` method takes [aslotr::W](aslotr::W) writer structure
impl crate::Writable for ASLOTR {}
///ASlot register
pub mod aslotr;
///AInterrupt mask register2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aim](aim) module
pub type AIM = crate::Reg<u32, _AIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIM;
///`read()` method returns [aim::R](aim::R) reader structure
impl crate::Readable for AIM {}
///`write(|w| ..)` method takes [aim::W](aim::W) writer structure
impl crate::Writable for AIM {}
///AInterrupt mask register2
pub mod aim;
///AStatus register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [asr](asr) module
pub type ASR = crate::Reg<u32, _ASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASR;
///`read()` method returns [asr::R](asr::R) reader structure
impl crate::Readable for ASR {}
///`write(|w| ..)` method takes [asr::W](asr::W) writer structure
impl crate::Writable for ASR {}
///AStatus register
pub mod asr;
///AClear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aclrfr](aclrfr) module
pub type ACLRFR = crate::Reg<u32, _ACLRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACLRFR;
///`read()` method returns [aclrfr::R](aclrfr::R) reader structure
impl crate::Readable for ACLRFR {}
///`write(|w| ..)` method takes [aclrfr::W](aclrfr::W) writer structure
impl crate::Writable for ACLRFR {}
///AClear flag register
pub mod aclrfr;
///AData register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adr](adr) module
pub type ADR = crate::Reg<u32, _ADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADR;
///`read()` method returns [adr::R](adr::R) reader structure
impl crate::Readable for ADR {}
///`write(|w| ..)` method takes [adr::W](adr::W) writer structure
impl crate::Writable for ADR {}
///AData register
pub mod adr;
///PDM control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdmcr](pdmcr) module
pub type PDMCR = crate::Reg<u32, _PDMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMCR;
///`read()` method returns [pdmcr::R](pdmcr::R) reader structure
impl crate::Readable for PDMCR {}
///`write(|w| ..)` method takes [pdmcr::W](pdmcr::W) writer structure
impl crate::Writable for PDMCR {}
///PDM control register
pub mod pdmcr;
///PDM delay register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdmdly](pdmdly) module
pub type PDMDLY = crate::Reg<u32, _PDMDLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMDLY;
///`read()` method returns [pdmdly::R](pdmdly::R) reader structure
impl crate::Readable for PDMDLY {}
///`write(|w| ..)` method takes [pdmdly::W](pdmdly::W) writer structure
impl crate::Writable for PDMDLY {}
///PDM delay register
pub mod pdmdly;
