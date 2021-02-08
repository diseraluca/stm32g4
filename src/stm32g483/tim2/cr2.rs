///Reader of register CR2
pub type R = crate::R<u32, super::CR2>;
///Writer for register CR2
pub type W = crate::W<u32, super::CR2>;
///Register CR2 `reset()`'s with value 0
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `MMS_3`
pub type MMS_3_R = crate::R<bool, bool>;
///Write proxy for field `MMS_3`
pub struct MMS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> MMS_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
///Reader of field `MMS2`
pub type MMS2_R = crate::R<u8, u8>;
///Write proxy for field `MMS2`
pub struct MMS2_W<'a> {
    w: &'a mut W,
}
impl<'a> MMS2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
///Reader of field `OIS6`
pub type OIS6_R = crate::R<bool, bool>;
///Write proxy for field `OIS6`
pub struct OIS6_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS6_W<'a> {
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
///Reader of field `OIS5`
pub type OIS5_R = crate::R<bool, bool>;
///Write proxy for field `OIS5`
pub struct OIS5_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS5_W<'a> {
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
///Reader of field `OIS4N`
pub type OIS4N_R = crate::R<bool, bool>;
///Write proxy for field `OIS4N`
pub struct OIS4N_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS4N_W<'a> {
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
///Reader of field `OIS4`
pub type OIS4_R = crate::R<bool, bool>;
///Write proxy for field `OIS4`
pub struct OIS4_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS4_W<'a> {
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
///Reader of field `OIS3N`
pub type OIS3N_R = crate::R<bool, bool>;
///Write proxy for field `OIS3N`
pub struct OIS3N_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS3N_W<'a> {
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
///Reader of field `OIS3`
pub type OIS3_R = crate::R<bool, bool>;
///Write proxy for field `OIS3`
pub struct OIS3_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS3_W<'a> {
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
///Reader of field `OIS2N`
pub type OIS2N_R = crate::R<bool, bool>;
///Write proxy for field `OIS2N`
pub struct OIS2N_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS2N_W<'a> {
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
///Reader of field `OIS2`
pub type OIS2_R = crate::R<bool, bool>;
///Write proxy for field `OIS2`
pub struct OIS2_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS2_W<'a> {
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
///Reader of field `OIS1N`
pub type OIS1N_R = crate::R<bool, bool>;
///Write proxy for field `OIS1N`
pub struct OIS1N_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS1N_W<'a> {
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
///Reader of field `OIS1`
pub type OIS1_R = crate::R<bool, bool>;
///Write proxy for field `OIS1`
pub struct OIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS1_W<'a> {
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
///Reader of field `TI1S`
pub type TI1S_R = crate::R<bool, bool>;
///Write proxy for field `TI1S`
pub struct TI1S_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1S_W<'a> {
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
///Reader of field `MMS`
pub type MMS_R = crate::R<u8, u8>;
///Write proxy for field `MMS`
pub struct MMS_W<'a> {
    w: &'a mut W,
}
impl<'a> MMS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
///Reader of field `CCDS`
pub type CCDS_R = crate::R<bool, bool>;
///Write proxy for field `CCDS`
pub struct CCDS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCDS_W<'a> {
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
///Reader of field `CCUS`
pub type CCUS_R = crate::R<bool, bool>;
///Write proxy for field `CCUS`
pub struct CCUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCUS_W<'a> {
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
///Reader of field `CCPC`
pub type CCPC_R = crate::R<bool, bool>;
///Write proxy for field `CCPC`
pub struct CCPC_W<'a> {
    w: &'a mut W,
}
impl<'a> CCPC_W<'a> {
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
    ///Bit 25 - Master mode selection - bit 3
    #[inline(always)]
    pub fn mms_3(&self) -> MMS_3_R {
        MMS_3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bits 20:23 - Master mode selection 2
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bit 18 - Output Idle state 6 (OC6 output)
    #[inline(always)]
    pub fn ois6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 16 - Output Idle state 5 (OC5 output)
    #[inline(always)]
    pub fn ois5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - Output Idle state 4 (OC4N output)
    #[inline(always)]
    pub fn ois4n(&self) -> OIS4N_R {
        OIS4N_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Output Idle state 4
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Output Idle state 3
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Output Idle state 3
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Output Idle state 2
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Output Idle state 2
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Output Idle state 1
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Output Idle state 1
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 25 - Master mode selection - bit 3
    #[inline(always)]
    pub fn mms_3(&mut self) -> MMS_3_W {
        MMS_3_W { w: self }
    }
    ///Bits 20:23 - Master mode selection 2
    #[inline(always)]
    pub fn mms2(&mut self) -> MMS2_W {
        MMS2_W { w: self }
    }
    ///Bit 18 - Output Idle state 6 (OC6 output)
    #[inline(always)]
    pub fn ois6(&mut self) -> OIS6_W {
        OIS6_W { w: self }
    }
    ///Bit 16 - Output Idle state 5 (OC5 output)
    #[inline(always)]
    pub fn ois5(&mut self) -> OIS5_W {
        OIS5_W { w: self }
    }
    ///Bit 15 - Output Idle state 4 (OC4N output)
    #[inline(always)]
    pub fn ois4n(&mut self) -> OIS4N_W {
        OIS4N_W { w: self }
    }
    ///Bit 14 - Output Idle state 4
    #[inline(always)]
    pub fn ois4(&mut self) -> OIS4_W {
        OIS4_W { w: self }
    }
    ///Bit 13 - Output Idle state 3
    #[inline(always)]
    pub fn ois3n(&mut self) -> OIS3N_W {
        OIS3N_W { w: self }
    }
    ///Bit 12 - Output Idle state 3
    #[inline(always)]
    pub fn ois3(&mut self) -> OIS3_W {
        OIS3_W { w: self }
    }
    ///Bit 11 - Output Idle state 2
    #[inline(always)]
    pub fn ois2n(&mut self) -> OIS2N_W {
        OIS2N_W { w: self }
    }
    ///Bit 10 - Output Idle state 2
    #[inline(always)]
    pub fn ois2(&mut self) -> OIS2_W {
        OIS2_W { w: self }
    }
    ///Bit 9 - Output Idle state 1
    #[inline(always)]
    pub fn ois1n(&mut self) -> OIS1N_W {
        OIS1N_W { w: self }
    }
    ///Bit 8 - Output Idle state 1
    #[inline(always)]
    pub fn ois1(&mut self) -> OIS1_W {
        OIS1_W { w: self }
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W {
        TI1S_W { w: self }
    }
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W {
        MMS_W { w: self }
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W {
        CCDS_W { w: self }
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W {
        CCUS_W { w: self }
    }
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W {
        CCPC_W { w: self }
    }
}
