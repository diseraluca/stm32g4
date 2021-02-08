///Writer for register KEYR
pub type W = crate::W<u32, super::KEYR>;
///Register KEYR `reset()`'s with value 0
impl crate::ResetValue for super::KEYR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///KEYR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum KEYR_AW {
    ///1164378403: First value to unlock the FLASH_CR register
    KEY1 = 1164378403,
    ///3455027627: Second value to unlock the FLASH_CR register
    KEY2 = 3455027627,
}
impl From<KEYR_AW> for u32 {
    #[inline(always)]
    fn from(variant: KEYR_AW) -> Self {
        variant as _
    }
}
///Write proxy for field `KEYR`
pub struct KEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: KEYR_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///First value to unlock the FLASH_CR register
    #[inline(always)]
    pub fn key1(self) -> &'a mut W {
        self.variant(KEYR_AW::KEY1)
    }
    ///Second value to unlock the FLASH_CR register
    #[inline(always)]
    pub fn key2(self) -> &'a mut W {
        self.variant(KEYR_AW::KEY2)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    ///Bits 0:31 - KEYR
    #[inline(always)]
    pub fn keyr(&mut self) -> KEYR_W {
        KEYR_W { w: self }
    }
}
