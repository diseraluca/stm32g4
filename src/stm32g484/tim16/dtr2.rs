///Reader of register DTR2
pub type R = crate::R<u32, super::DTR2>;
///Writer for register DTR2
pub type W = crate::W<u32, super::DTR2>;
///Register DTR2 `reset()`'s with value 0
impl crate::ResetValue for super::DTR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DTGF`
pub type DTGF_R = crate::R<u8, u8>;
///Write proxy for field `DTGF`
pub struct DTGF_W<'a> {
    w: &'a mut W,
}
impl<'a> DTGF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
///Reader of field `DTAE`
pub type DTAE_R = crate::R<bool, bool>;
///Write proxy for field `DTAE`
pub struct DTAE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTAE_W<'a> {
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
///Reader of field `DTPE`
pub type DTPE_R = crate::R<bool, bool>;
///Write proxy for field `DTPE`
pub struct DTPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPE_W<'a> {
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
impl R {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtgf(&self) -> DTGF_R {
        DTGF_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 16 - Deadtime Asymmetric Enable
    #[inline(always)]
    pub fn dtae(&self) -> DTAE_R {
        DTAE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Deadtime Preload Enable
    #[inline(always)]
    pub fn dtpe(&self) -> DTPE_R {
        DTPE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtgf(&mut self) -> DTGF_W {
        DTGF_W { w: self }
    }
    ///Bit 16 - Deadtime Asymmetric Enable
    #[inline(always)]
    pub fn dtae(&mut self) -> DTAE_W {
        DTAE_W { w: self }
    }
    ///Bit 17 - Deadtime Preload Enable
    #[inline(always)]
    pub fn dtpe(&mut self) -> DTPE_W {
        DTPE_W { w: self }
    }
}
