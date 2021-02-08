///Reader of register TXDR
pub type R = crate::R<u32, super::TXDR>;
///Writer for register TXDR
pub type W = crate::W<u32, super::TXDR>;
///Register TXDR `reset()`'s with value 0
impl crate::ResetValue for super::TXDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TXDATA`
pub type TXDATA_R = crate::R<u8, u8>;
///Write proxy for field `TXDATA`
pub struct TXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - TXDATA
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - TXDATA
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W {
        TXDATA_W { w: self }
    }
}
