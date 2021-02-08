///Reader of register CCR
pub type R = crate::R<u32, super::CCR>;
///Writer for register CCR
pub type W = crate::W<u32, super::CCR>;
///Register CCR `reset()`'s with value 0
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Dual ADC mode selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DUAL_A {
    ///0: Independent mode
    INDEPENDENT = 0,
    ///1: Dual, combined regular simultaneous + injected simultaneous mode
    DUALRJ = 1,
    ///2: Dual, combined regular simultaneous + alternate trigger mode
    DUALRA = 2,
    ///3: Dual, combined interleaved mode + injected simultaneous mode
    DUALIJ = 3,
    ///5: Dual, injected simultaneous mode only
    DUALJ = 5,
    ///6: Dual, regular simultaneous mode only
    DUALR = 6,
    ///7: Dual, interleaved mode only
    DUALI = 7,
    ///9: Dual, alternate trigger mode only
    DUALA = 9,
}
impl From<DUAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DUAL_A) -> Self {
        variant as _
    }
}
///Reader of field `DUAL`
pub type DUAL_R = crate::R<u8, DUAL_A>;
impl DUAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DUAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DUAL_A::INDEPENDENT),
            1 => Val(DUAL_A::DUALRJ),
            2 => Val(DUAL_A::DUALRA),
            3 => Val(DUAL_A::DUALIJ),
            5 => Val(DUAL_A::DUALJ),
            6 => Val(DUAL_A::DUALR),
            7 => Val(DUAL_A::DUALI),
            9 => Val(DUAL_A::DUALA),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `INDEPENDENT`
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == DUAL_A::INDEPENDENT
    }
    ///Checks if the value of the field is `DUALRJ`
    #[inline(always)]
    pub fn is_dual_rj(&self) -> bool {
        *self == DUAL_A::DUALRJ
    }
    ///Checks if the value of the field is `DUALRA`
    #[inline(always)]
    pub fn is_dual_ra(&self) -> bool {
        *self == DUAL_A::DUALRA
    }
    ///Checks if the value of the field is `DUALIJ`
    #[inline(always)]
    pub fn is_dual_ij(&self) -> bool {
        *self == DUAL_A::DUALIJ
    }
    ///Checks if the value of the field is `DUALJ`
    #[inline(always)]
    pub fn is_dual_j(&self) -> bool {
        *self == DUAL_A::DUALJ
    }
    ///Checks if the value of the field is `DUALR`
    #[inline(always)]
    pub fn is_dual_r(&self) -> bool {
        *self == DUAL_A::DUALR
    }
    ///Checks if the value of the field is `DUALI`
    #[inline(always)]
    pub fn is_dual_i(&self) -> bool {
        *self == DUAL_A::DUALI
    }
    ///Checks if the value of the field is `DUALA`
    #[inline(always)]
    pub fn is_dual_a(&self) -> bool {
        *self == DUAL_A::DUALA
    }
}
///Write proxy for field `DUAL`
pub struct DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DUAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Independent mode
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(DUAL_A::INDEPENDENT)
    }
    ///Dual, combined regular simultaneous + injected simultaneous mode
    #[inline(always)]
    pub fn dual_rj(self) -> &'a mut W {
        self.variant(DUAL_A::DUALRJ)
    }
    ///Dual, combined regular simultaneous + alternate trigger mode
    #[inline(always)]
    pub fn dual_ra(self) -> &'a mut W {
        self.variant(DUAL_A::DUALRA)
    }
    ///Dual, combined interleaved mode + injected simultaneous mode
    #[inline(always)]
    pub fn dual_ij(self) -> &'a mut W {
        self.variant(DUAL_A::DUALIJ)
    }
    ///Dual, injected simultaneous mode only
    #[inline(always)]
    pub fn dual_j(self) -> &'a mut W {
        self.variant(DUAL_A::DUALJ)
    }
    ///Dual, regular simultaneous mode only
    #[inline(always)]
    pub fn dual_r(self) -> &'a mut W {
        self.variant(DUAL_A::DUALR)
    }
    ///Dual, interleaved mode only
    #[inline(always)]
    pub fn dual_i(self) -> &'a mut W {
        self.variant(DUAL_A::DUALI)
    }
    ///Dual, alternate trigger mode only
    #[inline(always)]
    pub fn dual_a(self) -> &'a mut W {
        self.variant(DUAL_A::DUALA)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
