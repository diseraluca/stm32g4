///Reader of register APB1L_FZ
pub type R = crate::R<u32, super::APB1L_FZ>;
///Writer for register APB1L_FZ
pub type W = crate::W<u32, super::APB1L_FZ>;
///Register APB1L_FZ `reset()`'s with value 0
impl crate::ResetValue for super::APB1L_FZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DBG_TIMER2_STOP`
pub type DBG_TIMER2_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_TIMER2_STOP`
pub struct DBG_TIMER2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIMER2_STOP_W<'a> {
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
///Reader of field `DBG_TIM3_STOP`
pub type DBG_TIM3_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_TIM3_STOP`
pub struct DBG_TIM3_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM3_STOP_W<'a> {
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
///Reader of field `DBG_TIM4_STOP`
pub type DBG_TIM4_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_TIM4_STOP`
pub struct DBG_TIM4_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM4_STOP_W<'a> {
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
///Reader of field `DBG_TIM5_STOP`
pub type DBG_TIM5_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_TIM5_STOP`
pub struct DBG_TIM5_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM5_STOP_W<'a> {
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
///Reader of field `DBG_TIMER6_STOP`
pub type DBG_TIMER6_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_TIMER6_STOP`
pub struct DBG_TIMER6_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIMER6_STOP_W<'a> {
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
///Reader of field `DBG_TIM7_STOP`
pub type DBG_TIM7_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_TIM7_STOP`
pub struct DBG_TIM7_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM7_STOP_W<'a> {
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
///Reader of field `DBG_RTC_STOP`
pub type DBG_RTC_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_RTC_STOP`
pub struct DBG_RTC_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_RTC_STOP_W<'a> {
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
///Reader of field `DBG_WWDG_STOP`
pub type DBG_WWDG_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_WWDG_STOP`
pub struct DBG_WWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_WWDG_STOP_W<'a> {
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
///Reader of field `DBG_IWDG_STOP`
pub type DBG_IWDG_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_IWDG_STOP`
pub struct DBG_IWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_IWDG_STOP_W<'a> {
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
///Reader of field `DBG_I2C1_STOP`
pub type DBG_I2C1_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_I2C1_STOP`
pub struct DBG_I2C1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C1_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
///Reader of field `DBG_I2C2_STOP`
pub type DBG_I2C2_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_I2C2_STOP`
pub struct DBG_I2C2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C2_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
///Reader of field `DBG_I2C3_STOP`
pub type DBG_I2C3_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_I2C3_STOP`
pub struct DBG_I2C3_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C3_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
///Reader of field `DBG_LPTIMER_STOP`
pub type DBG_LPTIMER_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_LPTIMER_STOP`
pub struct DBG_LPTIMER_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LPTIMER_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bit 0 - Debug Timer 2 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer2_stop(&self) -> DBG_TIMER2_STOP_R {
        DBG_TIMER2_STOP_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TIM3 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - TIM4 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - TIM5 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Debug Timer 6 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer6_stop(&self) -> DBG_TIMER6_STOP_R {
        DBG_TIMER6_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - TIM7 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 10 - Debug RTC stopped when Core is halted
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Debug Window Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Debug Independent Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c2_stop(&self) -> DBG_I2C2_STOP_R {
        DBG_I2C2_STOP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 30 - I2C3 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - LPTIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptimer_stop(&self) -> DBG_LPTIMER_STOP_R {
        DBG_LPTIMER_STOP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Debug Timer 2 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer2_stop(&mut self) -> DBG_TIMER2_STOP_W {
        DBG_TIMER2_STOP_W { w: self }
    }
    ///Bit 1 - TIM3 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W {
        DBG_TIM3_STOP_W { w: self }
    }
    ///Bit 2 - TIM4 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W {
        DBG_TIM4_STOP_W { w: self }
    }
    ///Bit 3 - TIM5 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W {
        DBG_TIM5_STOP_W { w: self }
    }
    ///Bit 4 - Debug Timer 6 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer6_stop(&mut self) -> DBG_TIMER6_STOP_W {
        DBG_TIMER6_STOP_W { w: self }
    }
    ///Bit 5 - TIM7 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W {
        DBG_TIM7_STOP_W { w: self }
    }
    ///Bit 10 - Debug RTC stopped when Core is halted
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W {
        DBG_RTC_STOP_W { w: self }
    }
    ///Bit 11 - Debug Window Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W {
        DBG_WWDG_STOP_W { w: self }
    }
    ///Bit 12 - Debug Independent Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W {
        DBG_IWDG_STOP_W { w: self }
    }
    ///Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W {
        DBG_I2C1_STOP_W { w: self }
    }
    ///Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c2_stop(&mut self) -> DBG_I2C2_STOP_W {
        DBG_I2C2_STOP_W { w: self }
    }
    ///Bit 30 - I2C3 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W {
        DBG_I2C3_STOP_W { w: self }
    }
    ///Bit 31 - LPTIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptimer_stop(&mut self) -> DBG_LPTIMER_STOP_W {
        DBG_LPTIMER_STOP_W { w: self }
    }
}
