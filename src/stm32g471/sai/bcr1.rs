///Reader of register BCR1
pub type R = crate::R<u32, super::BCR1>;
///Writer for register BCR1
pub type W = crate::W<u32, super::BCR1>;
///Register BCR1 `reset()`'s with value 0x40
impl crate::ResetValue for super::BCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
///Reader of field `MCKEN`
pub type MCKEN_R = crate::R<bool, bool>;
///Write proxy for field `MCKEN`
pub struct MCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKEN_W<'a> {
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
///Reader of field `OSR`
pub type OSR_R = crate::R<bool, bool>;
///Write proxy for field `OSR`
pub struct OSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSR_W<'a> {
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
///Reader of field `MCJDIV`
pub type MCJDIV_R = crate::R<u8, u8>;
///Write proxy for field `MCJDIV`
pub struct MCJDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCJDIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | (((value as u32) & 0x3f) << 20);
        self.w
    }
}
///Reader of field `NODIV`
pub type NODIV_R = crate::R<bool, bool>;
///Write proxy for field `NODIV`
pub struct NODIV_W<'a> {
    w: &'a mut W,
}
impl<'a> NODIV_W<'a> {
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
///Reader of field `DMAEN`
pub type DMAEN_R = crate::R<bool, bool>;
///Write proxy for field `DMAEN`
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
///Reader of field `SAIBEN`
pub type SAIBEN_R = crate::R<bool, bool>;
///Write proxy for field `SAIBEN`
pub struct SAIBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAIBEN_W<'a> {
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
///Reader of field `OutDri`
pub type OUTDRI_R = crate::R<bool, bool>;
///Write proxy for field `OutDri`
pub struct OUTDRI_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDRI_W<'a> {
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
///Reader of field `MONO`
pub type MONO_R = crate::R<bool, bool>;
///Write proxy for field `MONO`
pub struct MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_W<'a> {
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
///Reader of field `SYNCEN`
pub type SYNCEN_R = crate::R<u8, u8>;
///Write proxy for field `SYNCEN`
pub struct SYNCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
///Reader of field `CKSTR`
pub type CKSTR_R = crate::R<bool, bool>;
///Write proxy for field `CKSTR`
pub struct CKSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSTR_W<'a> {
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
///Reader of field `LSBFIRST`
pub type LSBFIRST_R = crate::R<bool, bool>;
///Write proxy for field `LSBFIRST`
pub struct LSBFIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBFIRST_W<'a> {
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
///Reader of field `DS`
pub type DS_R = crate::R<u8, u8>;
///Write proxy for field `DS`
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
///Reader of field `PRTCFG`
pub type PRTCFG_R = crate::R<u8, u8>;
///Write proxy for field `PRTCFG`
pub struct PRTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTCFG_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
///Reader of field `MODE`
pub type MODE_R = crate::R<u8, u8>;
///Write proxy for field `MODE`
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    ///Bit 27 - MCKEN
    #[inline(always)]
    pub fn mcken(&self) -> MCKEN_R {
        MCKEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 26 - OSR
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bits 20:25 - Master clock divider
    #[inline(always)]
    pub fn mcjdiv(&self) -> MCJDIV_R {
        MCJDIV_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 19 - No divider
    #[inline(always)]
    pub fn nodiv(&self) -> NODIV_R {
        NODIV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 17 - DMA enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - Audio block B enable
    #[inline(always)]
    pub fn saiben(&self) -> SAIBEN_R {
        SAIBEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 13 - Output drive
    #[inline(always)]
    pub fn out_dri(&self) -> OUTDRI_R {
        OUTDRI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Mono mode
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bits 10:11 - Synchronization enable
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bit 9 - Clock strobing edge
    #[inline(always)]
    pub fn ckstr(&self) -> CKSTR_R {
        CKSTR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Least significant bit first
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 5:7 - Data size
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    ///Bits 2:3 - Protocol configuration
    #[inline(always)]
    pub fn prtcfg(&self) -> PRTCFG_R {
        PRTCFG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 0:1 - Audio block mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bit 27 - MCKEN
    #[inline(always)]
    pub fn mcken(&mut self) -> MCKEN_W {
        MCKEN_W { w: self }
    }
    ///Bit 26 - OSR
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W {
        OSR_W { w: self }
    }
    ///Bits 20:25 - Master clock divider
    #[inline(always)]
    pub fn mcjdiv(&mut self) -> MCJDIV_W {
        MCJDIV_W { w: self }
    }
    ///Bit 19 - No divider
    #[inline(always)]
    pub fn nodiv(&mut self) -> NODIV_W {
        NODIV_W { w: self }
    }
    ///Bit 17 - DMA enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    ///Bit 16 - Audio block B enable
    #[inline(always)]
    pub fn saiben(&mut self) -> SAIBEN_W {
        SAIBEN_W { w: self }
    }
    ///Bit 13 - Output drive
    #[inline(always)]
    pub fn out_dri(&mut self) -> OUTDRI_W {
        OUTDRI_W { w: self }
    }
    ///Bit 12 - Mono mode
    #[inline(always)]
    pub fn mono(&mut self) -> MONO_W {
        MONO_W { w: self }
    }
    ///Bits 10:11 - Synchronization enable
    #[inline(always)]
    pub fn syncen(&mut self) -> SYNCEN_W {
        SYNCEN_W { w: self }
    }
    ///Bit 9 - Clock strobing edge
    #[inline(always)]
    pub fn ckstr(&mut self) -> CKSTR_W {
        CKSTR_W { w: self }
    }
    ///Bit 8 - Least significant bit first
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W {
        LSBFIRST_W { w: self }
    }
    ///Bits 5:7 - Data size
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
    ///Bits 2:3 - Protocol configuration
    #[inline(always)]
    pub fn prtcfg(&mut self) -> PRTCFG_W {
        PRTCFG_W { w: self }
    }
    ///Bits 0:1 - Audio block mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
