///Reader of register ARR
pub type R = crate::R<u32, super::ARR>;
///Writer for register ARR
pub type W = crate::W<u32, super::ARR>;
///Register ARR `reset()`'s with value 0xffff_ffff
impl crate::ResetValue for super::ARR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
///Reader of field `ARR`
pub type ARR_R = crate::R<u16, u16>;
///Write proxy for field `ARR`
pub struct ARR_W<'a> {
    w: &'a mut W,
}
impl<'a> ARR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Auto-reload value
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Auto-reload value
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W {
        ARR_W { w: self }
    }
}
