///Reader of register CR
pub type R = crate::R<u32, super::CR>;
///Writer for register CR
pub type W = crate::W<u32, super::CR>;
///Register CR `reset()`'s with value 0x2000_0000
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_0000
    }
}
///ADC calibration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCAL_A {
    ///0: Calibration complete
    COMPLETE = 0,
    ///1: Start the calibration of the ADC
    CALIBRATION = 1,
}
impl From<ADCAL_A> for bool {
    #[inline(always)]
    fn from(variant: ADCAL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ADCAL`
pub type ADCAL_R = crate::R<bool, ADCAL_A>;
impl ADCAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADCAL_A {
        match self.bits {
            false => ADCAL_A::COMPLETE,
            true => ADCAL_A::CALIBRATION,
        }
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == ADCAL_A::COMPLETE
    }
    ///Checks if the value of the field is `CALIBRATION`
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        *self == ADCAL_A::CALIBRATION
    }
}
///Write proxy for field `ADCAL`
pub struct ADCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCAL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADCAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Calibration complete
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(ADCAL_A::COMPLETE)
    }
    ///Start the calibration of the ADC
    #[inline(always)]
    pub fn calibration(self) -> &'a mut W {
        self.variant(ADCAL_A::CALIBRATION)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
///Differential mode for calibration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCALDIF_A {
    ///0: Calibration for single-ended mode
    SINGLEENDED = 0,
    ///1: Calibration for differential mode
    DIFFERENTIAL = 1,
}
impl From<ADCALDIF_A> for bool {
    #[inline(always)]
    fn from(variant: ADCALDIF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ADCALDIF`
pub type ADCALDIF_R = crate::R<bool, ADCALDIF_A>;
impl ADCALDIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADCALDIF_A {
        match self.bits {
            false => ADCALDIF_A::SINGLEENDED,
            true => ADCALDIF_A::DIFFERENTIAL,
        }
    }
    ///Checks if the value of the field is `SINGLEENDED`
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == ADCALDIF_A::SINGLEENDED
    }
    ///Checks if the value of the field is `DIFFERENTIAL`
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == ADCALDIF_A::DIFFERENTIAL
    }
}
///Write proxy for field `ADCALDIF`
pub struct ADCALDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCALDIF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADCALDIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Calibration for single-ended mode
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(ADCALDIF_A::SINGLEENDED)
    }
    ///Calibration for differential mode
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(ADCALDIF_A::DIFFERENTIAL)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
///Deep-power-down enable
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEEPPWD_A {
    ///0: ADC not in Deep-power down
    DISABLED = 0,
    ///1: ADC in Deep-power-down (default reset state)
    ENABLED = 1,
}
impl From<DEEPPWD_A> for bool {
    #[inline(always)]
    fn from(variant: DEEPPWD_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DEEPPWD`
pub type DEEPPWD_R = crate::R<bool, DEEPPWD_A>;
impl DEEPPWD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DEEPPWD_A {
        match self.bits {
            false => DEEPPWD_A::DISABLED,
            true => DEEPPWD_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEEPPWD_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEEPPWD_A::ENABLED
    }
}
///Write proxy for field `DEEPPWD`
pub struct DEEPPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEPPWD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DEEPPWD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///ADC not in Deep-power down
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DEEPPWD_A::DISABLED)
    }
    ///ADC in Deep-power-down (default reset state)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DEEPPWD_A::ENABLED)
    }
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
///ADC voltage regulator enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADVREGEN_A {
    ///0: ADC voltage regulator disabled
    DISABLED = 0,
    ///1: ADC voltage regulator enabled
    ENABLED = 1,
}
impl From<ADVREGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADVREGEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ADVREGEN`
pub type ADVREGEN_R = crate::R<bool, ADVREGEN_A>;
impl ADVREGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADVREGEN_A {
        match self.bits {
            false => ADVREGEN_A::DISABLED,
            true => ADVREGEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADVREGEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADVREGEN_A::ENABLED
    }
}
///Write proxy for field `ADVREGEN`
pub struct ADVREGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVREGEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADVREGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///ADC voltage regulator disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADVREGEN_A::DISABLED)
    }
    ///ADC voltage regulator enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADVREGEN_A::ENABLED)
    }
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
///ADC stop of injected conversion command
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JADSTP_A {
    ///1: Stop conversion of channel
    STOP = 1,
}
impl From<JADSTP_A> for bool {
    #[inline(always)]
    fn from(variant: JADSTP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `JADSTP`
pub type JADSTP_R = crate::R<bool, JADSTP_A>;
impl JADSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, JADSTP_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(JADSTP_A::STOP),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `STOP`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == JADSTP_A::STOP
    }
}
///Write proxy for field `JADSTP`
pub struct JADSTP_W<'a> {
    w: &'a mut W,
}
impl<'a> JADSTP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JADSTP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Stop conversion of channel
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(JADSTP_A::STOP)
    }
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
///ADC stop of regular conversion command
pub type ADSTP_A = JADSTP_A;
///Reader of field `ADSTP`
pub type ADSTP_R = crate::R<bool, JADSTP_A>;
///Write proxy for field `ADSTP`
pub struct ADSTP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSTP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADSTP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Stop conversion of channel
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(JADSTP_A::STOP)
    }
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
///ADC start of injected conversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JADSTART_A {
    ///1: Starts conversion of channel
    START = 1,
}
impl From<JADSTART_A> for bool {
    #[inline(always)]
    fn from(variant: JADSTART_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `JADSTART`
pub type JADSTART_R = crate::R<bool, JADSTART_A>;
impl JADSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, JADSTART_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(JADSTART_A::START),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `START`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == JADSTART_A::START
    }
}
///Write proxy for field `JADSTART`
pub struct JADSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> JADSTART_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JADSTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Starts conversion of channel
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(JADSTART_A::START)
    }
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
///ADC start of regular conversion
pub type ADSTART_A = JADSTART_A;
///Reader of field `ADSTART`
pub type ADSTART_R = crate::R<bool, JADSTART_A>;
///Write proxy for field `ADSTART`
pub struct ADSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSTART_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADSTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Starts conversion of channel
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(JADSTART_A::START)
    }
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
///ADC disable command
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDIS_A {
    ///0: Disable ADC conversion and go to power down mode
    DISABLE = 0,
}
impl From<ADDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ADDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ADDIS`
pub type ADDIS_R = crate::R<bool, ADDIS_A>;
impl ADDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ADDIS_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(ADDIS_A::DISABLE),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `DISABLE`
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADDIS_A::DISABLE
    }
}
///Write proxy for field `ADDIS`
pub struct ADDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Disable ADC conversion and go to power down mode
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADDIS_A::DISABLE)
    }
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
///ADC enable control
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADEN_A {
    ///1: Enable ADC
    ENABLE = 1,
}
impl From<ADEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ADEN`
pub type ADEN_R = crate::R<bool, ADEN_A>;
impl ADEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ADEN_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ADEN_A::ENABLE),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `ENABLE`
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADEN_A::ENABLE
    }
}
///Write proxy for field `ADEN`
pub struct ADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Enable ADC
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADEN_A::ENABLE)
    }
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
impl R {
    ///Bit 31 - ADC calibration
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 30 - Differential mode for calibration
    #[inline(always)]
    pub fn adcaldif(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 29 - Deep-power-down enable
    #[inline(always)]
    pub fn deeppwd(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 28 - ADC voltage regulator enable
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 5 - ADC stop of injected conversion command
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - ADC stop of regular conversion command
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - ADC start of injected conversion
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - ADC start of regular conversion
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - ADC disable command
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - ADC enable control
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 31 - ADC calibration
    #[inline(always)]
    pub fn adcal(&mut self) -> ADCAL_W {
        ADCAL_W { w: self }
    }
    ///Bit 30 - Differential mode for calibration
    #[inline(always)]
    pub fn adcaldif(&mut self) -> ADCALDIF_W {
        ADCALDIF_W { w: self }
    }
    ///Bit 29 - Deep-power-down enable
    #[inline(always)]
    pub fn deeppwd(&mut self) -> DEEPPWD_W {
        DEEPPWD_W { w: self }
    }
    ///Bit 28 - ADC voltage regulator enable
    #[inline(always)]
    pub fn advregen(&mut self) -> ADVREGEN_W {
        ADVREGEN_W { w: self }
    }
    ///Bit 5 - ADC stop of injected conversion command
    #[inline(always)]
    pub fn jadstp(&mut self) -> JADSTP_W {
        JADSTP_W { w: self }
    }
    ///Bit 4 - ADC stop of regular conversion command
    #[inline(always)]
    pub fn adstp(&mut self) -> ADSTP_W {
        ADSTP_W { w: self }
    }
    ///Bit 3 - ADC start of injected conversion
    #[inline(always)]
    pub fn jadstart(&mut self) -> JADSTART_W {
        JADSTART_W { w: self }
    }
    ///Bit 2 - ADC start of regular conversion
    #[inline(always)]
    pub fn adstart(&mut self) -> ADSTART_W {
        ADSTART_W { w: self }
    }
    ///Bit 1 - ADC disable command
    #[inline(always)]
    pub fn addis(&mut self) -> ADDIS_W {
        ADDIS_W { w: self }
    }
    ///Bit 0 - ADC enable control
    #[inline(always)]
    pub fn aden(&mut self) -> ADEN_W {
        ADEN_W { w: self }
    }
}
