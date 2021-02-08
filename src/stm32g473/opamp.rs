///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OPAMP1 control/status register
    pub opamp1_csr: OPAMP1_CSR,
    ///0x04 - OPAMP2 control/status register
    pub opamp2_csr: OPAMP2_CSR,
    ///0x08 - OPAMP3 control/status register
    pub opamp3_csr: OPAMP3_CSR,
    ///0x0c - OPAMP4 control/status register
    pub opamp4_csr: OPAMP4_CSR,
    ///0x10 - OPAMP5 control/status register
    pub opamp5_csr: OPAMP5_CSR,
    ///0x14 - OPAMP6 control/status register
    pub opamp6_csr: OPAMP6_CSR,
    ///0x18 - OPAMP1 control/status register
    pub opamp1_tcmr: OPAMP1_TCMR,
    ///0x1c - OPAMP2 control/status register
    pub opamp2_tcmr: OPAMP2_TCMR,
    ///0x20 - OPAMP3 control/status register
    pub opamp3_tcmr: OPAMP3_TCMR,
    ///0x24 - OPAMP4 control/status register
    pub opamp4_tcmr: OPAMP4_TCMR,
    ///0x28 - OPAMP5 control/status register
    pub opamp5_tcmr: OPAMP5_TCMR,
    ///0x2c - OPAMP6 control/status register
    pub opamp6_tcmr: OPAMP6_TCMR,
}
///OPAMP1 control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp1_csr](opamp1_csr) module
pub type OPAMP1_CSR = crate::Reg<u32, _OPAMP1_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP1_CSR;
///`read()` method returns [opamp1_csr::R](opamp1_csr::R) reader structure
impl crate::Readable for OPAMP1_CSR {}
///`write(|w| ..)` method takes [opamp1_csr::W](opamp1_csr::W) writer structure
impl crate::Writable for OPAMP1_CSR {}
///OPAMP1 control/status register
pub mod opamp1_csr;
///OPAMP2 control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp2_csr](opamp2_csr) module
pub type OPAMP2_CSR = crate::Reg<u32, _OPAMP2_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP2_CSR;
///`read()` method returns [opamp2_csr::R](opamp2_csr::R) reader structure
impl crate::Readable for OPAMP2_CSR {}
///`write(|w| ..)` method takes [opamp2_csr::W](opamp2_csr::W) writer structure
impl crate::Writable for OPAMP2_CSR {}
///OPAMP2 control/status register
pub mod opamp2_csr;
///OPAMP3 control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp3_csr](opamp3_csr) module
pub type OPAMP3_CSR = crate::Reg<u32, _OPAMP3_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP3_CSR;
///`read()` method returns [opamp3_csr::R](opamp3_csr::R) reader structure
impl crate::Readable for OPAMP3_CSR {}
///`write(|w| ..)` method takes [opamp3_csr::W](opamp3_csr::W) writer structure
impl crate::Writable for OPAMP3_CSR {}
///OPAMP3 control/status register
pub mod opamp3_csr;
///OPAMP4 control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp4_csr](opamp4_csr) module
pub type OPAMP4_CSR = crate::Reg<u32, _OPAMP4_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP4_CSR;
///`read()` method returns [opamp4_csr::R](opamp4_csr::R) reader structure
impl crate::Readable for OPAMP4_CSR {}
///`write(|w| ..)` method takes [opamp4_csr::W](opamp4_csr::W) writer structure
impl crate::Writable for OPAMP4_CSR {}
///OPAMP4 control/status register
pub mod opamp4_csr;
///OPAMP5 control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp5_csr](opamp5_csr) module
pub type OPAMP5_CSR = crate::Reg<u32, _OPAMP5_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP5_CSR;
///`read()` method returns [opamp5_csr::R](opamp5_csr::R) reader structure
impl crate::Readable for OPAMP5_CSR {}
///`write(|w| ..)` method takes [opamp5_csr::W](opamp5_csr::W) writer structure
impl crate::Writable for OPAMP5_CSR {}
///OPAMP5 control/status register
pub mod opamp5_csr;
///OPAMP6 control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp6_csr](opamp6_csr) module
pub type OPAMP6_CSR = crate::Reg<u32, _OPAMP6_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP6_CSR;
///`read()` method returns [opamp6_csr::R](opamp6_csr::R) reader structure
impl crate::Readable for OPAMP6_CSR {}
///`write(|w| ..)` method takes [opamp6_csr::W](opamp6_csr::W) writer structure
impl crate::Writable for OPAMP6_CSR {}
///OPAMP6 control/status register
pub mod opamp6_csr;
///OPAMP1 control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp1_tcmr](opamp1_tcmr) module
pub type OPAMP1_TCMR = crate::Reg<u32, _OPAMP1_TCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP1_TCMR;
///`read()` method returns [opamp1_tcmr::R](opamp1_tcmr::R) reader structure
impl crate::Readable for OPAMP1_TCMR {}
///`write(|w| ..)` method takes [opamp1_tcmr::W](opamp1_tcmr::W) writer structure
impl crate::Writable for OPAMP1_TCMR {}
///OPAMP1 control/status register
pub mod opamp1_tcmr;
///OPAMP2 control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp2_tcmr](opamp2_tcmr) module
pub type OPAMP2_TCMR = crate::Reg<u32, _OPAMP2_TCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP2_TCMR;
///`read()` method returns [opamp2_tcmr::R](opamp2_tcmr::R) reader structure
impl crate::Readable for OPAMP2_TCMR {}
///`write(|w| ..)` method takes [opamp2_tcmr::W](opamp2_tcmr::W) writer structure
impl crate::Writable for OPAMP2_TCMR {}
///OPAMP2 control/status register
pub mod opamp2_tcmr;
///OPAMP3 control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp3_tcmr](opamp3_tcmr) module
pub type OPAMP3_TCMR = crate::Reg<u32, _OPAMP3_TCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP3_TCMR;
///`read()` method returns [opamp3_tcmr::R](opamp3_tcmr::R) reader structure
impl crate::Readable for OPAMP3_TCMR {}
///`write(|w| ..)` method takes [opamp3_tcmr::W](opamp3_tcmr::W) writer structure
impl crate::Writable for OPAMP3_TCMR {}
///OPAMP3 control/status register
pub mod opamp3_tcmr;
///OPAMP4 control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp4_tcmr](opamp4_tcmr) module
pub type OPAMP4_TCMR = crate::Reg<u32, _OPAMP4_TCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP4_TCMR;
///`read()` method returns [opamp4_tcmr::R](opamp4_tcmr::R) reader structure
impl crate::Readable for OPAMP4_TCMR {}
///`write(|w| ..)` method takes [opamp4_tcmr::W](opamp4_tcmr::W) writer structure
impl crate::Writable for OPAMP4_TCMR {}
///OPAMP4 control/status register
pub mod opamp4_tcmr;
///OPAMP5 control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp5_tcmr](opamp5_tcmr) module
pub type OPAMP5_TCMR = crate::Reg<u32, _OPAMP5_TCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP5_TCMR;
///`read()` method returns [opamp5_tcmr::R](opamp5_tcmr::R) reader structure
impl crate::Readable for OPAMP5_TCMR {}
///`write(|w| ..)` method takes [opamp5_tcmr::W](opamp5_tcmr::W) writer structure
impl crate::Writable for OPAMP5_TCMR {}
///OPAMP5 control/status register
pub mod opamp5_tcmr;
///OPAMP6 control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp6_tcmr](opamp6_tcmr) module
pub type OPAMP6_TCMR = crate::Reg<u32, _OPAMP6_TCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP6_TCMR;
///`read()` method returns [opamp6_tcmr::R](opamp6_tcmr::R) reader structure
impl crate::Readable for OPAMP6_TCMR {}
///`write(|w| ..)` method takes [opamp6_tcmr::W](opamp6_tcmr::W) writer structure
impl crate::Writable for OPAMP6_TCMR {}
///OPAMP6 control/status register
pub mod opamp6_tcmr;
