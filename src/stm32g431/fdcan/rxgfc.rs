///Reader of register RXGFC
pub type R = crate::R<u32, super::RXGFC>;
///Writer for register RXGFC
pub type W = crate::W<u32, super::RXGFC>;
///Register RXGFC `reset()`'s with value 0
impl crate::ResetValue for super::RXGFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `RRFE`
pub type RRFE_R = crate::R<bool, bool>;
///Write proxy for field `RRFE`
pub struct RRFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RRFE_W<'a> {
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
///Reader of field `RRFS`
pub type RRFS_R = crate::R<bool, bool>;
///Write proxy for field `RRFS`
pub struct RRFS_W<'a> {
    w: &'a mut W,
}
impl<'a> RRFS_W<'a> {
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
///Write proxy for field `ANFE`
pub struct ANFE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANFE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
///Write proxy for field `ANFS`
pub struct ANFS_W<'a> {
    w: &'a mut W,
}
impl<'a> ANFS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    ///Bit 0 - RRFE
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - RRFS
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - RRFE
    #[inline(always)]
    pub fn rrfe(&mut self) -> RRFE_W {
        RRFE_W { w: self }
    }
    ///Bit 1 - RRFS
    #[inline(always)]
    pub fn rrfs(&mut self) -> RRFS_W {
        RRFS_W { w: self }
    }
    ///Bits 2:3 - ANFE
    #[inline(always)]
    pub fn anfe(&mut self) -> ANFE_W {
        ANFE_W { w: self }
    }
    ///Bits 4:5 - ANFS
    #[inline(always)]
    pub fn anfs(&mut self) -> ANFS_W {
        ANFS_W { w: self }
    }
}
