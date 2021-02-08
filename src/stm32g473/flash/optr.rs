///Reader of register OPTR
pub type R = crate::R<u32, super::OPTR>;
///Writer for register OPTR
pub type W = crate::W<u32, super::OPTR>;
///Register OPTR `reset()`'s with value 0xf000_0000
impl crate::ResetValue for super::OPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf000_0000
    }
}
///Read protection level
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RDP_A {
    ///170: Level 0, read protection not active
    LEVEL0 = 170,
    ///204: Level 2, chip read protection active
    LEVEL2 = 204,
    ///0: Level 1, memories read protection active
    LEVEL1 = 0,
}
impl From<RDP_A> for u8 {
    #[inline(always)]
    fn from(variant: RDP_A) -> Self {
        variant as _
    }
}
///Reader of field `RDP`
pub type RDP_R = crate::R<u8, RDP_A>;
impl RDP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RDP_A> {
        use crate::Variant::*;
        match self.bits {
            170 => Val(RDP_A::LEVEL0),
            204 => Val(RDP_A::LEVEL2),
            0 => Val(RDP_A::LEVEL1),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `LEVEL0`
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RDP_A::LEVEL0
    }
    ///Checks if the value of the field is `LEVEL2`
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RDP_A::LEVEL2
    }
    ///Checks if the value of the field is `LEVEL1`
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == RDP_A::LEVEL1
    }
}
///Write proxy for field `RDP`
pub struct RDP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RDP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Level 0, read protection not active
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(RDP_A::LEVEL0)
    }
    ///Level 2, chip read protection active
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(RDP_A::LEVEL2)
    }
    ///Level 1, memories read protection active
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(RDP_A::LEVEL1)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
///BOR reset Level
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOR_LEV_A {
    ///0: BOR Level 0. Reset level threshold is around 1.7 V
    LEVEL0 = 0,
    ///1: BOR Level 1. Reset level threshold is around 2.0 V
    LEVEL1 = 1,
    ///2: BOR Level 2. Reset level threshold is around 2.2 V
    LEVEL2 = 2,
    ///3: BOR Level 3. Reset level threshold is around 2.5 V
    LEVEL3 = 3,
    ///4: BOR Level 4. Reset level threshold is around 2.8 V
    LEVEL4 = 4,
}
impl From<BOR_LEV_A> for u8 {
    #[inline(always)]
    fn from(variant: BOR_LEV_A) -> Self {
        variant as _
    }
}
///Reader of field `BOR_LEV`
pub type BOR_LEV_R = crate::R<u8, BOR_LEV_A>;
impl BOR_LEV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BOR_LEV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BOR_LEV_A::LEVEL0),
            1 => Val(BOR_LEV_A::LEVEL1),
            2 => Val(BOR_LEV_A::LEVEL2),
            3 => Val(BOR_LEV_A::LEVEL3),
            4 => Val(BOR_LEV_A::LEVEL4),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `LEVEL0`
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == BOR_LEV_A::LEVEL0
    }
    ///Checks if the value of the field is `LEVEL1`
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == BOR_LEV_A::LEVEL1
    }
    ///Checks if the value of the field is `LEVEL2`
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == BOR_LEV_A::LEVEL2
    }
    ///Checks if the value of the field is `LEVEL3`
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == BOR_LEV_A::LEVEL3
    }
    ///Checks if the value of the field is `LEVEL4`
    #[inline(always)]
    pub fn is_level4(&self) -> bool {
        *self == BOR_LEV_A::LEVEL4
    }
}
///Write proxy for field `BOR_LEV`
pub struct BOR_LEV_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR_LEV_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BOR_LEV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///BOR Level 0. Reset level threshold is around 1.7 V
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(BOR_LEV_A::LEVEL0)
    }
    ///BOR Level 1. Reset level threshold is around 2.0 V
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(BOR_LEV_A::LEVEL1)
    }
    ///BOR Level 2. Reset level threshold is around 2.2 V
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(BOR_LEV_A::LEVEL2)
    }
    ///BOR Level 3. Reset level threshold is around 2.5 V
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(BOR_LEV_A::LEVEL3)
    }
    ///BOR Level 4. Reset level threshold is around 2.8 V
    #[inline(always)]
    pub fn level4(self) -> &'a mut W {
        self.variant(BOR_LEV_A::LEVEL4)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
