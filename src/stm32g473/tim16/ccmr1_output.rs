///Reader of register CCMR1_Output
pub type R = crate::R<u32, super::CCMR1_OUTPUT>;
///Writer for register CCMR1_Output
pub type W = crate::W<u32, super::CCMR1_OUTPUT>;
///Register CCMR1_Output `reset()`'s with value 0
impl crate::ResetValue for super::CCMR1_OUTPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `OC1M_3`
pub type OC1M_3_R = crate::R<bool, bool>;
///Write proxy for field `OC1M_3`
pub struct OC1M_3_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1M_3_W<'a> {
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
///Reader of field `OC1M`
pub type OC1M_R = crate::R<u8, u8>;
///Write proxy for field `OC1M`
pub struct OC1M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1M_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
///Reader of field `OC1PE`
pub type OC1PE_R = crate::R<bool, bool>;
///Write proxy for field `OC1PE`
pub struct OC1PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1PE_W<'a> {
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
///Reader of field `OC1FE`
pub type OC1FE_R = crate::R<bool, bool>;
///Write proxy for field `OC1FE`
pub struct OC1FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1FE_W<'a> {
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
///Reader of field `CC1S`
pub type CC1S_R = crate::R<u8, u8>;
///Write proxy for field `CC1S`
pub struct CC1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1S_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    ///Bit 16 - Output Compare 1 mode
    #[inline(always)]
    pub fn oc1m_3(&self) -> OC1M_3_R {
        OC1M_3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bits 4:6 - Output Compare 1 mode
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bit 3 - Output Compare 1 preload enable
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Output Compare 1 fast enable
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bit 16 - Output Compare 1 mode
    #[inline(always)]
    pub fn oc1m_3(&mut self) -> OC1M_3_W {
        OC1M_3_W { w: self }
    }
    ///Bits 4:6 - Output Compare 1 mode
    #[inline(always)]
    pub fn oc1m(&mut self) -> OC1M_W {
        OC1M_W { w: self }
    }
    ///Bit 3 - Output Compare 1 preload enable
    #[inline(always)]
    pub fn oc1pe(&mut self) -> OC1PE_W {
        OC1PE_W { w: self }
    }
    ///Bit 2 - Output Compare 1 fast enable
    #[inline(always)]
    pub fn oc1fe(&mut self) -> OC1FE_W {
        OC1FE_W { w: self }
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W {
        CC1S_W { w: self }
    }
}
