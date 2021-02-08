///Reader of register BSLOTR
pub type R = crate::R<u32, super::BSLOTR>;
///Writer for register BSLOTR
pub type W = crate::W<u32, super::BSLOTR>;
///Register BSLOTR `reset()`'s with value 0
impl crate::ResetValue for super::BSLOTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `SLOTEN`
pub type SLOTEN_R = crate::R<u16, u16>;
///Write proxy for field `SLOTEN`
pub struct SLOTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTEN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
///Reader of field `NBSLOT`
pub type NBSLOT_R = crate::R<u8, u8>;
///Write proxy for field `NBSLOT`
pub struct NBSLOT_W<'a> {
    w: &'a mut W,
}
impl<'a> NBSLOT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
///Reader of field `SLOTSZ`
pub type SLOTSZ_R = crate::R<u8, u8>;
///Write proxy for field `SLOTSZ`
pub struct SLOTSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTSZ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
///Reader of field `FBOFF`
pub type FBOFF_R = crate::R<u8, u8>;
///Write proxy for field `FBOFF`
pub struct FBOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FBOFF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    ///Bits 16:31 - Slot enable
    #[inline(always)]
    pub fn sloten(&self) -> SLOTEN_R {
        SLOTEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    ///Bits 8:11 - Number of slots in an audio frame
    #[inline(always)]
    pub fn nbslot(&self) -> NBSLOT_R {
        NBSLOT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 6:7 - Slot size
    #[inline(always)]
    pub fn slotsz(&self) -> SLOTSZ_R {
        SLOTSZ_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    ///Bits 0:4 - First bit offset
    #[inline(always)]
    pub fn fboff(&self) -> FBOFF_R {
        FBOFF_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    ///Bits 16:31 - Slot enable
    #[inline(always)]
    pub fn sloten(&mut self) -> SLOTEN_W {
        SLOTEN_W { w: self }
    }
    ///Bits 8:11 - Number of slots in an audio frame
    #[inline(always)]
    pub fn nbslot(&mut self) -> NBSLOT_W {
        NBSLOT_W { w: self }
    }
    ///Bits 6:7 - Slot size
    #[inline(always)]
    pub fn slotsz(&mut self) -> SLOTSZ_W {
        SLOTSZ_W { w: self }
    }
    ///Bits 0:4 - First bit offset
    #[inline(always)]
    pub fn fboff(&mut self) -> FBOFF_W {
        FBOFF_W { w: self }
    }
}
