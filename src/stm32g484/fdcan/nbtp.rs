///Reader of register NBTP
pub type R = crate::R<u32, super::NBTP>;
///Writer for register NBTP
pub type W = crate::W<u32, super::NBTP>;
///Register NBTP `reset()`'s with value 0x0a33
impl crate::ResetValue for super::NBTP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a33
    }
}
///Reader of field `TSEG2`
pub type TSEG2_R = crate::R<u8, u8>;
///Write proxy for field `TSEG2`
pub struct TSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
///Reader of field `NTSEG1`
pub type NTSEG1_R = crate::R<u8, u8>;
///Write proxy for field `NTSEG1`
pub struct NTSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> NTSEG1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Reader of field `NBRP`
pub type NBRP_R = crate::R<u16, u16>;
///Write proxy for field `NBRP`
pub struct NBRP_W<'a> {
    w: &'a mut W,
}
impl<'a> NBRP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
///Reader of field `NSJW`
pub type NSJW_R = crate::R<u8, u8>;
///Write proxy for field `NSJW`
pub struct NSJW_W<'a> {
    w: &'a mut W,
}
impl<'a> NSJW_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
impl R {
    ///Bits 0:6 - TSEG2
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:15 - NTSEG1
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:24 - NBRP
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bits 25:31 - NSJW
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - TSEG2
    #[inline(always)]
    pub fn tseg2(&mut self) -> TSEG2_W {
        TSEG2_W { w: self }
    }
    ///Bits 8:15 - NTSEG1
    #[inline(always)]
    pub fn ntseg1(&mut self) -> NTSEG1_W {
        NTSEG1_W { w: self }
    }
    ///Bits 16:24 - NBRP
    #[inline(always)]
    pub fn nbrp(&mut self) -> NBRP_W {
        NBRP_W { w: self }
    }
    ///Bits 25:31 - NSJW
    #[inline(always)]
    pub fn nsjw(&mut self) -> NSJW_W {
        NSJW_W { w: self }
    }
}
