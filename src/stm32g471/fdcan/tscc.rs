///Reader of register TSCC
pub type R = crate::R<u32, super::TSCC>;
///Writer for register TSCC
pub type W = crate::W<u32, super::TSCC>;
///Register TSCC `reset()`'s with value 0
impl crate::ResetValue for super::TSCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TSS`
pub type TSS_R = crate::R<u8, u8>;
///Write proxy for field `TSS`
pub struct TSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
///Reader of field `TCP`
pub type TCP_R = crate::R<u8, u8>;
///Write proxy for field `TCP`
pub struct TCP_W<'a> {
    w: &'a mut W,
}
impl<'a> TCP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:1 - TSS
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0x03) as u8)
    }
    ///Bits 16:19 - TCP
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:1 - TSS
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W {
        TSS_W { w: self }
    }
    ///Bits 16:19 - TCP
    #[inline(always)]
    pub fn tcp(&mut self) -> TCP_W {
        TCP_W { w: self }
    }
}
