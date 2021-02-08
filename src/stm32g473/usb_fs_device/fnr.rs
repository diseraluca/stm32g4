///Reader of register FNR
pub type R = crate::R<u32, super::FNR>;
///Reader of field `FN`
pub type FN_R = crate::R<u16, u16>;
///Reader of field `LSOF`
pub type LSOF_R = crate::R<u8, u8>;
///Reader of field `LCK`
pub type LCK_R = crate::R<bool, bool>;
///Reader of field `RXDM`
pub type RXDM_R = crate::R<bool, bool>;
///Reader of field `RXDP`
pub type RXDP_R = crate::R<bool, bool>;
impl R {
    ///Bits 0:10 - FN
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:12 - LSOF
    #[inline(always)]
    pub fn lsof(&self) -> LSOF_R {
        LSOF_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    ///Bit 13 - LCK
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - RXDM
    #[inline(always)]
    pub fn rxdm(&self) -> RXDM_R {
        RXDM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - RXDP
    #[inline(always)]
    pub fn rxdp(&self) -> RXDP_R {
        RXDP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
