///Reader of register EEFAR1
pub type R = crate::R<u32, super::EEFAR1>;
///Writer for register EEFAR1
pub type W = crate::W<u32, super::EEFAR1>;
///Register EEFAR1 `reset()`'s with value 0
impl crate::ResetValue for super::EEFAR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `EE5FLTR`
pub type EE5FLTR_R = crate::R<u8, u8>;
///Write proxy for field `EE5FLTR`
pub struct EE5FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE5FLTR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | (((value as u32) & 0x0f) << 25);
        self.w
    }
}
///Reader of field `EE5LTCH`
pub type EE5LTCH_R = crate::R<bool, bool>;
///Write proxy for field `EE5LTCH`
pub struct EE5LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE5LTCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
///Reader of field `EE4FLTR`
pub type EE4FLTR_R = crate::R<u8, u8>;
///Write proxy for field `EE4FLTR`
pub struct EE4FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE4FLTR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | (((value as u32) & 0x0f) << 19);
        self.w
    }
}
///Reader of field `EE4LTCH`
pub type EE4LTCH_R = crate::R<bool, bool>;
///Write proxy for field `EE4LTCH`
pub struct EE4LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE4LTCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
///Reader of field `EE3FLTR`
pub type EE3FLTR_R = crate::R<u8, u8>;
///Write proxy for field `EE3FLTR`
pub struct EE3FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE3FLTR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | (((value as u32) & 0x0f) << 13);
        self.w
    }
}
///Reader of field `EE3LTCH`
pub type EE3LTCH_R = crate::R<bool, bool>;
///Write proxy for field `EE3LTCH`
pub struct EE3LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE3LTCH_W<'a> {
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
///Reader of field `EE2FLTR`
pub type EE2FLTR_R = crate::R<u8, u8>;
///Write proxy for field `EE2FLTR`
pub struct EE2FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE2FLTR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
///Reader of field `EE2LTCH`
pub type EE2LTCH_R = crate::R<bool, bool>;
///Write proxy for field `EE2LTCH`
pub struct EE2LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE2LTCH_W<'a> {
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
///Reader of field `EE1FLTR`
pub type EE1FLTR_R = crate::R<u8, u8>;
///Write proxy for field `EE1FLTR`
pub struct EE1FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE1FLTR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
///Reader of field `EE1LTCH`
pub type EE1LTCH_R = crate::R<bool, bool>;
///Write proxy for field `EE1LTCH`
pub struct EE1LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE1LTCH_W<'a> {
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
    ///Bits 25:28 - External Event 5 filter
    #[inline(always)]
    pub fn ee5fltr(&self) -> EE5FLTR_R {
        EE5FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    ///Bit 24 - External Event 5 latch
    #[inline(always)]
    pub fn ee5ltch(&self) -> EE5LTCH_R {
        EE5LTCH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bits 19:22 - External Event 4 filter
    #[inline(always)]
    pub fn ee4fltr(&self) -> EE4FLTR_R {
        EE4FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    ///Bit 18 - External Event 4 latch
    #[inline(always)]
    pub fn ee4ltch(&self) -> EE4LTCH_R {
        EE4LTCH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bits 13:16 - External Event 3 filter
    #[inline(always)]
    pub fn ee3fltr(&self) -> EE3FLTR_R {
        EE3FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bit 12 - External Event 3 latch
    #[inline(always)]
    pub fn ee3ltch(&self) -> EE3LTCH_R {
        EE3LTCH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bits 7:10 - External Event 2 filter
    #[inline(always)]
    pub fn ee2fltr(&self) -> EE2FLTR_R {
        EE2FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    ///Bit 6 - External Event 2 latch
    #[inline(always)]
    pub fn ee2ltch(&self) -> EE2LTCH_R {
        EE2LTCH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bits 1:4 - External Event 1 filter
    #[inline(always)]
    pub fn ee1fltr(&self) -> EE1FLTR_R {
        EE1FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bit 0 - External Event 1 latch
    #[inline(always)]
    pub fn ee1ltch(&self) -> EE1LTCH_R {
        EE1LTCH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 25:28 - External Event 5 filter
    #[inline(always)]
    pub fn ee5fltr(&mut self) -> EE5FLTR_W {
        EE5FLTR_W { w: self }
    }
    ///Bit 24 - External Event 5 latch
    #[inline(always)]
    pub fn ee5ltch(&mut self) -> EE5LTCH_W {
        EE5LTCH_W { w: self }
    }
    ///Bits 19:22 - External Event 4 filter
    #[inline(always)]
    pub fn ee4fltr(&mut self) -> EE4FLTR_W {
        EE4FLTR_W { w: self }
    }
    ///Bit 18 - External Event 4 latch
    #[inline(always)]
    pub fn ee4ltch(&mut self) -> EE4LTCH_W {
        EE4LTCH_W { w: self }
    }
    ///Bits 13:16 - External Event 3 filter
    #[inline(always)]
    pub fn ee3fltr(&mut self) -> EE3FLTR_W {
        EE3FLTR_W { w: self }
    }
    ///Bit 12 - External Event 3 latch
    #[inline(always)]
    pub fn ee3ltch(&mut self) -> EE3LTCH_W {
        EE3LTCH_W { w: self }
    }
    ///Bits 7:10 - External Event 2 filter
    #[inline(always)]
    pub fn ee2fltr(&mut self) -> EE2FLTR_W {
        EE2FLTR_W { w: self }
    }
    ///Bit 6 - External Event 2 latch
    #[inline(always)]
    pub fn ee2ltch(&mut self) -> EE2LTCH_W {
        EE2LTCH_W { w: self }
    }
    ///Bits 1:4 - External Event 1 filter
    #[inline(always)]
    pub fn ee1fltr(&mut self) -> EE1FLTR_W {
        EE1FLTR_W { w: self }
    }
    ///Bit 0 - External Event 1 latch
    #[inline(always)]
    pub fn ee1ltch(&mut self) -> EE1LTCH_W {
        EE1LTCH_W { w: self }
    }
}
