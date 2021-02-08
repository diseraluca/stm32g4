///Reader of register MEMRMP
pub type R = crate::R<u32, super::MEMRMP>;
///Writer for register MEMRMP
pub type W = crate::W<u32, super::MEMRMP>;
///Register MEMRMP `reset()`'s with value 0
impl crate::ResetValue for super::MEMRMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `MEM_MODE`
pub type MEM_MODE_R = crate::R<u8, u8>;
///Write proxy for field `MEM_MODE`
pub struct MEM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MODE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
///Reader of field `FB_mode`
pub type FB_MODE_R = crate::R<bool, bool>;
///Write proxy for field `FB_mode`
pub struct FB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_MODE_W<'a> {
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
impl R {
    ///Bits 0:2 - Memory mapping selection
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 0x07) as u8)
    }
    ///Bit 8 - User Flash Bank mode
    #[inline(always)]
    pub fn fb_mode(&self) -> FB_MODE_R {
        FB_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:2 - Memory mapping selection
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W {
        MEM_MODE_W { w: self }
    }
    ///Bit 8 - User Flash Bank mode
    #[inline(always)]
    pub fn fb_mode(&mut self) -> FB_MODE_W {
        FB_MODE_W { w: self }
    }
}
