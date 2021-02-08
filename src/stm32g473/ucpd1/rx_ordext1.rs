///Reader of register RX_ORDEXT1
pub type R = crate::R<u32, super::RX_ORDEXT1>;
///Writer for register RX_ORDEXT1
pub type W = crate::W<u32, super::RX_ORDEXT1>;
///Register RX_ORDEXT1 `reset()`'s with value 0
impl crate::ResetValue for super::RX_ORDEXT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `RXSOPX1`
pub type RXSOPX1_R = crate::R<u32, u32>;
///Write proxy for field `RXSOPX1`
pub struct RXSOPX1_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSOPX1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:19 - RXSOPX1
    #[inline(always)]
    pub fn rxsopx1(&self) -> RXSOPX1_R {
        RXSOPX1_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    ///Bits 0:19 - RXSOPX1
    #[inline(always)]
    pub fn rxsopx1(&mut self) -> RXSOPX1_W {
        RXSOPX1_W { w: self }
    }
}
