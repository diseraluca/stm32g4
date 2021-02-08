///Reader of register SEC2R
pub type R = crate::R<u32, super::SEC2R>;
///Writer for register SEC2R
pub type W = crate::W<u32, super::SEC2R>;
///Register SEC2R `reset()`'s with value 0xffff_ff00
impl crate::ResetValue for super::SEC2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ff00
    }
}
///Reader of field `SEC_SIZE2`
pub type SEC_SIZE2_R = crate::R<u8, u8>;
///Write proxy for field `SEC_SIZE2`
pub struct SEC_SIZE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_SIZE2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - sets the number of pages used in the bank 2 Securable memory area
    #[inline(always)]
    pub fn sec_size2(&self) -> SEC_SIZE2_R {
        SEC_SIZE2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - sets the number of pages used in the bank 2 Securable memory area
    #[inline(always)]
    pub fn sec_size2(&mut self) -> SEC_SIZE2_W {
        SEC_SIZE2_W { w: self }
    }
}
