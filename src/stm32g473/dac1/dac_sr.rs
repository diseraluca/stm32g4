///Reader of register DAC_SR
pub type R = crate::R<u32, super::DAC_SR>;
///Writer for register DAC_SR
pub type W = crate::W<u32, super::DAC_SR>;
///Register DAC_SR `reset()`'s with value 0
impl crate::ResetValue for super::DAC_SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DAC1RDY`
pub type DAC1RDY_R = crate::R<bool, bool>;
///Write proxy for field `DAC1RDY`
pub struct DAC1RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
///Reader of field `DORSTAT1`
pub type DORSTAT1_R = crate::R<bool, bool>;
///Write proxy for field `DORSTAT1`
pub struct DORSTAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DORSTAT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
///Reader of field `DMAUDR1`
pub type DMAUDR1_R = crate::R<bool, bool>;
///Write proxy for field `DMAUDR1`
pub struct DMAUDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
///Reader of field `CAL_FLAG1`
pub type CAL_FLAG1_R = crate::R<bool, bool>;
///Reader of field `BWST1`
pub type BWST1_R = crate::R<bool, bool>;
///Reader of field `DAC2RDY`
pub type DAC2RDY_R = crate::R<bool, bool>;
///Write proxy for field `DAC2RDY`
pub struct DAC2RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC2RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
///Reader of field `DORSTAT2`
pub type DORSTAT2_R = crate::R<bool, bool>;
///Write proxy for field `DORSTAT2`
pub struct DORSTAT2_W<'a> {
    w: &'a mut W,
}
impl<'a> DORSTAT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
///Reader of field `DMAUDR2`
pub type DMAUDR2_R = crate::R<bool, bool>;
///Write proxy for field `DMAUDR2`
pub struct DMAUDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDR2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
///Reader of field `CAL_FLAG2`
pub type CAL_FLAG2_R = crate::R<bool, bool>;
///Reader of field `BWST2`
pub type BWST2_R = crate::R<bool, bool>;
impl R {
    ///Bit 11 - DAC channel1 ready status bit
    #[inline(always)]
    pub fn dac1rdy(&self) -> DAC1RDY_R {
        DAC1RDY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - DAC channel1 output register status bit
    #[inline(always)]
    pub fn dorstat1(&self) -> DORSTAT1_R {
        DORSTAT1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - DAC Channel 1 calibration offset status This bit is set and cleared by hardware
    #[inline(always)]
    pub fn cal_flag1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - DAC Channel 1 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3LSI periods of synchronization).
    #[inline(always)]
    pub fn bwst1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 27 - DAC channel 2 ready status bit
    #[inline(always)]
    pub fn dac2rdy(&self) -> DAC2RDY_R {
        DAC2RDY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 28 - DAC channel 2 output register status bit
    #[inline(always)]
    pub fn dorstat2(&self) -> DORSTAT2_R {
        DORSTAT2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - DAC Channel 2 calibration offset status This bit is set and cleared by hardware
    #[inline(always)]
    pub fn cal_flag2(&self) -> CAL_FLAG2_R {
        CAL_FLAG2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - DAC Channel 2 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI periods of synchronization).
    #[inline(always)]
    pub fn bwst2(&self) -> BWST2_R {
        BWST2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 11 - DAC channel1 ready status bit
    #[inline(always)]
    pub fn dac1rdy(&mut self) -> DAC1RDY_W {
        DAC1RDY_W { w: self }
    }
    ///Bit 12 - DAC channel1 output register status bit
    #[inline(always)]
    pub fn dorstat1(&mut self) -> DORSTAT1_W {
        DORSTAT1_W { w: self }
    }
    ///Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
    #[inline(always)]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W {
        DMAUDR1_W { w: self }
    }
    ///Bit 27 - DAC channel 2 ready status bit
    #[inline(always)]
    pub fn dac2rdy(&mut self) -> DAC2RDY_W {
        DAC2RDY_W { w: self }
    }
    ///Bit 28 - DAC channel 2 output register status bit
    #[inline(always)]
    pub fn dorstat2(&mut self) -> DORSTAT2_W {
        DORSTAT2_W { w: self }
    }
    ///Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
    #[inline(always)]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W {
        DMAUDR2_W { w: self }
    }
}
