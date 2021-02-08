///Reader of register PCROP2ER
pub type R = crate::R<u32, super::PCROP2ER>;
///Writer for register PCROP2ER
pub type W = crate::W<u32, super::PCROP2ER>;
///Register PCROP2ER `reset()`'s with value 0
impl crate::ResetValue for super::PCROP2ER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PCROP2_END`
pub type PCROP2_END_R = crate::R<u16, u16>;
///Write proxy for field `PCROP2_END`
pub struct PCROP2_END_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP2_END_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    ///Bits 0:14 - PCROP area end offset
    #[inline(always)]
    pub fn pcrop2_end(&self) -> PCROP2_END_R {
        PCROP2_END_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    ///Bits 0:14 - PCROP area end offset
    #[inline(always)]
    pub fn pcrop2_end(&mut self) -> PCROP2_END_W {
        PCROP2_END_W { w: self }
    }
}