///nRST_STOP
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRST_STOP_A {
    ///0: Reset generated when entering the Stop mode
    DISABLED = 0,
    ///1: No reset generated when entering the Stop mode
    ENABLED = 1,
}
impl From<NRST_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: NRST_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `nRST_STOP`
pub type NRST_STOP_R = crate::R<bool, NRST_STOP_A>;
impl NRST_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NRST_STOP_A {
        match self.bits {
            false => NRST_STOP_A::DISABLED,
            true => NRST_STOP_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NRST_STOP_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NRST_STOP_A::ENABLED
    }
}
///Write proxy for field `nRST_STOP`
pub struct NRST_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NRST_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset generated when entering the Stop mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NRST_STOP_A::DISABLED)
    }
    ///No reset generated when entering the Stop mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NRST_STOP_A::ENABLED)
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
///nRST_STDBY
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRST_STDBY_A {
    ///0: Reset generated when entering the Standby mode
    DISABLED = 0,
    ///1: No reset generated when entering the Standby mode
    ENABLED = 1,
}
impl From<NRST_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: NRST_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `nRST_STDBY`
pub type NRST_STDBY_R = crate::R<bool, NRST_STDBY_A>;
impl NRST_STDBY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NRST_STDBY_A {
        match self.bits {
            false => NRST_STDBY_A::DISABLED,
            true => NRST_STDBY_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NRST_STDBY_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NRST_STDBY_A::ENABLED
    }
}
///Write proxy for field `nRST_STDBY`
pub struct NRST_STDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_STDBY_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NRST_STDBY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset generated when entering the Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NRST_STDBY_A::DISABLED)
    }
    ///No reset generated when entering the Standby mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NRST_STDBY_A::ENABLED)
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
///nRST_SHDW
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRST_SHDW_A {
    ///0: Reset generated when entering the Shutdown mode
    DISABLED = 0,
    ///1: No reset generated when entering the Shutdown mode
    ENABLED = 1,
}
impl From<NRST_SHDW_A> for bool {
    #[inline(always)]
    fn from(variant: NRST_SHDW_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `nRST_SHDW`
pub type NRST_SHDW_R = crate::R<bool, NRST_SHDW_A>;
impl NRST_SHDW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NRST_SHDW_A {
        match self.bits {
            false => NRST_SHDW_A::DISABLED,
            true => NRST_SHDW_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NRST_SHDW_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NRST_SHDW_A::ENABLED
    }
}
///Write proxy for field `nRST_SHDW`
pub struct NRST_SHDW_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_SHDW_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NRST_SHDW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Reset generated when entering the Shutdown mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NRST_SHDW_A::DISABLED)
    }
    ///No reset generated when entering the Shutdown mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NRST_SHDW_A::ENABLED)
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
///Independent watchdog selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDWG_SW_A {
    ///0: Hardware independent watchdog
    HARDWARE = 0,
    ///1: Software independent watchdog
    SOFTWARE = 1,
}
impl From<IDWG_SW_A> for bool {
    #[inline(always)]
    fn from(variant: IDWG_SW_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `IDWG_SW`
pub type IDWG_SW_R = crate::R<bool, IDWG_SW_A>;
impl IDWG_SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IDWG_SW_A {
        match self.bits {
            false => IDWG_SW_A::HARDWARE,
            true => IDWG_SW_A::SOFTWARE,
        }
    }
    ///Checks if the value of the field is `HARDWARE`
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == IDWG_SW_A::HARDWARE
    }
    ///Checks if the value of the field is `SOFTWARE`
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == IDWG_SW_A::SOFTWARE
    }
}
///Write proxy for field `IDWG_SW`
pub struct IDWG_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> IDWG_SW_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IDWG_SW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Hardware independent watchdog
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(IDWG_SW_A::HARDWARE)
    }
    ///Software independent watchdog
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(IDWG_SW_A::SOFTWARE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
///Independent watchdog counter freeze in Stop mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWDG_STOP_A {
    ///0: Independent watchdog counter is frozen in Stop mode
    DIASBLED = 0,
    ///1: Independent watchdog counter is running in Stop mode
    ENABLED = 1,
}
impl From<IWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `IWDG_STOP`
pub type IWDG_STOP_R = crate::R<bool, IWDG_STOP_A>;
impl IWDG_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IWDG_STOP_A {
        match self.bits {
            false => IWDG_STOP_A::DIASBLED,
            true => IWDG_STOP_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DIASBLED`
    #[inline(always)]
    pub fn is_diasbled(&self) -> bool {
        *self == IWDG_STOP_A::DIASBLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IWDG_STOP_A::ENABLED
    }
}
///Write proxy for field `IWDG_STOP`
pub struct IWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IWDG_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Independent watchdog counter is frozen in Stop mode
    #[inline(always)]
    pub fn diasbled(self) -> &'a mut W {
        self.variant(IWDG_STOP_A::DIASBLED)
    }
    ///Independent watchdog counter is running in Stop mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IWDG_STOP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
///Independent watchdog counter freeze in Standby mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWDG_STDBY_A {
    ///0: Independent watchdog counter is frozen in Standby mode
    DIASBLED = 0,
    ///1: Independent watchdog counter is running in Standby mode
    ENABLED = 1,
}
impl From<IWDG_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `IWDG_STDBY`
pub type IWDG_STDBY_R = crate::R<bool, IWDG_STDBY_A>;
impl IWDG_STDBY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IWDG_STDBY_A {
        match self.bits {
            false => IWDG_STDBY_A::DIASBLED,
            true => IWDG_STDBY_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DIASBLED`
    #[inline(always)]
    pub fn is_diasbled(&self) -> bool {
        *self == IWDG_STDBY_A::DIASBLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IWDG_STDBY_A::ENABLED
    }
}
///Write proxy for field `IWDG_STDBY`
pub struct IWDG_STDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG_STDBY_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IWDG_STDBY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Independent watchdog counter is frozen in Standby mode
    #[inline(always)]
    pub fn diasbled(self) -> &'a mut W {
        self.variant(IWDG_STDBY_A::DIASBLED)
    }
    ///Independent watchdog counter is running in Standby mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IWDG_STDBY_A::ENABLED)
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
///Window watchdog selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDG_SW_A {
    ///0: Hardware window watchdog
    HARDWARE = 0,
    ///1: Software window watchdog
    SOFTWARE = 1,
}
impl From<WWDG_SW_A> for bool {
    #[inline(always)]
    fn from(variant: WWDG_SW_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `WWDG_SW`
pub type WWDG_SW_R = crate::R<bool, WWDG_SW_A>;
impl WWDG_SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WWDG_SW_A {
        match self.bits {
            false => WWDG_SW_A::HARDWARE,
            true => WWDG_SW_A::SOFTWARE,
        }
    }
    ///Checks if the value of the field is `HARDWARE`
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == WWDG_SW_A::HARDWARE
    }
    ///Checks if the value of the field is `SOFTWARE`
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == WWDG_SW_A::SOFTWARE
    }
}
///Write proxy for field `WWDG_SW`
pub struct WWDG_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDG_SW_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WWDG_SW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Hardware window watchdog
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(WWDG_SW_A::HARDWARE)
    }
    ///Software window watchdog
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(WWDG_SW_A::SOFTWARE)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
///Reader of field `nBOOT1`
pub type NBOOT1_R = crate::R<bool, bool>;
///Write proxy for field `nBOOT1`
pub struct NBOOT1_W<'a> {
    w: &'a mut W,
}
impl<'a> NBOOT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
///Reader of field `SRAM_PE`
pub type SRAM_PE_R = crate::R<bool, bool>;
///Write proxy for field `SRAM_PE`
pub struct SRAM_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_PE_W<'a> {
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
///Reader of field `CCMSRAM_RST`
pub type CCMSRAM_RST_R = crate::R<bool, bool>;
///Write proxy for field `CCMSRAM_RST`
pub struct CCMSRAM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CCMSRAM_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
///nSWBOOT0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSWBOOT0_A {
    ///0: nSWBOOT0 option disabled
    DISABLED = 0,
    ///1: nSWBOOT0 option enabled
    ENABLED = 1,
}
impl From<NSWBOOT0_A> for bool {
    #[inline(always)]
    fn from(variant: NSWBOOT0_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `nSWBOOT0`
pub type NSWBOOT0_R = crate::R<bool, NSWBOOT0_A>;
impl NSWBOOT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NSWBOOT0_A {
        match self.bits {
            false => NSWBOOT0_A::DISABLED,
            true => NSWBOOT0_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NSWBOOT0_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NSWBOOT0_A::ENABLED
    }
}
///Write proxy for field `nSWBOOT0`
pub struct NSWBOOT0_W<'a> {
    w: &'a mut W,
}
impl<'a> NSWBOOT0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NSWBOOT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///nSWBOOT0 option disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NSWBOOT0_A::DISABLED)
    }
    ///nSWBOOT0 option enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NSWBOOT0_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
