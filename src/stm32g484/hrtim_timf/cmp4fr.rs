///Reader of register CMP4FR
pub type R = crate::R<u32, super::CMP4FR>;
///Writer for register CMP4FR
pub type W = crate::W<u32, super::CMP4FR>;
///Register CMP4FR `reset()`'s with value 0
impl crate::ResetValue for super::CMP4FR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CMP4x`
pub type CMP4X_R = crate::R<u16, u16>;
///Write proxy for field `CMP4x`
pub struct CMP4X_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP4X_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Timerx Compare 4 value
    #[inline(always)]
    pub fn cmp4x(&self) -> CMP4X_R {
        CMP4X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Timerx Compare 4 value
    #[inline(always)]
    pub fn cmp4x(&mut self) -> CMP4X_W {
        CMP4X_W { w: self }
    }
}
