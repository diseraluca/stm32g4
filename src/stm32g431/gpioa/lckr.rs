///Reader of register LCKR
pub type R = crate::R<u32, super::LCKR>;
///Writer for register LCKR
pub type W = crate::W<u32, super::LCKR>;
///Register LCKR `reset()`'s with value 0
impl crate::ResetValue for super::LCKR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `LCKK`
pub type LCKK_R = crate::R<bool, bool>;
///Write proxy for field `LCKK`
pub struct LCKK_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKK_W<'a> {
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
///Reader of field `LCK15`
pub type LCK15_R = crate::R<bool, bool>;
///Write proxy for field `LCK15`
pub struct LCK15_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK15_W<'a> {
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
///Reader of field `LCK14`
pub type LCK14_R = crate::R<bool, bool>;
///Write proxy for field `LCK14`
pub struct LCK14_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK14_W<'a> {
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
///Reader of field `LCK13`
pub type LCK13_R = crate::R<bool, bool>;
///Write proxy for field `LCK13`
pub struct LCK13_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK13_W<'a> {
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
///Reader of field `LCK12`
pub type LCK12_R = crate::R<bool, bool>;
///Write proxy for field `LCK12`
pub struct LCK12_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK12_W<'a> {
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
///Reader of field `LCK11`
pub type LCK11_R = crate::R<bool, bool>;
///Write proxy for field `LCK11`
pub struct LCK11_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK11_W<'a> {
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
///Reader of field `LCK10`
pub type LCK10_R = crate::R<bool, bool>;
///Write proxy for field `LCK10`
pub struct LCK10_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK10_W<'a> {
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
///Reader of field `LCK9`
pub type LCK9_R = crate::R<bool, bool>;
///Write proxy for field `LCK9`
pub struct LCK9_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK9_W<'a> {
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
///Reader of field `LCK8`
pub type LCK8_R = crate::R<bool, bool>;
///Write proxy for field `LCK8`
pub struct LCK8_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK8_W<'a> {
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
///Reader of field `LCK7`
pub type LCK7_R = crate::R<bool, bool>;
///Write proxy for field `LCK7`
pub struct LCK7_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK7_W<'a> {
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
///Reader of field `LCK6`
pub type LCK6_R = crate::R<bool, bool>;
///Write proxy for field `LCK6`
pub struct LCK6_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK6_W<'a> {
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
///Reader of field `LCK5`
pub type LCK5_R = crate::R<bool, bool>;
///Write proxy for field `LCK5`
pub struct LCK5_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK5_W<'a> {
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
///Reader of field `LCK4`
pub type LCK4_R = crate::R<bool, bool>;
///Write proxy for field `LCK4`
pub struct LCK4_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK4_W<'a> {
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
///Reader of field `LCK3`
pub type LCK3_R = crate::R<bool, bool>;
///Write proxy for field `LCK3`
pub struct LCK3_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK3_W<'a> {
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
///Reader of field `LCK2`
pub type LCK2_R = crate::R<bool, bool>;
///Write proxy for field `LCK2`
pub struct LCK2_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK2_W<'a> {
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
///Reader of field `LCK1`
pub type LCK1_R = crate::R<bool, bool>;
///Write proxy for field `LCK1`
pub struct LCK1_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK1_W<'a> {
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
///Reader of field `LCK0`
pub type LCK0_R = crate::R<bool, bool>;
///Write proxy for field `LCK0`
pub struct LCK0_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK0_W<'a> {
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
    ///Bit 16 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck15(&self) -> LCK15_R {
        LCK15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck14(&self) -> LCK14_R {
        LCK14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck13(&self) -> LCK13_R {
        LCK13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck12(&self) -> LCK12_R {
        LCK12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck11(&self) -> LCK11_R {
        LCK11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck10(&self) -> LCK10_R {
        LCK10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck9(&self) -> LCK9_R {
        LCK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck8(&self) -> LCK8_R {
        LCK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck7(&self) -> LCK7_R {
        LCK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck6(&self) -> LCK6_R {
        LCK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck5(&self) -> LCK5_R {
        LCK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck4(&self) -> LCK4_R {
        LCK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck2(&self) -> LCK2_R {
        LCK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 16 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lckk(&mut self) -> LCKK_W {
        LCKK_W { w: self }
    }
    ///Bit 15 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck15(&mut self) -> LCK15_W {
        LCK15_W { w: self }
    }
    ///Bit 14 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck14(&mut self) -> LCK14_W {
        LCK14_W { w: self }
    }
    ///Bit 13 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck13(&mut self) -> LCK13_W {
        LCK13_W { w: self }
    }
    ///Bit 12 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck12(&mut self) -> LCK12_W {
        LCK12_W { w: self }
    }
    ///Bit 11 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck11(&mut self) -> LCK11_W {
        LCK11_W { w: self }
    }
    ///Bit 10 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck10(&mut self) -> LCK10_W {
        LCK10_W { w: self }
    }
    ///Bit 9 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck9(&mut self) -> LCK9_W {
        LCK9_W { w: self }
    }
    ///Bit 8 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck8(&mut self) -> LCK8_W {
        LCK8_W { w: self }
    }
    ///Bit 7 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck7(&mut self) -> LCK7_W {
        LCK7_W { w: self }
    }
    ///Bit 6 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck6(&mut self) -> LCK6_W {
        LCK6_W { w: self }
    }
    ///Bit 5 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck5(&mut self) -> LCK5_W {
        LCK5_W { w: self }
    }
    ///Bit 4 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck4(&mut self) -> LCK4_W {
        LCK4_W { w: self }
    }
    ///Bit 3 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck3(&mut self) -> LCK3_W {
        LCK3_W { w: self }
    }
    ///Bit 2 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck2(&mut self) -> LCK2_W {
        LCK2_W { w: self }
    }
    ///Bit 1 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck1(&mut self) -> LCK1_W {
        LCK1_W { w: self }
    }
    ///Bit 0 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck0(&mut self) -> LCK0_W {
        LCK0_W { w: self }
    }
}
