///Reader of register WRP2AR
pub type R = crate::R<u32, super::WRP2AR>;
///Writer for register WRP2AR
pub type W = crate::W<u32, super::WRP2AR>;
///Register WRP2AR `reset()`'s with value 0
impl crate::ResetValue for super::WRP2AR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `WRP2A_STRT`
pub type WRP2A_STRT_R = crate::R<u8, u8>;
///Write proxy for field `WRP2A_STRT`
pub struct WRP2A_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP2A_STRT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
///Reader of field `WRP2A_END`
pub type WRP2A_END_R = crate::R<u8, u8>;
///Write proxy for field `WRP2A_END`
pub struct WRP2A_END_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP2A_END_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:6 - WRP first area "A" start offset
    #[inline(always)]
    pub fn wrp2a_strt(&self) -> WRP2A_STRT_R {
        WRP2A_STRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - WRP first area "A" end offset
    #[inline(always)]
    pub fn wrp2a_end(&self) -> WRP2A_END_R {
        WRP2A_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - WRP first area "A" start offset
    #[inline(always)]
    pub fn wrp2a_strt(&mut self) -> WRP2A_STRT_W {
        WRP2A_STRT_W { w: self }
    }
    ///Bits 16:22 - WRP first area "A" end offset
    #[inline(always)]
    pub fn wrp2a_end(&mut self) -> WRP2A_END_W {
        WRP2A_END_W { w: self }
    }
}
