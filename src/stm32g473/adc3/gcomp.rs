///Reader of register GCOMP
pub type R = crate::R<u32, super::GCOMP>;
///Writer for register GCOMP
pub type W = crate::W<u32, super::GCOMP>;
///Register GCOMP `reset()`'s with value 0
impl crate::ResetValue for super::GCOMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `GCOMPCOEFF`
pub type GCOMPCOEFF_R = crate::R<u16, u16>;
///Write proxy for field `GCOMPCOEFF`
pub struct GCOMPCOEFF_W<'a> {
    w: &'a mut W,
}
impl<'a> GCOMPCOEFF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    ///Bits 0:13 - Gain compensation coefficient
    #[inline(always)]
    pub fn gcompcoeff(&self) -> GCOMPCOEFF_R {
        GCOMPCOEFF_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    ///Bits 0:13 - Gain compensation coefficient
    #[inline(always)]
    pub fn gcompcoeff(&mut self) -> GCOMPCOEFF_W {
        GCOMPCOEFF_W { w: self }
    }
}
