///Reader of register SMCR
pub type R = crate::R<u32, super::SMCR>;
///Writer for register SMCR
pub type W = crate::W<u32, super::SMCR>;
///Register SMCR `reset()`'s with value 0
impl crate::ResetValue for super::SMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `SMSPS`
pub type SMSPS_R = crate::R<bool, bool>;
///Write proxy for field `SMSPS`
pub struct SMSPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SMSPS_W<'a> {
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
///Reader of field `SMSPE`
pub type SMSPE_R = crate::R<bool, bool>;
///Write proxy for field `SMSPE`
pub struct SMSPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMSPE_W<'a> {
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
///Reader of field `TS_4_3`
pub type TS_4_3_R = crate::R<u8, u8>;
///Write proxy for field `TS_4_3`
pub struct TS_4_3_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_4_3_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
///Reader of field `SMS_3`
pub type SMS_3_R = crate::R<bool, bool>;
///Write proxy for field `SMS_3`
pub struct SMS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMS_3_W<'a> {
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
///Reader of field `ETP`
pub type ETP_R = crate::R<bool, bool>;
///Write proxy for field `ETP`
pub struct ETP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
///Reader of field `ECE`
pub type ECE_R = crate::R<bool, bool>;
///Write proxy for field `ECE`
pub struct ECE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECE_W<'a> {
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
///Reader of field `ETPS`
pub type ETPS_R = crate::R<u8, u8>;
///Write proxy for field `ETPS`
pub struct ETPS_W<'a> {
    w: &'a mut W,
}
impl<'a> ETPS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
///Reader of field `ETF`
pub type ETF_R = crate::R<u8, u8>;
///Write proxy for field `ETF`
pub struct ETF_W<'a> {
    w: &'a mut W,
}
impl<'a> ETF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
///Reader of field `MSM`
pub type MSM_R = crate::R<bool, bool>;
///Write proxy for field `MSM`
pub struct MSM_W<'a> {
    w: &'a mut W,
}
impl<'a> MSM_W<'a> {
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
///Reader of field `TS`
pub type TS_R = crate::R<u8, u8>;
///Write proxy for field `TS`
pub struct TS_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
///Reader of field `OCCS`
pub type OCCS_R = crate::R<bool, bool>;
///Write proxy for field `OCCS`
pub struct OCCS_W<'a> {
    w: &'a mut W,
}
impl<'a> OCCS_W<'a> {
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
///Reader of field `SMS`
pub type SMS_R = crate::R<u8, u8>;
///Write proxy for field `SMS`
pub struct SMS_W<'a> {
    w: &'a mut W,
}
impl<'a> SMS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    ///Bit 25 - SMS Preload Source
    #[inline(always)]
    pub fn smsps(&self) -> SMSPS_R {
        SMSPS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 24 - SMS Preload Enable
    #[inline(always)]
    pub fn smspe(&self) -> SMSPE_R {
        SMSPE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bits 20:21 - Trigger selection - bit 4:3
    #[inline(always)]
    pub fn ts_4_3(&self) -> TS_4_3_R {
        TS_4_3_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    ///Bit 16 - Slave mode selection - bit 3
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 7 - Master/Slave mode
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bit 3 - OCREF clear selection
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bit 25 - SMS Preload Source
    #[inline(always)]
    pub fn smsps(&mut self) -> SMSPS_W {
        SMSPS_W { w: self }
    }
    ///Bit 24 - SMS Preload Enable
    #[inline(always)]
    pub fn smspe(&mut self) -> SMSPE_W {
        SMSPE_W { w: self }
    }
    ///Bits 20:21 - Trigger selection - bit 4:3
    #[inline(always)]
    pub fn ts_4_3(&mut self) -> TS_4_3_W {
        TS_4_3_W { w: self }
    }
    ///Bit 16 - Slave mode selection - bit 3
    #[inline(always)]
    pub fn sms_3(&mut self) -> SMS_3_W {
        SMS_3_W { w: self }
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W {
        ETP_W { w: self }
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&mut self) -> ECE_W {
        ECE_W { w: self }
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&mut self) -> ETPS_W {
        ETPS_W { w: self }
    }
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W {
        ETF_W { w: self }
    }
    ///Bit 7 - Master/Slave mode
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W {
        MSM_W { w: self }
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W {
        TS_W { w: self }
    }
    ///Bit 3 - OCREF clear selection
    #[inline(always)]
    pub fn occs(&mut self) -> OCCS_W {
        OCCS_W { w: self }
    }
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W {
        SMS_W { w: self }
    }
}