///nBOOT0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBOOT0_A {
    ///0: nBOOT0 option disabled
    DISABLED = 0,
    ///1: nBOOT0 option enabled
    ENABLED = 1,
}
impl From<NBOOT0_A> for bool {
    #[inline(always)]
    fn from(variant: NBOOT0_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `nBOOT0`
pub type NBOOT0_R = crate::R<bool, NBOOT0_A>;
impl NBOOT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NBOOT0_A {
        match self.bits {
            false => NBOOT0_A::DISABLED,
            true => NBOOT0_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NBOOT0_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NBOOT0_A::ENABLED
    }
}
///Write proxy for field `nBOOT0`
pub struct NBOOT0_W<'a> {
    w: &'a mut W,
}
impl<'a> NBOOT0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NBOOT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///nBOOT0 option disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NBOOT0_A::DISABLED)
    }
    ///nBOOT0 option enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NBOOT0_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
///NRST_MODE
///
///Value on reset: 3
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NRST_MODE_A {
    ///1: Reset Input only
    RESETINUT = 1,
    ///2: Standard GPIO pad functionality, only internal RESET possible
    GPIO = 2,
    ///3: NRST pin configured in reset input/output mode
    BIDIRECTIONALRESET = 3,
}
impl From<NRST_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: NRST_MODE_A) -> Self {
        variant as _
    }
}
///Reader of field `NRST_MODE`
pub type NRST_MODE_R = crate::R<u8, NRST_MODE_A>;
impl NRST_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NRST_MODE_A {
        match self.bits {
            1 => NRST_MODE_A::RESETINUT,
            2 => NRST_MODE_A::GPIO,
            3 => NRST_MODE_A::BIDIRECTIONALRESET,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `RESETINUT`
    #[inline(always)]
    pub fn is_reset_inut(&self) -> bool {
        *self == NRST_MODE_A::RESETINUT
    }
    ///Checks if the value of the field is `GPIO`
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == NRST_MODE_A::GPIO
    }
    ///Checks if the value of the field is `BIDIRECTIONALRESET`
    #[inline(always)]
    pub fn is_bidirectional_reset(&self) -> bool {
        *self == NRST_MODE_A::BIDIRECTIONALRESET
    }
}
///Write proxy for field `NRST_MODE`
pub struct NRST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_MODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NRST_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Reset Input only
    #[inline(always)]
    pub fn reset_inut(self) -> &'a mut W {
        self.variant(NRST_MODE_A::RESETINUT)
    }
    ///Standard GPIO pad functionality, only internal RESET possible
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(NRST_MODE_A::GPIO)
    }
    ///NRST pin configured in reset input/output mode
    #[inline(always)]
    pub fn bidirectional_reset(self) -> &'a mut W {
        self.variant(NRST_MODE_A::BIDIRECTIONALRESET)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
