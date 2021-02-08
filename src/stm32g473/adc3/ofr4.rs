///Reader of register OFR4
pub type R = crate::R<u32, super::OFR4>;
///Writer for register OFR4
pub type W = crate::W<u32, super::OFR4>;
///Register OFR4 `reset()`'s with value 0
impl crate::ResetValue for super::OFR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Offset 4 Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFSET4_EN_A {
    ///0: Offset disabled
    DISABLED = 0,
    ///1: Offset enabled
    ENABLED = 1,
}
impl From<OFFSET4_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OFFSET4_EN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OFFSET4_EN`
pub type OFFSET4_EN_R = crate::R<bool, OFFSET4_EN_A>;
impl OFFSET4_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OFFSET4_EN_A {
        match self.bits {
            false => OFFSET4_EN_A::DISABLED,
            true => OFFSET4_EN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OFFSET4_EN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OFFSET4_EN_A::ENABLED
    }
}
///Write proxy for field `OFFSET4_EN`
pub struct OFFSET4_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET4_EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OFFSET4_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Offset disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OFFSET4_EN_A::DISABLED)
    }
    ///Offset enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OFFSET4_EN_A::ENABLED)
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
///Reader of field `OFFSET4_CH`
pub type OFFSET4_CH_R = crate::R<u8, u8>;
///Write proxy for field `OFFSET4_CH`
pub struct OFFSET4_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET4_CH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
///Reader of field `SATEN`
pub type SATEN_R = crate::R<bool, bool>;
///Write proxy for field `SATEN`
pub struct SATEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SATEN_W<'a> {
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
///Reader of field `OFFSETPOS`
pub type OFFSETPOS_R = crate::R<bool, bool>;
///Write proxy for field `OFFSETPOS`
pub struct OFFSETPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETPOS_W<'a> {
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
///Reader of field `OFFSET4`
pub type OFFSET4_R = crate::R<u16, u16>;
///Write proxy for field `OFFSET4`
pub struct OFFSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET4_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    ///Bit 31 - Offset 4 Enable
    #[inline(always)]
    pub fn offset4_en(&self) -> OFFSET4_EN_R {
        OFFSET4_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bits 26:30 - Channel selection for the data offset 4
    #[inline(always)]
    pub fn offset4_ch(&self) -> OFFSET4_CH_R {
        OFFSET4_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 25 - Saturation enable
    #[inline(always)]
    pub fn saten(&self) -> SATEN_R {
        SATEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 24 - Positive offset
    #[inline(always)]
    pub fn offsetpos(&self) -> OFFSETPOS_R {
        OFFSETPOS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bits 0:11 - Data offset 4 for the channel programmed into bits OFFSET4_CH
    #[inline(always)]
    pub fn offset4(&self) -> OFFSET4_R {
        OFFSET4_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bit 31 - Offset 4 Enable
    #[inline(always)]
    pub fn offset4_en(&mut self) -> OFFSET4_EN_W {
        OFFSET4_EN_W { w: self }
    }
    ///Bits 26:30 - Channel selection for the data offset 4
    #[inline(always)]
    pub fn offset4_ch(&mut self) -> OFFSET4_CH_W {
        OFFSET4_CH_W { w: self }
    }
    ///Bit 25 - Saturation enable
    #[inline(always)]
    pub fn saten(&mut self) -> SATEN_W {
        SATEN_W { w: self }
    }
    ///Bit 24 - Positive offset
    #[inline(always)]
    pub fn offsetpos(&mut self) -> OFFSETPOS_W {
        OFFSETPOS_W { w: self }
    }
    ///Bits 0:11 - Data offset 4 for the channel programmed into bits OFFSET4_CH
    #[inline(always)]
    pub fn offset4(&mut self) -> OFFSET4_W {
        OFFSET4_W { w: self }
    }
}
