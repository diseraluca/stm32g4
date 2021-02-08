///Reader of register OFR3
pub type R = crate::R<u32, super::OFR3>;
///Writer for register OFR3
pub type W = crate::W<u32, super::OFR3>;
///Register OFR3 `reset()`'s with value 0
impl crate::ResetValue for super::OFR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Offset 3 Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFSET3_EN_A {
    ///0: Offset disabled
    DISABLED = 0,
    ///1: Offset enabled
    ENABLED = 1,
}
impl From<OFFSET3_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OFFSET3_EN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OFFSET3_EN`
pub type OFFSET3_EN_R = crate::R<bool, OFFSET3_EN_A>;
impl OFFSET3_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OFFSET3_EN_A {
        match self.bits {
            false => OFFSET3_EN_A::DISABLED,
            true => OFFSET3_EN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OFFSET3_EN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OFFSET3_EN_A::ENABLED
    }
}
///Write proxy for field `OFFSET3_EN`
pub struct OFFSET3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET3_EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OFFSET3_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Offset disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OFFSET3_EN_A::DISABLED)
    }
    ///Offset enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OFFSET3_EN_A::ENABLED)
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
///Reader of field `OFFSET3_CH`
pub type OFFSET3_CH_R = crate::R<u8, u8>;
///Write proxy for field `OFFSET3_CH`
pub struct OFFSET3_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET3_CH_W<'a> {
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
///Reader of field `OFFSET3`
pub type OFFSET3_R = crate::R<u16, u16>;
///Write proxy for field `OFFSET3`
pub struct OFFSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET3_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    ///Bit 31 - Offset 3 Enable
    #[inline(always)]
    pub fn offset3_en(&self) -> OFFSET3_EN_R {
        OFFSET3_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bits 26:30 - Channel selection for the data offset 3
    #[inline(always)]
    pub fn offset3_ch(&self) -> OFFSET3_CH_R {
        OFFSET3_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
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
    ///Bits 0:11 - Data offset 3 for the channel programmed into bits OFFSET3_CH
    #[inline(always)]
    pub fn offset3(&self) -> OFFSET3_R {
        OFFSET3_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bit 31 - Offset 3 Enable
    #[inline(always)]
    pub fn offset3_en(&mut self) -> OFFSET3_EN_W {
        OFFSET3_EN_W { w: self }
    }
    ///Bits 26:30 - Channel selection for the data offset 3
    #[inline(always)]
    pub fn offset3_ch(&mut self) -> OFFSET3_CH_W {
        OFFSET3_CH_W { w: self }
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
    ///Bits 0:11 - Data offset 3 for the channel programmed into bits OFFSET3_CH
    #[inline(always)]
    pub fn offset3(&mut self) -> OFFSET3_W {
        OFFSET3_W { w: self }
    }
}