///IRHEN
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRHEN_A {
    ///0: Internal resets are propagated as simple pulse on NRST pin
    DISABLED = 0,
    ///1: Internal resets drives NRST pin low until it is seen as low level
    ENABLED = 1,
}
impl From<IRHEN_A> for bool {
    #[inline(always)]
    fn from(variant: IRHEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `IRHEN`
pub type IRHEN_R = crate::R<bool, IRHEN_A>;
impl IRHEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IRHEN_A {
        match self.bits {
            false => IRHEN_A::DISABLED,
            true => IRHEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IRHEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IRHEN_A::ENABLED
    }
}
///Write proxy for field `IRHEN`
pub struct IRHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRHEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IRHEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Internal resets are propagated as simple pulse on NRST pin
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IRHEN_A::DISABLED)
    }
    ///Internal resets drives NRST pin low until it is seen as low level
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IRHEN_A::ENABLED)
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
///Dual-bank boot
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFB2_A {
    ///0: Dual-rank boot disabled
    DISABLED = 0,
    ///1: Dual-rank boot enabled
    ENABLED = 1,
}
impl From<BFB2_A> for bool {
    #[inline(always)]
    fn from(variant: BFB2_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `BFB2`
pub type BFB2_R = crate::R<bool, BFB2_A>;
impl BFB2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BFB2_A {
        match self.bits {
            false => BFB2_A::DISABLED,
            true => BFB2_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BFB2_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BFB2_A::ENABLED
    }
}
///Write proxy for field `BFB2`
pub struct BFB2_W<'a> {
    w: &'a mut W,
}
impl<'a> BFB2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BFB2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Dual-rank boot disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BFB2_A::DISABLED)
    }
    ///Dual-rank boot enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BFB2_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
