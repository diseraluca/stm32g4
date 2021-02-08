///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OPAMP1 control/status register
    pub opamp1_csr: OPAMP1_CSR,
    ///0x04 - OPAMP2 control/status register
    pub opamp2_csr: OPAMP2_CSR,
    ///0x08 - OPAMP3 control/status register
    pub opamp3_csr: OPAMP3_CSR,
    _reserved3: [u8; 12usize],
    ///0x18 - OPAMP1 control/status register
    pub opamp1_tcmr: OPAMP1_TCMR,
    ///0x1c - OPAMP2 control/status register
    pub opamp2_tcmr: OPAMP2_TCMR,
    ///0x20 - OPAMP3 control/status register
    pub opamp3_tcmr: OPAMP3_TCMR,
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
