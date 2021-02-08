///Reader of register TISEL
pub type R = crate::R<u32, super::TISEL>;
///Writer for register TISEL
pub type W = crate::W<u32, super::TISEL>;
///Register TISEL `reset()`'s with value 0
impl crate::ResetValue for super::TISEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TI1SEL`
pub type TI1SEL_R = crate::R<u8, u8>;
///Write proxy for field `TI1SEL`
pub struct TI1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1SEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 0:3 - TI1\[0\]
    ///to TI1\[15\]
    ///input selection
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - TI1\[0\]
    ///to TI1\[15\]
    ///input selection
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W {
        TI1SEL_W { w: self }
    }
}
