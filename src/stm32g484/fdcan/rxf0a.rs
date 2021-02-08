///Reader of register RXF0A
pub type R = crate::R<u32, super::RXF0A>;
///Writer for register RXF0A
pub type W = crate::W<u32, super::RXF0A>;
///Register RXF0A `reset()`'s with value 0
impl crate::ResetValue for super::RXF0A {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `F0AI`
pub type F0AI_R = crate::R<u8, u8>;
///Write proxy for field `F0AI`
pub struct F0AI_W<'a> {
    w: &'a mut W,
}
impl<'a> F0AI_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    ///Bits 0:5 - F0AI
    #[inline(always)]
    pub fn f0ai(&self) -> F0AI_R {
        F0AI_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:5 - F0AI
    #[inline(always)]
    pub fn f0ai(&mut self) -> F0AI_W {
        F0AI_W { w: self }
    }
}
