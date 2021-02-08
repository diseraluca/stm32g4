///Reader of register CMP2AR
pub type R = crate::R<u32, super::CMP2AR>;
///Writer for register CMP2AR
pub type W = crate::W<u32, super::CMP2AR>;
///Register CMP2AR `reset()`'s with value 0
impl crate::ResetValue for super::CMP2AR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CMP2x`
pub type CMP2X_R = crate::R<u16, u16>;
///Write proxy for field `CMP2x`
pub struct CMP2X_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2X_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Timerx Compare 2 value
    #[inline(always)]
    pub fn cmp2x(&self) -> CMP2X_R {
        CMP2X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Timerx Compare 2 value
    #[inline(always)]
    pub fn cmp2x(&mut self) -> CMP2X_W {
        CMP2X_W { w: self }
    }
}
