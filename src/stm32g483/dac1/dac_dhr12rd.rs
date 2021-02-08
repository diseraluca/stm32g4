///Reader of register DAC_DHR12RD
pub type R = crate::R<u32, super::DAC_DHR12RD>;
///Writer for register DAC_DHR12RD
pub type W = crate::W<u32, super::DAC_DHR12RD>;
///Register DAC_DHR12RD `reset()`'s with value 0
impl crate::ResetValue for super::DAC_DHR12RD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DACC1DHR`
pub type DACC1DHR_R = crate::R<u16, u16>;
///Write proxy for field `DACC1DHR`
pub struct DACC1DHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC1DHR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
///Reader of field `DACC2DHR`
pub type DACC2DHR_R = crate::R<u16, u16>;
///Write proxy for field `DACC2DHR`
pub struct DACC2DHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC2DHR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:11 - DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1.
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2.
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1.
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W {
        DACC1DHR_W { w: self }
    }
    ///Bits 16:27 - DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2.
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W {
        DACC2DHR_W { w: self }
    }
}
