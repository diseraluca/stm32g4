///Reader of register I2SPR
pub type R = crate::R<u32, super::I2SPR>;
///Writer for register I2SPR
pub type W = crate::W<u32, super::I2SPR>;
///Register I2SPR `reset()`'s with value 0x02
impl crate::ResetValue for super::I2SPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
///Reader of field `I2SDIV`
pub type I2SDIV_R = crate::R<u8, u8>;
///Write proxy for field `I2SDIV`
pub struct I2SDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SDIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
///Reader of field `ODD`
pub type ODD_R = crate::R<bool, bool>;
///Write proxy for field `ODD`
pub struct ODD_W<'a> {
    w: &'a mut W,
}
impl<'a> ODD_W<'a> {
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
///Reader of field `MCKOE`
pub type MCKOE_R = crate::R<bool, bool>;
///Write proxy for field `MCKOE`
pub struct MCKOE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKOE_W<'a> {
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
impl R {
    ///Bits 0:7 - I2SDIV
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - ODD
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - MCKOE
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:7 - I2SDIV
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2SDIV_W {
        I2SDIV_W { w: self }
    }
    ///Bit 8 - ODD
    #[inline(always)]
    pub fn odd(&mut self) -> ODD_W {
        ODD_W { w: self }
    }
    ///Bit 9 - MCKOE
    #[inline(always)]
    pub fn mckoe(&mut self) -> MCKOE_W {
        MCKOE_W { w: self }
    }
}
