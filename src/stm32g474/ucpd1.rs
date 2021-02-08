///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - UCPD configuration register 1
    pub cfg1: CFG1,
    ///0x04 - UCPD configuration register 2
    pub cfg2: CFG2,
    _reserved2: [u8; 4usize],
    ///0x0c - UCPD configuration register 2
    pub cr: CR,
    ///0x10 - UCPD Interrupt Mask Register
    pub imr: IMR,
    ///0x14 - UCPD Status Register
    pub sr: SR,
    ///0x18 - UCPD Interrupt Clear Register
    pub icr: ICR,
    ///0x1c - UCPD Tx Ordered Set Type Register
    pub tx_ordset: TX_ORDSET,
    ///0x20 - UCPD Tx Paysize Register
    pub tx_paysz: TX_PAYSZ,
    ///0x24 - UCPD Tx Data Register
    pub txdr: TXDR,
    ///0x28 - UCPD Rx Ordered Set Register
    pub rx_ordset: RX_ORDSET,
    ///0x2c - UCPD Rx Paysize Register
    pub rx_paysz: RX_PAYSZ,
    ///0x30 - UCPD Rx Data Register
    pub rxdr: RXDR,
    ///0x34 - UCPD Rx Ordered Set Extension Register 1
    pub rx_ordext1: RX_ORDEXT1,
    ///0x38 - UCPD Rx Ordered Set Extension Register 2
    pub rx_ordext2: RX_ORDEXT2,
}
///UCPD configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfg1](cfg1) module
pub type CFG1 = crate::Reg<u32, _CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG1;
///`read()` method returns [cfg1::R](cfg1::R) reader structure
impl crate::Readable for CFG1 {}
///`write(|w| ..)` method takes [cfg1::W](cfg1::W) writer structure
impl crate::Writable for CFG1 {}
///UCPD configuration register 1
pub mod cfg1;
///UCPD configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfg2](cfg2) module
pub type CFG2 = crate::Reg<u32, _CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG2;
///`read()` method returns [cfg2::R](cfg2::R) reader structure
impl crate::Readable for CFG2 {}
///`write(|w| ..)` method takes [cfg2::W](cfg2::W) writer structure
impl crate::Writable for CFG2 {}
///UCPD configuration register 2
pub mod cfg2;
///UCPD configuration register 2
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
///UCPD configuration register 2
pub mod cr;
///UCPD Interrupt Mask Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [imr](imr) module
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
///`read()` method returns [imr::R](imr::R) reader structure
impl crate::Readable for IMR {}
///`write(|w| ..)` method takes [imr::W](imr::W) writer structure
impl crate::Writable for IMR {}
///UCPD Interrupt Mask Register
pub mod imr;
///UCPD Status Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](sr) module
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
///`read()` method returns [sr::R](sr::R) reader structure
impl crate::Readable for SR {}
///`write(|w| ..)` method takes [sr::W](sr::W) writer structure
impl crate::Writable for SR {}
///UCPD Status Register
pub mod sr;
///UCPD Interrupt Clear Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](icr) module
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
///`read()` method returns [icr::R](icr::R) reader structure
impl crate::Readable for ICR {}
///`write(|w| ..)` method takes [icr::W](icr::W) writer structure
impl crate::Writable for ICR {}
///UCPD Interrupt Clear Register
pub mod icr;
///UCPD Tx Ordered Set Type Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tx_ordset](tx_ordset) module
pub type TX_ORDSET = crate::Reg<u32, _TX_ORDSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_ORDSET;
///`read()` method returns [tx_ordset::R](tx_ordset::R) reader structure
impl crate::Readable for TX_ORDSET {}
///`write(|w| ..)` method takes [tx_ordset::W](tx_ordset::W) writer structure
impl crate::Writable for TX_ORDSET {}
///UCPD Tx Ordered Set Type Register
pub mod tx_ordset;
///UCPD Tx Paysize Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tx_paysz](tx_paysz) module
pub type TX_PAYSZ = crate::Reg<u32, _TX_PAYSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_PAYSZ;
///`read()` method returns [tx_paysz::R](tx_paysz::R) reader structure
impl crate::Readable for TX_PAYSZ {}
///`write(|w| ..)` method takes [tx_paysz::W](tx_paysz::W) writer structure
impl crate::Writable for TX_PAYSZ {}
///UCPD Tx Paysize Register
pub mod tx_paysz;
///UCPD Tx Data Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txdr](txdr) module
pub type TXDR = crate::Reg<u32, _TXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDR;
///`read()` method returns [txdr::R](txdr::R) reader structure
impl crate::Readable for TXDR {}
///`write(|w| ..)` method takes [txdr::W](txdr::W) writer structure
impl crate::Writable for TXDR {}
///UCPD Tx Data Register
pub mod txdr;
///UCPD Rx Ordered Set Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rx_ordset](rx_ordset) module
pub type RX_ORDSET = crate::Reg<u32, _RX_ORDSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_ORDSET;
///`read()` method returns [rx_ordset::R](rx_ordset::R) reader structure
impl crate::Readable for RX_ORDSET {}
///UCPD Rx Ordered Set Register
pub mod rx_ordset;
///UCPD Rx Paysize Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rx_paysz](rx_paysz) module
pub type RX_PAYSZ = crate::Reg<u32, _RX_PAYSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_PAYSZ;
///`read()` method returns [rx_paysz::R](rx_paysz::R) reader structure
impl crate::Readable for RX_PAYSZ {}
///UCPD Rx Paysize Register
pub mod rx_paysz;
///UCPD Rx Data Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rxdr](rxdr) module
pub type RXDR = crate::Reg<u32, _RXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDR;
///`read()` method returns [rxdr::R](rxdr::R) reader structure
impl crate::Readable for RXDR {}
///UCPD Rx Data Register
pub mod rxdr;
///UCPD Rx Ordered Set Extension Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rx_ordext1](rx_ordext1) module
pub type RX_ORDEXT1 = crate::Reg<u32, _RX_ORDEXT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_ORDEXT1;
///`read()` method returns [rx_ordext1::R](rx_ordext1::R) reader structure
impl crate::Readable for RX_ORDEXT1 {}
///`write(|w| ..)` method takes [rx_ordext1::W](rx_ordext1::W) writer structure
impl crate::Writable for RX_ORDEXT1 {}
///UCPD Rx Ordered Set Extension Register 1
pub mod rx_ordext1;
///UCPD Rx Ordered Set Extension Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rx_ordext2](rx_ordext2) module
pub type RX_ORDEXT2 = crate::Reg<u32, _RX_ORDEXT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_ORDEXT2;
///`read()` method returns [rx_ordext2::R](rx_ordext2::R) reader structure
impl crate::Readable for RX_ORDEXT2 {}
///`write(|w| ..)` method takes [rx_ordext2::W](rx_ordext2::W) writer structure
impl crate::Writable for RX_ORDEXT2 {}
///UCPD Rx Ordered Set Extension Register 2
pub mod rx_ordext2;
