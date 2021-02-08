///Reader of register AF2
pub type R = crate::R<u32, super::AF2>;
///Writer for register AF2
pub type W = crate::W<u32, super::AF2>;
///Register AF2 `reset()`'s with value 0
impl crate::ResetValue for super::AF2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `OCRSEL`
pub type OCRSEL_R = crate::R<u8, u8>;
///Write proxy for field `OCRSEL`
pub struct OCRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRSEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    ///Bits 16:18 - OCREF_CLR source selection
    #[inline(always)]
    pub fn ocrsel(&self) -> OCRSEL_R {
        OCRSEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    ///Bits 16:18 - OCREF_CLR source selection
    #[inline(always)]
    pub fn ocrsel(&mut self) -> OCRSEL_W {
        OCRSEL_W { w: self }
    }
}
