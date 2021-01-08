#[doc = "Reader of register IE"]
pub type R = crate::R<u32, super::IE>;
#[doc = "Writer for register IE"]
pub type W = crate::W<u32, super::IE>;
#[doc = "Register IE `reset()`'s with value 0"]
impl crate::ResetValue for super::IE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RF0NE`"]
pub type RF0NE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF0NE`"]
pub struct RF0NE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0NE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RF0WE`"]
pub type RF0WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF0WE`"]
pub struct RF0WE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0WE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RF0FE`"]
pub type RF0FE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF0FE`"]
pub struct RF0FE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0FE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RF0LE`"]
pub type RF0LE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF0LE`"]
pub struct RF0LE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0LE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RF1NE`"]
pub type RF1NE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF1NE`"]
pub struct RF1NE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1NE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RF1WE`"]
pub type RF1WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF1WE`"]
pub struct RF1WE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1WE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RF1FE`"]
pub type RF1FE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF1FE`"]
pub struct RF1FE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1FE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RF1LE`"]
pub type RF1LE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RF1LE`"]
pub struct RF1LE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1LE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `HPME`"]
pub type HPME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPME`"]
pub struct HPME_W<'a> {
    w: &'a mut W,
}
impl<'a> HPME_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TCE`"]
pub type TCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCE`"]
pub struct TCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TCFE`"]
pub type TCFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCFE`"]
pub struct TCFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCFE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TFEE`"]
pub type TFEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFEE`"]
pub struct TFEE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFEE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TEFNE`"]
pub type TEFNE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEFNE`"]
pub struct TEFNE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFNE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TEFWE`"]
pub type TEFWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEFWE`"]
pub struct TEFWE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFWE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TEFFE`"]
pub type TEFFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEFFE`"]
pub struct TEFFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFFE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TEFLE`"]
pub type TEFLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEFLE`"]
pub struct TEFLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `TSWE`"]
pub type TSWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSWE`"]
pub struct TSWE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSWE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `MRAFE`"]
pub type MRAFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MRAFE`"]
pub struct MRAFE_W<'a> {
    w: &'a mut W,
}
impl<'a> MRAFE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `TOOE`"]
pub type TOOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOOE`"]
pub struct TOOE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOOE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `DRX`"]
pub type DRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRX`"]
pub struct DRX_W<'a> {
    w: &'a mut W,
}
impl<'a> DRX_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `BECE`"]
pub type BECE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BECE`"]
pub struct BECE_W<'a> {
    w: &'a mut W,
}
impl<'a> BECE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `BEUE`"]
pub type BEUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEUE`"]
pub struct BEUE_W<'a> {
    w: &'a mut W,
}
impl<'a> BEUE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ELOE`"]
pub type ELOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ELOE`"]
pub struct ELOE_W<'a> {
    w: &'a mut W,
}
impl<'a> ELOE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `EPE`"]
pub type EPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPE`"]
pub struct EPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `EWE`"]
pub type EWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWE`"]
pub struct EWE_W<'a> {
    w: &'a mut W,
}
impl<'a> EWE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `BOE`"]
pub type BOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOE`"]
pub struct BOE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `WDIE`"]
pub type WDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDIE`"]
pub struct WDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `PEAE`"]
pub type PEAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEAE`"]
pub struct PEAE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `PEDE`"]
pub type PEDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEDE`"]
pub struct PEDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEDE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `ARAE`"]
pub type ARAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARAE`"]
pub struct ARAE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARAE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RF0NE"]
    #[inline(always)]
    pub fn rf0ne(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RF0WE"]
    #[inline(always)]
    pub fn rf0we(&self) -> RF0WE_R {
        RF0WE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RF0FE"]
    #[inline(always)]
    pub fn rf0fe(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RF0LE"]
    #[inline(always)]
    pub fn rf0le(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RF1NE"]
    #[inline(always)]
    pub fn rf1ne(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RF1WE"]
    #[inline(always)]
    pub fn rf1we(&self) -> RF1WE_R {
        RF1WE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RF1FE"]
    #[inline(always)]
    pub fn rf1fe(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RF1LE"]
    #[inline(always)]
    pub fn rf1le(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HPME"]
    #[inline(always)]
    pub fn hpme(&self) -> HPME_R {
        HPME_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TCE"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TCFE"]
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TFEE"]
    #[inline(always)]
    pub fn tfee(&self) -> TFEE_R {
        TFEE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TEFNE"]
    #[inline(always)]
    pub fn tefne(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TEFWE"]
    #[inline(always)]
    pub fn tefwe(&self) -> TEFWE_R {
        TEFWE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TEFFE"]
    #[inline(always)]
    pub fn teffe(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TEFLE"]
    #[inline(always)]
    pub fn tefle(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TSWE"]
    #[inline(always)]
    pub fn tswe(&self) -> TSWE_R {
        TSWE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MRAFE"]
    #[inline(always)]
    pub fn mrafe(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TOOE"]
    #[inline(always)]
    pub fn tooe(&self) -> TOOE_R {
        TOOE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DRX"]
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - BECE"]
    #[inline(always)]
    pub fn bece(&self) -> BECE_R {
        BECE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BEUE"]
    #[inline(always)]
    pub fn beue(&self) -> BEUE_R {
        BEUE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ELOE"]
    #[inline(always)]
    pub fn eloe(&self) -> ELOE_R {
        ELOE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - EPE"]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - EWE"]
    #[inline(always)]
    pub fn ewe(&self) -> EWE_R {
        EWE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - BOE"]
    #[inline(always)]
    pub fn boe(&self) -> BOE_R {
        BOE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - WDIE"]
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PEAE"]
    #[inline(always)]
    pub fn peae(&self) -> PEAE_R {
        PEAE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PEDE"]
    #[inline(always)]
    pub fn pede(&self) -> PEDE_R {
        PEDE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ARAE"]
    #[inline(always)]
    pub fn arae(&self) -> ARAE_R {
        ARAE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RF0NE"]
    #[inline(always)]
    pub fn rf0ne(&mut self) -> RF0NE_W {
        RF0NE_W { w: self }
    }
    #[doc = "Bit 1 - RF0WE"]
    #[inline(always)]
    pub fn rf0we(&mut self) -> RF0WE_W {
        RF0WE_W { w: self }
    }
    #[doc = "Bit 2 - RF0FE"]
    #[inline(always)]
    pub fn rf0fe(&mut self) -> RF0FE_W {
        RF0FE_W { w: self }
    }
    #[doc = "Bit 3 - RF0LE"]
    #[inline(always)]
    pub fn rf0le(&mut self) -> RF0LE_W {
        RF0LE_W { w: self }
    }
    #[doc = "Bit 4 - RF1NE"]
    #[inline(always)]
    pub fn rf1ne(&mut self) -> RF1NE_W {
        RF1NE_W { w: self }
    }
    #[doc = "Bit 5 - RF1WE"]
    #[inline(always)]
    pub fn rf1we(&mut self) -> RF1WE_W {
        RF1WE_W { w: self }
    }
    #[doc = "Bit 6 - RF1FE"]
    #[inline(always)]
    pub fn rf1fe(&mut self) -> RF1FE_W {
        RF1FE_W { w: self }
    }
    #[doc = "Bit 7 - RF1LE"]
    #[inline(always)]
    pub fn rf1le(&mut self) -> RF1LE_W {
        RF1LE_W { w: self }
    }
    #[doc = "Bit 8 - HPME"]
    #[inline(always)]
    pub fn hpme(&mut self) -> HPME_W {
        HPME_W { w: self }
    }
    #[doc = "Bit 9 - TCE"]
    #[inline(always)]
    pub fn tce(&mut self) -> TCE_W {
        TCE_W { w: self }
    }
    #[doc = "Bit 10 - TCFE"]
    #[inline(always)]
    pub fn tcfe(&mut self) -> TCFE_W {
        TCFE_W { w: self }
    }
    #[doc = "Bit 11 - TFEE"]
    #[inline(always)]
    pub fn tfee(&mut self) -> TFEE_W {
        TFEE_W { w: self }
    }
    #[doc = "Bit 12 - TEFNE"]
    #[inline(always)]
    pub fn tefne(&mut self) -> TEFNE_W {
        TEFNE_W { w: self }
    }
    #[doc = "Bit 13 - TEFWE"]
    #[inline(always)]
    pub fn tefwe(&mut self) -> TEFWE_W {
        TEFWE_W { w: self }
    }
    #[doc = "Bit 14 - TEFFE"]
    #[inline(always)]
    pub fn teffe(&mut self) -> TEFFE_W {
        TEFFE_W { w: self }
    }
    #[doc = "Bit 15 - TEFLE"]
    #[inline(always)]
    pub fn tefle(&mut self) -> TEFLE_W {
        TEFLE_W { w: self }
    }
    #[doc = "Bit 16 - TSWE"]
    #[inline(always)]
    pub fn tswe(&mut self) -> TSWE_W {
        TSWE_W { w: self }
    }
    #[doc = "Bit 17 - MRAFE"]
    #[inline(always)]
    pub fn mrafe(&mut self) -> MRAFE_W {
        MRAFE_W { w: self }
    }
    #[doc = "Bit 18 - TOOE"]
    #[inline(always)]
    pub fn tooe(&mut self) -> TOOE_W {
        TOOE_W { w: self }
    }
    #[doc = "Bit 19 - DRX"]
    #[inline(always)]
    pub fn drx(&mut self) -> DRX_W {
        DRX_W { w: self }
    }
    #[doc = "Bit 20 - BECE"]
    #[inline(always)]
    pub fn bece(&mut self) -> BECE_W {
        BECE_W { w: self }
    }
    #[doc = "Bit 21 - BEUE"]
    #[inline(always)]
    pub fn beue(&mut self) -> BEUE_W {
        BEUE_W { w: self }
    }
    #[doc = "Bit 22 - ELOE"]
    #[inline(always)]
    pub fn eloe(&mut self) -> ELOE_W {
        ELOE_W { w: self }
    }
    #[doc = "Bit 23 - EPE"]
    #[inline(always)]
    pub fn epe(&mut self) -> EPE_W {
        EPE_W { w: self }
    }
    #[doc = "Bit 24 - EWE"]
    #[inline(always)]
    pub fn ewe(&mut self) -> EWE_W {
        EWE_W { w: self }
    }
    #[doc = "Bit 25 - BOE"]
    #[inline(always)]
    pub fn boe(&mut self) -> BOE_W {
        BOE_W { w: self }
    }
    #[doc = "Bit 26 - WDIE"]
    #[inline(always)]
    pub fn wdie(&mut self) -> WDIE_W {
        WDIE_W { w: self }
    }
    #[doc = "Bit 27 - PEAE"]
    #[inline(always)]
    pub fn peae(&mut self) -> PEAE_W {
        PEAE_W { w: self }
    }
    #[doc = "Bit 28 - PEDE"]
    #[inline(always)]
    pub fn pede(&mut self) -> PEDE_W {
        PEDE_W { w: self }
    }
    #[doc = "Bit 29 - ARAE"]
    #[inline(always)]
    pub fn arae(&mut self) -> ARAE_W {
        ARAE_W { w: self }
    }
}
