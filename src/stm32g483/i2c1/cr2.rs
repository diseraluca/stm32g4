///Reader of register CR2
pub type R = crate::R<u32, super::CR2>;
///Writer for register CR2
pub type W = crate::W<u32, super::CR2>;
///Register CR2 `reset()`'s with value 0
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PECBYTE`
pub type PECBYTE_R = crate::R<bool, bool>;
///Write proxy for field `PECBYTE`
pub struct PECBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> PECBYTE_W<'a> {
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
///Reader of field `AUTOEND`
pub type AUTOEND_R = crate::R<bool, bool>;
///Write proxy for field `AUTOEND`
pub struct AUTOEND_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOEND_W<'a> {
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
///Reader of field `RELOAD`
pub type RELOAD_R = crate::R<bool, bool>;
///Write proxy for field `RELOAD`
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
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
///Reader of field `NBYTES`
pub type NBYTES_R = crate::R<u8, u8>;
///Write proxy for field `NBYTES`
pub struct NBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBYTES_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///Reader of field `NACK`
pub type NACK_R = crate::R<bool, bool>;
///Write proxy for field `NACK`
pub struct NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> NACK_W<'a> {
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
///Reader of field `STOP`
pub type STOP_R = crate::R<bool, bool>;
///Write proxy for field `STOP`
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
///Reader of field `START`
pub type START_R = crate::R<bool, bool>;
///Write proxy for field `START`
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
///Reader of field `HEAD10R`
pub type HEAD10R_R = crate::R<bool, bool>;
///Write proxy for field `HEAD10R`
pub struct HEAD10R_W<'a> {
    w: &'a mut W,
}
impl<'a> HEAD10R_W<'a> {
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
///Reader of field `ADD10`
pub type ADD10_R = crate::R<bool, bool>;
///Write proxy for field `ADD10`
pub struct ADD10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD10_W<'a> {
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
///Reader of field `RD_WRN`
pub type RD_WRN_R = crate::R<bool, bool>;
///Write proxy for field `RD_WRN`
pub struct RD_WRN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_WRN_W<'a> {
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
///Reader of field `SADD`
pub type SADD_R = crate::R<u16, u16>;
///Write proxy for field `SADD`
pub struct SADD_W<'a> {
    w: &'a mut W,
}
impl<'a> SADD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    ///Bit 26 - Packet error checking byte
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 25 - Automatic end mode (master mode)
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 24 - NBYTES reload mode
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bits 16:23 - Number of bytes
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 15 - NACK generation (slave mode)
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Stop generation (master mode)
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Start generation
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - 10-bit address header only read direction (master receiver mode)
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - 10-bit addressing mode (master mode)
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Transfer direction (master mode)
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bits 0:9 - Slave address bit (master mode)
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bit 26 - Packet error checking byte
    #[inline(always)]
    pub fn pecbyte(&mut self) -> PECBYTE_W {
        PECBYTE_W { w: self }
    }
    ///Bit 25 - Automatic end mode (master mode)
    #[inline(always)]
    pub fn autoend(&mut self) -> AUTOEND_W {
        AUTOEND_W { w: self }
    }
    ///Bit 24 - NBYTES reload mode
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
    ///Bits 16:23 - Number of bytes
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W {
        NBYTES_W { w: self }
    }
    ///Bit 15 - NACK generation (slave mode)
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W { w: self }
    }
    ///Bit 14 - Stop generation (master mode)
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    ///Bit 13 - Start generation
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    ///Bit 12 - 10-bit address header only read direction (master receiver mode)
    #[inline(always)]
    pub fn head10r(&mut self) -> HEAD10R_W {
        HEAD10R_W { w: self }
    }
    ///Bit 11 - 10-bit addressing mode (master mode)
    #[inline(always)]
    pub fn add10(&mut self) -> ADD10_W {
        ADD10_W { w: self }
    }
    ///Bit 10 - Transfer direction (master mode)
    #[inline(always)]
    pub fn rd_wrn(&mut self) -> RD_WRN_W {
        RD_WRN_W { w: self }
    }
    ///Bits 0:9 - Slave address bit (master mode)
    #[inline(always)]
    pub fn sadd(&mut self) -> SADD_W {
        SADD_W { w: self }
    }
}
