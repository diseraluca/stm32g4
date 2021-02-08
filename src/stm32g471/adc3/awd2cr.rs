///Reader of register AWD2CR
pub type R = crate::R<u32, super::AWD2CR>;
///Writer for register AWD2CR
pub type W = crate::W<u32, super::AWD2CR>;
///Register AWD2CR `reset()`'s with value 0
impl crate::ResetValue for super::AWD2CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///AWD2CH
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD2CH0_A {
    ///0: Input channel not monitored by AWDx
    NOTMONITORED = 0,
    ///1: Input channel monitored by AWDx
    MONITORED = 1,
}
impl From<AWD2CH0_A> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH0_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `AWD2CH0`
pub type AWD2CH0_R = crate::R<bool, AWD2CH0_A>;
impl AWD2CH0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWD2CH0_A {
        match self.bits {
            false => AWD2CH0_A::NOTMONITORED,
            true => AWD2CH0_A::MONITORED,
        }
    }
    ///Checks if the value of the field is `NOTMONITORED`
    #[inline(always)]
    pub fn is_not_monitored(&self) -> bool {
        *self == AWD2CH0_A::NOTMONITORED
    }
    ///Checks if the value of the field is `MONITORED`
    #[inline(always)]
    pub fn is_monitored(&self) -> bool {
        *self == AWD2CH0_A::MONITORED
    }
}
///Write proxy for field `AWD2CH0`
pub struct AWD2CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
///AWD2CH
pub type AWD2CH1_A = AWD2CH0_A;
///Reader of field `AWD2CH1`
pub type AWD2CH1_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH1`
pub struct AWD2CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
///AWD2CH
pub type AWD2CH2_A = AWD2CH0_A;
///Reader of field `AWD2CH2`
pub type AWD2CH2_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH2`
pub struct AWD2CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///AWD2CH
pub type AWD2CH3_A = AWD2CH0_A;
///Reader of field `AWD2CH3`
pub type AWD2CH3_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH3`
pub struct AWD2CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///AWD2CH
pub type AWD2CH4_A = AWD2CH0_A;
///Reader of field `AWD2CH4`
pub type AWD2CH4_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH4`
pub struct AWD2CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
///AWD2CH
pub type AWD2CH5_A = AWD2CH0_A;
///Reader of field `AWD2CH5`
pub type AWD2CH5_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH5`
pub struct AWD2CH5_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
///AWD2CH
pub type AWD2CH6_A = AWD2CH0_A;
///Reader of field `AWD2CH6`
pub type AWD2CH6_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH6`
pub struct AWD2CH6_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
///AWD2CH
pub type AWD2CH7_A = AWD2CH0_A;
///Reader of field `AWD2CH7`
pub type AWD2CH7_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH7`
pub struct AWD2CH7_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
///AWD2CH
pub type AWD2CH8_A = AWD2CH0_A;
///Reader of field `AWD2CH8`
pub type AWD2CH8_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH8`
pub struct AWD2CH8_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
///AWD2CH
pub type AWD2CH9_A = AWD2CH0_A;
///Reader of field `AWD2CH9`
pub type AWD2CH9_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH9`
pub struct AWD2CH9_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///AWD2CH
pub type AWD2CH10_A = AWD2CH0_A;
///Reader of field `AWD2CH10`
pub type AWD2CH10_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH10`
pub struct AWD2CH10_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
///AWD2CH
pub type AWD2CH11_A = AWD2CH0_A;
///Reader of field `AWD2CH11`
pub type AWD2CH11_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH11`
pub struct AWD2CH11_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
///AWD2CH
pub type AWD2CH12_A = AWD2CH0_A;
///Reader of field `AWD2CH12`
pub type AWD2CH12_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH12`
pub struct AWD2CH12_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
///AWD2CH
pub type AWD2CH13_A = AWD2CH0_A;
///Reader of field `AWD2CH13`
pub type AWD2CH13_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH13`
pub struct AWD2CH13_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
///AWD2CH
pub type AWD2CH14_A = AWD2CH0_A;
///Reader of field `AWD2CH14`
pub type AWD2CH14_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH14`
pub struct AWD2CH14_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
///AWD2CH
pub type AWD2CH15_A = AWD2CH0_A;
///Reader of field `AWD2CH15`
pub type AWD2CH15_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH15`
pub struct AWD2CH15_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
///AWD2CH
pub type AWD2CH16_A = AWD2CH0_A;
///Reader of field `AWD2CH16`
pub type AWD2CH16_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH16`
pub struct AWD2CH16_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH16_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
///AWD2CH
pub type AWD2CH17_A = AWD2CH0_A;
///Reader of field `AWD2CH17`
pub type AWD2CH17_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH17`
pub struct AWD2CH17_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH17_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
///AWD2CH
pub type AWD2CH18_A = AWD2CH0_A;
///Reader of field `AWD2CH18`
pub type AWD2CH18_R = crate::R<bool, AWD2CH0_A>;
///Write proxy for field `AWD2CH18`
pub struct AWD2CH18_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2CH18_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2CH18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NOTMONITORED)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::MONITORED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    ///Bit 0 - AWD2CH
    #[inline(always)]
    pub fn awd2ch0(&self) -> AWD2CH0_R {
        AWD2CH0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - AWD2CH
    #[inline(always)]
    pub fn awd2ch1(&self) -> AWD2CH1_R {
        AWD2CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - AWD2CH
    #[inline(always)]
    pub fn awd2ch2(&self) -> AWD2CH2_R {
        AWD2CH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - AWD2CH
    #[inline(always)]
    pub fn awd2ch3(&self) -> AWD2CH3_R {
        AWD2CH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - AWD2CH
    #[inline(always)]
    pub fn awd2ch4(&self) -> AWD2CH4_R {
        AWD2CH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - AWD2CH
    #[inline(always)]
    pub fn awd2ch5(&self) -> AWD2CH5_R {
        AWD2CH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - AWD2CH
    #[inline(always)]
    pub fn awd2ch6(&self) -> AWD2CH6_R {
        AWD2CH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - AWD2CH
    #[inline(always)]
    pub fn awd2ch7(&self) -> AWD2CH7_R {
        AWD2CH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - AWD2CH
    #[inline(always)]
    pub fn awd2ch8(&self) -> AWD2CH8_R {
        AWD2CH8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - AWD2CH
    #[inline(always)]
    pub fn awd2ch9(&self) -> AWD2CH9_R {
        AWD2CH9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - AWD2CH
    #[inline(always)]
    pub fn awd2ch10(&self) -> AWD2CH10_R {
        AWD2CH10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - AWD2CH
    #[inline(always)]
    pub fn awd2ch11(&self) -> AWD2CH11_R {
        AWD2CH11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - AWD2CH
    #[inline(always)]
    pub fn awd2ch12(&self) -> AWD2CH12_R {
        AWD2CH12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - AWD2CH
    #[inline(always)]
    pub fn awd2ch13(&self) -> AWD2CH13_R {
        AWD2CH13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - AWD2CH
    #[inline(always)]
    pub fn awd2ch14(&self) -> AWD2CH14_R {
        AWD2CH14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - AWD2CH
    #[inline(always)]
    pub fn awd2ch15(&self) -> AWD2CH15_R {
        AWD2CH15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - AWD2CH
    #[inline(always)]
    pub fn awd2ch16(&self) -> AWD2CH16_R {
        AWD2CH16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - AWD2CH
    #[inline(always)]
    pub fn awd2ch17(&self) -> AWD2CH17_R {
        AWD2CH17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - AWD2CH
    #[inline(always)]
    pub fn awd2ch18(&self) -> AWD2CH18_R {
        AWD2CH18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - AWD2CH
    #[inline(always)]
    pub fn awd2ch0(&mut self) -> AWD2CH0_W {
        AWD2CH0_W { w: self }
    }
    ///Bit 1 - AWD2CH
    #[inline(always)]
    pub fn awd2ch1(&mut self) -> AWD2CH1_W {
        AWD2CH1_W { w: self }
    }
    ///Bit 2 - AWD2CH
    #[inline(always)]
    pub fn awd2ch2(&mut self) -> AWD2CH2_W {
        AWD2CH2_W { w: self }
    }
    ///Bit 3 - AWD2CH
    #[inline(always)]
    pub fn awd2ch3(&mut self) -> AWD2CH3_W {
        AWD2CH3_W { w: self }
    }
    ///Bit 4 - AWD2CH
    #[inline(always)]
    pub fn awd2ch4(&mut self) -> AWD2CH4_W {
        AWD2CH4_W { w: self }
    }
    ///Bit 5 - AWD2CH
    #[inline(always)]
    pub fn awd2ch5(&mut self) -> AWD2CH5_W {
        AWD2CH5_W { w: self }
    }
    ///Bit 6 - AWD2CH
    #[inline(always)]
    pub fn awd2ch6(&mut self) -> AWD2CH6_W {
        AWD2CH6_W { w: self }
    }
    ///Bit 7 - AWD2CH
    #[inline(always)]
    pub fn awd2ch7(&mut self) -> AWD2CH7_W {
        AWD2CH7_W { w: self }
    }
    ///Bit 8 - AWD2CH
    #[inline(always)]
    pub fn awd2ch8(&mut self) -> AWD2CH8_W {
        AWD2CH8_W { w: self }
    }
    ///Bit 9 - AWD2CH
    #[inline(always)]
    pub fn awd2ch9(&mut self) -> AWD2CH9_W {
        AWD2CH9_W { w: self }
    }
    ///Bit 10 - AWD2CH
    #[inline(always)]
    pub fn awd2ch10(&mut self) -> AWD2CH10_W {
        AWD2CH10_W { w: self }
    }
    ///Bit 11 - AWD2CH
    #[inline(always)]
    pub fn awd2ch11(&mut self) -> AWD2CH11_W {
        AWD2CH11_W { w: self }
    }
    ///Bit 12 - AWD2CH
    #[inline(always)]
    pub fn awd2ch12(&mut self) -> AWD2CH12_W {
        AWD2CH12_W { w: self }
    }
    ///Bit 13 - AWD2CH
    #[inline(always)]
    pub fn awd2ch13(&mut self) -> AWD2CH13_W {
        AWD2CH13_W { w: self }
    }
    ///Bit 14 - AWD2CH
    #[inline(always)]
    pub fn awd2ch14(&mut self) -> AWD2CH14_W {
        AWD2CH14_W { w: self }
    }
    ///Bit 15 - AWD2CH
    #[inline(always)]
    pub fn awd2ch15(&mut self) -> AWD2CH15_W {
        AWD2CH15_W { w: self }
    }
    ///Bit 16 - AWD2CH
    #[inline(always)]
    pub fn awd2ch16(&mut self) -> AWD2CH16_W {
        AWD2CH16_W { w: self }
    }
    ///Bit 17 - AWD2CH
    #[inline(always)]
    pub fn awd2ch17(&mut self) -> AWD2CH17_W {
        AWD2CH17_W { w: self }
    }
    ///Bit 18 - AWD2CH
    #[inline(always)]
    pub fn awd2ch18(&mut self) -> AWD2CH18_W {
        AWD2CH18_W { w: self }
    }
}
