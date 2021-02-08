///Reader of register BWTR4
pub type R = crate::R<u32, super::BWTR4>;
///Writer for register BWTR4
pub type W = crate::W<u32, super::BWTR4>;
///Register BWTR4 `reset()`'s with value 0x0fff_ffff
impl crate::ResetValue for super::BWTR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff_ffff
    }
}
///Reader of field `DATAHLD`
pub type DATAHLD_R = crate::R<u8, u8>;
///Write proxy for field `DATAHLD`
pub struct DATAHLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAHLD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
///Reader of field `ACCMOD`
pub type ACCMOD_R = crate::R<u8, u8>;
///Write proxy for field `ACCMOD`
pub struct ACCMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCMOD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
///Reader of field `BUSTURN`
pub type BUSTURN_R = crate::R<u8, u8>;
///Write proxy for field `BUSTURN`
pub struct BUSTURN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSTURN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
///Reader of field `DATAST`
pub type DATAST_R = crate::R<u8, u8>;
///Write proxy for field `DATAST`
pub struct DATAST_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAST_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Reader of field `ADDHLD`
pub type ADDHLD_R = crate::R<u8, u8>;
///Write proxy for field `ADDHLD`
pub struct ADDHLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDHLD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
///Reader of field `ADDSET`
pub type ADDSET_R = crate::R<u8, u8>;
///Write proxy for field `ADDSET`
pub struct ADDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDSET_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 30:31 - DATAHLD
    #[inline(always)]
    pub fn datahld(&self) -> DATAHLD_R {
        DATAHLD_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    ///Bits 28:29 - ACCMOD
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    ///Bits 16:19 - BUSTURN
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 8:15 - DATAST
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 4:7 - ADDHLD
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 0:3 - ADDSET
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 30:31 - DATAHLD
    #[inline(always)]
    pub fn datahld(&mut self) -> DATAHLD_W {
        DATAHLD_W { w: self }
    }
    ///Bits 28:29 - ACCMOD
    #[inline(always)]
    pub fn accmod(&mut self) -> ACCMOD_W {
        ACCMOD_W { w: self }
    }
    ///Bits 16:19 - BUSTURN
    #[inline(always)]
    pub fn busturn(&mut self) -> BUSTURN_W {
        BUSTURN_W { w: self }
    }
    ///Bits 8:15 - DATAST
    #[inline(always)]
    pub fn datast(&mut self) -> DATAST_W {
        DATAST_W { w: self }
    }
    ///Bits 4:7 - ADDHLD
    #[inline(always)]
    pub fn addhld(&mut self) -> ADDHLD_W {
        ADDHLD_W { w: self }
    }
    ///Bits 0:3 - ADDSET
    #[inline(always)]
    pub fn addset(&mut self) -> ADDSET_W {
        ADDSET_W { w: self }
    }
}
