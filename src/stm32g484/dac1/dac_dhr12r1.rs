#[doc = "Reader of register DAC_DHR12R1"]
pub type R = crate::R<u32, super::DAC_DHR12R1>;
#[doc = "Writer for register DAC_DHR12R1"]
pub type W = crate::W<u32, super::DAC_DHR12R1>;
#[doc = "Register DAC_DHR12R1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC_DHR12R1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DACC1DHR`"]
pub type DACC1DHR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DACC1DHR`"]
pub struct DACC1DHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC1DHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `DACC1DHRB`"]
pub type DACC1DHRB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DACC1DHRB`"]
pub struct DACC1DHRB_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC1DHRB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC channel1 12-bit right-aligned data B"]
    #[inline(always)]
    pub fn dacc1dhrb(&self) -> DACC1DHRB_R {
        DACC1DHRB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W {
        DACC1DHR_W { w: self }
    }
    #[doc = "Bits 16:27 - DAC channel1 12-bit right-aligned data B"]
    #[inline(always)]
    pub fn dacc1dhrb(&mut self) -> DACC1DHRB_W {
        DACC1DHRB_W { w: self }
    }
}
