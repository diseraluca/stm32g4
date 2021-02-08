///Writer for register WDATA
pub type W = crate::W<u32, super::WDATA>;
///Register WDATA `reset()`'s with value 0
impl crate::ResetValue for super::WDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Write proxy for field `WDATA`
pub struct WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> WDATA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    ///Bits 0:15 - WDATA
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W {
        WDATA_W { w: self }
    }
}
