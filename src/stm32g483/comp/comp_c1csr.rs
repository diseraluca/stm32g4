///Reader of register COMP_C1CSR
pub type R = crate::R<u32, super::COMP_C1CSR>;
///Writer for register COMP_C1CSR
pub type W = crate::W<u32, super::COMP_C1CSR>;
///Register COMP_C1CSR `reset()`'s with value 0
impl crate::ResetValue for super::COMP_C1CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `EN`
pub type EN_R = crate::R<bool, bool>;
///Write proxy for field `EN`
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
///Reader of field `COMP_DEGLITCH_EN`
pub type COMP_DEGLITCH_EN_R = crate::R<bool, bool>;
///Write proxy for field `COMP_DEGLITCH_EN`
pub struct COMP_DEGLITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_DEGLITCH_EN_W<'a> {
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
///Reader of field `INMSEL`
pub type INMSEL_R = crate::R<u8, u8>;
///Write proxy for field `INMSEL`
pub struct INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INMSEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
///Reader of field `INPSEL`
pub type INPSEL_R = crate::R<bool, bool>;
///Write proxy for field `INPSEL`
pub struct INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPSEL_W<'a> {
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
///Reader of field `POL`
pub type POL_R = crate::R<bool, bool>;
///Write proxy for field `POL`
pub struct POL_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_W<'a> {
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
///Reader of field `HYST`
pub type HYST_R = crate::R<u8, u8>;
///Write proxy for field `HYST`
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
///Reader of field `BLANKSEL`
pub type BLANKSEL_R = crate::R<u8, u8>;
///Write proxy for field `BLANKSEL`
pub struct BLANKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLANKSEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
///Reader of field `BRGEN`
pub type BRGEN_R = crate::R<bool, bool>;
///Write proxy for field `BRGEN`
pub struct BRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
///Reader of field `SCALEN`
pub type SCALEN_R = crate::R<bool, bool>;
///Write proxy for field `SCALEN`
pub struct SCALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
///Reader of field `VALUE`
pub type VALUE_R = crate::R<bool, bool>;
///Reader of field `LOCK`
pub type LOCK_R = crate::R<bool, bool>;
///Write proxy for field `LOCK`
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - COMP_DEGLITCH_EN
    #[inline(always)]
    pub fn comp_deglitch_en(&self) -> COMP_DEGLITCH_EN_R {
        COMP_DEGLITCH_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bits 4:6 - INMSEL
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bit 8 - INPSEL
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 15 - POL
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 16:18 - HYST
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    ///Bits 19:21 - BLANKSEL
    #[inline(always)]
    pub fn blanksel(&self) -> BLANKSEL_R {
        BLANKSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    ///Bit 22 - BRGEN
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - SCALEN
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 30 - VALUE
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    ///Bit 1 - COMP_DEGLITCH_EN
    #[inline(always)]
    pub fn comp_deglitch_en(&mut self) -> COMP_DEGLITCH_EN_W {
        COMP_DEGLITCH_EN_W { w: self }
    }
    ///Bits 4:6 - INMSEL
    #[inline(always)]
    pub fn inmsel(&mut self) -> INMSEL_W {
        INMSEL_W { w: self }
    }
    ///Bit 8 - INPSEL
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W {
        INPSEL_W { w: self }
    }
    ///Bit 15 - POL
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W { w: self }
    }
    ///Bits 16:18 - HYST
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
    ///Bits 19:21 - BLANKSEL
    #[inline(always)]
    pub fn blanksel(&mut self) -> BLANKSEL_W {
        BLANKSEL_W { w: self }
    }
    ///Bit 22 - BRGEN
    #[inline(always)]
    pub fn brgen(&mut self) -> BRGEN_W {
        BRGEN_W { w: self }
    }
    ///Bit 23 - SCALEN
    #[inline(always)]
    pub fn scalen(&mut self) -> SCALEN_W {
        SCALEN_W { w: self }
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
