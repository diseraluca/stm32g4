///Reader of register RWD
pub type R = crate::R<u32, super::RWD>;
///Writer for register RWD
pub type W = crate::W<u32, super::RWD>;
///Register RWD `reset()`'s with value 0
impl crate::ResetValue for super::RWD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `WDC`
pub type WDC_R = crate::R<u8, u8>;
///Write proxy for field `WDC`
pub struct WDC_W<'a> {
    w: &'a mut W,
}
impl<'a> WDC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
///Reader of field `WDV`
pub type WDV_R = crate::R<u8, u8>;
impl R {
    ///Bits 0:7 - WDC
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - WDV
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - WDC
    #[inline(always)]
    pub fn wdc(&mut self) -> WDC_W {
        WDC_W { w: self }
    }
}
