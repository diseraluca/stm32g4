///Reader of register BCR2
pub type R = crate::R<u32, super::BCR2>;
///Writer for register BCR2
pub type W = crate::W<u32, super::BCR2>;
///Register BCR2 `reset()`'s with value 0
impl crate::ResetValue for super::BCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `COMP`
pub type COMP_R = crate::R<u8, u8>;
///Write proxy for field `COMP`
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
///Reader of field `CPL`
pub type CPL_R = crate::R<bool, bool>;
///Write proxy for field `CPL`
pub struct CPL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPL_W<'a> {
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
///Reader of field `MUTECN`
pub type MUTECN_R = crate::R<u8, u8>;
///Write proxy for field `MUTECN`
pub struct MUTECN_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTECN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | (((value as u32) & 0x3f) << 7);
        self.w
    }
}
///Reader of field `MUTEVAL`
pub type MUTEVAL_R = crate::R<bool, bool>;
///Write proxy for field `MUTEVAL`
pub struct MUTEVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTEVAL_W<'a> {
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
///Reader of field `MUTE`
pub type MUTE_R = crate::R<bool, bool>;
///Write proxy for field `MUTE`
pub struct MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_W<'a> {
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
///Reader of field `TRIS`
pub type TRIS_R = crate::R<bool, bool>;
///Write proxy for field `TRIS`
pub struct TRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIS_W<'a> {
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
///Reader of field `FFLUS`
pub type FFLUS_R = crate::R<bool, bool>;
///Write proxy for field `FFLUS`
pub struct FFLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLUS_W<'a> {
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
///Reader of field `FTH`
pub type FTH_R = crate::R<u8, u8>;
///Write proxy for field `FTH`
pub struct FTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FTH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    ///Bits 14:15 - Companding mode
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    ///Bit 13 - Complement bit
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bits 7:12 - Mute counter
    #[inline(always)]
    pub fn mutecn(&self) -> MUTECN_R {
        MUTECN_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    ///Bit 6 - Mute value
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Mute
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Tristate management on data line
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - FIFO flush
    #[inline(always)]
    pub fn fflus(&self) -> FFLUS_R {
        FFLUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 0:2 - FIFO threshold
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 14:15 - Companding mode
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
    ///Bit 13 - Complement bit
    #[inline(always)]
    pub fn cpl(&mut self) -> CPL_W {
        CPL_W { w: self }
    }
    ///Bits 7:12 - Mute counter
    #[inline(always)]
    pub fn mutecn(&mut self) -> MUTECN_W {
        MUTECN_W { w: self }
    }
    ///Bit 6 - Mute value
    #[inline(always)]
    pub fn muteval(&mut self) -> MUTEVAL_W {
        MUTEVAL_W { w: self }
    }
    ///Bit 5 - Mute
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W {
        MUTE_W { w: self }
    }
    ///Bit 4 - Tristate management on data line
    #[inline(always)]
    pub fn tris(&mut self) -> TRIS_W {
        TRIS_W { w: self }
    }
    ///Bit 3 - FIFO flush
    #[inline(always)]
    pub fn fflus(&mut self) -> FFLUS_W {
        FFLUS_W { w: self }
    }
    ///Bits 0:2 - FIFO threshold
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W {
        FTH_W { w: self }
    }
}
