///Reader of register TDCR
pub type R = crate::R<u32, super::TDCR>;
///Writer for register TDCR
pub type W = crate::W<u32, super::TDCR>;
///Register TDCR `reset()`'s with value 0
impl crate::ResetValue for super::TDCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TDCF`
pub type TDCF_R = crate::R<u8, u8>;
///Write proxy for field `TDCF`
pub struct TDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
///Reader of field `TDCO`
pub type TDCO_R = crate::R<u8, u8>;
///Write proxy for field `TDCO`
pub struct TDCO_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCO_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
impl R {
    ///Bits 0:6 - TDCF
    #[inline(always)]
    pub fn tdcf(&self) -> TDCF_R {
        TDCF_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:14 - TDCO
    #[inline(always)]
    pub fn tdco(&self) -> TDCO_R {
        TDCO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - TDCF
    #[inline(always)]
    pub fn tdcf(&mut self) -> TDCF_W {
        TDCF_W { w: self }
    }
    ///Bits 8:14 - TDCO
    #[inline(always)]
    pub fn tdco(&mut self) -> TDCO_W {
        TDCO_W { w: self }
    }
}
