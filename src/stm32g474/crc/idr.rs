///Reader of register IDR
pub type R = crate::R<u32, super::IDR>;
///Writer for register IDR
pub type W = crate::W<u32, super::IDR>;
///Register IDR `reset()`'s with value 0
impl crate::ResetValue for super::IDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `IDR`
pub type IDR_R = crate::R<u32, u32>;
///Write proxy for field `IDR`
pub struct IDR_W<'a> {
    w: &'a mut W,
}
impl<'a> IDR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - General-purpose 8-bit data register bits
    #[inline(always)]
    pub fn idr(&self) -> IDR_R {
        IDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - General-purpose 8-bit data register bits
    #[inline(always)]
    pub fn idr(&mut self) -> IDR_W {
        IDR_W { w: self }
    }
}
