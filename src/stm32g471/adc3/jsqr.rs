///Reader of register JSQR
pub type R = crate::R<u32, super::JSQR>;
///Writer for register JSQR
pub type W = crate::W<u32, super::JSQR>;
///Register JSQR `reset()`'s with value 0
impl crate::ResetValue for super::JSQR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `JSQ4`
pub type JSQ4_R = crate::R<u8, u8>;
///Write proxy for field `JSQ4`
pub struct JSQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> JSQ4_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
///Reader of field `JSQ3`
pub type JSQ3_R = crate::R<u8, u8>;
///Write proxy for field `JSQ3`
pub struct JSQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> JSQ3_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | (((value as u32) & 0x1f) << 21);
        self.w
    }
}
///Reader of field `JSQ2`
pub type JSQ2_R = crate::R<u8, u8>;
///Write proxy for field `JSQ2`
pub struct JSQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> JSQ2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
///Reader of field `JSQ1`
pub type JSQ1_R = crate::R<u8, u8>;
///Write proxy for field `JSQ1`
pub struct JSQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> JSQ1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 9)) | (((value as u32) & 0x1f) << 9);
        self.w
    }
}
///External Trigger Enable and Polarity Selection for injected channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum JEXTEN_A {
    ///0: Trigger detection disabled
    DISABLED = 0,
    ///1: Trigger detection on the rising edge
    RISINGEDGE = 1,
    ///2: Trigger detection on the falling edge
    FALLINGEDGE = 2,
    ///3: Trigger detection on both the rising and falling edges
    BOTHEDGES = 3,
}
impl From<JEXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTEN_A) -> Self {
        variant as _
    }
}
///Reader of field `JEXTEN`
pub type JEXTEN_R = crate::R<u8, JEXTEN_A>;
impl JEXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JEXTEN_A {
        match self.bits {
            0 => JEXTEN_A::DISABLED,
            1 => JEXTEN_A::RISINGEDGE,
            2 => JEXTEN_A::FALLINGEDGE,
            3 => JEXTEN_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTEN_A::DISABLED
    }
    ///Checks if the value of the field is `RISINGEDGE`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == JEXTEN_A::RISINGEDGE
    }
    ///Checks if the value of the field is `FALLINGEDGE`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == JEXTEN_A::FALLINGEDGE
    }
    ///Checks if the value of the field is `BOTHEDGES`
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == JEXTEN_A::BOTHEDGES
    }
}
///Write proxy for field `JEXTEN`
pub struct JEXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: JEXTEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Trigger detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEXTEN_A::DISABLED)
    }
    ///Trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(JEXTEN_A::RISINGEDGE)
    }
    ///Trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(JEXTEN_A::FALLINGEDGE)
    }
    ///Trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(JEXTEN_A::BOTHEDGES)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
///Reader of field `JEXTSEL`
pub type JEXTSEL_R = crate::R<u8, u8>;
///Write proxy for field `JEXTSEL`
pub struct JEXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTSEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | (((value as u32) & 0x1f) << 2);
        self.w
    }
}
///Reader of field `JL`
pub type JL_R = crate::R<u8, u8>;
///Write proxy for field `JL`
pub struct JL_W<'a> {
    w: &'a mut W,
}
impl<'a> JL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    ///Bits 27:31 - JSQ4
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    ///Bits 21:25 - 3rd conversion in the injected sequence
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bits 15:19 - 2nd conversion in the injected sequence
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 9:13 - 1st conversion in the injected sequence
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    ///Bits 7:8 - External Trigger Enable and Polarity Selection for injected channels
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    ///Bits 2:6 - External Trigger Selection for injected group
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bits 0:1 - Injected channel sequence length
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 27:31 - JSQ4
    #[inline(always)]
    pub fn jsq4(&mut self) -> JSQ4_W {
        JSQ4_W { w: self }
    }
    ///Bits 21:25 - 3rd conversion in the injected sequence
    #[inline(always)]
    pub fn jsq3(&mut self) -> JSQ3_W {
        JSQ3_W { w: self }
    }
    ///Bits 15:19 - 2nd conversion in the injected sequence
    #[inline(always)]
    pub fn jsq2(&mut self) -> JSQ2_W {
        JSQ2_W { w: self }
    }
    ///Bits 9:13 - 1st conversion in the injected sequence
    #[inline(always)]
    pub fn jsq1(&mut self) -> JSQ1_W {
        JSQ1_W { w: self }
    }
    ///Bits 7:8 - External Trigger Enable and Polarity Selection for injected channels
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W {
        JEXTEN_W { w: self }
    }
    ///Bits 2:6 - External Trigger Selection for injected group
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W {
        JEXTSEL_W { w: self }
    }
    ///Bits 0:1 - Injected channel sequence length
    #[inline(always)]
    pub fn jl(&mut self) -> JL_W {
        JL_W { w: self }
    }
}
