///Reader of register TXBC
pub type R = crate::R<u32, super::TXBC>;
///Writer for register TXBC
pub type W = crate::W<u32, super::TXBC>;
///Register TXBC `reset()`'s with value 0
impl crate::ResetValue for super::TXBC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TBSA`
pub type TBSA_R = crate::R<u16, u16>;
///Write proxy for field `TBSA`
pub struct TBSA_W<'a> {
    w: &'a mut W,
}
impl<'a> TBSA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
///Reader of field `NDTB`
pub type NDTB_R = crate::R<u8, u8>;
///Write proxy for field `NDTB`
pub struct NDTB_W<'a> {
    w: &'a mut W,
}
impl<'a> NDTB_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
///Reader of field `TFQS`
pub type TFQS_R = crate::R<u8, u8>;
///Write proxy for field `TFQS`
pub struct TFQS_W<'a> {
    w: &'a mut W,
}
impl<'a> TFQS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
///Reader of field `TFQM`
pub type TFQM_R = crate::R<bool, bool>;
///Write proxy for field `TFQM`
pub struct TFQM_W<'a> {
    w: &'a mut W,
}
impl<'a> TFQM_W<'a> {
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
impl R {
    ///Bits 2:15 - TBSA
    #[inline(always)]
    pub fn tbsa(&self) -> TBSA_R {
        TBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:21 - NDTB
    #[inline(always)]
    pub fn ndtb(&self) -> NDTB_R {
        NDTB_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:29 - TFQS
    #[inline(always)]
    pub fn tfqs(&self) -> TFQS_R {
        TFQS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 30 - TFQM
    #[inline(always)]
    pub fn tfqm(&self) -> TFQM_R {
        TFQM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    ///Bits 2:15 - TBSA
    #[inline(always)]
    pub fn tbsa(&mut self) -> TBSA_W {
        TBSA_W { w: self }
    }
    ///Bits 16:21 - NDTB
    #[inline(always)]
    pub fn ndtb(&mut self) -> NDTB_W {
        NDTB_W { w: self }
    }
    ///Bits 24:29 - TFQS
    #[inline(always)]
    pub fn tfqs(&mut self) -> TFQS_W {
        TFQS_W { w: self }
    }
    ///Bit 30 - TFQM
    #[inline(always)]
    pub fn tfqm(&mut self) -> TFQM_W {
        TFQM_W { w: self }
    }
}
