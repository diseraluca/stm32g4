///Reader of register ACR
pub type R = crate::R<u32, super::ACR>;
///Writer for register ACR
pub type W = crate::W<u32, super::ACR>;
///Register ACR `reset()`'s with value 0x0600
impl crate::ResetValue for super::ACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0600
    }
}
///Latency
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LATENCY_A {
    ///0: Zero wait state
    WS0 = 0,
    ///1: One wait state
    WS1 = 1,
    ///2: Two wait state
    WS2 = 2,
    ///3: Three wait state
    WS3 = 3,
    ///4: Four wait state
    WS4 = 4,
    ///5: Five wait state
    WS5 = 5,
    ///6: Six wait state
    WS6 = 6,
    ///7: Seven wait state
    WS7 = 7,
    ///8: Eight wait state
    WS8 = 8,
    ///9: Nine wait state
    WS9 = 9,
    ///10: Ten wait state
    WS10 = 10,
    ///11: Eleven wait state
    WS11 = 11,
    ///12: Twelve wait state
    WS12 = 12,
    ///13: Thirteen wait state
    WS13 = 13,
    ///14: Fourteen wait state
    WS14 = 14,
    ///15: Fifteen wait state
    WS15 = 15,
}
impl From<LATENCY_A> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY_A) -> Self {
        variant as _
    }
}
///Reader of field `LATENCY`
pub type LATENCY_R = crate::R<u8, LATENCY_A>;
impl LATENCY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LATENCY_A {
        match self.bits {
            0 => LATENCY_A::WS0,
            1 => LATENCY_A::WS1,
            2 => LATENCY_A::WS2,
            3 => LATENCY_A::WS3,
            4 => LATENCY_A::WS4,
            5 => LATENCY_A::WS5,
            6 => LATENCY_A::WS6,
            7 => LATENCY_A::WS7,
            8 => LATENCY_A::WS8,
            9 => LATENCY_A::WS9,
            10 => LATENCY_A::WS10,
            11 => LATENCY_A::WS11,
            12 => LATENCY_A::WS12,
            13 => LATENCY_A::WS13,
            14 => LATENCY_A::WS14,
            15 => LATENCY_A::WS15,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `WS0`
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY_A::WS0
    }
    ///Checks if the value of the field is `WS1`
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY_A::WS1
    }
    ///Checks if the value of the field is `WS2`
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY_A::WS2
    }
    ///Checks if the value of the field is `WS3`
    #[inline(always)]
    pub fn is_ws3(&self) -> bool {
        *self == LATENCY_A::WS3
    }
    ///Checks if the value of the field is `WS4`
    #[inline(always)]
    pub fn is_ws4(&self) -> bool {
        *self == LATENCY_A::WS4
    }
    ///Checks if the value of the field is `WS5`
    #[inline(always)]
    pub fn is_ws5(&self) -> bool {
        *self == LATENCY_A::WS5
    }
    ///Checks if the value of the field is `WS6`
    #[inline(always)]
    pub fn is_ws6(&self) -> bool {
        *self == LATENCY_A::WS6
    }
    ///Checks if the value of the field is `WS7`
    #[inline(always)]
    pub fn is_ws7(&self) -> bool {
        *self == LATENCY_A::WS7
    }
    ///Checks if the value of the field is `WS8`
    #[inline(always)]
    pub fn is_ws8(&self) -> bool {
        *self == LATENCY_A::WS8
    }
    ///Checks if the value of the field is `WS9`
    #[inline(always)]
    pub fn is_ws9(&self) -> bool {
        *self == LATENCY_A::WS9
    }
    ///Checks if the value of the field is `WS10`
    #[inline(always)]
    pub fn is_ws10(&self) -> bool {
        *self == LATENCY_A::WS10
    }
    ///Checks if the value of the field is `WS11`
    #[inline(always)]
    pub fn is_ws11(&self) -> bool {
        *self == LATENCY_A::WS11
    }
    ///Checks if the value of the field is `WS12`
    #[inline(always)]
    pub fn is_ws12(&self) -> bool {
        *self == LATENCY_A::WS12
    }
    ///Checks if the value of the field is `WS13`
    #[inline(always)]
    pub fn is_ws13(&self) -> bool {
        *self == LATENCY_A::WS13
    }
    ///Checks if the value of the field is `WS14`
    #[inline(always)]
    pub fn is_ws14(&self) -> bool {
        *self == LATENCY_A::WS14
    }
    ///Checks if the value of the field is `WS15`
    #[inline(always)]
    pub fn is_ws15(&self) -> bool {
        *self == LATENCY_A::WS15
    }
}
///Write proxy for field `LATENCY`
pub struct LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> LATENCY_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LATENCY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Zero wait state
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(LATENCY_A::WS0)
    }
    ///One wait state
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(LATENCY_A::WS1)
    }
    ///Two wait state
    #[inline(always)]
    pub fn ws2(self) -> &'a mut W {
        self.variant(LATENCY_A::WS2)
    }
    ///Three wait state
    #[inline(always)]
    pub fn ws3(self) -> &'a mut W {
        self.variant(LATENCY_A::WS3)
    }
    ///Four wait state
    #[inline(always)]
    pub fn ws4(self) -> &'a mut W {
        self.variant(LATENCY_A::WS4)
    }
    ///Five wait state
    #[inline(always)]
    pub fn ws5(self) -> &'a mut W {
        self.variant(LATENCY_A::WS5)
    }
    ///Six wait state
    #[inline(always)]
    pub fn ws6(self) -> &'a mut W {
        self.variant(LATENCY_A::WS6)
    }
    ///Seven wait state
    #[inline(always)]
    pub fn ws7(self) -> &'a mut W {
        self.variant(LATENCY_A::WS7)
    }
    ///Eight wait state
    #[inline(always)]
    pub fn ws8(self) -> &'a mut W {
        self.variant(LATENCY_A::WS8)
    }
    ///Nine wait state
    #[inline(always)]
    pub fn ws9(self) -> &'a mut W {
        self.variant(LATENCY_A::WS9)
    }
    ///Ten wait state
    #[inline(always)]
    pub fn ws10(self) -> &'a mut W {
        self.variant(LATENCY_A::WS10)
    }
    ///Eleven wait state
    #[inline(always)]
    pub fn ws11(self) -> &'a mut W {
        self.variant(LATENCY_A::WS11)
    }
    ///Twelve wait state
    #[inline(always)]
    pub fn ws12(self) -> &'a mut W {
        self.variant(LATENCY_A::WS12)
    }
    ///Thirteen wait state
    #[inline(always)]
    pub fn ws13(self) -> &'a mut W {
        self.variant(LATENCY_A::WS13)
    }
    ///Fourteen wait state
    #[inline(always)]
    pub fn ws14(self) -> &'a mut W {
        self.variant(LATENCY_A::WS14)
    }
    ///Fifteen wait state
    #[inline(always)]
    pub fn ws15(self) -> &'a mut W {
        self.variant(LATENCY_A::WS15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
///Prefetch enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTEN_A {
    ///0: Prefetch disabled
    DISABLED = 0,
    ///1: Prefetch enabled
    ENABLED = 1,
}
impl From<PRFTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PRFTEN`
pub type PRFTEN_R = crate::R<bool, PRFTEN_A>;
impl PRFTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PRFTEN_A {
        match self.bits {
            false => PRFTEN_A::DISABLED,
            true => PRFTEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN_A::ENABLED
    }
}
///Write proxy for field `PRFTEN`
pub struct PRFTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRFTEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PRFTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Prefetch disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::DISABLED)
    }
    ///Prefetch enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::ENABLED)
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
///Instruction cache enable
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICEN_A {
    ///0: Instruction cache is disabled
    DISABLED = 0,
    ///1: Instruction cache is enabled
    ENABLED = 1,
}
impl From<ICEN_A> for bool {
    #[inline(always)]
    fn from(variant: ICEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ICEN`
