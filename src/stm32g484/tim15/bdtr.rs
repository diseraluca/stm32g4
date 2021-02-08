///Reader of register BDTR
pub type R = crate::R<u32, super::BDTR>;
///Writer for register BDTR
pub type W = crate::W<u32, super::BDTR>;
///Register BDTR `reset()`'s with value 0
impl crate::ResetValue for super::BDTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DTG`
pub type DTG_R = crate::R<u8, u8>;
///Write proxy for field `DTG`
pub struct DTG_W<'a> {
    w: &'a mut W,
}
impl<'a> DTG_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
///Reader of field `LOCK`
pub type LOCK_R = crate::R<u8, u8>;
///Write proxy for field `LOCK`
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
///Reader of field `OSSI`
pub type OSSI_R = crate::R<bool, bool>;
///Write proxy for field `OSSI`
pub struct OSSI_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSI_W<'a> {
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
///Reader of field `OSSR`
pub type OSSR_R = crate::R<bool, bool>;
///Write proxy for field `OSSR`
pub struct OSSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSR_W<'a> {
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
///Reader of field `BKE`
pub type BKE_R = crate::R<bool, bool>;
///Write proxy for field `BKE`
pub struct BKE_W<'a> {
    w: &'a mut W,
}
impl<'a> BKE_W<'a> {
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
///Reader of field `BKP`
pub type BKP_R = crate::R<bool, bool>;
///Write proxy for field `BKP`
pub struct BKP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_W<'a> {
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
///Reader of field `AOE`
pub type AOE_R = crate::R<bool, bool>;
///Write proxy for field `AOE`
pub struct AOE_W<'a> {
    w: &'a mut W,
}
impl<'a> AOE_W<'a> {
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
///Reader of field `MOE`
pub type MOE_R = crate::R<bool, bool>;
///Write proxy for field `MOE`
pub struct MOE_W<'a> {
    w: &'a mut W,
}
impl<'a> MOE_W<'a> {
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
///Reader of field `BKF`
pub type BKF_R = crate::R<u8, u8>;
///Write proxy for field `BKF`
pub struct BKF_W<'a> {
    w: &'a mut W,
}
impl<'a> BKF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
///Reader of field `BKDSRM`
pub type BKDSRM_R = crate::R<bool, bool>;
///Write proxy for field `BKDSRM`
pub struct BKDSRM_W<'a> {
    w: &'a mut W,
}
impl<'a> BKDSRM_W<'a> {
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
///Reader of field `BKBID`
pub type BKBID_R = crate::R<bool, bool>;
///Write proxy for field `BKBID`
pub struct BKBID_W<'a> {
    w: &'a mut W,
}
impl<'a> BKBID_W<'a> {
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
impl R {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 16:19 - Break filter
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 26 - BKDSRM
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 28 - BKBID
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&mut self) -> DTG_W {
        DTG_W { w: self }
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&mut self) -> OSSI_W {
        OSSI_W { w: self }
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&mut self) -> OSSR_W {
        OSSR_W { w: self }
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&mut self) -> BKE_W {
        BKE_W { w: self }
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W {
        BKP_W { w: self }
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&mut self) -> AOE_W {
        AOE_W { w: self }
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W {
        MOE_W { w: self }
    }
    ///Bits 16:19 - Break filter
    #[inline(always)]
    pub fn bkf(&mut self) -> BKF_W {
        BKF_W { w: self }
    }
    ///Bit 26 - BKDSRM
    #[inline(always)]
    pub fn bkdsrm(&mut self) -> BKDSRM_W {
        BKDSRM_W { w: self }
    }
    ///Bit 28 - BKBID
    #[inline(always)]
    pub fn bkbid(&mut self) -> BKBID_W {
        BKBID_W { w: self }
    }
}
