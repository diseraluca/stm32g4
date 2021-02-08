///Reader of register REPER
pub type R = crate::R<u32, super::REPER>;
///Writer for register REPER
pub type W = crate::W<u32, super::REPER>;
///Register REPER `reset()`'s with value 0
impl crate::ResetValue for super::REPER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `REPx`
pub type REPX_R = crate::R<u8, u8>;
///Write proxy for field `REPx`
pub struct REPX_W<'a> {
    w: &'a mut W,
}
impl<'a> REPX_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Timerx Repetition counter value
    #[inline(always)]
    pub fn repx(&self) -> REPX_R {
        REPX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Timerx Repetition counter value
    #[inline(always)]
    pub fn repx(&mut self) -> REPX_W {
        REPX_W { w: self }
    }
}
