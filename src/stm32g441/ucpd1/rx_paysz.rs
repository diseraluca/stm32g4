///Reader of register RX_PAYSZ
pub type R = crate::R<u32, super::RX_PAYSZ>;
///Reader of field `RXPAYSZ`
pub type RXPAYSZ_R = crate::R<u16, u16>;
impl R {
    ///Bits 0:9 - RXPAYSZ
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
