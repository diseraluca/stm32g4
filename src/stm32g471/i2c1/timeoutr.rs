///Reader of register TIMEOUTR
pub type R = crate::R<u32, super::TIMEOUTR>;
///Writer for register TIMEOUTR
pub type W = crate::W<u32, super::TIMEOUTR>;
///Register TIMEOUTR `reset()`'s with value 0
impl crate::ResetValue for super::TIMEOUTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TIMEOUTA`
pub type TIMEOUTA_R = crate::R<u16, u16>;
///Write proxy for field `TIMEOUTA`
pub struct TIMEOUTA_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUTA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
///Reader of field `TIDLE`
pub type TIDLE_R = crate::R<bool, bool>;
///Write proxy for field `TIDLE`
pub struct TIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIDLE_W<'a> {
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
///Reader of field `TIMOUTEN`
pub type TIMOUTEN_R = crate::R<bool, bool>;
///Write proxy for field `TIMOUTEN`
pub struct TIMOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMOUTEN_W<'a> {
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
///Reader of field `TIMEOUTB`
pub type TIMEOUTB_R = crate::R<u16, u16>;
///Write proxy for field `TIMEOUTB`
pub struct TIMEOUTB_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUTB_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
///Reader of field `TEXTEN`
pub type TEXTEN_R = crate::R<bool, bool>;
///Write proxy for field `TEXTEN`
pub struct TEXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEXTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 0:11 - Bus timeout A
    #[inline(always)]
    pub fn timeouta(&self) -> TIMEOUTA_R {
        TIMEOUTA_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 12 - Idle clock timeout detection
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 15 - Clock timeout enable
    #[inline(always)]
    pub fn timouten(&self) -> TIMOUTEN_R {
        TIMOUTEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 16:27 - Bus timeout B
    #[inline(always)]
    pub fn timeoutb(&self) -> TIMEOUTB_R {
        TIMEOUTB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - Extended clock timeout enable
    #[inline(always)]
    pub fn texten(&self) -> TEXTEN_R {
        TEXTEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:11 - Bus timeout A
    #[inline(always)]
    pub fn timeouta(&mut self) -> TIMEOUTA_W {
        TIMEOUTA_W { w: self }
    }
    ///Bit 12 - Idle clock timeout detection
    #[inline(always)]
    pub fn tidle(&mut self) -> TIDLE_W {
        TIDLE_W { w: self }
    }
    ///Bit 15 - Clock timeout enable
    #[inline(always)]
    pub fn timouten(&mut self) -> TIMOUTEN_W {
        TIMOUTEN_W { w: self }
    }
    ///Bits 16:27 - Bus timeout B
    #[inline(always)]
    pub fn timeoutb(&mut self) -> TIMEOUTB_W {
        TIMEOUTB_W { w: self }
    }
    ///Bit 31 - Extended clock timeout enable
    #[inline(always)]
    pub fn texten(&mut self) -> TEXTEN_W {
        TEXTEN_W { w: self }
    }
}