///Reader of field `DELAY`
pub type DELAY_R = crate::R<u8, u8>;
///Write proxy for field `DELAY`
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
///Reader of field `DMACFG`
pub type DMACFG_R = crate::R<bool, bool>;
///Write proxy for field `DMACFG`
pub struct DMACFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACFG_W<'a> {
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
///Reader of field `MDMA`
pub type MDMA_R = crate::R<u8, u8>;
///Write proxy for field `MDMA`
pub struct MDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
///ADC clock mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKMODE_A {
    ///0: Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock
    ASYNCHRONOUS = 0,
    ///1: Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck
    SYNCDIV1 = 1,
    ///2: Use AHB clock rcc_hclk3 divided by 2
    SYNCDIV2 = 2,
    ///3: Use AHB clock rcc_hclk3 divided by 4
    SYNCDIV4 = 3,
}
impl From<CKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as _
    }
}
///Reader of field `CKMODE`
pub type CKMODE_R = crate::R<u8, CKMODE_A>;
impl CKMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CKMODE_A {
        match self.bits {
            0 => CKMODE_A::ASYNCHRONOUS,
            1 => CKMODE_A::SYNCDIV1,
            2 => CKMODE_A::SYNCDIV2,
            3 => CKMODE_A::SYNCDIV4,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `ASYNCHRONOUS`
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == CKMODE_A::ASYNCHRONOUS
    }
    ///Checks if the value of the field is `SYNCDIV1`
    #[inline(always)]
    pub fn is_sync_div1(&self) -> bool {
        *self == CKMODE_A::SYNCDIV1
    }
    ///Checks if the value of the field is `SYNCDIV2`
    #[inline(always)]
    pub fn is_sync_div2(&self) -> bool {
        *self == CKMODE_A::SYNCDIV2
    }
    ///Checks if the value of the field is `SYNCDIV4`
    #[inline(always)]
    pub fn is_sync_div4(&self) -> bool {
        *self == CKMODE_A::SYNCDIV4
    }
}
///Write proxy for field `CKMODE`
pub struct CKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CKMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(CKMODE_A::ASYNCHRONOUS)
    }
    ///Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck
    #[inline(always)]
    pub fn sync_div1(self) -> &'a mut W {
        self.variant(CKMODE_A::SYNCDIV1)
    }
    ///Use AHB clock rcc_hclk3 divided by 2
    #[inline(always)]
    pub fn sync_div2(self) -> &'a mut W {
        self.variant(CKMODE_A::SYNCDIV2)
    }
    ///Use AHB clock rcc_hclk3 divided by 4
    #[inline(always)]
    pub fn sync_div4(self) -> &'a mut W {
        self.variant(CKMODE_A::SYNCDIV4)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
///VREFINT enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFEN_A {
    ///0: V_REFINT channel disabled
    DISABLED = 0,
    ///1: V_REFINT channel enabled
    ENABLED = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `VREFEN`
pub type VREFEN_R = crate::R<bool, VREFEN_A>;
impl VREFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::DISABLED,
            true => VREFEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREFEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREFEN_A::ENABLED
    }
}
///Write proxy for field `VREFEN`
pub struct VREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: VREFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///V_REFINT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREFEN_A::DISABLED)
    }
    ///V_REFINT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREFEN_A::ENABLED)
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
///V_TS temperature sensor channel selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSENSESEL_A {
    ///0: Temperature sensor channel disabled
    DISABLED = 0,
    ///1: Temperature sensor channel enabled
    ENABLED = 1,
}
impl From<VSENSESEL_A> for bool {
    #[inline(always)]
    fn from(variant: VSENSESEL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `VSENSESEL`
pub type VSENSESEL_R = crate::R<bool, VSENSESEL_A>;
impl VSENSESEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VSENSESEL_A {
        match self.bits {
            false => VSENSESEL_A::DISABLED,
            true => VSENSESEL_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VSENSESEL_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VSENSESEL_A::ENABLED
    }
}
///Write proxy for field `VSENSESEL`
pub struct VSENSESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VSENSESEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: VSENSESEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Temperature sensor channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VSENSESEL_A::DISABLED)
    }
    ///Temperature sensor channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VSENSESEL_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
///V_BAT battery voltage channel selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATSEL_A {
    ///0: V_BAT channel disabled
    DISABLED = 0,
    ///1: V_BAT channel enabled
    ENABLED = 1,
}
impl From<VBATSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VBATSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `VBATSEL`
pub type VBATSEL_R = crate::R<bool, VBATSEL_A>;
impl VBATSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VBATSEL_A {
        match self.bits {
            false => VBATSEL_A::DISABLED,
            true => VBATSEL_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATSEL_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATSEL_A::ENABLED
    }
}
///Write proxy for field `VBATSEL`
pub struct VBATSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: VBATSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///V_BAT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VBATSEL_A::DISABLED)
    }
    ///V_BAT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VBATSEL_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
///Reader of field `PRESC`
pub type PRESC_R = crate::R<u8, u8>;
///Write proxy for field `PRESC`
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
impl R {
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 13 - DMA configuration (for multi-ADC mode)
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bits 14:15 - Direct memory access mode for multi ADC mode
    #[inline(always)]
    pub fn mdma(&self) -> MDMA_R {
        MDMA_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - V_TS temperature sensor channel selection
    #[inline(always)]
    pub fn vsensesel(&self) -> VSENSESEL_R {
        VSENSESEL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - V_BAT battery voltage channel selection
    #[inline(always)]
    pub fn vbatsel(&self) -> VBATSEL_R {
        VBATSEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    pub fn dual(&mut self) -> DUAL_W {
        DUAL_W { w: self }
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W { w: self }
    }
    ///Bit 13 - DMA configuration (for multi-ADC mode)
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W {
        DMACFG_W { w: self }
    }
    ///Bits 14:15 - Direct memory access mode for multi ADC mode
    #[inline(always)]
    pub fn mdma(&mut self) -> MDMA_W {
        MDMA_W { w: self }
    }
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W {
        CKMODE_W { w: self }
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W {
        VREFEN_W { w: self }
    }
    ///Bit 23 - V_TS temperature sensor channel selection
    #[inline(always)]
    pub fn vsensesel(&mut self) -> VSENSESEL_W {
        VSENSESEL_W { w: self }
    }
    ///Bit 24 - V_BAT battery voltage channel selection
    #[inline(always)]
    pub fn vbatsel(&mut self) -> VBATSEL_W {
        VBATSEL_W { w: self }
    }
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
}
