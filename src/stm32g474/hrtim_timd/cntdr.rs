///Reader of register CNTDR
pub type R = crate::R<u32, super::CNTDR>;
///Writer for register CNTDR
pub type W = crate::W<u32, super::CNTDR>;
///Register CNTDR `reset()`'s with value 0
impl crate::ResetValue for super::CNTDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CNTx`
pub type CNTX_R = crate::R<u16, u16>;
///Write proxy for field `CNTx`
pub struct CNTX_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTX_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Timerx Counter value
    #[inline(always)]
    pub fn cntx(&self) -> CNTX_R {
        CNTX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Timerx Counter value
    #[inline(always)]
    pub fn cntx(&mut self) -> CNTX_W {
        CNTX_W { w: self }
    }
}
