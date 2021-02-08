///Reader of register OR
pub type R = crate::R<u32, super::OR>;
///Writer for register OR
pub type W = crate::W<u32, super::OR>;
///Register OR `reset()`'s with value 0
impl crate::ResetValue for super::OR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `IN1`
pub type IN1_R = crate::R<bool, bool>;
///Write proxy for field `IN1`
pub struct IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1_W<'a> {
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
///Reader of field `IN2`
pub type IN2_R = crate::R<bool, bool>;
///Write proxy for field `IN2`
pub struct IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///Reader of field `IN1_2_1`
pub type IN1_2_1_R = crate::R<u8, u8>;
///Write proxy for field `IN1_2_1`
pub struct IN1_2_1_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1_2_1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
///Reader of field `IN2_2_1`
pub type IN2_2_1_R = crate::R<u8, u8>;
///Write proxy for field `IN2_2_1`
pub struct IN2_2_1_W<'a> {
    w: &'a mut W,
}
impl<'a> IN2_2_1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    ///Bit 0 - IN1
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - IN2
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bits 2:3 - IN1_2_1
    #[inline(always)]
    pub fn in1_2_1(&self) -> IN1_2_1_R {
        IN1_2_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 4:5 - IN2_2_1
    #[inline(always)]
    pub fn in2_2_1(&self) -> IN2_2_1_R {
        IN2_2_1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    ///Bit 0 - IN1
    #[inline(always)]
    pub fn in1(&mut self) -> IN1_W {
        IN1_W { w: self }
    }
    ///Bit 1 - IN2
    #[inline(always)]
    pub fn in2(&mut self) -> IN2_W {
        IN2_W { w: self }
    }
    ///Bits 2:3 - IN1_2_1
    #[inline(always)]
    pub fn in1_2_1(&mut self) -> IN1_2_1_W {
        IN1_2_1_W { w: self }
    }
    ///Bits 4:5 - IN2_2_1
    #[inline(always)]
    pub fn in2_2_1(&mut self) -> IN2_2_1_W {
        IN2_2_1_W { w: self }
    }
}
