///Reader of register SUSP1R
pub type R = crate::R<u32, super::SUSP1R>;
///Writer for register SUSP1R
pub type W = crate::W<u32, super::SUSP1R>;
///Register SUSP1R `reset()`'s with value 0
impl crate::ResetValue for super::SUSP1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `SUSP`
pub type SUSP_R = crate::R<u32, u32>;
///Write proxy for field `SUSP`
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - AES suspend
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - AES suspend
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
}
