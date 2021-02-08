///Reader of register PCROP2SR
pub type R = crate::R<u32, super::PCROP2SR>;
///Writer for register PCROP2SR
pub type W = crate::W<u32, super::PCROP2SR>;
///Register PCROP2SR `reset()`'s with value 0xffff_0000
impl crate::ResetValue for super::PCROP2SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_0000
    }
}
///Reader of field `PCROP2_STRT`
pub type PCROP2_STRT_R = crate::R<u16, u16>;
///Write proxy for field `PCROP2_STRT`
pub struct PCROP2_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP2_STRT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    ///Bits 0:14 - PCROP area start offset
    #[inline(always)]
    pub fn pcrop2_strt(&self) -> PCROP2_STRT_R {
        PCROP2_STRT_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    ///Bits 0:14 - PCROP area start offset
    #[inline(always)]
    pub fn pcrop2_strt(&mut self) -> PCROP2_STRT_W {
        PCROP2_STRT_W { w: self }
    }
}
