///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Comparator control/status register
    pub comp_c1csr: COMP_C1CSR,
    ///0x04 - Comparator control/status register
    pub comp_c2csr: COMP_C2CSR,
    ///0x08 - Comparator control/status register
    pub comp_c3csr: COMP_C3CSR,
    ///0x0c - Comparator control/status register
    pub comp_c4csr: COMP_C4CSR,
    ///0x10 - Comparator control/status register
    pub comp_c5csr: COMP_C5CSR,
    ///0x14 - Comparator control/status register
    pub comp_c6csr: COMP_C6CSR,
    ///0x18 - Comparator control/status register
    pub comp_c7csr: COMP_C7CSR,
}
///Comparator control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [comp_c1csr](comp_c1csr) module
pub type COMP_C1CSR = crate::Reg<u32, _COMP_C1CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_C1CSR;
///`read()` method returns [comp_c1csr::R](comp_c1csr::R) reader structure
impl crate::Readable for COMP_C1CSR {}
///`write(|w| ..)` method takes [comp_c1csr::W](comp_c1csr::W) writer structure
impl crate::Writable for COMP_C1CSR {}
///Comparator control/status register
pub mod comp_c1csr;
///Comparator control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [comp_c2csr](comp_c2csr) module
pub type COMP_C2CSR = crate::Reg<u32, _COMP_C2CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_C2CSR;
///`read()` method returns [comp_c2csr::R](comp_c2csr::R) reader structure
impl crate::Readable for COMP_C2CSR {}
///`write(|w| ..)` method takes [comp_c2csr::W](comp_c2csr::W) writer structure
impl crate::Writable for COMP_C2CSR {}
///Comparator control/status register
pub mod comp_c2csr;
///Comparator control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [comp_c3csr](comp_c3csr) module
pub type COMP_C3CSR = crate::Reg<u32, _COMP_C3CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_C3CSR;
///`read()` method returns [comp_c3csr::R](comp_c3csr::R) reader structure
impl crate::Readable for COMP_C3CSR {}
///`write(|w| ..)` method takes [comp_c3csr::W](comp_c3csr::W) writer structure
impl crate::Writable for COMP_C3CSR {}
///Comparator control/status register
pub mod comp_c3csr;
///Comparator control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [comp_c4csr](comp_c4csr) module
pub type COMP_C4CSR = crate::Reg<u32, _COMP_C4CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_C4CSR;
///`read()` method returns [comp_c4csr::R](comp_c4csr::R) reader structure
impl crate::Readable for COMP_C4CSR {}
///`write(|w| ..)` method takes [comp_c4csr::W](comp_c4csr::W) writer structure
impl crate::Writable for COMP_C4CSR {}
///Comparator control/status register
pub mod comp_c4csr;
///Comparator control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [comp_c5csr](comp_c5csr) module
pub type COMP_C5CSR = crate::Reg<u32, _COMP_C5CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_C5CSR;
///`read()` method returns [comp_c5csr::R](comp_c5csr::R) reader structure
impl crate::Readable for COMP_C5CSR {}
///`write(|w| ..)` method takes [comp_c5csr::W](comp_c5csr::W) writer structure
impl crate::Writable for COMP_C5CSR {}
///Comparator control/status register
pub mod comp_c5csr;
///Comparator control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [comp_c6csr](comp_c6csr) module
pub type COMP_C6CSR = crate::Reg<u32, _COMP_C6CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_C6CSR;
///`read()` method returns [comp_c6csr::R](comp_c6csr::R) reader structure
impl crate::Readable for COMP_C6CSR {}
///`write(|w| ..)` method takes [comp_c6csr::W](comp_c6csr::W) writer structure
impl crate::Writable for COMP_C6CSR {}
///Comparator control/status register
pub mod comp_c6csr;
///Comparator control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [comp_c7csr](comp_c7csr) module
pub type COMP_C7CSR = crate::Reg<u32, _COMP_C7CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_C7CSR;
///`read()` method returns [comp_c7csr::R](comp_c7csr::R) reader structure
impl crate::Readable for COMP_C7CSR {}
///`write(|w| ..)` method takes [comp_c7csr::W](comp_c7csr::W) writer structure
impl crate::Writable for COMP_C7CSR {}
///Comparator control/status register
pub mod comp_c7csr;
