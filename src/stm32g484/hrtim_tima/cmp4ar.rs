#[doc = "Reader of register CMP4AR"]
pub type R = crate::R<u32, super::CMP4AR>;
#[doc = "Writer for register CMP4AR"]
pub type W = crate::W<u32, super::CMP4AR>;
#[doc = "Register CMP4AR `reset()`'s with value 0"]
impl crate::ResetValue for super::CMP4AR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP4x`"]
pub type CMP4X_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMP4x`"]
pub struct CMP4X_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP4X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 4 value"]
    #[inline(always)]
    pub fn cmp4x(&self) -> CMP4X_R {
        CMP4X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 4 value"]
    #[inline(always)]
    pub fn cmp4x(&mut self) -> CMP4X_W {
        CMP4X_W { w: self }
    }
}
