///Reader of register CR
pub type R = crate::R<u32, super::CR>;
///Writer for register CR
pub type W = crate::W<u32, super::CR>;
///Register CR `reset()`'s with value 0
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `REV_OUT`
pub type REV_OUT_R = crate::R<bool, bool>;
///Write proxy for field `REV_OUT`
pub struct REV_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_OUT_W<'a> {
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
///Reader of field `REV_IN`
pub type REV_IN_R = crate::R<u8, u8>;
///Write proxy for field `REV_IN`
pub struct REV_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_IN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
///Reader of field `POLYSIZE`
pub type POLYSIZE_R = crate::R<u8, u8>;
///Write proxy for field `POLYSIZE`
pub struct POLYSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> POLYSIZE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
///Write proxy for field `RESET`
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
impl R {
    ///Bit 7 - Reverse output data
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 5:6 - Reverse input data
    #[inline(always)]
    pub fn rev_in(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    ///Bits 3:4 - Polynomial size
    #[inline(always)]
    pub fn polysize(&self) -> POLYSIZE_R {
        POLYSIZE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    ///Bit 7 - Reverse output data
    #[inline(always)]
    pub fn rev_out(&mut self) -> REV_OUT_W {
        REV_OUT_W { w: self }
    }
    ///Bits 5:6 - Reverse input data
    #[inline(always)]
    pub fn rev_in(&mut self) -> REV_IN_W {
        REV_IN_W { w: self }
    }
    ///Bits 3:4 - Polynomial size
    #[inline(always)]
    pub fn polysize(&mut self) -> POLYSIZE_W {
        POLYSIZE_W { w: self }
    }
    ///Bit 0 - RESET bit
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
}
