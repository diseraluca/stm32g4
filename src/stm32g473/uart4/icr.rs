///Writer for register ICR
pub type W = crate::W<u32, super::ICR>;
///Register ICR `reset()`'s with value 0
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Write proxy for field `WUCF`
pub struct WUCF_W<'a> {
    w: &'a mut W,
}
impl<'a> WUCF_W<'a> {
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
///Write proxy for field `CMCF`
pub struct CMCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
///Write proxy for field `UDRCF`
pub struct UDRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRCF_W<'a> {
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
///Write proxy for field `EOBCF`
pub struct EOBCF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOBCF_W<'a> {
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
///Write proxy for field `RTOCF`
pub struct RTOCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTOCF_W<'a> {
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
///Write proxy for field `CTSCF`
pub struct CTSCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSCF_W<'a> {
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
///Write proxy for field `LBDCF`
pub struct LBDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> LBDCF_W<'a> {
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
///Write proxy for field `TCBGTCF`
pub struct TCBGTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCBGTCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
///Write proxy for field `TCCF`
pub struct TCCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCF_W<'a> {
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
///Write proxy for field `TXFECF`
pub struct TXFECF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFECF_W<'a> {
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
///Write proxy for field `IDLECF`
pub struct IDLECF_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLECF_W<'a> {
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
///Write proxy for field `ORECF`
pub struct ORECF_W<'a> {
    w: &'a mut W,
}
impl<'a> ORECF_W<'a> {
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
///Write proxy for field `NCF`
pub struct NCF_W<'a> {
    w: &'a mut W,
}
impl<'a> NCF_W<'a> {
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
///Write proxy for field `FECF`
pub struct FECF_W<'a> {
    w: &'a mut W,
}
impl<'a> FECF_W<'a> {
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
///Write proxy for field `PECF`
pub struct PECF_W<'a> {
    w: &'a mut W,
}
impl<'a> PECF_W<'a> {
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
impl W {
    ///Bit 20 - Wakeup from Stop mode clear flag
    #[inline(always)]
    pub fn wucf(&mut self) -> WUCF_W {
        WUCF_W { w: self }
    }
    ///Bit 17 - Character match clear flag
    #[inline(always)]
    pub fn cmcf(&mut self) -> CMCF_W {
        CMCF_W { w: self }
    }
    ///Bit 13 - UDRCF
    #[inline(always)]
    pub fn udrcf(&mut self) -> UDRCF_W {
        UDRCF_W { w: self }
    }
    ///Bit 12 - End of block clear flag
    #[inline(always)]
    pub fn eobcf(&mut self) -> EOBCF_W {
        EOBCF_W { w: self }
    }
    ///Bit 11 - Receiver timeout clear flag
    #[inline(always)]
    pub fn rtocf(&mut self) -> RTOCF_W {
        RTOCF_W { w: self }
    }
    ///Bit 9 - CTS clear flag
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W {
        CTSCF_W { w: self }
    }
    ///Bit 8 - LIN break detection clear flag
    #[inline(always)]
    pub fn lbdcf(&mut self) -> LBDCF_W {
        LBDCF_W { w: self }
    }
    ///Bit 7 - TCBGTCF
    #[inline(always)]
    pub fn tcbgtcf(&mut self) -> TCBGTCF_W {
        TCBGTCF_W { w: self }
    }
    ///Bit 6 - Transmission complete clear flag
    #[inline(always)]
    pub fn tccf(&mut self) -> TCCF_W {
        TCCF_W { w: self }
    }
    ///Bit 5 - TXFECF
    #[inline(always)]
    pub fn txfecf(&mut self) -> TXFECF_W {
        TXFECF_W { w: self }
    }
    ///Bit 4 - Idle line detected clear flag
    #[inline(always)]
    pub fn idlecf(&mut self) -> IDLECF_W {
        IDLECF_W { w: self }
    }
    ///Bit 3 - Overrun error clear flag
    #[inline(always)]
    pub fn orecf(&mut self) -> ORECF_W {
        ORECF_W { w: self }
    }
    ///Bit 2 - Noise detected clear flag
    #[inline(always)]
    pub fn ncf(&mut self) -> NCF_W {
        NCF_W { w: self }
    }
    ///Bit 1 - Framing error clear flag
    #[inline(always)]
    pub fn fecf(&mut self) -> FECF_W {
        FECF_W { w: self }
    }
    ///Bit 0 - Parity error clear flag
    #[inline(always)]
    pub fn pecf(&mut self) -> PECF_W {
        PECF_W { w: self }
    }
}
