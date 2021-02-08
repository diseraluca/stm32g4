///Reader of register AFRCR
pub type R = crate::R<u32, super::AFRCR>;
///Writer for register AFRCR
pub type W = crate::W<u32, super::AFRCR>;
///Register AFRCR `reset()`'s with value 0x07
impl crate::ResetValue for super::AFRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
///Reader of field `FSOFF`
pub type FSOFF_R = crate::R<bool, bool>;
///Write proxy for field `FSOFF`
pub struct FSOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FSOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
///Reader of field `FSPOL`
pub type FSPOL_R = crate::R<bool, bool>;
///Write proxy for field `FSPOL`
pub struct FSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSPOL_W<'a> {
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
///Reader of field `FSDEF`
pub type FSDEF_R = crate::R<bool, bool>;
///Write proxy for field `FSDEF`
pub struct FSDEF_W<'a> {
    w: &'a mut W,
}
impl<'a> FSDEF_W<'a> {
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
///Reader of field `FSALL`
pub type FSALL_R = crate::R<u8, u8>;
///Write proxy for field `FSALL`
pub struct FSALL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSALL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
///Reader of field `FRL`
pub type FRL_R = crate::R<u8, u8>;
///Write proxy for field `FRL`
pub struct FRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bit 18 - Frame synchronization offset
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - Frame synchronization polarity
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - Frame synchronization definition
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bits 8:14 - Frame synchronization active level length
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bits 0:7 - Frame length
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bit 18 - Frame synchronization offset
    #[inline(always)]
    pub fn fsoff(&mut self) -> FSOFF_W {
        FSOFF_W { w: self }
    }
    ///Bit 17 - Frame synchronization polarity
    #[inline(always)]
    pub fn fspol(&mut self) -> FSPOL_W {
        FSPOL_W { w: self }
    }
    ///Bit 16 - Frame synchronization definition
    #[inline(always)]
    pub fn fsdef(&mut self) -> FSDEF_W {
        FSDEF_W { w: self }
    }
    ///Bits 8:14 - Frame synchronization active level length
    #[inline(always)]
    pub fn fsall(&mut self) -> FSALL_W {
        FSALL_W { w: self }
    }
    ///Bits 0:7 - Frame length
    #[inline(always)]
    pub fn frl(&mut self) -> FRL_W {
        FRL_W { w: self }
    }
}