pub type ICEN_R = crate::R<bool, ICEN_A>;
impl ICEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ICEN_A {
        match self.bits {
            false => ICEN_A::DISABLED,
            true => ICEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICEN_A::ENABLED
    }
}
///Write proxy for field `ICEN`
pub struct ICEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ICEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Instruction cache is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ICEN_A::DISABLED)
    }
    ///Instruction cache is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ICEN_A::ENABLED)
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
///Data cache enable
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCEN_A {
    ///0: Data cache is disabled
    DISABLED = 0,
    ///1: Data cache is enabled
    ENABLED = 1,
}
impl From<DCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DCEN`
pub type DCEN_R = crate::R<bool, DCEN_A>;
impl DCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DCEN_A {
        match self.bits {
            false => DCEN_A::DISABLED,
            true => DCEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCEN_A::ENABLED
    }
}
///Write proxy for field `DCEN`
pub struct DCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Data cache is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCEN_A::DISABLED)
    }
    ///Data cache is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCEN_A::ENABLED)
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
///Instruction cache reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICRST_A {
    ///0: Instruction cache is not reset
    DISABLED = 0,
    ///1: Instruction cache is reset
    ENABLED = 1,
}
impl From<ICRST_A> for bool {
    #[inline(always)]
    fn from(variant: ICRST_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ICRST`
