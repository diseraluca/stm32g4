///Reader of register CCER
pub type R = crate::R<u32, super::CCER>;
///Writer for register CCER
pub type W = crate::W<u32, super::CCER>;
///Register CCER `reset()`'s with value 0
impl crate::ResetValue for super::CCER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CC6P`
pub type CC6P_R = crate::R<bool, bool>;
///Write proxy for field `CC6P`
pub struct CC6P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC6P_W<'a> {
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
///Reader of field `CC6E`
pub type CC6E_R = crate::R<bool, bool>;
///Write proxy for field `CC6E`
pub struct CC6E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC6E_W<'a> {
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
///Reader of field `CC5P`
pub type CC5P_R = crate::R<bool, bool>;
///Write proxy for field `CC5P`
pub struct CC5P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC5P_W<'a> {
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
///Reader of field `CC5E`
pub type CC5E_R = crate::R<bool, bool>;
///Write proxy for field `CC5E`
pub struct CC5E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC5E_W<'a> {
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
///Reader of field `CC4NP`
pub type CC4NP_R = crate::R<bool, bool>;
///Write proxy for field `CC4NP`
pub struct CC4NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4NP_W<'a> {
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
///Reader of field `CC4NE`
pub type CC4NE_R = crate::R<bool, bool>;
///Write proxy for field `CC4NE`
pub struct CC4NE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4NE_W<'a> {
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
///Reader of field `CC4P`
pub type CC4P_R = crate::R<bool, bool>;
///Write proxy for field `CC4P`
pub struct CC4P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4P_W<'a> {
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
///Reader of field `CC4E`
pub type CC4E_R = crate::R<bool, bool>;
///Write proxy for field `CC4E`
pub struct CC4E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4E_W<'a> {
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
///Reader of field `CC3NP`
pub type CC3NP_R = crate::R<bool, bool>;
///Write proxy for field `CC3NP`
pub struct CC3NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3NP_W<'a> {
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
///Reader of field `CC3NE`
pub type CC3NE_R = crate::R<bool, bool>;
///Write proxy for field `CC3NE`
pub struct CC3NE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3NE_W<'a> {
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
///Reader of field `CC3P`
pub type CC3P_R = crate::R<bool, bool>;
///Write proxy for field `CC3P`
pub struct CC3P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3P_W<'a> {
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
///Reader of field `CC3E`
pub type CC3E_R = crate::R<bool, bool>;
///Write proxy for field `CC3E`
pub struct CC3E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3E_W<'a> {
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
///Reader of field `CC2NP`
pub type CC2NP_R = crate::R<bool, bool>;
///Write proxy for field `CC2NP`
pub struct CC2NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2NP_W<'a> {
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
///Reader of field `CC2NE`
pub type CC2NE_R = crate::R<bool, bool>;
///Write proxy for field `CC2NE`
pub struct CC2NE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2NE_W<'a> {
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
///Reader of field `CC2P`
pub type CC2P_R = crate::R<bool, bool>;
///Write proxy for field `CC2P`
pub struct CC2P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2P_W<'a> {
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
///Reader of field `CC2E`
pub type CC2E_R = crate::R<bool, bool>;
///Write proxy for field `CC2E`
pub struct CC2E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2E_W<'a> {
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
///Reader of field `CC1NP`
pub type CC1NP_R = crate::R<bool, bool>;
///Write proxy for field `CC1NP`
pub struct CC1NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1NP_W<'a> {
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
///Reader of field `CC1NE`
pub type CC1NE_R = crate::R<bool, bool>;
///Write proxy for field `CC1NE`
pub struct CC1NE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1NE_W<'a> {
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
///Reader of field `CC1P`
pub type CC1P_R = crate::R<bool, bool>;
///Write proxy for field `CC1P`
pub struct CC1P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1P_W<'a> {
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
///Reader of field `CC1E`
pub type CC1E_R = crate::R<bool, bool>;
///Write proxy for field `CC1E`
pub struct CC1E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1E_W<'a> {
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
    ///Bit 21 - Capture/Compare 6 output polarity
    #[inline(always)]
    pub fn cc6p(&self) -> CC6P_R {
        CC6P_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 20 - Capture/Compare 6 output enable
    #[inline(always)]
    pub fn cc6e(&self) -> CC6E_R {
        CC6E_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 17 - Capture/Compare 5 output polarity
    #[inline(always)]
    pub fn cc5p(&self) -> CC5P_R {
        CC5P_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - Capture/Compare 5 output enable
    #[inline(always)]
    pub fn cc5e(&self) -> CC5E_R {
        CC5E_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - Capture/Compare 4 complementary output polarity
    #[inline(always)]
    pub fn cc4np(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Capture/Compare 4 complementary output enable
    #[inline(always)]
    pub fn cc4ne(&self) -> CC4NE_R {
        CC4NE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Capture/Compare 4 output enable
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Capture/Compare 3 complementary output enable
    #[inline(always)]
    pub fn cc3ne(&self) -> CC3NE_R {
        CC3NE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Capture/Compare 3 output enable
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Capture/Compare 2 complementary output enable
    #[inline(always)]
    pub fn cc2ne(&self) -> CC2NE_R {
        CC2NE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Capture/Compare 2 output enable
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Capture/Compare 1 complementary output enable
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 21 - Capture/Compare 6 output polarity
    #[inline(always)]
    pub fn cc6p(&mut self) -> CC6P_W {
        CC6P_W { w: self }
    }
    ///Bit 20 - Capture/Compare 6 output enable
    #[inline(always)]
    pub fn cc6e(&mut self) -> CC6E_W {
        CC6E_W { w: self }
    }
    ///Bit 17 - Capture/Compare 5 output polarity
    #[inline(always)]
    pub fn cc5p(&mut self) -> CC5P_W {
        CC5P_W { w: self }
    }
    ///Bit 16 - Capture/Compare 5 output enable
    #[inline(always)]
    pub fn cc5e(&mut self) -> CC5E_W {
        CC5E_W { w: self }
    }
    ///Bit 15 - Capture/Compare 4 complementary output polarity
    #[inline(always)]
    pub fn cc4np(&mut self) -> CC4NP_W {
        CC4NP_W { w: self }
    }
    ///Bit 14 - Capture/Compare 4 complementary output enable
    #[inline(always)]
    pub fn cc4ne(&mut self) -> CC4NE_W {
        CC4NE_W { w: self }
    }
    ///Bit 13 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc4p(&mut self) -> CC4P_W {
        CC4P_W { w: self }
    }
    ///Bit 12 - Capture/Compare 4 output enable
    #[inline(always)]
    pub fn cc4e(&mut self) -> CC4E_W {
        CC4E_W { w: self }
    }
    ///Bit 11 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc3np(&mut self) -> CC3NP_W {
        CC3NP_W { w: self }
    }
    ///Bit 10 - Capture/Compare 3 complementary output enable
    #[inline(always)]
    pub fn cc3ne(&mut self) -> CC3NE_W {
        CC3NE_W { w: self }
    }
    ///Bit 9 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc3p(&mut self) -> CC3P_W {
        CC3P_W { w: self }
    }
    ///Bit 8 - Capture/Compare 3 output enable
    #[inline(always)]
    pub fn cc3e(&mut self) -> CC3E_W {
        CC3E_W { w: self }
    }
    ///Bit 7 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2np(&mut self) -> CC2NP_W {
        CC2NP_W { w: self }
    }
    ///Bit 6 - Capture/Compare 2 complementary output enable
    #[inline(always)]
    pub fn cc2ne(&mut self) -> CC2NE_W {
        CC2NE_W { w: self }
    }
    ///Bit 5 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W {
        CC2P_W { w: self }
    }
    ///Bit 4 - Capture/Compare 2 output enable
    #[inline(always)]
    pub fn cc2e(&mut self) -> CC2E_W {
        CC2E_W { w: self }
    }
    ///Bit 3 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W {
        CC1NP_W { w: self }
    }
    ///Bit 2 - Capture/Compare 1 complementary output enable
    #[inline(always)]
    pub fn cc1ne(&mut self) -> CC1NE_W {
        CC1NE_W { w: self }
    }
    ///Bit 1 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W {
        CC1P_W { w: self }
    }
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W {
        CC1E_W { w: self }
    }
}
