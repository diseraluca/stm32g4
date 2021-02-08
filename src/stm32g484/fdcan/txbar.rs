///Reader of register TXBAR
pub type R = crate::R<u32, super::TXBAR>;
///Writer for register TXBAR
pub type W = crate::W<u32, super::TXBAR>;
///Register TXBAR `reset()`'s with value 0
impl crate::ResetValue for super::TXBAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `AR`
pub type AR_R = crate::R<u32, u32>;
///Write proxy for field `AR`
pub struct AR_W<'a> {
    w: &'a mut W,
}
impl<'a> AR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - AR
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - AR
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W {
        AR_W { w: self }
    }
}
