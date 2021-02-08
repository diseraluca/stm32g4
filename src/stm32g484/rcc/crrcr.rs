///Reader of register CRRCR
pub type R = crate::R<u32, super::CRRCR>;
///Writer for register CRRCR
pub type W = crate::W<u32, super::CRRCR>;
///Register CRRCR `reset()`'s with value 0
impl crate::ResetValue for super::CRRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `HSI48ON`
pub type HSI48ON_R = crate::R<bool, bool>;
///Write proxy for field `HSI48ON`
pub struct HSI48ON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48ON_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
///Reader of field `HSI48RDY`
pub type HSI48RDY_R = crate::R<bool, bool>;
///Reader of field `HSI48CAL`
pub type HSI48CAL_R = crate::R<u16, u16>;
impl R {
    ///Bit 0 - HSI48 clock enable
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - HSI48 clock ready flag
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bits 7:15 - HSI48 clock calibration
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl W {
    ///Bit 0 - HSI48 clock enable
    #[inline(always)]
    pub fn hsi48on(&mut self) -> HSI48ON_W {
        HSI48ON_W { w: self }
    }
}
