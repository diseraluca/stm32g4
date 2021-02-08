///Writer for register BDMADR
pub type W = crate::W<u32, super::BDMADR>;
///Register BDMADR `reset()`'s with value 0
impl crate::ResetValue for super::BDMADR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Write proxy for field `BDMADR`
pub struct BDMADR_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMADR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    ///Bits 0:31 - Burst DMA Data register
    #[inline(always)]
    pub fn bdmadr(&mut self) -> BDMADR_W {
        BDMADR_W { w: self }
    }
}
