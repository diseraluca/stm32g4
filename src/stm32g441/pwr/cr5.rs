///Reader of register CR5
pub type R = crate::R<u32, super::CR5>;
///Writer for register CR5
pub type W = crate::W<u32, super::CR5>;
///Register CR5 `reset()`'s with value 0x0100
impl crate::ResetValue for super::CR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
///Reader of field `R1MODE`
pub type R1MODE_R = crate::R<bool, bool>;
///Write proxy for field `R1MODE`
pub struct R1MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> R1MODE_W<'a> {
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
impl R {
    ///Bit 0 - Main regular range 1 mode
    #[inline(always)]
    pub fn r1mode(&self) -> R1MODE_R {
        R1MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Main regular range 1 mode
    #[inline(always)]
    pub fn r1mode(&mut self) -> R1MODE_W {
        R1MODE_W { w: self }
    }
}