pub type ICRST_R = crate::R<bool, ICRST_A>;
impl ICRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ICRST_A {
        match self.bits {
            false => ICRST_A::DISABLED,
            true => ICRST_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICRST_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICRST_A::ENABLED
    }
}
///Write proxy for field `ICRST`
pub struct ICRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ICRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ICRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Instruction cache is not reset
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ICRST_A::DISABLED)
    }
    ///Instruction cache is reset
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ICRST_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
///Data cache reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCRST_A {
    ///0: Data cache is not reset
    DISABLED = 0,
    ///1: Data cache is reset
    ENABLED = 1,
}
impl From<DCRST_A> for bool {
    #[inline(always)]
    fn from(variant: DCRST_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DCRST`
pub type DCRST_R = crate::R<bool, DCRST_A>;
impl DCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DCRST_A {
        match self.bits {
            false => DCRST_A::DISABLED,
            true => DCRST_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCRST_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCRST_A::ENABLED
    }
}
///Write proxy for field `DCRST`
pub struct DCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DCRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Data cache is not reset
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCRST_A::DISABLED)
    }
    ///Data cache is reset
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCRST_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
///Flash Power-down mode during Low-power run mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN_PD_A {
    ///0: Flash in Idle mode
    DISABLED = 0,
    ///1: Flash in Power-down mode
    ENABLED = 1,
}
impl From<RUN_PD_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_PD_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RUN_PD`
pub type RUN_PD_R = crate::R<bool, RUN_PD_A>;
impl RUN_PD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RUN_PD_A {
        match self.bits {
            false => RUN_PD_A::DISABLED,
            true => RUN_PD_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RUN_PD_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RUN_PD_A::ENABLED
    }
}
///Write proxy for field `RUN_PD`
pub struct RUN_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_PD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RUN_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Flash in Idle mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RUN_PD_A::DISABLED)
    }
    ///Flash in Power-down mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RUN_PD_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
///Flash Power-down mode during Low-power sleep mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_PD_A {
    ///0: Flash in Idle mode during Sleep or Low-power sleep mode
    DISABLED = 0,
    ///1: Flash in Power-down mode during Sleep and Low-power sleep modes
    ENABLED = 1,
}
impl From<SLEEP_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_PD_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SLEEP_PD`
pub type SLEEP_PD_R = crate::R<bool, SLEEP_PD_A>;
impl SLEEP_PD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_PD_A {
        match self.bits {
            false => SLEEP_PD_A::DISABLED,
            true => SLEEP_PD_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLEEP_PD_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLEEP_PD_A::ENABLED
    }
}
///Write proxy for field `SLEEP_PD`
pub struct SLEEP_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_PD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SLEEP_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Flash in Idle mode during Sleep or Low-power sleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLEEP_PD_A::DISABLED)
    }
    ///Flash in Power-down mode during Sleep and Low-power sleep modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLEEP_PD_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
///Debug software enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_SWEN_A {
    ///0: Debugger Disabled
    DISABLED = 0,
    ///1: Debugger Enabled
    ENABLED = 1,
}
impl From<DBG_SWEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_SWEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DBG_SWEN`
pub type DBG_SWEN_R = crate::R<bool, DBG_SWEN_A>;
impl DBG_SWEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_SWEN_A {
        match self.bits {
            false => DBG_SWEN_A::DISABLED,
            true => DBG_SWEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBG_SWEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBG_SWEN_A::ENABLED
    }
}
///Write proxy for field `DBG_SWEN`
pub struct DBG_SWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_SWEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_SWEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Debugger Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBG_SWEN_A::DISABLED)
    }
    ///Debugger Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBG_SWEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    ///Bits 0:3 - Latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Instruction cache enable
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Data cache enable
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Instruction cache reset
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Data cache reset
    #[inline(always)]
    pub fn dcrst(&self) -> DCRST_R {
        DCRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Flash Power-down mode during Low-power run mode
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Flash Power-down mode during Low-power sleep mode
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 18 - Debug software enable
    #[inline(always)]
    pub fn dbg_swen(&self) -> DBG_SWEN_R {
        DBG_SWEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:3 - Latency
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W {
        LATENCY_W { w: self }
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W {
        PRFTEN_W { w: self }
    }
    ///Bit 9 - Instruction cache enable
    #[inline(always)]
    pub fn icen(&mut self) -> ICEN_W {
        ICEN_W { w: self }
    }
    ///Bit 10 - Data cache enable
    #[inline(always)]
    pub fn dcen(&mut self) -> DCEN_W {
        DCEN_W { w: self }
    }
    ///Bit 11 - Instruction cache reset
    #[inline(always)]
    pub fn icrst(&mut self) -> ICRST_W {
        ICRST_W { w: self }
    }
    ///Bit 12 - Data cache reset
    #[inline(always)]
    pub fn dcrst(&mut self) -> DCRST_W {
        DCRST_W { w: self }
    }
    ///Bit 13 - Flash Power-down mode during Low-power run mode
    #[inline(always)]
    pub fn run_pd(&mut self) -> RUN_PD_W {
        RUN_PD_W { w: self }
    }
    ///Bit 14 - Flash Power-down mode during Low-power sleep mode
    #[inline(always)]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W {
        SLEEP_PD_W { w: self }
    }
    ///Bit 18 - Debug software enable
    #[inline(always)]
    pub fn dbg_swen(&mut self) -> DBG_SWEN_W {
        DBG_SWEN_W { w: self }
    }
}
