///Reader of register TXBCR
pub type R = crate::R<u32, super::TXBCR>;
///Writer for register TXBCR
pub type W = crate::W<u32, super::TXBCR>;
///Register TXBCR `reset()`'s with value 0
impl crate::ResetValue for super::TXBCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CR`
pub type CR_R = crate::R<u32, u32>;
///Write proxy for field `CR`
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CR
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CR
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
}