///
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBANK_A {
    ///0: Single bank mode with 128 bits data read width
    SINGLE = 0,
    ///1: Dual bank mode with 64 bits data
    DUAL = 1,
}
impl From<DBANK_A> for bool {
    #[inline(always)]
    fn from(variant: DBANK_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DBANK`
pub type DBANK_R = crate::R<bool, DBANK_A>;
impl DBANK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBANK_A {
        match self.bits {
            false => DBANK_A::SINGLE,
            true => DBANK_A::DUAL,
        }
    }
    ///Checks if the value of the field is `SINGLE`
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DBANK_A::SINGLE
    }
    ///Checks if the value of the field is `DUAL`
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == DBANK_A::DUAL
    }
}
///Write proxy for field `DBANK`
pub struct DBANK_W<'a> {
    w: &'a mut W,
}
impl<'a> DBANK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBANK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Single bank mode with 128 bits data read width
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DBANK_A::SINGLE)
    }
    ///Dual bank mode with 64 bits data
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(DBANK_A::DUAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bit 12 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - nRST_SHDW
    #[inline(always)]
    pub fn n_rst_shdw(&self) -> NRST_SHDW_R {
        NRST_SHDW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn idwg_sw(&self) -> IDWG_SW_R {
        IDWG_SW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 23 - Boot configuration
    #[inline(always)]
    pub fn n_boot1(&self) -> NBOOT1_R {
        NBOOT1_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - SRAM1 and CCM SRAM parity check enable
    #[inline(always)]
    pub fn sram_pe(&self) -> SRAM_PE_R {
        SRAM_PE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - CCM SRAM Erase when system reset
    #[inline(always)]
    pub fn ccmsram_rst(&self) -> CCMSRAM_RST_R {
        CCMSRAM_RST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - nSWBOOT0
    #[inline(always)]
    pub fn n_swboot0(&self) -> NSWBOOT0_R {
        NSWBOOT0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - nBOOT0
    #[inline(always)]
    pub fn n_boot0(&self) -> NBOOT0_R {
        NBOOT0_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bits 28:29 - NRST_MODE
    #[inline(always)]
    pub fn nrst_mode(&self) -> NRST_MODE_R {
        NRST_MODE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    ///Bit 30 - IRHEN
    #[inline(always)]
    pub fn irhen(&self) -> IRHEN_R {
        IRHEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 20 - Dual-bank boot
    #[inline(always)]
    pub fn bfb2(&self) -> BFB2_R {
        BFB2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 22
    #[inline(always)]
    pub fn dbank(&self) -> DBANK_R {
        DBANK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W {
        RDP_W { w: self }
    }
    ///Bits 8:10 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W {
        BOR_LEV_W { w: self }
    }
    ///Bit 12 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> NRST_STOP_W {
        NRST_STOP_W { w: self }
    }
    ///Bit 13 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> NRST_STDBY_W {
        NRST_STDBY_W { w: self }
    }
    ///Bit 14 - nRST_SHDW
    #[inline(always)]
    pub fn n_rst_shdw(&mut self) -> NRST_SHDW_W {
        NRST_SHDW_W { w: self }
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn idwg_sw(&mut self) -> IDWG_SW_W {
        IDWG_SW_W { w: self }
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W {
        IWDG_STOP_W { w: self }
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W {
        IWDG_STDBY_W { w: self }
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W {
        WWDG_SW_W { w: self }
    }
    ///Bit 23 - Boot configuration
    #[inline(always)]
    pub fn n_boot1(&mut self) -> NBOOT1_W {
        NBOOT1_W { w: self }
    }
    ///Bit 24 - SRAM1 and CCM SRAM parity check enable
    #[inline(always)]
    pub fn sram_pe(&mut self) -> SRAM_PE_W {
        SRAM_PE_W { w: self }
    }
    ///Bit 25 - CCM SRAM Erase when system reset
    #[inline(always)]
    pub fn ccmsram_rst(&mut self) -> CCMSRAM_RST_W {
        CCMSRAM_RST_W { w: self }
    }
    ///Bit 26 - nSWBOOT0
    #[inline(always)]
    pub fn n_swboot0(&mut self) -> NSWBOOT0_W {
        NSWBOOT0_W { w: self }
    }
    ///Bit 27 - nBOOT0
    #[inline(always)]
    pub fn n_boot0(&mut self) -> NBOOT0_W {
        NBOOT0_W { w: self }
    }
    ///Bits 28:29 - NRST_MODE
    #[inline(always)]
    pub fn nrst_mode(&mut self) -> NRST_MODE_W {
        NRST_MODE_W { w: self }
    }
    ///Bit 30 - IRHEN
    #[inline(always)]
    pub fn irhen(&mut self) -> IRHEN_W {
        IRHEN_W { w: self }
    }
    ///Bit 20 - Dual-bank boot
    #[inline(always)]
    pub fn bfb2(&mut self) -> BFB2_W {
        BFB2_W { w: self }
    }
    ///Bit 22
    #[inline(always)]
    pub fn dbank(&mut self) -> DBANK_W {
        DBANK_W { w: self }
    }
}
