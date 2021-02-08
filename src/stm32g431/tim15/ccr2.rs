///Reader of register CCR2
pub type R = crate::R<u32, super::CCR2>;
///Writer for register CCR2
pub type W = crate::W<u32, super::CCR2>;
///Register CCR2 `reset()`'s with value 0
impl crate::ResetValue for super::CCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CCR2`
pub type CCR2_R = crate::R<u16, u16>;
///Write proxy for field `CCR2`
pub struct CCR2_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Capture/Compare 1 value
    #[inline(always)]
    pub fn ccr2(&self) -> CCR2_R {
        CCR2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Capture/Compare 1 value
    #[inline(always)]
    pub fn ccr2(&mut self) -> CCR2_W {
        CCR2_W { w: self }
    }
}
