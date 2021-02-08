///Reader of register PSR
pub type R = crate::R<u32, super::PSR>;
///Writer for register PSR
pub type W = crate::W<u32, super::PSR>;
///Register PSR `reset()`'s with value 0x0707
impl crate::ResetValue for super::PSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0707
    }
}
///Reader of field `LEC`
pub type LEC_R = crate::R<u8, u8>;
///Write proxy for field `LEC`
pub struct LEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
///Write proxy for field `ACT`
pub struct ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
///Reader of field `EP`
pub type EP_R = crate::R<bool, bool>;
///Write proxy for field `EP`
pub struct EP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
///Reader of field `EW`
pub type EW_R = crate::R<bool, bool>;
///Write proxy for field `EW`
pub struct EW_W<'a> {
    w: &'a mut W,
}
impl<'a> EW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
///Reader of field `BO`
pub type BO_R = crate::R<bool, bool>;
///Write proxy for field `BO`
pub struct BO_W<'a> {
    w: &'a mut W,
}
impl<'a> BO_W<'a> {
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
///Write proxy for field `DLEC`
pub struct DLEC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLEC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
///Reader of field `RESI`
pub type RESI_R = crate::R<bool, bool>;
///Write proxy for field `RESI`
pub struct RESI_W<'a> {
    w: &'a mut W,
}
impl<'a> RESI_W<'a> {
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
///Reader of field `RBRS`
pub type RBRS_R = crate::R<bool, bool>;
///Write proxy for field `RBRS`
pub struct RBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RBRS_W<'a> {
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
///Reader of field `REDL`
pub type REDL_R = crate::R<bool, bool>;
///Write proxy for field `REDL`
pub struct REDL_W<'a> {
    w: &'a mut W,
}
impl<'a> REDL_W<'a> {
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
///Reader of field `PXE`
pub type PXE_R = crate::R<bool, bool>;
///Write proxy for field `PXE`
pub struct PXE_W<'a> {
    w: &'a mut W,
}
impl<'a> PXE_W<'a> {
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
///Reader of field `TDCV`
pub type TDCV_R = crate::R<u8, u8>;
///Write proxy for field `TDCV`
pub struct TDCV_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:2 - LEC
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    ///Bit 5 - EP
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - EW
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - BO
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 11 - RESI
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - RBRS
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - REDL
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - PXE
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bits 16:22 - TDCV
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:2 - LEC
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W {
        LEC_W { w: self }
    }
    ///Bits 3:4 - ACT
    #[inline(always)]
    pub fn act(&mut self) -> ACT_W {
        ACT_W { w: self }
    }
    ///Bit 5 - EP
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W {
        EP_W { w: self }
    }
    ///Bit 6 - EW
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W {
        EW_W { w: self }
    }
    ///Bit 7 - BO
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W {
        BO_W { w: self }
    }
    ///Bits 8:10 - DLEC
    #[inline(always)]
    pub fn dlec(&mut self) -> DLEC_W {
        DLEC_W { w: self }
    }
    ///Bit 11 - RESI
    #[inline(always)]
    pub fn resi(&mut self) -> RESI_W {
        RESI_W { w: self }
    }
    ///Bit 12 - RBRS
    #[inline(always)]
    pub fn rbrs(&mut self) -> RBRS_W {
        RBRS_W { w: self }
    }
    ///Bit 13 - REDL
    #[inline(always)]
    pub fn redl(&mut self) -> REDL_W {
        REDL_W { w: self }
    }
    ///Bit 14 - PXE
    #[inline(always)]
    pub fn pxe(&mut self) -> PXE_W {
        PXE_W { w: self }
    }
    ///Bits 16:22 - TDCV
    #[inline(always)]
    pub fn tdcv(&mut self) -> TDCV_W {
        TDCV_W { w: self }
    }
}
