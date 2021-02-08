///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Writer for register SR
pub type W = crate::W<u32, super::SR>;
///Register SR `reset()`'s with value 0
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TXIS`
pub type TXIS_R = crate::R<bool, bool>;
///Write proxy for field `TXIS`
pub struct TXIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIS_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
///Reader of field `TXMSGDISC`
pub type TXMSGDISC_R = crate::R<bool, bool>;
///Write proxy for field `TXMSGDISC`
pub struct TXMSGDISC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGDISC_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///Reader of field `TXMSGSENT`
pub type TXMSGSENT_R = crate::R<bool, bool>;
///Write proxy for field `TXMSGSENT`
pub struct TXMSGSENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGSENT_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///Reader of field `TXMSGABT`
pub type TXMSGABT_R = crate::R<bool, bool>;
///Write proxy for field `TXMSGABT`
pub struct TXMSGABT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGABT_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///Reader of field `HRSTDISC`
pub type HRSTDISC_R = crate::R<bool, bool>;
///Write proxy for field `HRSTDISC`
pub struct HRSTDISC_W<'a> {
    w: &'a mut W,
}
impl<'a> HRSTDISC_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
///Reader of field `HRSTSENT`
pub type HRSTSENT_R = crate::R<bool, bool>;
///Write proxy for field `HRSTSENT`
pub struct HRSTSENT_W<'a> {
    w: &'a mut W,
}
impl<'a> HRSTSENT_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
///Reader of field `TXUND`
pub type TXUND_R = crate::R<bool, bool>;
///Write proxy for field `TXUND`
pub struct TXUND_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUND_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
///Reader of field `RXNE`
pub type RXNE_R = crate::R<bool, bool>;
///Write proxy for field `RXNE`
pub struct RXNE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNE_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
///Reader of field `RXORDDET`
pub type RXORDDET_R = crate::R<bool, bool>;
///Write proxy for field `RXORDDET`
pub struct RXORDDET_W<'a> {
    w: &'a mut W,
}
impl<'a> RXORDDET_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///Reader of field `RXHRSTDET`
pub type RXHRSTDET_R = crate::R<bool, bool>;
///Write proxy for field `RXHRSTDET`
pub struct RXHRSTDET_W<'a> {
    w: &'a mut W,
}
impl<'a> RXHRSTDET_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
///Reader of field `RXOVR`
pub type RXOVR_R = crate::R<bool, bool>;
///Write proxy for field `RXOVR`
pub struct RXOVR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVR_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
///Reader of field `RXMSGEND`
pub type RXMSGEND_R = crate::R<bool, bool>;
///Write proxy for field `RXMSGEND`
pub struct RXMSGEND_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMSGEND_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
///Reader of field `RXERR`
pub type RXERR_R = crate::R<bool, bool>;
///Write proxy for field `RXERR`
pub struct RXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERR_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
///Reader of field `TYPECEVT1`
pub type TYPECEVT1_R = crate::R<bool, bool>;
///Write proxy for field `TYPECEVT1`
pub struct TYPECEVT1_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPECEVT1_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
///Reader of field `TYPECEVT2`
pub type TYPECEVT2_R = crate::R<bool, bool>;
///Write proxy for field `TYPECEVT2`
pub struct TYPECEVT2_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPECEVT2_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
///Reader of field `TYPEC_VSTATE_CC1`
pub type TYPEC_VSTATE_CC1_R = crate::R<u8, u8>;
///Write proxy for field `TYPEC_VSTATE_CC1`
pub struct TYPEC_VSTATE_CC1_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPEC_VSTATE_CC1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
///Reader of field `TYPEC_VSTATE_CC2`
pub type TYPEC_VSTATE_CC2_R = crate::R<u8, u8>;
///Write proxy for field `TYPEC_VSTATE_CC2`
pub struct TYPEC_VSTATE_CC2_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPEC_VSTATE_CC2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
///Reader of field `FRSEVT`
pub type FRSEVT_R = crate::R<bool, bool>;
///Write proxy for field `FRSEVT`
pub struct FRSEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSEVT_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    ///Bit 0 - TXIS
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TXMSGDISC
    #[inline(always)]
    pub fn txmsgdisc(&self) -> TXMSGDISC_R {
        TXMSGDISC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - TXMSGSENT
    #[inline(always)]
    pub fn txmsgsent(&self) -> TXMSGSENT_R {
        TXMSGSENT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - TXMSGABT
    #[inline(always)]
    pub fn txmsgabt(&self) -> TXMSGABT_R {
        TXMSGABT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - HRSTDISC
    #[inline(always)]
    pub fn hrstdisc(&self) -> HRSTDISC_R {
        HRSTDISC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - HRSTSENT
    #[inline(always)]
    pub fn hrstsent(&self) -> HRSTSENT_R {
        HRSTSENT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - TXUND
    #[inline(always)]
    pub fn txund(&self) -> TXUND_R {
        TXUND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 8 - RXNE
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - RXORDDET
    #[inline(always)]
    pub fn rxorddet(&self) -> RXORDDET_R {
        RXORDDET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - RXHRSTDET
    #[inline(always)]
    pub fn rxhrstdet(&self) -> RXHRSTDET_R {
        RXHRSTDET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - RXOVR
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - RXMSGEND
    #[inline(always)]
    pub fn rxmsgend(&self) -> RXMSGEND_R {
        RXMSGEND_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - RXERR
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - TYPECEVT1
    #[inline(always)]
    pub fn typecevt1(&self) -> TYPECEVT1_R {
        TYPECEVT1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - TYPECEVT2
    #[inline(always)]
    pub fn typecevt2(&self) -> TYPECEVT2_R {
        TYPECEVT2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 16:17 - TYPEC_VSTATE_CC1
    #[inline(always)]
    pub fn typec_vstate_cc1(&self) -> TYPEC_VSTATE_CC1_R {
        TYPEC_VSTATE_CC1_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 18:19 - TYPEC_VSTATE_CC2
    #[inline(always)]
    pub fn typec_vstate_cc2(&self) -> TYPEC_VSTATE_CC2_R {
        TYPEC_VSTATE_CC2_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    ///Bit 20 - FRSEVT
    #[inline(always)]
    pub fn frsevt(&self) -> FRSEVT_R {
        FRSEVT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - TXIS
    #[inline(always)]
    pub fn txis(&mut self) -> TXIS_W {
        TXIS_W { w: self }
    }
    ///Bit 1 - TXMSGDISC
    #[inline(always)]
    pub fn txmsgdisc(&mut self) -> TXMSGDISC_W {
        TXMSGDISC_W { w: self }
    }
    ///Bit 2 - TXMSGSENT
    #[inline(always)]
    pub fn txmsgsent(&mut self) -> TXMSGSENT_W {
        TXMSGSENT_W { w: self }
    }
    ///Bit 3 - TXMSGABT
    #[inline(always)]
    pub fn txmsgabt(&mut self) -> TXMSGABT_W {
        TXMSGABT_W { w: self }
    }
    ///Bit 4 - HRSTDISC
    #[inline(always)]
    pub fn hrstdisc(&mut self) -> HRSTDISC_W {
        HRSTDISC_W { w: self }
    }
    ///Bit 5 - HRSTSENT
    #[inline(always)]
    pub fn hrstsent(&mut self) -> HRSTSENT_W {
        HRSTSENT_W { w: self }
    }
    ///Bit 6 - TXUND
    #[inline(always)]
    pub fn txund(&mut self) -> TXUND_W {
        TXUND_W { w: self }
    }
    ///Bit 8 - RXNE
    #[inline(always)]
    pub fn rxne(&mut self) -> RXNE_W {
        RXNE_W { w: self }
    }
    ///Bit 9 - RXORDDET
    #[inline(always)]
    pub fn rxorddet(&mut self) -> RXORDDET_W {
        RXORDDET_W { w: self }
    }
    ///Bit 10 - RXHRSTDET
    #[inline(always)]
    pub fn rxhrstdet(&mut self) -> RXHRSTDET_W {
        RXHRSTDET_W { w: self }
    }
    ///Bit 11 - RXOVR
    #[inline(always)]
    pub fn rxovr(&mut self) -> RXOVR_W {
        RXOVR_W { w: self }
    }
    ///Bit 12 - RXMSGEND
    #[inline(always)]
    pub fn rxmsgend(&mut self) -> RXMSGEND_W {
        RXMSGEND_W { w: self }
    }
    ///Bit 13 - RXERR
    #[inline(always)]
    pub fn rxerr(&mut self) -> RXERR_W {
        RXERR_W { w: self }
    }
    ///Bit 14 - TYPECEVT1
    #[inline(always)]
    pub fn typecevt1(&mut self) -> TYPECEVT1_W {
        TYPECEVT1_W { w: self }
    }
    ///Bit 15 - TYPECEVT2
    #[inline(always)]
    pub fn typecevt2(&mut self) -> TYPECEVT2_W {
        TYPECEVT2_W { w: self }
    }
    ///Bits 16:17 - TYPEC_VSTATE_CC1
    #[inline(always)]
    pub fn typec_vstate_cc1(&mut self) -> TYPEC_VSTATE_CC1_W {
        TYPEC_VSTATE_CC1_W { w: self }
    }
    ///Bits 18:19 - TYPEC_VSTATE_CC2
    #[inline(always)]
    pub fn typec_vstate_cc2(&mut self) -> TYPEC_VSTATE_CC2_W {
        TYPEC_VSTATE_CC2_W { w: self }
    }
    ///Bit 20 - FRSEVT
    #[inline(always)]
    pub fn frsevt(&mut self) -> FRSEVT_W {
        FRSEVT_W { w: self }
    }
}
