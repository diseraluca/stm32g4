///Reader of register PUPDR
pub type R = crate::R<u32, super::PUPDR>;
///Writer for register PUPDR
pub type W = crate::W<u32, super::PUPDR>;
///Register PUPDR `reset()`'s with value 0x6400_0000
impl crate::ResetValue for super::PUPDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6400_0000
    }
}
///Reader of field `PUPDR15`
pub type PUPDR15_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR15`
pub struct PUPDR15_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR15_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
///Reader of field `PUPDR14`
pub type PUPDR14_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR14`
pub struct PUPDR14_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR14_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
///Reader of field `PUPDR13`
pub type PUPDR13_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR13`
pub struct PUPDR13_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR13_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
///Reader of field `PUPDR12`
pub type PUPDR12_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR12`
pub struct PUPDR12_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR12_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
///Reader of field `PUPDR11`
pub type PUPDR11_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR11`
pub struct PUPDR11_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR11_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
///Reader of field `PUPDR10`
pub type PUPDR10_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR10`
pub struct PUPDR10_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR10_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
///Reader of field `PUPDR9`
pub type PUPDR9_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR9`
pub struct PUPDR9_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR9_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
///Reader of field `PUPDR8`
pub type PUPDR8_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR8`
pub struct PUPDR8_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR8_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
///Reader of field `PUPDR7`
pub type PUPDR7_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR7`
pub struct PUPDR7_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR7_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
///Reader of field `PUPDR6`
pub type PUPDR6_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR6`
pub struct PUPDR6_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR6_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
///Reader of field `PUPDR5`
pub type PUPDR5_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR5`
pub struct PUPDR5_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR5_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
///Reader of field `PUPDR4`
pub type PUPDR4_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR4`
pub struct PUPDR4_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR4_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
///Reader of field `PUPDR3`
pub type PUPDR3_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR3`
pub struct PUPDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR3_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
///Reader of field `PUPDR2`
pub type PUPDR2_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR2`
pub struct PUPDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
///Reader of field `PUPDR1`
pub type PUPDR1_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR1`
pub struct PUPDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
///Reader of field `PUPDR0`
pub type PUPDR0_R = crate::R<u8, u8>;
///Write proxy for field `PUPDR0`
pub struct PUPDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR0_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr15(&self) -> PUPDR15_R {
        PUPDR15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr14(&self) -> PUPDR14_R {
        PUPDR14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr13(&self) -> PUPDR13_R {
        PUPDR13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    ///Bits 24:25 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr12(&self) -> PUPDR12_R {
        PUPDR12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    ///Bits 22:23 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr11(&self) -> PUPDR11_R {
        PUPDR11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    ///Bits 20:21 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr10(&self) -> PUPDR10_R {
        PUPDR10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    ///Bits 18:19 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr9(&self) -> PUPDR9_R {
        PUPDR9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    ///Bits 16:17 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr8(&self) -> PUPDR8_R {
        PUPDR8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 14:15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr7(&self) -> PUPDR7_R {
        PUPDR7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr6(&self) -> PUPDR6_R {
        PUPDR6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr5(&self) -> PUPDR5_R {
        PUPDR5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr4(&self) -> PUPDR4_R {
        PUPDR4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr3(&self) -> PUPDR3_R {
        PUPDR3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr2(&self) -> PUPDR2_R {
        PUPDR2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr1(&self) -> PUPDR1_R {
        PUPDR1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr0(&self) -> PUPDR0_R {
        PUPDR0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr15(&mut self) -> PUPDR15_W {
        PUPDR15_W { w: self }
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr14(&mut self) -> PUPDR14_W {
        PUPDR14_W { w: self }
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr13(&mut self) -> PUPDR13_W {
        PUPDR13_W { w: self }
    }
    ///Bits 24:25 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr12(&mut self) -> PUPDR12_W {
        PUPDR12_W { w: self }
    }
    ///Bits 22:23 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr11(&mut self) -> PUPDR11_W {
        PUPDR11_W { w: self }
    }
    ///Bits 20:21 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr10(&mut self) -> PUPDR10_W {
        PUPDR10_W { w: self }
    }
    ///Bits 18:19 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr9(&mut self) -> PUPDR9_W {
        PUPDR9_W { w: self }
    }
    ///Bits 16:17 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr8(&mut self) -> PUPDR8_W {
        PUPDR8_W { w: self }
    }
    ///Bits 14:15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr7(&mut self) -> PUPDR7_W {
        PUPDR7_W { w: self }
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr6(&mut self) -> PUPDR6_W {
        PUPDR6_W { w: self }
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr5(&mut self) -> PUPDR5_W {
        PUPDR5_W { w: self }
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr4(&mut self) -> PUPDR4_W {
        PUPDR4_W { w: self }
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr3(&mut self) -> PUPDR3_W {
        PUPDR3_W { w: self }
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr2(&mut self) -> PUPDR2_W {
        PUPDR2_W { w: self }
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr1(&mut self) -> PUPDR1_W {
        PUPDR1_W { w: self }
    }
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr0(&mut self) -> PUPDR0_W {
        PUPDR0_W { w: self }
    }
}
