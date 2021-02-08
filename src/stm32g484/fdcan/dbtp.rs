///Reader of register DBTP
pub type R = crate::R<u32, super::DBTP>;
///Writer for register DBTP
pub type W = crate::W<u32, super::DBTP>;
///Register DBTP `reset()`'s with value 0x0a33
impl crate::ResetValue for super::DBTP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a33
    }
}
///Reader of field `DSJW`
pub type DSJW_R = crate::R<u8, u8>;
///Write proxy for field `DSJW`
pub struct DSJW_W<'a> {
    w: &'a mut W,
}
impl<'a> DSJW_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
///Reader of field `DTSEG2`
pub type DTSEG2_R = crate::R<u8, u8>;
///Write proxy for field `DTSEG2`
pub struct DTSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSEG2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
///Write proxy for field `DTSEG1`
pub struct DTSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSEG1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
///Reader of field `DBRP`
pub type DBRP_R = crate::R<u8, u8>;
///Write proxy for field `DBRP`
pub struct DBRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBRP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
///Reader of field `TDC`
pub type TDC_R = crate::R<bool, bool>;
impl R {
    ///Bits 0:3 - DSJW
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - DTSEG2
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 16:20 - DBRP
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 23 - TDC
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:3 - DSJW
    #[inline(always)]
    pub fn dsjw(&mut self) -> DSJW_W {
        DSJW_W { w: self }
    }
    ///Bits 4:7 - DTSEG2
    #[inline(always)]
    pub fn dtseg2(&mut self) -> DTSEG2_W {
        DTSEG2_W { w: self }
    }
    ///Bits 8:12 - DTSEG1
    #[inline(always)]
    pub fn dtseg1(&mut self) -> DTSEG1_W {
        DTSEG1_W { w: self }
    }
    ///Bits 16:20 - DBRP
    #[inline(always)]
    pub fn dbrp(&mut self) -> DBRP_W {
        DBRP_W { w: self }
    }
}
