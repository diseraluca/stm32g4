///Reader of register RX_ORDSET
pub type R = crate::R<u32, super::RX_ORDSET>;
///Reader of field `RXORDSET`
pub type RXORDSET_R = crate::R<u8, u8>;
///Reader of field `RXSOP3OF4`
pub type RXSOP3OF4_R = crate::R<bool, bool>;
///Reader of field `RXSOPKINVALID`
pub type RXSOPKINVALID_R = crate::R<u8, u8>;
impl R {
    ///Bits 0:2 - RXORDSET
    #[inline(always)]
    pub fn rxordset(&self) -> RXORDSET_R {
        RXORDSET_R::new((self.bits & 0x07) as u8)
    }
    ///Bit 3 - RXSOP3OF4
    #[inline(always)]
    pub fn rxsop3of4(&self) -> RXSOP3OF4_R {
        RXSOP3OF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 4:6 - RXSOPKINVALID
    #[inline(always)]
    pub fn rxsopkinvalid(&self) -> RXSOPKINVALID_R {
        RXSOPKINVALID_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
