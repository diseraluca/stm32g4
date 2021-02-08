///Reader of register SCSR
pub type R = crate::R<u32, super::SCSR>;
///Writer for register SCSR
pub type W = crate::W<u32, super::SCSR>;
///Register SCSR `reset()`'s with value 0
impl crate::ResetValue for super::SCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CCMER`
pub type CCMER_R = crate::R<bool, bool>;
///Write proxy for field `CCMER`
pub struct CCMER_W<'a> {
    w: &'a mut W,
}
impl<'a> CCMER_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
///Reader of field `CCMBSY`
pub type CCMBSY_R = crate::R<bool, bool>;
impl R {
    ///Bit 0 - CCM SRAM Erase
    #[inline(always)]
    pub fn ccmer(&self) -> CCMER_R {
        CCMER_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - CCM SRAM busy by erase operation
    #[inline(always)]
    pub fn ccmbsy(&self) -> CCMBSY_R {
        CCMBSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - CCM SRAM Erase
    #[inline(always)]
    pub fn ccmer(&mut self) -> CCMER_W {
        CCMER_W { w: self }
    }
}
