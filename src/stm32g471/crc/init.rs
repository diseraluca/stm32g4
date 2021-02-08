///Reader of register INIT
pub type R = crate::R<u32, super::INIT>;
///Writer for register INIT
pub type W = crate::W<u32, super::INIT>;
///Register INIT `reset()`'s with value 0xffff_ffff
impl crate::ResetValue for super::INIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
///Reader of field `CRC_INIT`
pub type CRC_INIT_R = crate::R<u32, u32>;
///Write proxy for field `CRC_INIT`
pub struct CRC_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_INIT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Programmable initial CRC value
    #[inline(always)]
    pub fn crc_init(&self) -> CRC_INIT_R {
        CRC_INIT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Programmable initial CRC value
    #[inline(always)]
    pub fn crc_init(&mut self) -> CRC_INIT_W {
        CRC_INIT_W { w: self }
    }
}
