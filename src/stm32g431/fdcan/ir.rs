///Reader of register IR
pub type R = crate::R<u32, super::IR>;
///Writer for register IR
pub type W = crate::W<u32, super::IR>;
///Register IR `reset()`'s with value 0
impl crate::ResetValue for super::IR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `RF0N`
pub type RF0N_R = crate::R<bool, bool>;
///Write proxy for field `RF0N`
pub struct RF0N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0N_W<'a> {
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
///Reader of field `RF0W`
pub type RF0W_R = crate::R<bool, bool>;
///Write proxy for field `RF0W`
pub struct RF0W_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0W_W<'a> {
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
///Reader of field `RF0F`
pub type RF0F_R = crate::R<bool, bool>;
///Write proxy for field `RF0F`
pub struct RF0F_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0F_W<'a> {
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
///Reader of field `RF0L`
pub type RF0L_R = crate::R<bool, bool>;
///Write proxy for field `RF0L`
pub struct RF0L_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0L_W<'a> {
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
///Reader of field `RF1N`
pub type RF1N_R = crate::R<bool, bool>;
///Write proxy for field `RF1N`
pub struct RF1N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1N_W<'a> {
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
///Reader of field `RF1W`
pub type RF1W_R = crate::R<bool, bool>;
///Write proxy for field `RF1W`
pub struct RF1W_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1W_W<'a> {
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
///Reader of field `RF1F`
pub type RF1F_R = crate::R<bool, bool>;
///Write proxy for field `RF1F`
pub struct RF1F_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1F_W<'a> {
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
///Reader of field `RF1L`
pub type RF1L_R = crate::R<bool, bool>;
///Write proxy for field `RF1L`
pub struct RF1L_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1L_W<'a> {
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
///Reader of field `HPM`
pub type HPM_R = crate::R<bool, bool>;
///Write proxy for field `HPM`
pub struct HPM_W<'a> {
    w: &'a mut W,
}
impl<'a> HPM_W<'a> {
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
///Reader of field `TC`
pub type TC_R = crate::R<bool, bool>;
///Write proxy for field `TC`
pub struct TC_W<'a> {
    w: &'a mut W,
}
impl<'a> TC_W<'a> {
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
///Reader of field `TCF`
pub type TCF_R = crate::R<bool, bool>;
///Write proxy for field `TCF`
pub struct TCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF_W<'a> {
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
///Reader of field `TFE`
pub type TFE_R = crate::R<bool, bool>;
///Write proxy for field `TFE`
pub struct TFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFE_W<'a> {
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
///Reader of field `TEFN`
pub type TEFN_R = crate::R<bool, bool>;
///Write proxy for field `TEFN`
pub struct TEFN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFN_W<'a> {
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
///Reader of field `TEFW`
pub type TEFW_R = crate::R<bool, bool>;
///Write proxy for field `TEFW`
pub struct TEFW_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFW_W<'a> {
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
///Reader of field `TEFF`
pub type TEFF_R = crate::R<bool, bool>;
///Write proxy for field `TEFF`
pub struct TEFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFF_W<'a> {
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
///Reader of field `TEFL`
pub type TEFL_R = crate::R<bool, bool>;
///Write proxy for field `TEFL`
pub struct TEFL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFL_W<'a> {
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
///Reader of field `TSW`
pub type TSW_R = crate::R<bool, bool>;
///Write proxy for field `TSW`
pub struct TSW_W<'a> {
    w: &'a mut W,
}
impl<'a> TSW_W<'a> {
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
///Reader of field `MRAF`
pub type MRAF_R = crate::R<bool, bool>;
///Write proxy for field `MRAF`
pub struct MRAF_W<'a> {
    w: &'a mut W,
}
impl<'a> MRAF_W<'a> {
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
///Reader of field `TOO`
pub type TOO_R = crate::R<bool, bool>;
///Write proxy for field `TOO`
pub struct TOO_W<'a> {
    w: &'a mut W,
}
impl<'a> TOO_W<'a> {
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
///Reader of field `DRX`
pub type DRX_R = crate::R<bool, bool>;
///Write proxy for field `DRX`
pub struct DRX_W<'a> {
    w: &'a mut W,
}
impl<'a> DRX_W<'a> {
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
///Reader of field `ELO`
pub type ELO_R = crate::R<bool, bool>;
///Write proxy for field `ELO`
pub struct ELO_W<'a> {
    w: &'a mut W,
}
impl<'a> ELO_W<'a> {
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
///Reader of field `EP`
pub type EP_R = crate::R<bool, bool>;
///Write proxy for field `EP`
pub struct EP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
///Reader of field `EW`
pub type EW_R = crate::R<bool, bool>;
///Write proxy for field `EW`
pub struct EW_W<'a> {
    w: &'a mut W,
}
impl<'a> EW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
///Reader of field `BO`
pub type BO_R = crate::R<bool, bool>;
///Write proxy for field `BO`
pub struct BO_W<'a> {
    w: &'a mut W,
}
impl<'a> BO_W<'a> {
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
///Reader of field `WDI`
pub type WDI_R = crate::R<bool, bool>;
///Write proxy for field `WDI`
pub struct WDI_W<'a> {
    w: &'a mut W,
}
impl<'a> WDI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
///Reader of field `PEA`
pub type PEA_R = crate::R<bool, bool>;
///Write proxy for field `PEA`
pub struct PEA_W<'a> {
    w: &'a mut W,
}
impl<'a> PEA_W<'a> {
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
///Reader of field `PED`
pub type PED_R = crate::R<bool, bool>;
///Write proxy for field `PED`
pub struct PED_W<'a> {
    w: &'a mut W,
}
impl<'a> PED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
///Reader of field `ARA`
pub type ARA_R = crate::R<bool, bool>;
///Write proxy for field `ARA`
pub struct ARA_W<'a> {
    w: &'a mut W,
}
impl<'a> ARA_W<'a> {
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
impl R {
    ///Bit 0 - RF0N
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - RF0W
    #[inline(always)]
    pub fn rf0w(&self) -> RF0W_R {
        RF0W_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - RF0F
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - RF0L
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - RF1N
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - RF1W
    #[inline(always)]
    pub fn rf1w(&self) -> RF1W_R {
        RF1W_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - RF1F
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - RF1L
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - HPM
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - TC
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - TCF
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - TFE
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - TEFN
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - TEFW
    #[inline(always)]
    pub fn tefw(&self) -> TEFW_R {
        TEFW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - TEFF
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - TEFL
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - TSW
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - MRAF
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - TOO
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - DRX
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 22 - ELO
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - EP
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - EW
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - BO
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - WDI
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - PEA
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 28 - PED
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - ARA
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - RF0N
    #[inline(always)]
    pub fn rf0n(&mut self) -> RF0N_W {
        RF0N_W { w: self }
    }
    ///Bit 1 - RF0W
    #[inline(always)]
    pub fn rf0w(&mut self) -> RF0W_W {
        RF0W_W { w: self }
    }
    ///Bit 2 - RF0F
    #[inline(always)]
    pub fn rf0f(&mut self) -> RF0F_W {
        RF0F_W { w: self }
    }
    ///Bit 3 - RF0L
    #[inline(always)]
    pub fn rf0l(&mut self) -> RF0L_W {
        RF0L_W { w: self }
    }
    ///Bit 4 - RF1N
    #[inline(always)]
    pub fn rf1n(&mut self) -> RF1N_W {
        RF1N_W { w: self }
    }
    ///Bit 5 - RF1W
    #[inline(always)]
    pub fn rf1w(&mut self) -> RF1W_W {
        RF1W_W { w: self }
    }
    ///Bit 6 - RF1F
    #[inline(always)]
    pub fn rf1f(&mut self) -> RF1F_W {
        RF1F_W { w: self }
    }
    ///Bit 7 - RF1L
    #[inline(always)]
    pub fn rf1l(&mut self) -> RF1L_W {
        RF1L_W { w: self }
    }
    ///Bit 8 - HPM
    #[inline(always)]
    pub fn hpm(&mut self) -> HPM_W {
        HPM_W { w: self }
    }
    ///Bit 9 - TC
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W {
        TC_W { w: self }
    }
    ///Bit 10 - TCF
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W {
        TCF_W { w: self }
    }
    ///Bit 11 - TFE
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W {
        TFE_W { w: self }
    }
    ///Bit 12 - TEFN
    #[inline(always)]
    pub fn tefn(&mut self) -> TEFN_W {
        TEFN_W { w: self }
    }
    ///Bit 13 - TEFW
    #[inline(always)]
    pub fn tefw(&mut self) -> TEFW_W {
        TEFW_W { w: self }
    }
    ///Bit 14 - TEFF
    #[inline(always)]
    pub fn teff(&mut self) -> TEFF_W {
        TEFF_W { w: self }
    }
    ///Bit 15 - TEFL
    #[inline(always)]
    pub fn tefl(&mut self) -> TEFL_W {
        TEFL_W { w: self }
    }
    ///Bit 16 - TSW
    #[inline(always)]
    pub fn tsw(&mut self) -> TSW_W {
        TSW_W { w: self }
    }
    ///Bit 17 - MRAF
    #[inline(always)]
    pub fn mraf(&mut self) -> MRAF_W {
        MRAF_W { w: self }
    }
    ///Bit 18 - TOO
    #[inline(always)]
    pub fn too(&mut self) -> TOO_W {
        TOO_W { w: self }
    }
    ///Bit 19 - DRX
    #[inline(always)]
    pub fn drx(&mut self) -> DRX_W {
        DRX_W { w: self }
    }
    ///Bit 22 - ELO
    #[inline(always)]
    pub fn elo(&mut self) -> ELO_W {
        ELO_W { w: self }
    }
    ///Bit 23 - EP
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W {
        EP_W { w: self }
    }
    ///Bit 24 - EW
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W {
        EW_W { w: self }
    }
    ///Bit 25 - BO
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W {
        BO_W { w: self }
    }
    ///Bit 26 - WDI
    #[inline(always)]
    pub fn wdi(&mut self) -> WDI_W {
        WDI_W { w: self }
    }
    ///Bit 27 - PEA
    #[inline(always)]
    pub fn pea(&mut self) -> PEA_W {
        PEA_W { w: self }
    }
    ///Bit 28 - PED
    #[inline(always)]
    pub fn ped(&mut self) -> PED_W {
        PED_W { w: self }
    }
    ///Bit 29 - ARA
    #[inline(always)]
    pub fn ara(&mut self) -> ARA_W {
        ARA_W { w: self }
    }
}
