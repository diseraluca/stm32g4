///Reader of register CNDTR6
pub type R = crate::R<u32, super::CNDTR6>;
///Writer for register CNDTR6
pub type W = crate::W<u32, super::CNDTR6>;
///Register CNDTR6 `reset()`'s with value 0
impl crate::ResetValue for super::CNDTR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `NDT`
pub type NDT_R = crate::R<u16, u16>;
///Write proxy for field `NDT`
pub struct NDT_W<'a> {
    w: &'a mut W,
}
impl<'a> NDT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Number of data items to transfer
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Number of data items to transfer
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W {
        NDT_W { w: self }
    }
}
