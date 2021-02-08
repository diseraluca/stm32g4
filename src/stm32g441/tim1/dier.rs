///Reader of register DIER
pub type R = crate::R<u32, super::DIER>;
///Writer for register DIER
pub type W = crate::W<u32, super::DIER>;
///Register DIER `reset()`'s with value 0
impl crate::ResetValue for super::DIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TERRIE`
pub type TERRIE_R = crate::R<bool, bool>;
///Write proxy for field `TERRIE`
pub struct TERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TERRIE_W<'a> {
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
///Reader of field `IERRIE`
pub type IERRIE_R = crate::R<bool, bool>;
///Write proxy for field `IERRIE`
pub struct IERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IERRIE_W<'a> {
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
///Reader of field `DIRIE`
pub type DIRIE_R = crate::R<bool, bool>;
///Write proxy for field `DIRIE`
pub struct DIRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
///Reader of field `IDXIE`
pub type IDXIE_R = crate::R<bool, bool>;
///Write proxy for field `IDXIE`
pub struct IDXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDXIE_W<'a> {
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
///Reader of field `TDE`
pub type TDE_R = crate::R<bool, bool>;
///Write proxy for field `TDE`
pub struct TDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDE_W<'a> {
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
///Reader of field `COMDE`
pub type COMDE_R = crate::R<bool, bool>;
///Write proxy for field `COMDE`
pub struct COMDE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMDE_W<'a> {
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
///Reader of field `CC4DE`
pub type CC4DE_R = crate::R<bool, bool>;
///Write proxy for field `CC4DE`
pub struct CC4DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4DE_W<'a> {
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
///Reader of field `CC3DE`
pub type CC3DE_R = crate::R<bool, bool>;
///Write proxy for field `CC3DE`
pub struct CC3DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3DE_W<'a> {
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
///Reader of field `CC2DE`
pub type CC2DE_R = crate::R<bool, bool>;
///Write proxy for field `CC2DE`
pub struct CC2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2DE_W<'a> {
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
///Reader of field `CC1DE`
pub type CC1DE_R = crate::R<bool, bool>;
///Write proxy for field `CC1DE`
pub struct CC1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1DE_W<'a> {
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
///Reader of field `UDE`
pub type UDE_R = crate::R<bool, bool>;
///Write proxy for field `UDE`
pub struct UDE_W<'a> {
    w: &'a mut W,
}
impl<'a> UDE_W<'a> {
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
///Reader of field `TIE`
pub type TIE_R = crate::R<bool, bool>;
///Write proxy for field `TIE`
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
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
///Reader of field `CC4IE`
pub type CC4IE_R = crate::R<bool, bool>;
///Write proxy for field `CC4IE`
pub struct CC4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4IE_W<'a> {
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
///Reader of field `CC3IE`
pub type CC3IE_R = crate::R<bool, bool>;
///Write proxy for field `CC3IE`
pub struct CC3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3IE_W<'a> {
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
///Reader of field `CC2IE`
pub type CC2IE_R = crate::R<bool, bool>;
///Write proxy for field `CC2IE`
pub struct CC2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2IE_W<'a> {
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
///Reader of field `CC1IE`
pub type CC1IE_R = crate::R<bool, bool>;
///Write proxy for field `CC1IE`
pub struct CC1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1IE_W<'a> {
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
///Reader of field `UIE`
pub type UIE_R = crate::R<bool, bool>;
///Write proxy for field `UIE`
pub struct UIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UIE_W<'a> {
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
///Reader of field `BIE`
pub type BIE_R = crate::R<bool, bool>;
///Write proxy for field `BIE`
pub struct BIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIE_W<'a> {
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
///Reader of field `COMIE`
pub type COMIE_R = crate::R<bool, bool>;
///Write proxy for field `COMIE`
pub struct COMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMIE_W<'a> {
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
impl R {
    ///Bit 23 - Transition Error interrupt enable
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 22 - Index Error interrupt enable
    #[inline(always)]
    pub fn ierrie(&self) -> IERRIE_R {
        IERRIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 21 - Direction Change interrupt enable
    #[inline(always)]
    pub fn dirie(&self) -> DIRIE_R {
        DIRIE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 20 - Index interrupt enable
    #[inline(always)]
    pub fn idxie(&self) -> IDXIE_R {
        IDXIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - COM DMA request enable
    #[inline(always)]
    pub fn comde(&self) -> COMDE_R {
        COMDE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Capture/Compare 4 DMA request enable
    #[inline(always)]
    pub fn cc4de(&self) -> CC4DE_R {
        CC4DE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Capture/Compare 3 DMA request enable
    #[inline(always)]
    pub fn cc3de(&self) -> CC3DE_R {
        CC3DE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Capture/Compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 4 - Capture/Compare 4 interrupt enable
    #[inline(always)]
    pub fn cc4ie(&self) -> CC4IE_R {
        CC4IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Capture/Compare 3 interrupt enable
    #[inline(always)]
    pub fn cc3ie(&self) -> CC3IE_R {
        CC3IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 7 - Break interrupt enable
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    ///Bit 23 - Transition Error interrupt enable
    #[inline(always)]
    pub fn terrie(&mut self) -> TERRIE_W {
        TERRIE_W { w: self }
    }
    ///Bit 22 - Index Error interrupt enable
    #[inline(always)]
    pub fn ierrie(&mut self) -> IERRIE_W {
        IERRIE_W { w: self }
    }
    ///Bit 21 - Direction Change interrupt enable
    #[inline(always)]
    pub fn dirie(&mut self) -> DIRIE_W {
        DIRIE_W { w: self }
    }
    ///Bit 20 - Index interrupt enable
    #[inline(always)]
    pub fn idxie(&mut self) -> IDXIE_W {
        IDXIE_W { w: self }
    }
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W {
        TDE_W { w: self }
    }
    ///Bit 13 - COM DMA request enable
    #[inline(always)]
    pub fn comde(&mut self) -> COMDE_W {
        COMDE_W { w: self }
    }
    ///Bit 12 - Capture/Compare 4 DMA request enable
    #[inline(always)]
    pub fn cc4de(&mut self) -> CC4DE_W {
        CC4DE_W { w: self }
    }
    ///Bit 11 - Capture/Compare 3 DMA request enable
    #[inline(always)]
    pub fn cc3de(&mut self) -> CC3DE_W {
        CC3DE_W { w: self }
    }
    ///Bit 10 - Capture/Compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&mut self) -> CC2DE_W {
        CC2DE_W { w: self }
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W {
        CC1DE_W { w: self }
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W {
        UDE_W { w: self }
    }
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    ///Bit 4 - Capture/Compare 4 interrupt enable
    #[inline(always)]
    pub fn cc4ie(&mut self) -> CC4IE_W {
        CC4IE_W { w: self }
    }
    ///Bit 3 - Capture/Compare 3 interrupt enable
    #[inline(always)]
    pub fn cc3ie(&mut self) -> CC3IE_W {
        CC3IE_W { w: self }
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W {
        CC2IE_W { w: self }
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W {
        CC1IE_W { w: self }
    }
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W {
        UIE_W { w: self }
    }
    ///Bit 7 - Break interrupt enable
    #[inline(always)]
    pub fn bie(&mut self) -> BIE_W {
        BIE_W { w: self }
    }
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&mut self) -> COMIE_W {
        COMIE_W { w: self }
    }
}
