///Reader of register FLTCR
pub type R = crate::R<u32, super::FLTCR>;
///Writer for register FLTCR
pub type W = crate::W<u32, super::FLTCR>;
///Register FLTCR `reset()`'s with value 0
impl crate::ResetValue for super::FLTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TAMPFREQ`
pub type TAMPFREQ_R = crate::R<u8, u8>;
///Write proxy for field `TAMPFREQ`
pub struct TAMPFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPFREQ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
///Reader of field `TAMPFLT`
pub type TAMPFLT_R = crate::R<u8, u8>;
///Write proxy for field `TAMPFLT`
pub struct TAMPFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPFLT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
///Reader of field `TAMPPRCH`
pub type TAMPPRCH_R = crate::R<u8, u8>;
///Write proxy for field `TAMPPRCH`
pub struct TAMPPRCH_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPPRCH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
///Reader of field `TAMPPUDIS`
pub type TAMPPUDIS_R = crate::R<bool, bool>;
///Write proxy for field `TAMPPUDIS`
pub struct TAMPPUDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPPUDIS_W<'a> {
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
impl R {
    ///Bits 0:2 - TAMPFREQ
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 3:4 - TAMPFLT
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    ///Bits 5:6 - TAMPPRCH
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    ///Bit 7 - TAMPPUDIS
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:2 - TAMPFREQ
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W {
        TAMPFREQ_W { w: self }
    }
    ///Bits 3:4 - TAMPFLT
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W {
        TAMPFLT_W { w: self }
    }
    ///Bits 5:6 - TAMPPRCH
    #[inline(always)]
    pub fn tampprch(&mut self) -> TAMPPRCH_W {
        TAMPPRCH_W { w: self }
    }
    ///Bit 7 - TAMPPUDIS
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W {
        TAMPPUDIS_W { w: self }
    }
}
