///Reader of register ASR
pub type R = crate::R<u32, super::ASR>;
///Writer for register ASR
pub type W = crate::W<u32, super::ASR>;
///Register ASR `reset()`'s with value 0
impl crate::ResetValue for super::ASR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `FLVL`
pub type FLVL_R = crate::R<u8, u8>;
///Write proxy for field `FLVL`
pub struct FLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLVL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
///Reader of field `LFSDET`
pub type LFSDET_R = crate::R<bool, bool>;
///Write proxy for field `LFSDET`
pub struct LFSDET_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSDET_W<'a> {
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
///Reader of field `AFSDET`
pub type AFSDET_R = crate::R<bool, bool>;
///Write proxy for field `AFSDET`
pub struct AFSDET_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSDET_W<'a> {
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
///Reader of field `CNRDY`
pub type CNRDY_R = crate::R<bool, bool>;
///Write proxy for field `CNRDY`
pub struct CNRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CNRDY_W<'a> {
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
///Reader of field `FREQ`
pub type FREQ_R = crate::R<bool, bool>;
///Write proxy for field `FREQ`
pub struct FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_W<'a> {
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
///Reader of field `WCKCFG`
pub type WCKCFG_R = crate::R<bool, bool>;
///Write proxy for field `WCKCFG`
pub struct WCKCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> WCKCFG_W<'a> {
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
///Reader of field `MUTEDET`
pub type MUTEDET_R = crate::R<bool, bool>;
///Write proxy for field `MUTEDET`
pub struct MUTEDET_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTEDET_W<'a> {
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
///Reader of field `OVRUDR`
pub type OVRUDR_R = crate::R<bool, bool>;
///Write proxy for field `OVRUDR`
pub struct OVRUDR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRUDR_W<'a> {
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
    ///Bits 16:18 - FIFO level threshold
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    ///Bit 6 - Late frame synchronization detection
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Anticipated frame synchronization detection
    #[inline(always)]
    pub fn afsdet(&self) -> AFSDET_R {
        AFSDET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Codec not ready
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - FIFO request
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Wrong clock configuration flag. This bit is read only
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Mute detection
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Overrun / underrun
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 16:18 - FIFO level threshold
    #[inline(always)]
    pub fn flvl(&mut self) -> FLVL_W {
        FLVL_W { w: self }
    }
    ///Bit 6 - Late frame synchronization detection
    #[inline(always)]
    pub fn lfsdet(&mut self) -> LFSDET_W {
        LFSDET_W { w: self }
    }
    ///Bit 5 - Anticipated frame synchronization detection
    #[inline(always)]
    pub fn afsdet(&mut self) -> AFSDET_W {
        AFSDET_W { w: self }
    }
    ///Bit 4 - Codec not ready
    #[inline(always)]
    pub fn cnrdy(&mut self) -> CNRDY_W {
        CNRDY_W { w: self }
    }
    ///Bit 3 - FIFO request
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W { w: self }
    }
    ///Bit 2 - Wrong clock configuration flag. This bit is read only
    #[inline(always)]
    pub fn wckcfg(&mut self) -> WCKCFG_W {
        WCKCFG_W { w: self }
    }
    ///Bit 1 - Mute detection
    #[inline(always)]
    pub fn mutedet(&mut self) -> MUTEDET_W {
        MUTEDET_W { w: self }
    }
    ///Bit 0 - Overrun / underrun
    #[inline(always)]
    pub fn ovrudr(&mut self) -> OVRUDR_W {
        OVRUDR_W { w: self }
    }
}
