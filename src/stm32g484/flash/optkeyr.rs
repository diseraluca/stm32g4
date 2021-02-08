///Writer for register OPTKEYR
pub type W = crate::W<u32, super::OPTKEYR>;
///Register OPTKEYR `reset()`'s with value 0
impl crate::ResetValue for super::OPTKEYR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Option byte key
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum OPTKEYR_AW {
    ///135866939: First value to unlock the FLASH_OPTR register
    KEY1 = 135866939,
    ///1281191551: Second value to unlock the FLASH_OPTR register
    KEY2 = 1281191551,
}
impl From<OPTKEYR_AW> for u32 {
    #[inline(always)]
    fn from(variant: OPTKEYR_AW) -> Self {
        variant as _
    }
}
///Write proxy for field `OPTKEYR`
pub struct OPTKEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTKEYR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OPTKEYR_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///First value to unlock the FLASH_OPTR register
    #[inline(always)]
    pub fn key1(self) -> &'a mut W {
        self.variant(OPTKEYR_AW::KEY1)
    }
    ///Second value to unlock the FLASH_OPTR register
    #[inline(always)]
    pub fn key2(self) -> &'a mut W {
        self.variant(OPTKEYR_AW::KEY2)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    ///Bits 0:31 - Option byte key
    #[inline(always)]
    pub fn optkeyr(&mut self) -> OPTKEYR_W {
        OPTKEYR_W { w: self }
    }
}
