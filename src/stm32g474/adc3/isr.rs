///Reader of register ISR
pub type R = crate::R<u32, super::ISR>;
///Writer for register ISR
pub type W = crate::W<u32, super::ISR>;
///Register ISR `reset()`'s with value 0
impl crate::ResetValue for super::ISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Injected context queue overflow
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JQOVF_A {
    ///0: No injected context queue overflow has occurred
    NOOVERFLOW = 0,
    ///1: Injected context queue overflow has occurred
    OVERFLOW = 1,
}
impl From<JQOVF_A> for bool {
    #[inline(always)]
    fn from(variant: JQOVF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `JQOVF`
pub type JQOVF_R = crate::R<bool, JQOVF_A>;
impl JQOVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JQOVF_A {
        match self.bits {
            false => JQOVF_A::NOOVERFLOW,
            true => JQOVF_A::OVERFLOW,
        }
    }
    ///Checks if the value of the field is `NOOVERFLOW`
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == JQOVF_A::NOOVERFLOW
    }
    ///Checks if the value of the field is `OVERFLOW`
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == JQOVF_A::OVERFLOW
    }
}
///Injected context queue overflow
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JQOVF_AW {
    ///1: Clear injected context queue overflow flag
    CLEAR = 1,
}
impl From<JQOVF_AW> for bool {
    #[inline(always)]
    fn from(variant: JQOVF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `JQOVF`
pub struct JQOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> JQOVF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JQOVF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clear injected context queue overflow flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(JQOVF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
///Analog watchdog 3 flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD3_A {
    ///0: No analog watchdog event occurred
    NOEVENT = 0,
    ///1: Analog watchdog event occurred
    EVENT = 1,
}
impl From<AWD3_A> for bool {
    #[inline(always)]
    fn from(variant: AWD3_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `AWD3`
pub type AWD3_R = crate::R<bool, AWD3_A>;
impl AWD3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWD3_A {
        match self.bits {
            false => AWD3_A::NOEVENT,
            true => AWD3_A::EVENT,
        }
    }
    ///Checks if the value of the field is `NOEVENT`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD3_A::NOEVENT
    }
    ///Checks if the value of the field is `EVENT`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD3_A::EVENT
    }
}
///Analog watchdog 3 flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD3_AW {
    ///1: Clear analog watchdog event occurred flag
    CLEAR = 1,
}
impl From<AWD3_AW> for bool {
    #[inline(always)]
    fn from(variant: AWD3_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `AWD3`
pub struct AWD3_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clear analog watchdog event occurred flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AWD3_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///Analog watchdog 2 flag
pub type AWD2_A = AWD3_A;
///Reader of field `AWD2`
pub type AWD2_R = crate::R<bool, AWD3_A>;
///Analog watchdog 2 flag
pub type AWD2_AW = AWD3_AW;
///Write proxy for field `AWD2`
pub struct AWD2_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clear analog watchdog event occurred flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AWD3_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
///Analog watchdog 1 flag
pub type AWD1_A = AWD3_A;
///Reader of field `AWD1`
pub type AWD1_R = crate::R<bool, AWD3_A>;
///Analog watchdog 1 flag
pub type AWD1_AW = AWD3_AW;
///Write proxy for field `AWD1`
pub struct AWD1_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clear analog watchdog event occurred flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AWD3_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
///Injected channel end of sequence flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOS_A {
    ///0: Injected sequence is not complete
    NOTCOMPLETE = 0,
    ///1: Injected sequence complete
    COMPLETE = 1,
}
impl From<JEOS_A> for bool {
    #[inline(always)]
    fn from(variant: JEOS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `JEOS`
pub type JEOS_R = crate::R<bool, JEOS_A>;
impl JEOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JEOS_A {
        match self.bits {
            false => JEOS_A::NOTCOMPLETE,
            true => JEOS_A::COMPLETE,
        }
    }
    ///Checks if the value of the field is `NOTCOMPLETE`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOS_A::NOTCOMPLETE
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOS_A::COMPLETE
    }
}
///Injected channel end of sequence flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOS_AW {
    ///1: Clear Injected sequence complete flag
    CLEAR = 1,
}
impl From<JEOS_AW> for bool {
    #[inline(always)]
    fn from(variant: JEOS_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `JEOS`
pub struct JEOS_W<'a> {
    w: &'a mut W,
}
impl<'a> JEOS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JEOS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clear Injected sequence complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(JEOS_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
///Injected channel end of conversion flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOC_A {
    ///0: Injected conversion is not complete
    NOTCOMPLETE = 0,
    ///1: Injected conversion complete
    COMPLETE = 1,
}
impl From<JEOC_A> for bool {
    #[inline(always)]
    fn from(variant: JEOC_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `JEOC`
pub type JEOC_R = crate::R<bool, JEOC_A>;
impl JEOC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JEOC_A {
        match self.bits {
            false => JEOC_A::NOTCOMPLETE,
            true => JEOC_A::COMPLETE,
        }
    }
    ///Checks if the value of the field is `NOTCOMPLETE`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOC_A::NOTCOMPLETE
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOC_A::COMPLETE
    }
}
///Injected channel end of conversion flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOC_AW {
    ///1: Clear injected conversion complete flag
    CLEAR = 1,
}
impl From<JEOC_AW> for bool {
    #[inline(always)]
    fn from(variant: JEOC_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `JEOC`
pub struct JEOC_W<'a> {
    w: &'a mut W,
}
impl<'a> JEOC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JEOC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clear injected conversion complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(JEOC_AW::CLEAR)
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
///ADC overrun
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    ///0: No overrun occurred
    NOOVERRUN = 0,
    ///1: Overrun occurred
    OVERRUN = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OVR`
pub type OVR_R = crate::R<bool, OVR_A>;
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::NOOVERRUN,
            true => OVR_A::OVERRUN,
        }
    }
    ///Checks if the value of the field is `NOOVERRUN`
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_A::NOOVERRUN
    }
    ///Checks if the value of the field is `OVERRUN`
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_A::OVERRUN
    }
}
///ADC overrun
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_AW {
    ///1: Clear overrun occurred flag
    CLEAR = 1,
}
impl From<OVR_AW> for bool {
    #[inline(always)]
    fn from(variant: OVR_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `OVR`
pub struct OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clear overrun occurred flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVR_AW::CLEAR)
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
///End of regular sequence flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOS_A {
    ///0: Regular sequence is not complete
    NOTCOMPLETE = 0,
    ///1: Regular sequence complete
    COMPLETE = 1,
}
impl From<EOS_A> for bool {
    #[inline(always)]
    fn from(variant: EOS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EOS`
pub type EOS_R = crate::R<bool, EOS_A>;
impl EOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOS_A {
        match self.bits {
            false => EOS_A::NOTCOMPLETE,
            true => EOS_A::COMPLETE,
        }
    }
    ///Checks if the value of the field is `NOTCOMPLETE`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOS_A::NOTCOMPLETE
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOS_A::COMPLETE
    }
}
///End of regular sequence flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOS_AW {
    ///1: Clear regular sequence complete flag
    CLEAR = 1,
}
impl From<EOS_AW> for bool {
    #[inline(always)]
    fn from(variant: EOS_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `EOS`
pub struct EOS_W<'a> {
    w: &'a mut W,
}
impl<'a> EOS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clear regular sequence complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOS_AW::CLEAR)
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
///End of conversion flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC_A {
    ///0: Regular conversion is not complete
    NOTCOMPLETE = 0,
    ///1: Regular conversion complete
    COMPLETE = 1,
}
impl From<EOC_A> for bool {
    #[inline(always)]
    fn from(variant: EOC_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EOC`
pub type EOC_R = crate::R<bool, EOC_A>;
impl EOC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOC_A {
        match self.bits {
            false => EOC_A::NOTCOMPLETE,
            true => EOC_A::COMPLETE,
        }
    }
    ///Checks if the value of the field is `NOTCOMPLETE`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC_A::NOTCOMPLETE
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOC_A::COMPLETE
    }
}
///End of conversion flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC_AW {
    ///1: Clear regular conversion complete flag
    CLEAR = 1,
}
impl From<EOC_AW> for bool {
    #[inline(always)]
    fn from(variant: EOC_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `EOC`
pub struct EOC_W<'a> {
    w: &'a mut W,
}
impl<'a> EOC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clear regular conversion complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOC_AW::CLEAR)
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
///End of sampling flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOSMP_A {
    ///0: End of sampling phase no yet reached
    NOTENDED = 0,
    ///1: End of sampling phase reached
    ENDED = 1,
}
impl From<EOSMP_A> for bool {
    #[inline(always)]
    fn from(variant: EOSMP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EOSMP`
pub type EOSMP_R = crate::R<bool, EOSMP_A>;
impl EOSMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOSMP_A {
        match self.bits {
            false => EOSMP_A::NOTENDED,
            true => EOSMP_A::ENDED,
        }
    }
    ///Checks if the value of the field is `NOTENDED`
    #[inline(always)]
    pub fn is_not_ended(&self) -> bool {
        *self == EOSMP_A::NOTENDED
    }
    ///Checks if the value of the field is `ENDED`
    #[inline(always)]
    pub fn is_ended(&self) -> bool {
        *self == EOSMP_A::ENDED
    }
}
///End of sampling flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOSMP_AW {
    ///1: Clear end of sampling phase reached flag
    CLEAR = 1,
}
impl From<EOSMP_AW> for bool {
    #[inline(always)]
    fn from(variant: EOSMP_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `EOSMP`
pub struct EOSMP_W<'a> {
    w: &'a mut W,
}
impl<'a> EOSMP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOSMP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clear end of sampling phase reached flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOSMP_AW::CLEAR)
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
///ADC ready
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADRDY_A {
    ///0: ADC is not ready to start conversion
    NOTREADY = 0,
    ///1: ADC is ready to start conversion
    READY = 1,
}
impl From<ADRDY_A> for bool {
    #[inline(always)]
    fn from(variant: ADRDY_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ADRDY`
pub type ADRDY_R = crate::R<bool, ADRDY_A>;
impl ADRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADRDY_A {
        match self.bits {
            false => ADRDY_A::NOTREADY,
            true => ADRDY_A::READY,
        }
    }
    ///Checks if the value of the field is `NOTREADY`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ADRDY_A::NOTREADY
    }
    ///Checks if the value of the field is `READY`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ADRDY_A::READY
    }
}
///ADC ready
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADRDY_AW {
    ///1: Clear ADC is ready to start conversion flag
    CLEAR = 1,
}
impl From<ADRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: ADRDY_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `ADRDY`
pub struct ADRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRDY_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADRDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clear ADC is ready to start conversion flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADRDY_AW::CLEAR)
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
    ///Bit 10 - Injected context queue overflow
    #[inline(always)]
    pub fn jqovf(&self) -> JQOVF_R {
        JQOVF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Analog watchdog 3 flag
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Analog watchdog 2 flag
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - Analog watchdog 1 flag
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Injected channel end of sequence flag
    #[inline(always)]
    pub fn jeos(&self) -> JEOS_R {
        JEOS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Injected channel end of conversion flag
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - ADC overrun
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - End of regular sequence flag
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - End of conversion flag
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - End of sampling flag
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - ADC ready
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 10 - Injected context queue overflow
    #[inline(always)]
    pub fn jqovf(&mut self) -> JQOVF_W {
        JQOVF_W { w: self }
    }
    ///Bit 9 - Analog watchdog 3 flag
    #[inline(always)]
    pub fn awd3(&mut self) -> AWD3_W {
        AWD3_W { w: self }
    }
    ///Bit 8 - Analog watchdog 2 flag
    #[inline(always)]
    pub fn awd2(&mut self) -> AWD2_W {
        AWD2_W { w: self }
    }
    ///Bit 7 - Analog watchdog 1 flag
    #[inline(always)]
    pub fn awd1(&mut self) -> AWD1_W {
        AWD1_W { w: self }
    }
    ///Bit 6 - Injected channel end of sequence flag
    #[inline(always)]
    pub fn jeos(&mut self) -> JEOS_W {
        JEOS_W { w: self }
    }
    ///Bit 5 - Injected channel end of conversion flag
    #[inline(always)]
    pub fn jeoc(&mut self) -> JEOC_W {
        JEOC_W { w: self }
    }
    ///Bit 4 - ADC overrun
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W {
        OVR_W { w: self }
    }
    ///Bit 3 - End of regular sequence flag
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W {
        EOS_W { w: self }
    }
    ///Bit 2 - End of conversion flag
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W {
        EOC_W { w: self }
    }
    ///Bit 1 - End of sampling flag
    #[inline(always)]
    pub fn eosmp(&mut self) -> EOSMP_W {
        EOSMP_W { w: self }
    }
    ///Bit 0 - ADC ready
    #[inline(always)]
    pub fn adrdy(&mut self) -> ADRDY_W {
        ADRDY_W { w: self }
    }
}
