///Reader of register DLLCR
pub type R = crate::R<u32, super::DLLCR>;
///Writer for register DLLCR
pub type W = crate::W<u32, super::DLLCR>;
///Register DLLCR `reset()`'s with value 0
impl crate::ResetValue for super::DLLCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CALRTE`
pub type CALRTE_R = crate::R<u8, u8>;
///Write proxy for field `CALRTE`
pub struct CALRTE_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRTE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
///Reader of field `CALEN`
pub type CALEN_R = crate::R<bool, bool>;
///Write proxy for field `CALEN`
pub struct CALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CALEN_W<'a> {
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
///Reader of field `CAL`
pub type CAL_R = crate::R<bool, bool>;
///Write proxy for field `CAL`
pub struct CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_W<'a> {
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
    ///Bits 2:3 - DLL Calibration rate
    #[inline(always)]
    pub fn calrte(&self) -> CALRTE_R {
        CALRTE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bit 1 - DLL Calibration Enable
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - DLL Calibration Start
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 2:3 - DLL Calibration rate
    #[inline(always)]
    pub fn calrte(&mut self) -> CALRTE_W {
        CALRTE_W { w: self }
    }
    ///Bit 1 - DLL Calibration Enable
    #[inline(always)]
    pub fn calen(&mut self) -> CALEN_W {
        CALEN_W { w: self }
    }
    ///Bit 0 - DLL Calibration Start
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W { w: self }
    }
}
