///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - FMAC X1 Buffer Configuration register
    pub x1bufcfg: X1BUFCFG,
    ///0x04 - FMAC X2 Buffer Configuration register
    pub x2bufcfg: X2BUFCFG,
    ///0x08 - FMAC Y Buffer Configuration register
    pub ybufcfg: YBUFCFG,
    ///0x0c - FMAC Parameter register
    pub param: PARAM,
    ///0x10 - FMAC Control register
    pub cr: CR,
    ///0x14 - FMAC Status register
    pub sr: SR,
    ///0x18 - FMAC Write Data register
    pub wdata: WDATA,
    ///0x1c - FMAC Read Data register
    pub rdata: RDATA,
}
///FMAC X1 Buffer Configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [x1bufcfg](x1bufcfg) module
pub type X1BUFCFG = crate::Reg<u32, _X1BUFCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _X1BUFCFG;
///`read()` method returns [x1bufcfg::R](x1bufcfg::R) reader structure
impl crate::Readable for X1BUFCFG {}
///`write(|w| ..)` method takes [x1bufcfg::W](x1bufcfg::W) writer structure
impl crate::Writable for X1BUFCFG {}
///FMAC X1 Buffer Configuration register
pub mod x1bufcfg;
///FMAC X2 Buffer Configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [x2bufcfg](x2bufcfg) module
pub type X2BUFCFG = crate::Reg<u32, _X2BUFCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _X2BUFCFG;
///`read()` method returns [x2bufcfg::R](x2bufcfg::R) reader structure
impl crate::Readable for X2BUFCFG {}
///`write(|w| ..)` method takes [x2bufcfg::W](x2bufcfg::W) writer structure
impl crate::Writable for X2BUFCFG {}
///FMAC X2 Buffer Configuration register
pub mod x2bufcfg;
///FMAC Y Buffer Configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ybufcfg](ybufcfg) module
pub type YBUFCFG = crate::Reg<u32, _YBUFCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _YBUFCFG;
///`read()` method returns [ybufcfg::R](ybufcfg::R) reader structure
impl crate::Readable for YBUFCFG {}
///`write(|w| ..)` method takes [ybufcfg::W](ybufcfg::W) writer structure
impl crate::Writable for YBUFCFG {}
///FMAC Y Buffer Configuration register
pub mod ybufcfg;
///FMAC Parameter register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [param](param) module
pub type PARAM = crate::Reg<u32, _PARAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAM;
///`read()` method returns [param::R](param::R) reader structure
impl crate::Readable for PARAM {}
///`write(|w| ..)` method takes [param::W](param::W) writer structure
impl crate::Writable for PARAM {}
///FMAC Parameter register
pub mod param;
///FMAC Control register
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
///FMAC Control register
pub mod cr;
///FMAC Status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](sr) module
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
///`read()` method returns [sr::R](sr::R) reader structure
impl crate::Readable for SR {}
///FMAC Status register
pub mod sr;
///FMAC Write Data register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wdata](wdata) module
pub type WDATA = crate::Reg<u32, _WDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDATA;
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
