///Reader of register CCR3
pub type R = crate::R<u32, super::CCR3>;
///Writer for register CCR3
pub type W = crate::W<u32, super::CCR3>;
///Register CCR3 `reset()`'s with value 0
impl crate::ResetValue for super::CCR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CCR3`
pub type CCR3_R = crate::R<u16, u16>;
///Write proxy for field `CCR3`
pub struct CCR3_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR3_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Capture/Compare value
    #[inline(always)]
    pub fn ccr3(&self) -> CCR3_R {
        CCR3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Capture/Compare value
    #[inline(always)]
    pub fn ccr3(&mut self) -> CCR3_W {
        CCR3_W { w: self }
    }
}
