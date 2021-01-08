#[doc = "Reader of register BKP17R"]
pub type R = crate::R<u32, super::BKP17R>;
#[doc = "Writer for register BKP17R"]
pub type W = crate::W<u32, super::BKP17R>;
#[doc = "Register BKP17R `reset()`'s with value 0"]
impl crate::ResetValue for super::BKP17R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BKP`"]
pub type BKP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BKP`"]
pub struct BKP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W {
        BKP_W { w: self }
    }
}
