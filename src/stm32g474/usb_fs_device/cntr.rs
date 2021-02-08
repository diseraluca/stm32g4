///Reader of register CNTR
pub type R = crate::R<u32, super::CNTR>;
///Writer for register CNTR
pub type W = crate::W<u32, super::CNTR>;
///Register CNTR `reset()`'s with value 0
impl crate::ResetValue for super::CNTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `FRES`
pub type FRES_R = crate::R<bool, bool>;
///Write proxy for field `FRES`
pub struct FRES_W<'a> {
    w: &'a mut W,
}
impl<'a> FRES_W<'a> {
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
///Reader of field `PDWN`
pub type PDWN_R = crate::R<bool, bool>;
///Write proxy for field `PDWN`
pub struct PDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDWN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///Reader of field `LP_MODE`
pub type LP_MODE_R = crate::R<bool, bool>;
///Write proxy for field `LP_MODE`
pub struct LP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///Reader of field `FSUSP`
pub type FSUSP_R = crate::R<bool, bool>;
///Write proxy for field `FSUSP`
pub struct FSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSUSP_W<'a> {
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
///Reader of field `RESUME`
pub type RESUME_R = crate::R<bool, bool>;
///Write proxy for field `RESUME`
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
///Reader of field `L1RESUME`
pub type L1RESUME_R = crate::R<bool, bool>;
///Write proxy for field `L1RESUME`
pub struct L1RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> L1RESUME_W<'a> {
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
///Reader of field `L1REQM`
pub type L1REQM_R = crate::R<bool, bool>;
///Write proxy for field `L1REQM`
pub struct L1REQM_W<'a> {
    w: &'a mut W,
}
impl<'a> L1REQM_W<'a> {
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
///Reader of field `ESOFM`
pub type ESOFM_R = crate::R<bool, bool>;
///Write proxy for field `ESOFM`
pub struct ESOFM_W<'a> {
    w: &'a mut W,
}
impl<'a> ESOFM_W<'a> {
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
///Reader of field `SOFM`
pub type SOFM_R = crate::R<bool, bool>;
///Write proxy for field `SOFM`
pub struct SOFM_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFM_W<'a> {
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
///Reader of field `RESETM`
pub type RESETM_R = crate::R<bool, bool>;
///Write proxy for field `RESETM`
pub struct RESETM_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETM_W<'a> {
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
///Reader of field `SUSPM`
pub type SUSPM_R = crate::R<bool, bool>;
///Write proxy for field `SUSPM`
pub struct SUSPM_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPM_W<'a> {
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
///Reader of field `WKUPM`
pub type WKUPM_R = crate::R<bool, bool>;
///Write proxy for field `WKUPM`
pub struct WKUPM_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPM_W<'a> {
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
///Reader of field `ERRM`
pub type ERRM_R = crate::R<bool, bool>;
///Write proxy for field `ERRM`
pub struct ERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRM_W<'a> {
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
///Reader of field `PMAOVRM`
pub type PMAOVRM_R = crate::R<bool, bool>;
///Write proxy for field `PMAOVRM`
pub struct PMAOVRM_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAOVRM_W<'a> {
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
///Reader of field `CTRM`
pub type CTRM_R = crate::R<bool, bool>;
///Write proxy for field `CTRM`
pub struct CTRM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRM_W<'a> {
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
impl R {
    ///Bit 0 - FRES
    #[inline(always)]
    pub fn fres(&self) -> FRES_R {
        FRES_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - PDWN
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - LP_MODE
    #[inline(always)]
    pub fn lp_mode(&self) -> LP_MODE_R {
        LP_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - FSUSP
    #[inline(always)]
    pub fn fsusp(&self) -> FSUSP_R {
        FSUSP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - RESUME
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - L1RESUME
    #[inline(always)]
    pub fn l1resume(&self) -> L1RESUME_R {
        L1RESUME_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 7 - L1REQM
    #[inline(always)]
    pub fn l1reqm(&self) -> L1REQM_R {
        L1REQM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - ESOFM
    #[inline(always)]
    pub fn esofm(&self) -> ESOFM_R {
        ESOFM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - SOFM
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - RESETM
    #[inline(always)]
    pub fn resetm(&self) -> RESETM_R {
        RESETM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - SUSPM
    #[inline(always)]
    pub fn suspm(&self) -> SUSPM_R {
        SUSPM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - WKUPM
    #[inline(always)]
    pub fn wkupm(&self) -> WKUPM_R {
        WKUPM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - ERRM
    #[inline(always)]
    pub fn errm(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - PMAOVRM
    #[inline(always)]
    pub fn pmaovrm(&self) -> PMAOVRM_R {
        PMAOVRM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - CTRM
    #[inline(always)]
    pub fn ctrm(&self) -> CTRM_R {
        CTRM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - FRES
    #[inline(always)]
    pub fn fres(&mut self) -> FRES_W {
        FRES_W { w: self }
    }
    ///Bit 1 - PDWN
    #[inline(always)]
    pub fn pdwn(&mut self) -> PDWN_W {
        PDWN_W { w: self }
    }
    ///Bit 2 - LP_MODE
    #[inline(always)]
    pub fn lp_mode(&mut self) -> LP_MODE_W {
        LP_MODE_W { w: self }
    }
    ///Bit 3 - FSUSP
    #[inline(always)]
    pub fn fsusp(&mut self) -> FSUSP_W {
        FSUSP_W { w: self }
    }
    ///Bit 4 - RESUME
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    ///Bit 5 - L1RESUME
    #[inline(always)]
    pub fn l1resume(&mut self) -> L1RESUME_W {
        L1RESUME_W { w: self }
    }
    ///Bit 7 - L1REQM
    #[inline(always)]
    pub fn l1reqm(&mut self) -> L1REQM_W {
        L1REQM_W { w: self }
    }
    ///Bit 8 - ESOFM
    #[inline(always)]
    pub fn esofm(&mut self) -> ESOFM_W {
        ESOFM_W { w: self }
    }
    ///Bit 9 - SOFM
    #[inline(always)]
    pub fn sofm(&mut self) -> SOFM_W {
        SOFM_W { w: self }
    }
    ///Bit 10 - RESETM
    #[inline(always)]
    pub fn resetm(&mut self) -> RESETM_W {
        RESETM_W { w: self }
    }
    ///Bit 11 - SUSPM
    #[inline(always)]
    pub fn suspm(&mut self) -> SUSPM_W {
        SUSPM_W { w: self }
    }
    ///Bit 12 - WKUPM
    #[inline(always)]
    pub fn wkupm(&mut self) -> WKUPM_W {
        WKUPM_W { w: self }
    }
    ///Bit 13 - ERRM
    #[inline(always)]
    pub fn errm(&mut self) -> ERRM_W {
        ERRM_W { w: self }
    }
    ///Bit 14 - PMAOVRM
    #[inline(always)]
    pub fn pmaovrm(&mut self) -> PMAOVRM_W {
        PMAOVRM_W { w: self }
    }
    ///Bit 15 - CTRM
    #[inline(always)]
    pub fn ctrm(&mut self) -> CTRM_W {
        CTRM_W { w: self }
    }
}
