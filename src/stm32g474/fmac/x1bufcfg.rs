///Reader of register X1BUFCFG
pub type R = crate::R<u32, super::X1BUFCFG>;
///Writer for register X1BUFCFG
pub type W = crate::W<u32, super::X1BUFCFG>;
///Register X1BUFCFG `reset()`'s with value 0
impl crate::ResetValue for super::X1BUFCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `X1_BASE`
pub type X1_BASE_R = crate::R<u8, u8>;
///Write proxy for field `X1_BASE`
pub struct X1_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> X1_BASE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
///Reader of field `X1_BUF_SIZE`
pub type X1_BUF_SIZE_R = crate::R<u8, u8>;
///Write proxy for field `X1_BUF_SIZE`
pub struct X1_BUF_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> X1_BUF_SIZE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Reader of field `FULL_WM`
pub type FULL_WM_R = crate::R<u8, u8>;
///Write proxy for field `FULL_WM`
pub struct FULL_WM_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_WM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    ///Bits 0:7 - X1_BASE
    #[inline(always)]
    pub fn x1_base(&self) -> X1_BASE_R {
        X1_BASE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - X1_BUF_SIZE
    #[inline(always)]
    pub fn x1_buf_size(&self) -> X1_BUF_SIZE_R {
        X1_BUF_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 24:25 - FULL_WM
    #[inline(always)]
    pub fn full_wm(&self) -> FULL_WM_R {
        FULL_WM_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:7 - X1_BASE
    #[inline(always)]
    pub fn x1_base(&mut self) -> X1_BASE_W {
        X1_BASE_W { w: self }
    }
    ///Bits 8:15 - X1_BUF_SIZE
    #[inline(always)]
    pub fn x1_buf_size(&mut self) -> X1_BUF_SIZE_W {
        X1_BUF_SIZE_W { w: self }
    }
    ///Bits 24:25 - FULL_WM
    #[inline(always)]
    pub fn full_wm(&mut self) -> FULL_WM_W {
        FULL_WM_W { w: self }
    }
}
