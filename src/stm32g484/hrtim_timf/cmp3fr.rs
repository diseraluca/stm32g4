///Reader of register CMP3FR
pub type R = crate::R<u32, super::CMP3FR>;
///Writer for register CMP3FR
pub type W = crate::W<u32, super::CMP3FR>;
///Register CMP3FR `reset()`'s with value 0
impl crate::ResetValue for super::CMP3FR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CMP3x`
pub type CMP3X_R = crate::R<u16, u16>;
///Write proxy for field `CMP3x`
pub struct CMP3X_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3X_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Timerx Compare 3 value
    #[inline(always)]
    pub fn cmp3x(&self) -> CMP3X_R {
        CMP3X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Timerx Compare 3 value
    #[inline(always)]
    pub fn cmp3x(&mut self) -> CMP3X_W {
        CMP3X_W { w: self }
    }
}
