///Reader of register TOCC
pub type R = crate::R<u32, super::TOCC>;
///Writer for register TOCC
pub type W = crate::W<u32, super::TOCC>;
///Register TOCC `reset()`'s with value 0xffff_0000
impl crate::ResetValue for super::TOCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_0000
    }
}
///Reader of field `ETOC`
pub type ETOC_R = crate::R<bool, bool>;
///Write proxy for field `ETOC`
pub struct ETOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETOC_W<'a> {
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
///Write proxy for field `TOS`
pub struct TOS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
///Reader of field `TOP`
pub type TOP_R = crate::R<u16, u16>;
///Write proxy for field `TOP`
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bit 0 - ETOC
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 16:31 - TOP
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bit 0 - ETOC
    #[inline(always)]
    pub fn etoc(&mut self) -> ETOC_W {
        ETOC_W { w: self }
    }
    ///Bits 1:2 - TOS
    #[inline(always)]
    pub fn tos(&mut self) -> TOS_W {
        TOS_W { w: self }
    }
    ///Bits 16:31 - TOP
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
}
