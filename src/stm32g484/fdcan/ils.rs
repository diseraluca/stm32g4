///Reader of register ILS
pub type R = crate::R<u32, super::ILS>;
///Writer for register ILS
pub type W = crate::W<u32, super::ILS>;
///Register ILS `reset()`'s with value 0
impl crate::ResetValue for super::ILS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `RF0NL`
pub type RF0NL_R = crate::R<bool, bool>;
///Write proxy for field `RF0NL`
pub struct RF0NL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0NL_W<'a> {
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
///Reader of field `RF0WL`
pub type RF0WL_R = crate::R<bool, bool>;
///Write proxy for field `RF0WL`
pub struct RF0WL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0WL_W<'a> {
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
///Reader of field `RF0FL`
pub type RF0FL_R = crate::R<bool, bool>;
///Write proxy for field `RF0FL`
pub struct RF0FL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0FL_W<'a> {
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
///Reader of field `RF0LL`
pub type RF0LL_R = crate::R<bool, bool>;
///Write proxy for field `RF0LL`
pub struct RF0LL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0LL_W<'a> {
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
///Reader of field `RF1NL`
pub type RF1NL_R = crate::R<bool, bool>;
///Write proxy for field `RF1NL`
pub struct RF1NL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1NL_W<'a> {
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
///Reader of field `RF1WL`
pub type RF1WL_R = crate::R<bool, bool>;
///Write proxy for field `RF1WL`
pub struct RF1WL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1WL_W<'a> {
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
///Reader of field `RF1FL`
pub type RF1FL_R = crate::R<bool, bool>;
///Write proxy for field `RF1FL`
pub struct RF1FL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1FL_W<'a> {
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
///Reader of field `RF1LL`
pub type RF1LL_R = crate::R<bool, bool>;
///Write proxy for field `RF1LL`
pub struct RF1LL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1LL_W<'a> {
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
///Reader of field `HPML`
pub type HPML_R = crate::R<bool, bool>;
///Write proxy for field `HPML`
pub struct HPML_W<'a> {
    w: &'a mut W,
}
impl<'a> HPML_W<'a> {
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
///Reader of field `TCL`
pub type TCL_R = crate::R<bool, bool>;
///Write proxy for field `TCL`
pub struct TCL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCL_W<'a> {
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
///Reader of field `TCFL`
pub type TCFL_R = crate::R<bool, bool>;
///Write proxy for field `TCFL`
pub struct TCFL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCFL_W<'a> {
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
///Reader of field `TFEL`
pub type TFEL_R = crate::R<bool, bool>;
///Write proxy for field `TFEL`
pub struct TFEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TFEL_W<'a> {
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
///Reader of field `TEFNL`
pub type TEFNL_R = crate::R<bool, bool>;
///Write proxy for field `TEFNL`
pub struct TEFNL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFNL_W<'a> {
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
///Reader of field `TEFWL`
pub type TEFWL_R = crate::R<bool, bool>;
///Write proxy for field `TEFWL`
pub struct TEFWL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFWL_W<'a> {
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
///Reader of field `TEFFL`
pub type TEFFL_R = crate::R<bool, bool>;
///Write proxy for field `TEFFL`
pub struct TEFFL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFFL_W<'a> {
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
///Reader of field `TEFLL`
pub type TEFLL_R = crate::R<bool, bool>;
///Write proxy for field `TEFLL`
pub struct TEFLL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFLL_W<'a> {
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
///Reader of field `TSWL`
pub type TSWL_R = crate::R<bool, bool>;
///Write proxy for field `TSWL`
pub struct TSWL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSWL_W<'a> {
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
///Reader of field `MRAFL`
pub type MRAFL_R = crate::R<bool, bool>;
///Write proxy for field `MRAFL`
pub struct MRAFL_W<'a> {
    w: &'a mut W,
}
impl<'a> MRAFL_W<'a> {
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
///Reader of field `TOOL`
pub type TOOL_R = crate::R<bool, bool>;
///Write proxy for field `TOOL`
pub struct TOOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOOL_W<'a> {
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
///Reader of field `DRXL`
pub type DRXL_R = crate::R<bool, bool>;
///Write proxy for field `DRXL`
pub struct DRXL_W<'a> {
    w: &'a mut W,
}
impl<'a> DRXL_W<'a> {
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
///Reader of field `BECL`
pub type BECL_R = crate::R<bool, bool>;
///Write proxy for field `BECL`
pub struct BECL_W<'a> {
    w: &'a mut W,
}
impl<'a> BECL_W<'a> {
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
///Reader of field `BEUL`
pub type BEUL_R = crate::R<bool, bool>;
///Write proxy for field `BEUL`
pub struct BEUL_W<'a> {
    w: &'a mut W,
}
impl<'a> BEUL_W<'a> {
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
///Reader of field `ELOL`
pub type ELOL_R = crate::R<bool, bool>;
///Write proxy for field `ELOL`
pub struct ELOL_W<'a> {
    w: &'a mut W,
}
impl<'a> ELOL_W<'a> {
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
///Reader of field `EPL`
pub type EPL_R = crate::R<bool, bool>;
///Write proxy for field `EPL`
pub struct EPL_W<'a> {
    w: &'a mut W,
}
impl<'a> EPL_W<'a> {
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
///Reader of field `EWL`
pub type EWL_R = crate::R<bool, bool>;
///Write proxy for field `EWL`
pub struct EWL_W<'a> {
    w: &'a mut W,
}
impl<'a> EWL_W<'a> {
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
///Reader of field `BOL`
pub type BOL_R = crate::R<bool, bool>;
///Write proxy for field `BOL`
pub struct BOL_W<'a> {
    w: &'a mut W,
}
impl<'a> BOL_W<'a> {
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
///Reader of field `WDIL`
pub type WDIL_R = crate::R<bool, bool>;
///Write proxy for field `WDIL`
pub struct WDIL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDIL_W<'a> {
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
///Reader of field `PEAL`
pub type PEAL_R = crate::R<bool, bool>;
///Write proxy for field `PEAL`
pub struct PEAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAL_W<'a> {
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
///Reader of field `PEDL`
pub type PEDL_R = crate::R<bool, bool>;
///Write proxy for field `PEDL`
pub struct PEDL_W<'a> {
    w: &'a mut W,
}
impl<'a> PEDL_W<'a> {
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
///Reader of field `ARAL`
pub type ARAL_R = crate::R<bool, bool>;
///Write proxy for field `ARAL`
pub struct ARAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ARAL_W<'a> {
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
    ///Bit 0 - RF0NL
    #[inline(always)]
    pub fn rf0nl(&self) -> RF0NL_R {
        RF0NL_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - RF0WL
    #[inline(always)]
    pub fn rf0wl(&self) -> RF0WL_R {
        RF0WL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - RF0FL
    #[inline(always)]
    pub fn rf0fl(&self) -> RF0FL_R {
        RF0FL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - RF0LL
    #[inline(always)]
    pub fn rf0ll(&self) -> RF0LL_R {
        RF0LL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - RF1NL
    #[inline(always)]
    pub fn rf1nl(&self) -> RF1NL_R {
        RF1NL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - RF1WL
    #[inline(always)]
    pub fn rf1wl(&self) -> RF1WL_R {
        RF1WL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - RF1FL
    #[inline(always)]
    pub fn rf1fl(&self) -> RF1FL_R {
        RF1FL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - RF1LL
    #[inline(always)]
    pub fn rf1ll(&self) -> RF1LL_R {
        RF1LL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - HPML
    #[inline(always)]
    pub fn hpml(&self) -> HPML_R {
        HPML_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - TCL
    #[inline(always)]
    pub fn tcl(&self) -> TCL_R {
        TCL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - TCFL
    #[inline(always)]
    pub fn tcfl(&self) -> TCFL_R {
        TCFL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - TFEL
    #[inline(always)]
    pub fn tfel(&self) -> TFEL_R {
        TFEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - TEFNL
    #[inline(always)]
    pub fn tefnl(&self) -> TEFNL_R {
        TEFNL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - TEFWL
    #[inline(always)]
    pub fn tefwl(&self) -> TEFWL_R {
        TEFWL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - TEFFL
    #[inline(always)]
    pub fn teffl(&self) -> TEFFL_R {
        TEFFL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - TEFLL
    #[inline(always)]
    pub fn tefll(&self) -> TEFLL_R {
        TEFLL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - TSWL
    #[inline(always)]
    pub fn tswl(&self) -> TSWL_R {
        TSWL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - MRAFL
    #[inline(always)]
    pub fn mrafl(&self) -> MRAFL_R {
        MRAFL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - TOOL
    #[inline(always)]
    pub fn tool(&self) -> TOOL_R {
        TOOL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - DRXL
    #[inline(always)]
    pub fn drxl(&self) -> DRXL_R {
        DRXL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - BECL
    #[inline(always)]
    pub fn becl(&self) -> BECL_R {
        BECL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - BEUL
    #[inline(always)]
    pub fn beul(&self) -> BEUL_R {
        BEUL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - ELOL
    #[inline(always)]
    pub fn elol(&self) -> ELOL_R {
        ELOL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - EPL
    #[inline(always)]
    pub fn epl(&self) -> EPL_R {
        EPL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - EWL
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - BOL
    #[inline(always)]
    pub fn bol(&self) -> BOL_R {
        BOL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - WDIL
    #[inline(always)]
    pub fn wdil(&self) -> WDIL_R {
        WDIL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - PEAL
    #[inline(always)]
    pub fn peal(&self) -> PEAL_R {
        PEAL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 28 - PEDL
    #[inline(always)]
    pub fn pedl(&self) -> PEDL_R {
        PEDL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - ARAL
    #[inline(always)]
    pub fn aral(&self) -> ARAL_R {
        ARAL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - RF0NL
    #[inline(always)]
    pub fn rf0nl(&mut self) -> RF0NL_W {
        RF0NL_W { w: self }
    }
    ///Bit 1 - RF0WL
    #[inline(always)]
    pub fn rf0wl(&mut self) -> RF0WL_W {
        RF0WL_W { w: self }
    }
    ///Bit 2 - RF0FL
    #[inline(always)]
    pub fn rf0fl(&mut self) -> RF0FL_W {
        RF0FL_W { w: self }
    }
    ///Bit 3 - RF0LL
    #[inline(always)]
    pub fn rf0ll(&mut self) -> RF0LL_W {
        RF0LL_W { w: self }
    }
    ///Bit 4 - RF1NL
    #[inline(always)]
    pub fn rf1nl(&mut self) -> RF1NL_W {
        RF1NL_W { w: self }
    }
    ///Bit 5 - RF1WL
    #[inline(always)]
    pub fn rf1wl(&mut self) -> RF1WL_W {
        RF1WL_W { w: self }
    }
    ///Bit 6 - RF1FL
    #[inline(always)]
    pub fn rf1fl(&mut self) -> RF1FL_W {
        RF1FL_W { w: self }
    }
    ///Bit 7 - RF1LL
    #[inline(always)]
    pub fn rf1ll(&mut self) -> RF1LL_W {
        RF1LL_W { w: self }
    }
    ///Bit 8 - HPML
    #[inline(always)]
    pub fn hpml(&mut self) -> HPML_W {
        HPML_W { w: self }
    }
    ///Bit 9 - TCL
    #[inline(always)]
    pub fn tcl(&mut self) -> TCL_W {
        TCL_W { w: self }
    }
    ///Bit 10 - TCFL
    #[inline(always)]
    pub fn tcfl(&mut self) -> TCFL_W {
        TCFL_W { w: self }
    }
    ///Bit 11 - TFEL
    #[inline(always)]
    pub fn tfel(&mut self) -> TFEL_W {
        TFEL_W { w: self }
    }
    ///Bit 12 - TEFNL
    #[inline(always)]
    pub fn tefnl(&mut self) -> TEFNL_W {
        TEFNL_W { w: self }
    }
    ///Bit 13 - TEFWL
    #[inline(always)]
    pub fn tefwl(&mut self) -> TEFWL_W {
        TEFWL_W { w: self }
    }
    ///Bit 14 - TEFFL
    #[inline(always)]
    pub fn teffl(&mut self) -> TEFFL_W {
        TEFFL_W { w: self }
    }
    ///Bit 15 - TEFLL
    #[inline(always)]
    pub fn tefll(&mut self) -> TEFLL_W {
        TEFLL_W { w: self }
    }
    ///Bit 16 - TSWL
    #[inline(always)]
    pub fn tswl(&mut self) -> TSWL_W {
        TSWL_W { w: self }
    }
    ///Bit 17 - MRAFL
    #[inline(always)]
    pub fn mrafl(&mut self) -> MRAFL_W {
        MRAFL_W { w: self }
    }
    ///Bit 18 - TOOL
    #[inline(always)]
    pub fn tool(&mut self) -> TOOL_W {
        TOOL_W { w: self }
    }
    ///Bit 19 - DRXL
    #[inline(always)]
    pub fn drxl(&mut self) -> DRXL_W {
        DRXL_W { w: self }
    }
    ///Bit 20 - BECL
    #[inline(always)]
    pub fn becl(&mut self) -> BECL_W {
        BECL_W { w: self }
    }
    ///Bit 21 - BEUL
    #[inline(always)]
    pub fn beul(&mut self) -> BEUL_W {
        BEUL_W { w: self }
    }
    ///Bit 22 - ELOL
    #[inline(always)]
    pub fn elol(&mut self) -> ELOL_W {
        ELOL_W { w: self }
    }
    ///Bit 23 - EPL
    #[inline(always)]
    pub fn epl(&mut self) -> EPL_W {
        EPL_W { w: self }
    }
    ///Bit 24 - EWL
    #[inline(always)]
    pub fn ewl(&mut self) -> EWL_W {
        EWL_W { w: self }
    }
    ///Bit 25 - BOL
    #[inline(always)]
    pub fn bol(&mut self) -> BOL_W {
        BOL_W { w: self }
    }
    ///Bit 26 - WDIL
    #[inline(always)]
    pub fn wdil(&mut self) -> WDIL_W {
        WDIL_W { w: self }
    }
    ///Bit 27 - PEAL
    #[inline(always)]
    pub fn peal(&mut self) -> PEAL_W {
        PEAL_W { w: self }
    }
    ///Bit 28 - PEDL
    #[inline(always)]
    pub fn pedl(&mut self) -> PEDL_W {
        PEDL_W { w: self }
    }
    ///Bit 29 - ARAL
    #[inline(always)]
    pub fn aral(&mut self) -> ARAL_W {
        ARAL_W { w: self }
    }
}
