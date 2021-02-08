///Reader of register CCMR1_Input
pub type R = crate::R<u32, super::CCMR1_INPUT>;
///Writer for register CCMR1_Input
pub type W = crate::W<u32, super::CCMR1_INPUT>;
///Register CCMR1_Input `reset()`'s with value 0
impl crate::ResetValue for super::CCMR1_INPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `IC2F`
pub type IC2F_R = crate::R<u8, u8>;
///Write proxy for field `IC2F`
pub struct IC2F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2F_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
///Reader of field `IC2PSC`
pub type IC2PSC_R = crate::R<u8, u8>;
///Write proxy for field `IC2PSC`
pub struct IC2PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2PSC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
///Reader of field `CC2S`
pub type CC2S_R = crate::R<u8, u8>;
///Write proxy for field `CC2S`
pub struct CC2S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2S_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
///Reader of field `IC1F`
pub type IC1F_R = crate::R<u8, u8>;
///Write proxy for field `IC1F`
pub struct IC1F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1F_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
///Reader of field `IC1PSC`
pub type IC1PSC_R = crate::R<u8, u8>;
///Write proxy for field `IC1PSC`
pub struct IC1PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1PSC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
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
    ///Bits 12:15 - IC2F
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 10:11 - IC2PSC
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bits 8:9 - CC2S
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 4:7 - Input capture 1 filter
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 2:3 - Input capture 1 prescaler
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 12:15 - IC2F
    #[inline(always)]
    pub fn ic2f(&mut self) -> IC2F_W {
        IC2F_W { w: self }
    }
    ///Bits 10:11 - IC2PSC
    #[inline(always)]
    pub fn ic2psc(&mut self) -> IC2PSC_W {
        IC2PSC_W { w: self }
    }
    ///Bits 8:9 - CC2S
    #[inline(always)]
    pub fn cc2s(&mut self) -> CC2S_W {
        CC2S_W { w: self }
    }
    ///Bits 4:7 - Input capture 1 filter
    #[inline(always)]
    pub fn ic1f(&mut self) -> IC1F_W {
        IC1F_W { w: self }
    }
    ///Bits 2:3 - Input capture 1 prescaler
    #[inline(always)]
    pub fn ic1psc(&mut self) -> IC1PSC_W {
        IC1PSC_W { w: self }
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W {
        CC1S_W { w: self }
    }
}
