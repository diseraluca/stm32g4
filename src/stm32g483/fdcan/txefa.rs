///Reader of register TXEFA
pub type R = crate::R<u32, super::TXEFA>;
///Writer for register TXEFA
pub type W = crate::W<u32, super::TXEFA>;
///Register TXEFA `reset()`'s with value 0
impl crate::ResetValue for super::TXEFA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `EFAI`
pub type EFAI_R = crate::R<u8, u8>;
///Write proxy for field `EFAI`
pub struct EFAI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFAI_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    ///Bits 0:4 - EFAI
    #[inline(always)]
    pub fn efai(&self) -> EFAI_R {
        EFAI_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - EFAI
    #[inline(always)]
    pub fn efai(&mut self) -> EFAI_W {
        EFAI_W { w: self }
    }
}
