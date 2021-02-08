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
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
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
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W {
        SMS_W { w: self }
    }
}
