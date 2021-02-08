///Reader of register AHB2SMENR
pub type R = crate::R<u32, super::AHB2SMENR>;
///Writer for register AHB2SMENR
pub type W = crate::W<u32, super::AHB2SMENR>;
///Register AHB2SMENR `reset()`'s with value 0x050f_667f
impl crate::ResetValue for super::AHB2SMENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x050f_667f
    }
}
///Reader of field `GPIOASMEN`
pub type GPIOASMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOASMEN`
pub struct GPIOASMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOASMEN_W<'a> {
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
///Reader of field `GPIOBSMEN`
pub type GPIOBSMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOBSMEN`
pub struct GPIOBSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///Reader of field `GPIOCSMEN`
pub type GPIOCSMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOCSMEN`
pub struct GPIOCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///Reader of field `GPIODSMEN`
pub type GPIODSMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIODSMEN`
pub struct GPIODSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///Reader of field `GPIOESMEN`
pub type GPIOESMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOESMEN`
pub struct GPIOESMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOESMEN_W<'a> {
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
///Reader of field `GPIOFSMEN`
pub type GPIOFSMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOFSMEN`
pub struct GPIOFSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOFSMEN_W<'a> {
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
///Reader of field `GPIOGSMEN`
pub type GPIOGSMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOGSMEN`
pub struct GPIOGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOGSMEN_W<'a> {
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
///Reader of field `ADC12SMEN`
pub type ADC12SMEN_R = crate::R<bool, bool>;
///Write proxy for field `ADC12SMEN`
pub struct ADC12SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SMEN_W<'a> {
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
///Reader of field `ADC345SMEN`
pub type ADC345SMEN_R = crate::R<bool, bool>;
///Write proxy for field `ADC345SMEN`
pub struct ADC345SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC345SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
///Reader of field `DAC1SMEN`
pub type DAC1SMEN_R = crate::R<bool, bool>;
///Write proxy for field `DAC1SMEN`
pub struct DAC1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
///Reader of field `DAC2SMEN`
pub type DAC2SMEN_R = crate::R<bool, bool>;
///Write proxy for field `DAC2SMEN`
pub struct DAC2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC2SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
///Reader of field `DAC3SMEN`
pub type DAC3SMEN_R = crate::R<bool, bool>;
///Write proxy for field `DAC3SMEN`
pub struct DAC3SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC3SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
///Reader of field `DAC4SMEN`
pub type DAC4SMEN_R = crate::R<bool, bool>;
///Write proxy for field `DAC4SMEN`
pub struct DAC4SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC4SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
///Reader of field `AESMEN`
pub type AESMEN_R = crate::R<bool, bool>;
///Write proxy for field `AESMEN`
pub struct AESMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AESMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
///Reader of field `RNGEN`
pub type RNGEN_R = crate::R<bool, bool>;
///Write proxy for field `RNGEN`
pub struct RNGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
///Reader of field `CCMSRAMSMEN`
pub type CCMSRAMSMEN_R = crate::R<bool, bool>;
///Write proxy for field `CCMSRAMSMEN`
pub struct CCMSRAMSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCMSRAMSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///Reader of field `SRAM2SMEN`
pub type SRAM2SMEN_R = crate::R<bool, bool>;
///Write proxy for field `SRAM2SMEN`
pub struct SRAM2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    ///Bit 0 - IO port A clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - IO port B clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - IO port C clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - IO port D clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - IO port E clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - IO port F clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - IO port G clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiogsmen(&self) -> GPIOGSMEN_R {
        GPIOGSMEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 13 - ADC clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn adc12smen(&self) -> ADC12SMEN_R {
        ADC12SMEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - DCMI clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn adc345smen(&self) -> ADC345SMEN_R {
        ADC345SMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 16 - AES accelerator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - HASH clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dac2smen(&self) -> DAC2SMEN_R {
        DAC2SMEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - DAC3 clock enable during sleep mode
    #[inline(always)]
    pub fn dac3smen(&self) -> DAC3SMEN_R {
        DAC3SMEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - DAC4 clock enable during sleep mode
    #[inline(always)]
    pub fn dac4smen(&self) -> DAC4SMEN_R {
        DAC4SMEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 24 - Cryptography clock enable during sleep mode
    #[inline(always)]
    pub fn aesmen(&self) -> AESMEN_R {
        AESMEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 26 - Random Number Generator clock enable during sleep mode
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 9 - CCM SRAM interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn ccmsramsmen(&self) -> CCMSRAMSMEN_R {
        CCMSRAMSMEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - IO port A clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W {
        GPIOASMEN_W { w: self }
    }
    ///Bit 1 - IO port B clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W {
        GPIOBSMEN_W { w: self }
    }
    ///Bit 2 - IO port C clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W {
        GPIOCSMEN_W { w: self }
    }
    ///Bit 3 - IO port D clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W {
        GPIODSMEN_W { w: self }
    }
    ///Bit 4 - IO port E clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W {
        GPIOESMEN_W { w: self }
    }
    ///Bit 5 - IO port F clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W {
        GPIOFSMEN_W { w: self }
    }
    ///Bit 6 - IO port G clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiogsmen(&mut self) -> GPIOGSMEN_W {
        GPIOGSMEN_W { w: self }
    }
    ///Bit 13 - ADC clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn adc12smen(&mut self) -> ADC12SMEN_W {
        ADC12SMEN_W { w: self }
    }
    ///Bit 14 - DCMI clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn adc345smen(&mut self) -> ADC345SMEN_W {
        ADC345SMEN_W { w: self }
    }
    ///Bit 16 - AES accelerator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W {
        DAC1SMEN_W { w: self }
    }
    ///Bit 17 - HASH clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dac2smen(&mut self) -> DAC2SMEN_W {
        DAC2SMEN_W { w: self }
    }
    ///Bit 18 - DAC3 clock enable during sleep mode
    #[inline(always)]
    pub fn dac3smen(&mut self) -> DAC3SMEN_W {
        DAC3SMEN_W { w: self }
    }
    ///Bit 19 - DAC4 clock enable during sleep mode
    #[inline(always)]
    pub fn dac4smen(&mut self) -> DAC4SMEN_W {
        DAC4SMEN_W { w: self }
    }
    ///Bit 24 - Cryptography clock enable during sleep mode
    #[inline(always)]
    pub fn aesmen(&mut self) -> AESMEN_W {
        AESMEN_W { w: self }
    }
    ///Bit 26 - Random Number Generator clock enable during sleep mode
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W {
        RNGEN_W { w: self }
    }
    ///Bit 9 - CCM SRAM interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn ccmsramsmen(&mut self) -> CCMSRAMSMEN_W {
        CCMSRAMSMEN_W { w: self }
    }
    ///Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W {
        SRAM2SMEN_W { w: self }
    }
}
