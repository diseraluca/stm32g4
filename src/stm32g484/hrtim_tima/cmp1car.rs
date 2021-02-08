///Reader of register CMP1CAR
pub type R = crate::R<u32, super::CMP1CAR>;
///Writer for register CMP1CAR
pub type W = crate::W<u32, super::CMP1CAR>;
///Register CMP1CAR `reset()`'s with value 0
impl crate::ResetValue for super::CMP1CAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `REPx`
pub type REPX_R = crate::R<u8, u8>;
///Write proxy for field `REPx`
pub struct REPX_W<'a> {
    w: &'a mut W,
}
impl<'a> REPX_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///Reader of field `CMP1x`
pub type CMP1X_R = crate::R<u16, u16>;
///Write proxy for field `CMP1x`
pub struct CMP1X_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1X_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 16:23 - Timerx Repetition value (aliased from HRTIM_REPx register)
    #[inline(always)]
    pub fn repx(&self) -> REPX_R {
        REPX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 0:15 - Timerx Compare 1 value
    #[inline(always)]
    pub fn cmp1x(&self) -> CMP1X_R {
        CMP1X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 16:23 - Timerx Repetition value (aliased from HRTIM_REPx register)
    #[inline(always)]
    pub fn repx(&mut self) -> REPX_W {
        REPX_W { w: self }
    }
    ///Bits 0:15 - Timerx Compare 1 value
    #[inline(always)]
    pub fn cmp1x(&mut self) -> CMP1X_W {
        CMP1X_W { w: self }
    }
}
