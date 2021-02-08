///Reader of register MCMP3R
pub type R = crate::R<u32, super::MCMP3R>;
///Writer for register MCMP3R
pub type W = crate::W<u32, super::MCMP3R>;
///Register MCMP3R `reset()`'s with value 0
impl crate::ResetValue for super::MCMP3R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `MCMP3`
pub type MCMP3_R = crate::R<u16, u16>;
///Write proxy for field `MCMP3`
pub struct MCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP3_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Master Timer Compare 3 value
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Master Timer Compare 3 value
    #[inline(always)]
    pub fn mcmp3(&mut self) -> MCMP3_W {
        MCMP3_W { w: self }
    }
}
