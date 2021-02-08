///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - CORDIC Control Status register
    pub csr: CSR,
    ///0x04 - FMAC Write Data register
    pub wdata: WDATA,
    ///0x08 - FMAC Read Data register
    pub rdata: RDATA,
}
///CORDIC Control Status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](csr) module
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
///`read()` method returns [csr::R](csr::R) reader structure
impl crate::Readable for CSR {}
///`write(|w| ..)` method takes [csr::W](csr::W) writer structure
impl crate::Writable for CSR {}
///CORDIC Control Status register
pub mod csr;
///FMAC Write Data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wdata](wdata) module
pub type WDATA = crate::Reg<u32, _WDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDATA;
///`read()` method returns [wdata::R](wdata::R) reader structure
impl crate::Readable for WDATA {}
///`write(|w| ..)` method takes [wdata::W](wdata::W) writer structure
impl crate::Writable for WDATA {}
///FMAC Write Data register
pub mod wdata;
///FMAC Read Data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rdata](rdata) module
pub type RDATA = crate::Reg<u32, _RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDATA;
///`read()` method returns [rdata::R](rdata::R) reader structure
impl crate::Readable for RDATA {}
///FMAC Read Data register
pub mod rdata;
