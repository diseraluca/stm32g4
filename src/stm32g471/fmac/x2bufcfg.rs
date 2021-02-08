///Reader of register X2BUFCFG
pub type R = crate::R<u32, super::X2BUFCFG>;
///Writer for register X2BUFCFG
pub type W = crate::W<u32, super::X2BUFCFG>;
///Register X2BUFCFG `reset()`'s with value 0
impl crate::ResetValue for super::X2BUFCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `X2_BASE`
pub type X2_BASE_R = crate::R<u8, u8>;
///Write proxy for field `X2_BASE`
pub struct X2_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> X2_BASE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
///Reader of field `X2_BUF_SIZE`
pub type X2_BUF_SIZE_R = crate::R<u8, u8>;
///Write proxy for field `X2_BUF_SIZE`
pub struct X2_BUF_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> X2_BUF_SIZE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    ///Bits 0:7 - X1_BASE
    #[inline(always)]
    pub fn x2_base(&self) -> X2_BASE_R {
        X2_BASE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - X1_BUF_SIZE
    #[inline(always)]
    pub fn x2_buf_size(&self) -> X2_BUF_SIZE_R {
        X2_BUF_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - X1_BASE
    #[inline(always)]
    pub fn x2_base(&mut self) -> X2_BASE_W {
        X2_BASE_W { w: self }
    }
    ///Bits 8:15 - X1_BUF_SIZE
    #[inline(always)]
    pub fn x2_buf_size(&mut self) -> X2_BUF_SIZE_W {
        X2_BUF_SIZE_W { w: self }
    }
}
