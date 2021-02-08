///Reader of register CR2
pub type R = crate::R<u32, super::CR2>;
///Writer for register CR2
pub type W = crate::W<u32, super::CR2>;
///Register CR2 `reset()`'s with value 0
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PVMEN1`
pub type PVMEN1_R = crate::R<bool, bool>;
///Write proxy for field `PVMEN1`
pub struct PVMEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PVMEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
///Reader of field `PLS`
pub type PLS_R = crate::R<u8, u8>;
///Write proxy for field `PLS`
pub struct PLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
///Reader of field `PVDE`
pub type PVDE_R = crate::R<bool, bool>;
///Write proxy for field `PVDE`
pub struct PVDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDE_W<'a> {
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
///Reader of field `PVMEN2`
pub type PVMEN2_R = crate::R<bool, bool>;
///Write proxy for field `PVMEN2`
pub struct PVMEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PVMEN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
///Reader of field `PVMEN3`
pub type PVMEN3_R = crate::R<bool, bool>;
///Write proxy for field `PVMEN3`
pub struct PVMEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PVMEN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
///Reader of field `PVMEN4`
pub type PVMEN4_R = crate::R<bool, bool>;
///Write proxy for field `PVMEN4`
pub struct PVMEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PVMEN4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    ///Bit 4 - Peripheral voltage monitoring 1 enable: VDDA vs. COMP min voltage
    #[inline(always)]
    pub fn pvmen1(&self) -> PVMEN1_R {
        PVMEN1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bits 1:3 - Power voltage detector level selection
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    ///Bit 0 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 5 - Peripheral voltage monitoring 2 enable: VDDA vs. Fast DAC min voltage
    #[inline(always)]
    pub fn pvmen2(&self) -> PVMEN2_R {
        PVMEN2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. ADC min voltage 1.62V
    #[inline(always)]
    pub fn pvmen3(&self) -> PVMEN3_R {
        PVMEN3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. OPAMP/DAC min voltage
    #[inline(always)]
    pub fn pvmen4(&self) -> PVMEN4_R {
        PVMEN4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    ///Bit 4 - Peripheral voltage monitoring 1 enable: VDDA vs. COMP min voltage
    #[inline(always)]
    pub fn pvmen1(&mut self) -> PVMEN1_W {
        PVMEN1_W { w: self }
    }
    ///Bits 1:3 - Power voltage detector level selection
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W {
        PLS_W { w: self }
    }
    ///Bit 0 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W {
        PVDE_W { w: self }
    }
    ///Bit 5 - Peripheral voltage monitoring 2 enable: VDDA vs. Fast DAC min voltage
    #[inline(always)]
    pub fn pvmen2(&mut self) -> PVMEN2_W {
        PVMEN2_W { w: self }
    }
    ///Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. ADC min voltage 1.62V
    #[inline(always)]
    pub fn pvmen3(&mut self) -> PVMEN3_W {
        PVMEN3_W { w: self }
    }
    ///Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. OPAMP/DAC min voltage
    #[inline(always)]
    pub fn pvmen4(&mut self) -> PVMEN4_W {
        PVMEN4_W { w: self }
    }
}
