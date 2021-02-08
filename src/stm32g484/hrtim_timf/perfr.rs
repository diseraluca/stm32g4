///Reader of register PERFR
pub type R = crate::R<u32, super::PERFR>;
///Writer for register PERFR
pub type W = crate::W<u32, super::PERFR>;
///Register PERFR `reset()`'s with value 0xffff
impl crate::ResetValue for super::PERFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
///Reader of field `PERx`
pub type PERX_R = crate::R<u16, u16>;
///Write proxy for field `PERx`
pub struct PERX_W<'a> {
    w: &'a mut W,
}
impl<'a> PERX_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Timerx Period value
    #[inline(always)]
    pub fn perx(&self) -> PERX_R {
        PERX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Timerx Period value
    #[inline(always)]
    pub fn perx(&mut self) -> PERX_W {
        PERX_W { w: self }
    }
}
