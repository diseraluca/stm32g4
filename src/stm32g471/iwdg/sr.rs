#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `WVU`"]
pub type WVU_R = crate::R<bool, bool>;
#[doc = "Reader of field `RVU`"]
pub type RVU_R = crate::R<bool, bool>;
#[doc = "Reader of field `PVU`"]
pub type PVU_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - Watchdog counter window update"]
    #[inline(always)]
    pub fn wvu(&self) -> WVU_R {
        WVU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Watchdog counter reload update"]
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Watchdog prescaler value update"]
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 0x01) != 0)
    }
}
