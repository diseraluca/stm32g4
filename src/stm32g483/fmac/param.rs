///Reader of register PARAM
pub type R = crate::R<u32, super::PARAM>;
///Writer for register PARAM
pub type W = crate::W<u32, super::PARAM>;
///Register PARAM `reset()`'s with value 0
impl crate::ResetValue for super::PARAM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `START`
pub type START_R = crate::R<bool, bool>;
///Write proxy for field `START`
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
///Reader of field `FUNC`
pub type FUNC_R = crate::R<u8, u8>;
///Write proxy for field `FUNC`
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
///Reader of field `R`
pub type R_R = crate::R<u8, u8>;
///Write proxy for field `R`
pub struct R_W<'a> {
    w: &'a mut W,
}
impl<'a> R_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///Reader of field `Q`
pub type Q_R = crate::R<u8, u8>;
///Write proxy for field `Q`
pub struct Q_W<'a> {
    w: &'a mut W,
}
impl<'a> Q_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Reader of field `P`
pub type P_R = crate::R<u8, u8>;
///Write proxy for field `P`
pub struct P_W<'a> {
    w: &'a mut W,
}
impl<'a> P_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bit 31 - START
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bits 24:30 - FUNC
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    ///Bits 16:23 - R
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 8:15 - Q
    #[inline(always)]
    pub fn q(&self) -> Q_R {
        Q_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 0:7 - P
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bit 31 - START
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    ///Bits 24:30 - FUNC
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    ///Bits 16:23 - R
    #[inline(always)]
    pub fn r(&mut self) -> R_W {
        R_W { w: self }
    }
    ///Bits 8:15 - Q
    #[inline(always)]
    pub fn q(&mut self) -> Q_W {
        Q_W { w: self }
    }
    ///Bits 0:7 - P
    #[inline(always)]
    pub fn p(&mut self) -> P_W {
        P_W { w: self }
    }
}
