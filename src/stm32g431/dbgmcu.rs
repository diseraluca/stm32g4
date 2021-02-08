///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - MCU Device ID Code Register
    pub idcode: IDCODE,
    ///0x04 - Debug MCU Configuration Register
    pub cr: CR,
    ///0x08 - APB Low Freeze Register 1
    pub apb1l_fz: APB1L_FZ,
    ///0x0c - APB Low Freeze Register 2
    pub apb1h_fz: APB1H_FZ,
    ///0x10 - APB High Freeze Register
    pub apb2_fz: APB2_FZ,
}
///MCU Device ID Code Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [idcode](idcode) module
pub type IDCODE = crate::Reg<u32, _IDCODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDCODE;
///`read()` method returns [idcode::R](idcode::R) reader structure
impl crate::Readable for IDCODE {}
///MCU Device ID Code Register
pub mod idcode;
///Debug MCU Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](cr) module
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
///`read()` method returns [cr::R](cr::R) reader structure
impl crate::Readable for CR {}
///`write(|w| ..)` method takes [cr::W](cr::W) writer structure
impl crate::Writable for CR {}
///Debug MCU Configuration Register
pub mod cr;
///APB Low Freeze Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1l_fz](apb1l_fz) module
pub type APB1L_FZ = crate::Reg<u32, _APB1L_FZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1L_FZ;
///`read()` method returns [apb1l_fz::R](apb1l_fz::R) reader structure
impl crate::Readable for APB1L_FZ {}
///`write(|w| ..)` method takes [apb1l_fz::W](apb1l_fz::W) writer structure
impl crate::Writable for APB1L_FZ {}
///APB Low Freeze Register 1
pub mod apb1l_fz;
///APB Low Freeze Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1h_fz](apb1h_fz) module
pub type APB1H_FZ = crate::Reg<u32, _APB1H_FZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1H_FZ;
///`read()` method returns [apb1h_fz::R](apb1h_fz::R) reader structure
impl crate::Readable for APB1H_FZ {}
///`write(|w| ..)` method takes [apb1h_fz::W](apb1h_fz::W) writer structure
impl crate::Writable for APB1H_FZ {}
///APB Low Freeze Register 2
pub mod apb1h_fz;
///APB High Freeze Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb2_fz](apb2_fz) module
pub type APB2_FZ = crate::Reg<u32, _APB2_FZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2_FZ;
///`read()` method returns [apb2_fz::R](apb2_fz::R) reader structure
impl crate::Readable for APB2_FZ {}
///`write(|w| ..)` method takes [apb2_fz::W](apb2_fz::W) writer structure
impl crate::Writable for APB2_FZ {}
///APB High Freeze Register
pub mod apb2_fz;
