///Reader of register IMR
pub type R = crate::R<u32, super::IMR>;
///Writer for register IMR
pub type W = crate::W<u32, super::IMR>;
///Register IMR `reset()`'s with value 0
impl crate::ResetValue for super::IMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TXISIE`
pub type TXISIE_R = crate::R<bool, bool>;
///Write proxy for field `TXISIE`
pub struct TXISIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXISIE_W<'a> {
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
///Reader of field `TXMSGDISCIE`
pub type TXMSGDISCIE_R = crate::R<bool, bool>;
///Write proxy for field `TXMSGDISCIE`
pub struct TXMSGDISCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGDISCIE_W<'a> {
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
///Reader of field `TXMSGSENTIE`
pub type TXMSGSENTIE_R = crate::R<bool, bool>;
///Write proxy for field `TXMSGSENTIE`
pub struct TXMSGSENTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGSENTIE_W<'a> {
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
///Reader of field `TXMSGABTIE`
pub type TXMSGABTIE_R = crate::R<bool, bool>;
///Write proxy for field `TXMSGABTIE`
pub struct TXMSGABTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGABTIE_W<'a> {
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
///Reader of field `HRSTDISCIE`
pub type HRSTDISCIE_R = crate::R<bool, bool>;
///Write proxy for field `HRSTDISCIE`
pub struct HRSTDISCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HRSTDISCIE_W<'a> {
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
///Reader of field `HRSTSENTIE`
pub type HRSTSENTIE_R = crate::R<bool, bool>;
///Write proxy for field `HRSTSENTIE`
pub struct HRSTSENTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HRSTSENTIE_W<'a> {
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
///Reader of field `TXUNDIE`
pub type TXUNDIE_R = crate::R<bool, bool>;
///Write proxy for field `TXUNDIE`
pub struct TXUNDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDIE_W<'a> {
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
///Reader of field `RXNEIE`
pub type RXNEIE_R = crate::R<bool, bool>;
///Write proxy for field `RXNEIE`
pub struct RXNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNEIE_W<'a> {
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
///Reader of field `RXORDDETIE`
pub type RXORDDETIE_R = crate::R<bool, bool>;
///Write proxy for field `RXORDDETIE`
pub struct RXORDDETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXORDDETIE_W<'a> {
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
///Reader of field `RXHRSTDETIE`
pub type RXHRSTDETIE_R = crate::R<bool, bool>;
///Write proxy for field `RXHRSTDETIE`
pub struct RXHRSTDETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXHRSTDETIE_W<'a> {
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
///Reader of field `RXOVRIE`
pub type RXOVRIE_R = crate::R<bool, bool>;
///Write proxy for field `RXOVRIE`
pub struct RXOVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVRIE_W<'a> {
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
///Reader of field `RXMSGENDIE`
pub type RXMSGENDIE_R = crate::R<bool, bool>;
///Write proxy for field `RXMSGENDIE`
pub struct RXMSGENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMSGENDIE_W<'a> {
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
///Reader of field `TYPECEVT1IE`
pub type TYPECEVT1IE_R = crate::R<bool, bool>;
///Write proxy for field `TYPECEVT1IE`
pub struct TYPECEVT1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPECEVT1IE_W<'a> {
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
///Reader of field `TYPECEVT2IE`
pub type TYPECEVT2IE_R = crate::R<bool, bool>;
///Write proxy for field `TYPECEVT2IE`
pub struct TYPECEVT2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPECEVT2IE_W<'a> {
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
///Reader of field `FRSEVTIE`
pub type FRSEVTIE_R = crate::R<bool, bool>;
///Write proxy for field `FRSEVTIE`
pub struct FRSEVTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSEVTIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    ///Bit 0 - TXISIE
    #[inline(always)]
    pub fn txisie(&self) -> TXISIE_R {
        TXISIE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TXMSGDISCIE
    #[inline(always)]
    pub fn txmsgdiscie(&self) -> TXMSGDISCIE_R {
        TXMSGDISCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - TXMSGSENTIE
    #[inline(always)]
    pub fn txmsgsentie(&self) -> TXMSGSENTIE_R {
        TXMSGSENTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - TXMSGABTIE
    #[inline(always)]
    pub fn txmsgabtie(&self) -> TXMSGABTIE_R {
        TXMSGABTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - HRSTDISCIE
    #[inline(always)]
    pub fn hrstdiscie(&self) -> HRSTDISCIE_R {
        HRSTDISCIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - HRSTSENTIE
    #[inline(always)]
    pub fn hrstsentie(&self) -> HRSTSENTIE_R {
        HRSTSENTIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - TXUNDIE
    #[inline(always)]
    pub fn txundie(&self) -> TXUNDIE_R {
        TXUNDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 8 - RXNEIE
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - RXORDDETIE
    #[inline(always)]
    pub fn rxorddetie(&self) -> RXORDDETIE_R {
        RXORDDETIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - RXHRSTDETIE
    #[inline(always)]
    pub fn rxhrstdetie(&self) -> RXHRSTDETIE_R {
        RXHRSTDETIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - RXOVRIE
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - RXMSGENDIE
    #[inline(always)]
    pub fn rxmsgendie(&self) -> RXMSGENDIE_R {
        RXMSGENDIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 14 - TYPECEVT1IE
    #[inline(always)]
    pub fn typecevt1ie(&self) -> TYPECEVT1IE_R {
        TYPECEVT1IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - TYPECEVT2IE
    #[inline(always)]
    pub fn typecevt2ie(&self) -> TYPECEVT2IE_R {
        TYPECEVT2IE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 20 - FRSEVTIE
    #[inline(always)]
    pub fn frsevtie(&self) -> FRSEVTIE_R {
        FRSEVTIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - TXISIE
    #[inline(always)]
    pub fn txisie(&mut self) -> TXISIE_W {
        TXISIE_W { w: self }
    }
    ///Bit 1 - TXMSGDISCIE
    #[inline(always)]
    pub fn txmsgdiscie(&mut self) -> TXMSGDISCIE_W {
        TXMSGDISCIE_W { w: self }
    }
    ///Bit 2 - TXMSGSENTIE
    #[inline(always)]
    pub fn txmsgsentie(&mut self) -> TXMSGSENTIE_W {
        TXMSGSENTIE_W { w: self }
    }
    ///Bit 3 - TXMSGABTIE
    #[inline(always)]
    pub fn txmsgabtie(&mut self) -> TXMSGABTIE_W {
        TXMSGABTIE_W { w: self }
    }
    ///Bit 4 - HRSTDISCIE
    #[inline(always)]
    pub fn hrstdiscie(&mut self) -> HRSTDISCIE_W {
        HRSTDISCIE_W { w: self }
    }
    ///Bit 5 - HRSTSENTIE
    #[inline(always)]
    pub fn hrstsentie(&mut self) -> HRSTSENTIE_W {
        HRSTSENTIE_W { w: self }
    }
    ///Bit 6 - TXUNDIE
    #[inline(always)]
    pub fn txundie(&mut self) -> TXUNDIE_W {
        TXUNDIE_W { w: self }
    }
    ///Bit 8 - RXNEIE
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W {
        RXNEIE_W { w: self }
    }
    ///Bit 9 - RXORDDETIE
    #[inline(always)]
    pub fn rxorddetie(&mut self) -> RXORDDETIE_W {
        RXORDDETIE_W { w: self }
    }
    ///Bit 10 - RXHRSTDETIE
    #[inline(always)]
    pub fn rxhrstdetie(&mut self) -> RXHRSTDETIE_W {
        RXHRSTDETIE_W { w: self }
    }
    ///Bit 11 - RXOVRIE
    #[inline(always)]
    pub fn rxovrie(&mut self) -> RXOVRIE_W {
        RXOVRIE_W { w: self }
    }
    ///Bit 12 - RXMSGENDIE
    #[inline(always)]
    pub fn rxmsgendie(&mut self) -> RXMSGENDIE_W {
        RXMSGENDIE_W { w: self }
    }
    ///Bit 14 - TYPECEVT1IE
    #[inline(always)]
    pub fn typecevt1ie(&mut self) -> TYPECEVT1IE_W {
        TYPECEVT1IE_W { w: self }
    }
    ///Bit 15 - TYPECEVT2IE
    #[inline(always)]
    pub fn typecevt2ie(&mut self) -> TYPECEVT2IE_W {
        TYPECEVT2IE_W { w: self }
    }
    ///Bit 20 - FRSEVTIE
    #[inline(always)]
    pub fn frsevtie(&mut self) -> FRSEVTIE_W {
        FRSEVTIE_W { w: self }
    }
}
