///Writer for register PDKEYR
pub type W = crate::W<u32, super::PDKEYR>;
///Register PDKEYR `reset()`'s with value 0
impl crate::ResetValue for super::PDKEYR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///RUN_PD in FLASH_ACR key
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PDKEYR_AW {
    ///68494903: First value to unlock the RUN_PD bit
    PDKEY1 = 68494903,
    ///4210818301: Secind value to unlock the RUN_PD bit
    PDKEY2 = 4210818301,
}
impl From<PDKEYR_AW> for u32 {
    #[inline(always)]
    fn from(variant: PDKEYR_AW) -> Self {
        variant as _
    }
}
///Write proxy for field `PDKEYR`
pub struct PDKEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> PDKEYR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PDKEYR_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///First value to unlock the RUN_PD bit
    #[inline(always)]
    pub fn pdkey1(self) -> &'a mut W {
        self.variant(PDKEYR_AW::PDKEY1)
    }
    ///Secind value to unlock the RUN_PD bit
    #[inline(always)]
    pub fn pdkey2(self) -> &'a mut W {
        self.variant(PDKEYR_AW::PDKEY2)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    ///Bits 0:31 - RUN_PD in FLASH_ACR key
    #[inline(always)]
    pub fn pdkeyr(&mut self) -> PDKEYR_W {
        PDKEYR_W { w: self }
    }
}
