///Reader of register TXBCIE
pub type R = crate::R<u32, super::TXBCIE>;
///Writer for register TXBCIE
pub type W = crate::W<u32, super::TXBCIE>;
///Register TXBCIE `reset()`'s with value 0
impl crate::ResetValue for super::TXBCIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CFIE`
pub type CFIE_R = crate::R<u32, u32>;
///Write proxy for field `CFIE`
pub struct CFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CFIE
    #[inline(always)]
    pub fn cfie(&self) -> CFIE_R {
        CFIE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CFIE
    #[inline(always)]
    pub fn cfie(&mut self) -> CFIE_W {
        CFIE_W { w: self }
    }
}
