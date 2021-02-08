///Reader of register MREP
pub type R = crate::R<u32, super::MREP>;
///Writer for register MREP
pub type W = crate::W<u32, super::MREP>;
///Register MREP `reset()`'s with value 0
impl crate::ResetValue for super::MREP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `MREP`
pub type MREP_R = crate::R<u8, u8>;
///Write proxy for field `MREP`
pub struct MREP_W<'a> {
    w: &'a mut W,
}
impl<'a> MREP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Master Timer Repetition counter value
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Master Timer Repetition counter value
    #[inline(always)]
    pub fn mrep(&mut self) -> MREP_W {
        MREP_W { w: self }
    }
}
