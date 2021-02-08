///Reader of register ECCR
pub type R = crate::R<u32, super::ECCR>;
///Writer for register ECCR
pub type W = crate::W<u32, super::ECCR>;
///Register ECCR `reset()`'s with value 0
impl crate::ResetValue for super::ECCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `ADDR_ECC`
pub type ADDR_ECC_R = crate::R<u32, u32>;
///BK_ECC
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK_ECC_A {
    ///0: Bank1 concerned with ECC error
    BANK1 = 0,
    ///1: Bank2 concerned with ECC error
    BANK2 = 1,
}
impl From<BK_ECC_A> for bool {
    #[inline(always)]
    fn from(variant: BK_ECC_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `BK_ECC`
pub type BK_ECC_R = crate::R<bool, BK_ECC_A>;
impl BK_ECC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BK_ECC_A {
        match self.bits {
            false => BK_ECC_A::BANK1,
            true => BK_ECC_A::BANK2,
        }
    }
    ///Checks if the value of the field is `BANK1`
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BK_ECC_A::BANK1
    }
    ///Checks if the value of the field is `BANK2`
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BK_ECC_A::BANK2
    }
}
///SYSF_ECC
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSF_ECC_A {
    ///0: No system flash ECC fail
    DISABLED = 0,
    ///1: System flash ECC fail
    ENABLED = 1,
}
impl From<SYSF_ECC_A> for bool {
    #[inline(always)]
    fn from(variant: SYSF_ECC_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SYSF_ECC`
pub type SYSF_ECC_R = crate::R<bool, SYSF_ECC_A>;
impl SYSF_ECC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SYSF_ECC_A {
        match self.bits {
            false => SYSF_ECC_A::DISABLED,
            true => SYSF_ECC_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSF_ECC_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSF_ECC_A::ENABLED
    }
}
///ECCIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCIE_A {
    ///0: ECC correction interrupt disabled
    DISABLED = 0,
    ///1: ECC correction interrupt enabled
    ENABLED = 1,
}
impl From<ECCIE_A> for bool {
    #[inline(always)]
    fn from(variant: ECCIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ECCIE`
pub type ECCIE_R = crate::R<bool, ECCIE_A>;
impl ECCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ECCIE_A {
        match self.bits {
            false => ECCIE_A::DISABLED,
            true => ECCIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECCIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECCIE_A::ENABLED
    }
}
///Write proxy for field `ECCIE`
pub struct ECCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ECCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///ECC correction interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ECCIE_A::DISABLED)
    }
    ///ECC correction interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ECCIE_A::ENABLED)
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
///ECC correction
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCC2_A {
    ///0: No ECC error detected
    INACTIVE = 0,
    ///1: ECC error detected
    ACTIVE = 1,
}
impl From<ECCC2_A> for bool {
    #[inline(always)]
    fn from(variant: ECCC2_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ECCC2`
pub type ECCC2_R = crate::R<bool, ECCC2_A>;
impl ECCC2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ECCC2_A {
        match self.bits {
            false => ECCC2_A::INACTIVE,
            true => ECCC2_A::ACTIVE,
        }
    }
    ///Checks if the value of the field is `INACTIVE`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ECCC2_A::INACTIVE
    }
    ///Checks if the value of the field is `ACTIVE`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ECCC2_A::ACTIVE
    }
}
///Write proxy for field `ECCC2`
pub struct ECCC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCC2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ECCC2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No ECC error detected
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(ECCC2_A::INACTIVE)
    }
    ///ECC error detected
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(ECCC2_A::ACTIVE)
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
///ECC2 detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCD2_A {
    ///0: Less than two ECC errors detected
    INACTIVE = 0,
    ///1: Two ECC errors detected
    ACTIVE = 1,
}
impl From<ECCD2_A> for bool {
    #[inline(always)]
    fn from(variant: ECCD2_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ECCD2`
pub type ECCD2_R = crate::R<bool, ECCD2_A>;
impl ECCD2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ECCD2_A {
        match self.bits {
            false => ECCD2_A::INACTIVE,
            true => ECCD2_A::ACTIVE,
        }
    }
    ///Checks if the value of the field is `INACTIVE`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ECCD2_A::INACTIVE
    }
    ///Checks if the value of the field is `ACTIVE`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ECCD2_A::ACTIVE
    }
}
///Write proxy for field `ECCD2`
pub struct ECCD2_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCD2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ECCD2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Less than two ECC errors detected
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(ECCD2_A::INACTIVE)
    }
    ///Two ECC errors detected
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(ECCD2_A::ACTIVE)
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
///ECC correction
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCC_A {
    ///0: No ECC errors detected
    INACTIVE = 0,
    ///1: ECC error detected
    ACTIVE = 1,
}
impl From<ECCC_A> for bool {
    #[inline(always)]
    fn from(variant: ECCC_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ECCC`
pub type ECCC_R = crate::R<bool, ECCC_A>;
impl ECCC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ECCC_A {
        match self.bits {
            false => ECCC_A::INACTIVE,
            true => ECCC_A::ACTIVE,
        }
    }
    ///Checks if the value of the field is `INACTIVE`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ECCC_A::INACTIVE
    }
    ///Checks if the value of the field is `ACTIVE`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ECCC_A::ACTIVE
    }
}
///Write proxy for field `ECCC`
pub struct ECCC_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ECCC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No ECC errors detected
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(ECCC_A::INACTIVE)
    }
    ///ECC error detected
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(ECCC_A::ACTIVE)
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
///ECC detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCD_A {
    ///0: Less than two ECC errors detected
    INACTIVE = 0,
    ///1: Two ECC errors detected
    ACTIVE = 1,
}
impl From<ECCD_A> for bool {
    #[inline(always)]
    fn from(variant: ECCD_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ECCD`
pub type ECCD_R = crate::R<bool, ECCD_A>;
impl ECCD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ECCD_A {
        match self.bits {
            false => ECCD_A::INACTIVE,
            true => ECCD_A::ACTIVE,
        }
    }
    ///Checks if the value of the field is `INACTIVE`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ECCD_A::INACTIVE
    }
    ///Checks if the value of the field is `ACTIVE`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ECCD_A::ACTIVE
    }
}
///Write proxy for field `ECCD`
pub struct ECCD_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ECCD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Less than two ECC errors detected
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(ECCD_A::INACTIVE)
    }
    ///Two ECC errors detected
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(ECCD_A::ACTIVE)
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
impl R {
    ///Bits 0:18 - ECC fail address
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new((self.bits & 0x0007_ffff) as u32)
    }
    ///Bit 21 - BK_ECC
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - SYSF_ECC
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 24 - ECCIE
    #[inline(always)]
    pub fn eccie(&self) -> ECCIE_R {
        ECCIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 28 - ECC correction
    #[inline(always)]
    pub fn eccc2(&self) -> ECCC2_R {
        ECCC2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - ECC2 detection
    #[inline(always)]
    pub fn eccd2(&self) -> ECCD2_R {
        ECCD2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - ECC correction
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - ECC detection
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 24 - ECCIE
    #[inline(always)]
    pub fn eccie(&mut self) -> ECCIE_W {
        ECCIE_W { w: self }
    }
    ///Bit 28 - ECC correction
    #[inline(always)]
    pub fn eccc2(&mut self) -> ECCC2_W {
        ECCC2_W { w: self }
    }
    ///Bit 29 - ECC2 detection
    #[inline(always)]
    pub fn eccd2(&mut self) -> ECCD2_W {
        ECCD2_W { w: self }
    }
    ///Bit 30 - ECC correction
    #[inline(always)]
    pub fn eccc(&mut self) -> ECCC_W {
        ECCC_W { w: self }
    }
    ///Bit 31 - ECC detection
    #[inline(always)]
    pub fn eccd(&mut self) -> ECCD_W {
        ECCD_W { w: self }
    }
}
