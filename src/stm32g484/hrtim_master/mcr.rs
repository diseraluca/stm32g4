///Reader of register MCR
pub type R = crate::R<u32, super::MCR>;
///Writer for register MCR
pub type W = crate::W<u32, super::MCR>;
///Register MCR `reset()`'s with value 0
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `BRSTDMA`
pub type BRSTDMA_R = crate::R<u8, u8>;
///Write proxy for field `BRSTDMA`
pub struct BRSTDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSTDMA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
///Reader of field `MREPU`
pub type MREPU_R = crate::R<bool, bool>;
///Write proxy for field `MREPU`
pub struct MREPU_W<'a> {
    w: &'a mut W,
}
impl<'a> MREPU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
///Reader of field `PREEN`
pub type PREEN_R = crate::R<bool, bool>;
///Write proxy for field `PREEN`
pub struct PREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
///Reader of field `DACSYNC`
pub type DACSYNC_R = crate::R<u8, u8>;
///Write proxy for field `DACSYNC`
pub struct DACSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> DACSYNC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
///Reader of field `TFCEN`
pub type TFCEN_R = crate::R<bool, bool>;
///Write proxy for field `TFCEN`
pub struct TFCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TFCEN_W<'a> {
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
///Reader of field `TECEN`
pub type TECEN_R = crate::R<bool, bool>;
///Write proxy for field `TECEN`
pub struct TECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TECEN_W<'a> {
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
///Reader of field `TDCEN`
pub type TDCEN_R = crate::R<bool, bool>;
///Write proxy for field `TDCEN`
pub struct TDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
///Reader of field `TCCEN`
pub type TCCEN_R = crate::R<bool, bool>;
///Write proxy for field `TCCEN`
pub struct TCCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
///Reader of field `TBCEN`
pub type TBCEN_R = crate::R<bool, bool>;
///Write proxy for field `TBCEN`
pub struct TBCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCEN_W<'a> {
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
///Reader of field `TACEN`
pub type TACEN_R = crate::R<bool, bool>;
///Write proxy for field `TACEN`
pub struct TACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TACEN_W<'a> {
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
///Reader of field `MCEN`
pub type MCEN_R = crate::R<bool, bool>;
///Write proxy for field `MCEN`
pub struct MCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEN_W<'a> {
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
///Reader of field `SYNC_SRC`
pub type SYNC_SRC_R = crate::R<u8, u8>;
///Write proxy for field `SYNC_SRC`
pub struct SYNC_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_SRC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
///Reader of field `SYNC_OUT`
pub type SYNC_OUT_R = crate::R<u8, u8>;
///Write proxy for field `SYNC_OUT`
pub struct SYNC_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_OUT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
///Reader of field `SYNCSTRTM`
pub type SYNCSTRTM_R = crate::R<bool, bool>;
///Write proxy for field `SYNCSTRTM`
pub struct SYNCSTRTM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCSTRTM_W<'a> {
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
///Reader of field `SYNCRSTM`
pub type SYNCRSTM_R = crate::R<bool, bool>;
///Write proxy for field `SYNCRSTM`
pub struct SYNCRSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCRSTM_W<'a> {
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
///Reader of field `SYNC_IN`
pub type SYNC_IN_R = crate::R<u8, u8>;
///Write proxy for field `SYNC_IN`
pub struct SYNC_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_IN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
///Reader of field `INTLVD`
pub type INTLVD_R = crate::R<u8, u8>;
///Write proxy for field `INTLVD`
pub struct INTLVD_W<'a> {
    w: &'a mut W,
}
impl<'a> INTLVD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
///Reader of field `HALF`
pub type HALF_R = crate::R<bool, bool>;
///Write proxy for field `HALF`
pub struct HALF_W<'a> {
    w: &'a mut W,
}
impl<'a> HALF_W<'a> {
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
///Reader of field `RETRIG`
pub type RETRIG_R = crate::R<bool, bool>;
///Write proxy for field `RETRIG`
pub struct RETRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRIG_W<'a> {
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
///Reader of field `CONT`
pub type CONT_R = crate::R<bool, bool>;
///Write proxy for field `CONT`
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
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
///Reader of field `CK_PSC`
pub type CK_PSC_R = crate::R<u8, u8>;
///Write proxy for field `CK_PSC`
pub struct CK_PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CK_PSC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    ///Bits 30:31 - Burst DMA Update
    #[inline(always)]
    pub fn brstdma(&self) -> BRSTDMA_R {
        BRSTDMA_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    ///Bit 29 - Master Timer Repetition update
    #[inline(always)]
    pub fn mrepu(&self) -> MREPU_R {
        MREPU_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 27 - Preload enable
    #[inline(always)]
    pub fn preen(&self) -> PREEN_R {
        PREEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bits 25:26 - AC Synchronization
    #[inline(always)]
    pub fn dacsync(&self) -> DACSYNC_R {
        DACSYNC_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    ///Bit 22 - Timer F counter enable
    #[inline(always)]
    pub fn tfcen(&self) -> TFCEN_R {
        TFCEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 21 - Timer E counter enable
    #[inline(always)]
    pub fn tecen(&self) -> TECEN_R {
        TECEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 20 - Timer D counter enable
    #[inline(always)]
    pub fn tdcen(&self) -> TDCEN_R {
        TDCEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 19 - Timer C counter enable
    #[inline(always)]
    pub fn tccen(&self) -> TCCEN_R {
        TCCEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 18 - Timer B counter enable
    #[inline(always)]
    pub fn tbcen(&self) -> TBCEN_R {
        TBCEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - Timer A counter enable
    #[inline(always)]
    pub fn tacen(&self) -> TACEN_R {
        TACEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - Master Counter enable
    #[inline(always)]
    pub fn mcen(&self) -> MCEN_R {
        MCEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bits 14:15 - Synchronization source
    #[inline(always)]
    pub fn sync_src(&self) -> SYNC_SRC_R {
        SYNC_SRC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    ///Bits 12:13 - Synchronization output
    #[inline(always)]
    pub fn sync_out(&self) -> SYNC_OUT_R {
        SYNC_OUT_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bit 11 - Synchronization Starts Master
    #[inline(always)]
    pub fn syncstrtm(&self) -> SYNCSTRTM_R {
        SYNCSTRTM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Synchronization Resets Master
    #[inline(always)]
    pub fn syncrstm(&self) -> SYNCRSTM_R {
        SYNCRSTM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bits 8:9 - synchronization input
    #[inline(always)]
    pub fn sync_in(&self) -> SYNC_IN_R {
        SYNC_IN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 6:7 - Interleaved mode
    #[inline(always)]
    pub fn intlvd(&self) -> INTLVD_R {
        INTLVD_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    ///Bit 5 - Half mode enable
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Master Re-triggerable mode
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Master Continuous mode
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 0:2 - HRTIM Master Clock prescaler
    #[inline(always)]
    pub fn ck_psc(&self) -> CK_PSC_R {
        CK_PSC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 30:31 - Burst DMA Update
    #[inline(always)]
    pub fn brstdma(&mut self) -> BRSTDMA_W {
        BRSTDMA_W { w: self }
    }
    ///Bit 29 - Master Timer Repetition update
    #[inline(always)]
    pub fn mrepu(&mut self) -> MREPU_W {
        MREPU_W { w: self }
    }
    ///Bit 27 - Preload enable
    #[inline(always)]
    pub fn preen(&mut self) -> PREEN_W {
        PREEN_W { w: self }
    }
    ///Bits 25:26 - AC Synchronization
    #[inline(always)]
    pub fn dacsync(&mut self) -> DACSYNC_W {
        DACSYNC_W { w: self }
    }
    ///Bit 22 - Timer F counter enable
    #[inline(always)]
    pub fn tfcen(&mut self) -> TFCEN_W {
        TFCEN_W { w: self }
    }
    ///Bit 21 - Timer E counter enable
    #[inline(always)]
    pub fn tecen(&mut self) -> TECEN_W {
        TECEN_W { w: self }
    }
    ///Bit 20 - Timer D counter enable
    #[inline(always)]
    pub fn tdcen(&mut self) -> TDCEN_W {
        TDCEN_W { w: self }
    }
    ///Bit 19 - Timer C counter enable
    #[inline(always)]
    pub fn tccen(&mut self) -> TCCEN_W {
        TCCEN_W { w: self }
    }
    ///Bit 18 - Timer B counter enable
    #[inline(always)]
    pub fn tbcen(&mut self) -> TBCEN_W {
        TBCEN_W { w: self }
    }
    ///Bit 17 - Timer A counter enable
    #[inline(always)]
    pub fn tacen(&mut self) -> TACEN_W {
        TACEN_W { w: self }
    }
    ///Bit 16 - Master Counter enable
    #[inline(always)]
    pub fn mcen(&mut self) -> MCEN_W {
        MCEN_W { w: self }
    }
    ///Bits 14:15 - Synchronization source
    #[inline(always)]
    pub fn sync_src(&mut self) -> SYNC_SRC_W {
        SYNC_SRC_W { w: self }
    }
    ///Bits 12:13 - Synchronization output
    #[inline(always)]
    pub fn sync_out(&mut self) -> SYNC_OUT_W {
        SYNC_OUT_W { w: self }
    }
    ///Bit 11 - Synchronization Starts Master
    #[inline(always)]
    pub fn syncstrtm(&mut self) -> SYNCSTRTM_W {
        SYNCSTRTM_W { w: self }
    }
    ///Bit 10 - Synchronization Resets Master
    #[inline(always)]
    pub fn syncrstm(&mut self) -> SYNCRSTM_W {
        SYNCRSTM_W { w: self }
    }
    ///Bits 8:9 - synchronization input
    #[inline(always)]
    pub fn sync_in(&mut self) -> SYNC_IN_W {
        SYNC_IN_W { w: self }
    }
    ///Bits 6:7 - Interleaved mode
    #[inline(always)]
    pub fn intlvd(&mut self) -> INTLVD_W {
        INTLVD_W { w: self }
    }
    ///Bit 5 - Half mode enable
    #[inline(always)]
    pub fn half(&mut self) -> HALF_W {
        HALF_W { w: self }
    }
    ///Bit 4 - Master Re-triggerable mode
    #[inline(always)]
    pub fn retrig(&mut self) -> RETRIG_W {
        RETRIG_W { w: self }
    }
    ///Bit 3 - Master Continuous mode
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    ///Bits 0:2 - HRTIM Master Clock prescaler
    #[inline(always)]
    pub fn ck_psc(&mut self) -> CK_PSC_W {
        CK_PSC_W { w: self }
    }
}
