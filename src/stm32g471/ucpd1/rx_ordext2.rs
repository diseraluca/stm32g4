///Reader of register RX_ORDEXT2
pub type R = crate::R<u32, super::RX_ORDEXT2>;
///Writer for register RX_ORDEXT2
pub type W = crate::W<u32, super::RX_ORDEXT2>;
///Register RX_ORDEXT2 `reset()`'s with value 0
impl crate::ResetValue for super::RX_ORDEXT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `RXSOPX2`
pub type RXSOPX2_R = crate::R<u32, u32>;
///Write proxy for field `RXSOPX2`
pub struct RXSOPX2_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSOPX2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:19 - RXSOPX2
    #[inline(always)]
    pub fn rxsopx2(&self) -> RXSOPX2_R {
        RXSOPX2_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    ///Bits 0:19 - RXSOPX2
    #[inline(always)]
    pub fn rxsopx2(&mut self) -> RXSOPX2_W {
        RXSOPX2_W { w: self }
    }
}
