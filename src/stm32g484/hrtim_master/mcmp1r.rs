///Reader of register MCMP1R
pub type R = crate::R<u32, super::MCMP1R>;
///Writer for register MCMP1R
pub type W = crate::W<u32, super::MCMP1R>;
///Register MCMP1R `reset()`'s with value 0
impl crate::ResetValue for super::MCMP1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `MCMP1`
pub type MCMP1_R = crate::R<u16, u16>;
///Write proxy for field `MCMP1`
pub struct MCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Master Timer Compare 1 value
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Master Timer Compare 1 value
    #[inline(always)]
    pub fn mcmp1(&mut self) -> MCMP1_W {
        MCMP1_W { w: self }
    }
}
