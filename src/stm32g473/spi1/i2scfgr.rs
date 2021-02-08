///Reader of register I2SCFGR
pub type R = crate::R<u32, super::I2SCFGR>;
///Writer for register I2SCFGR
pub type W = crate::W<u32, super::I2SCFGR>;
///Register I2SCFGR `reset()`'s with value 0
impl crate::ResetValue for super::I2SCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CHLEN`
pub type CHLEN_R = crate::R<bool, bool>;
///Write proxy for field `CHLEN`
pub struct CHLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHLEN_W<'a> {
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
///Reader of field `DATLEN`
pub type DATLEN_R = crate::R<u8, u8>;
///Write proxy for field `DATLEN`
pub struct DATLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATLEN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
///Reader of field `CKPOL`
pub type CKPOL_R = crate::R<bool, bool>;
///Write proxy for field `CKPOL`
pub struct CKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPOL_W<'a> {
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
///Reader of field `I2SSTD`
pub type I2SSTD_R = crate::R<u8, u8>;
///Write proxy for field `I2SSTD`
pub struct I2SSTD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SSTD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
///Reader of field `PCMSYNC`
pub type PCMSYNC_R = crate::R<bool, bool>;
///Write proxy for field `PCMSYNC`
pub struct PCMSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMSYNC_W<'a> {
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
///Reader of field `I2SCFG`
pub type I2SCFG_R = crate::R<u8, u8>;
///Write proxy for field `I2SCFG`
pub struct I2SCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SCFG_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
///Reader of field `I2SE`
pub type I2SE_R = crate::R<bool, bool>;
///Write proxy for field `I2SE`
pub struct I2SE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SE_W<'a> {
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
///Reader of field `I2SMOD`
pub type I2SMOD_R = crate::R<bool, bool>;
///Write proxy for field `I2SMOD`
pub struct I2SMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SMOD_W<'a> {
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
impl R {
    ///Bit 0 - CHLEN
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 1:2 - DATLEN
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    ///Bit 3 - CKPOL
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 4:5 - I2SSTD
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bit 7 - PCMSYNC
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 8:9 - I2SCFG
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bit 10 - I2SE
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - I2SMOD
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - CHLEN
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W {
        CHLEN_W { w: self }
    }
    ///Bits 1:2 - DATLEN
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W {
        DATLEN_W { w: self }
    }
    ///Bit 3 - CKPOL
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W {
        CKPOL_W { w: self }
    }
    ///Bits 4:5 - I2SSTD
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2SSTD_W {
        I2SSTD_W { w: self }
    }
    ///Bit 7 - PCMSYNC
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PCMSYNC_W {
        PCMSYNC_W { w: self }
    }
    ///Bits 8:9 - I2SCFG
    #[inline(always)]
    pub fn i2scfg(&mut self) -> I2SCFG_W {
        I2SCFG_W { w: self }
    }
    ///Bit 10 - I2SE
    #[inline(always)]
    pub fn i2se(&mut self) -> I2SE_W {
        I2SE_W { w: self }
    }
    ///Bit 11 - I2SMOD
    #[inline(always)]
    pub fn i2smod(&mut self) -> I2SMOD_W {
        I2SMOD_W { w: self }
    }
}
