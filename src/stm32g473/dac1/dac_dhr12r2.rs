#[doc = "Reader of register DAC_DHR12R2"]
pub type R = crate::R<u32, super::DAC_DHR12R2>;
#[doc = "Writer for register DAC_DHR12R2"]
pub type W = crate::W<u32, super::DAC_DHR12R2>;
#[doc = "Register DAC_DHR12R2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC_DHR12R2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DACC2DHR`"]
pub type DACC2DHR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DACC2DHR`"]
pub struct DACC2DHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC2DHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `DACC2DHRB`"]
pub type DACC2DHRB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DACC2DHRB`"]
pub struct DACC2DHRB_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC2DHRB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhrb(&self) -> DACC2DHRB_R {
        DACC2DHRB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W {
        DACC2DHR_W { w: self }
    }
    #[doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhrb(&mut self) -> DACC2DHRB_W {
        DACC2DHRB_W { w: self }
    }
}
