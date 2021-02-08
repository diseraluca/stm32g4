///Reader of register FLTINR4
pub type R = crate::R<u32, super::FLTINR4>;
///Writer for register FLTINR4
pub type W = crate::W<u32, super::FLTINR4>;
///Register FLTINR4 `reset()`'s with value 0
impl crate::ResetValue for super::FLTINR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `FLT6RSTM`
pub type FLT6RSTM_R = crate::R<bool, bool>;
///Write proxy for field `FLT6RSTM`
pub struct FLT6RSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6RSTM_W<'a> {
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
///Reader of field `FLT6CRES`
pub type FLT6CRES_R = crate::R<bool, bool>;
///Write proxy for field `FLT6CRES`
pub struct FLT6CRES_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6CRES_W<'a> {
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
///Reader of field `FLT6CNT`
pub type FLT6CNT_R = crate::R<u8, u8>;
///Write proxy for field `FLT6CNT`
pub struct FLT6CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6CNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
///Reader of field `FLT6BLKS`
pub type FLT6BLKS_R = crate::R<bool, bool>;
///Write proxy for field `FLT6BLKS`
pub struct FLT6BLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6BLKS_W<'a> {
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
///Reader of field `FLT6BLKE`
pub type FLT6BLKE_R = crate::R<bool, bool>;
///Write proxy for field `FLT6BLKE`
pub struct FLT6BLKE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6BLKE_W<'a> {
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
///Reader of field `FLT5RSTM`
pub type FLT5RSTM_R = crate::R<bool, bool>;
///Write proxy for field `FLT5RSTM`
pub struct FLT5RSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5RSTM_W<'a> {
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
///Reader of field `FLT5CRES`
pub type FLT5CRES_R = crate::R<bool, bool>;
///Write proxy for field `FLT5CRES`
pub struct FLT5CRES_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5CRES_W<'a> {
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
///Reader of field `FLT5CNT`
pub type FLT5CNT_R = crate::R<u8, u8>;
///Write proxy for field `FLT5CNT`
pub struct FLT5CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5CNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
///Reader of field `FLT5BLKS`
pub type FLT5BLKS_R = crate::R<bool, bool>;
///Write proxy for field `FLT5BLKS`
pub struct FLT5BLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5BLKS_W<'a> {
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
///Reader of field `FLT5BLKE`
pub type FLT5BLKE_R = crate::R<bool, bool>;
///Write proxy for field `FLT5BLKE`
pub struct FLT5BLKE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5BLKE_W<'a> {
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
impl R {
    ///Bit 15 - FLT6RSTM
    #[inline(always)]
    pub fn flt6rstm(&self) -> FLT6RSTM_R {
        FLT6RSTM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - FLT6CRES
    #[inline(always)]
    pub fn flt6cres(&self) -> FLT6CRES_R {
        FLT6CRES_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bits 10:13 - FLT6CNT
    #[inline(always)]
    pub fn flt6cnt(&self) -> FLT6CNT_R {
        FLT6CNT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 9 - FLT6BLKS
    #[inline(always)]
    pub fn flt6blks(&self) -> FLT6BLKS_R {
        FLT6BLKS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - FLT6BLKE
    #[inline(always)]
    pub fn flt6blke(&self) -> FLT6BLKE_R {
        FLT6BLKE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - FLT5RSTM
    #[inline(always)]
    pub fn flt5rstm(&self) -> FLT5RSTM_R {
        FLT5RSTM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - FLT5CRES
    #[inline(always)]
    pub fn flt5cres(&self) -> FLT5CRES_R {
        FLT5CRES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bits 2:5 - FLT5CNT
    #[inline(always)]
    pub fn flt5cnt(&self) -> FLT5CNT_R {
        FLT5CNT_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bit 1 - FLT5BLKS
    #[inline(always)]
    pub fn flt5blks(&self) -> FLT5BLKS_R {
        FLT5BLKS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - FLT5BLKE
    #[inline(always)]
    pub fn flt5blke(&self) -> FLT5BLKE_R {
        FLT5BLKE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 15 - FLT6RSTM
    #[inline(always)]
    pub fn flt6rstm(&mut self) -> FLT6RSTM_W {
        FLT6RSTM_W { w: self }
    }
    ///Bit 14 - FLT6CRES
    #[inline(always)]
    pub fn flt6cres(&mut self) -> FLT6CRES_W {
        FLT6CRES_W { w: self }
    }
    ///Bits 10:13 - FLT6CNT
    #[inline(always)]
    pub fn flt6cnt(&mut self) -> FLT6CNT_W {
        FLT6CNT_W { w: self }
    }
    ///Bit 9 - FLT6BLKS
    #[inline(always)]
    pub fn flt6blks(&mut self) -> FLT6BLKS_W {
        FLT6BLKS_W { w: self }
    }
    ///Bit 8 - FLT6BLKE
    #[inline(always)]
    pub fn flt6blke(&mut self) -> FLT6BLKE_W {
        FLT6BLKE_W { w: self }
    }
    ///Bit 7 - FLT5RSTM
    #[inline(always)]
    pub fn flt5rstm(&mut self) -> FLT5RSTM_W {
        FLT5RSTM_W { w: self }
    }
    ///Bit 6 - FLT5CRES
    #[inline(always)]
    pub fn flt5cres(&mut self) -> FLT5CRES_W {
        FLT5CRES_W { w: self }
    }
    ///Bits 2:5 - FLT5CNT
    #[inline(always)]
    pub fn flt5cnt(&mut self) -> FLT5CNT_W {
        FLT5CNT_W { w: self }
    }
    ///Bit 1 - FLT5BLKS
    #[inline(always)]
    pub fn flt5blks(&mut self) -> FLT5BLKS_W {
        FLT5BLKS_W { w: self }
    }
    ///Bit 0 - FLT5BLKE
    #[inline(always)]
    pub fn flt5blke(&mut self) -> FLT5BLKE_W {
        FLT5BLKE_W { w: self }
    }
}
