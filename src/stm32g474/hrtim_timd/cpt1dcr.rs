///Reader of register CPT1DCR
pub type R = crate::R<u32, super::CPT1DCR>;
///Writer for register CPT1DCR
pub type W = crate::W<u32, super::CPT1DCR>;
///Register CPT1DCR `reset()`'s with value 0
impl crate::ResetValue for super::CPT1DCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TECMP2`
pub type TECMP2_R = crate::R<bool, bool>;
///Write proxy for field `TECMP2`
pub struct TECMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TECMP2_W<'a> {
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
///Reader of field `TECMP1`
pub type TECMP1_R = crate::R<bool, bool>;
///Write proxy for field `TECMP1`
pub struct TECMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TECMP1_W<'a> {
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
///Reader of field `TE1RST`
pub type TE1RST_R = crate::R<bool, bool>;
///Write proxy for field `TE1RST`
pub struct TE1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TE1RST_W<'a> {
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
///Reader of field `TE1SET`
pub type TE1SET_R = crate::R<bool, bool>;
///Write proxy for field `TE1SET`
pub struct TE1SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TE1SET_W<'a> {
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
///Reader of field `TFCMP2`
pub type TFCMP2_R = crate::R<bool, bool>;
///Write proxy for field `TFCMP2`
pub struct TFCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TFCMP2_W<'a> {
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
///Reader of field `TFCMP1`
pub type TFCMP1_R = crate::R<bool, bool>;
///Write proxy for field `TFCMP1`
pub struct TFCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TFCMP1_W<'a> {
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
///Reader of field `TF1RST`
pub type TF1RST_R = crate::R<bool, bool>;
///Write proxy for field `TF1RST`
pub struct TF1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TF1RST_W<'a> {
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
///Reader of field `TF1SET`
pub type TF1SET_R = crate::R<bool, bool>;
///Write proxy for field `TF1SET`
pub struct TF1SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TF1SET_W<'a> {
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
///Reader of field `TCCMP2`
pub type TCCMP2_R = crate::R<bool, bool>;
///Write proxy for field `TCCMP2`
pub struct TCCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCMP2_W<'a> {
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
///Reader of field `TCCMP1`
pub type TCCMP1_R = crate::R<bool, bool>;
///Write proxy for field `TCCMP1`
pub struct TCCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCMP1_W<'a> {
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
///Reader of field `TC1RST`
pub type TC1RST_R = crate::R<bool, bool>;
///Write proxy for field `TC1RST`
pub struct TC1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1RST_W<'a> {
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
///Reader of field `TC1SET`
pub type TC1SET_R = crate::R<bool, bool>;
///Write proxy for field `TC1SET`
pub struct TC1SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1SET_W<'a> {
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
///Reader of field `TBCMP2`
pub type TBCMP2_R = crate::R<bool, bool>;
///Write proxy for field `TBCMP2`
pub struct TBCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCMP2_W<'a> {
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
///Reader of field `TBCMP1`
pub type TBCMP1_R = crate::R<bool, bool>;
///Write proxy for field `TBCMP1`
pub struct TBCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCMP1_W<'a> {
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
///Reader of field `TB1RST`
pub type TB1RST_R = crate::R<bool, bool>;
///Write proxy for field `TB1RST`
pub struct TB1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TB1RST_W<'a> {
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
///Reader of field `TB1SET`
pub type TB1SET_R = crate::R<bool, bool>;
///Write proxy for field `TB1SET`
pub struct TB1SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TB1SET_W<'a> {
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
///Reader of field `TACMP2`
pub type TACMP2_R = crate::R<bool, bool>;
///Write proxy for field `TACMP2`
pub struct TACMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TACMP2_W<'a> {
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
///Reader of field `TACMP1`
pub type TACMP1_R = crate::R<bool, bool>;
///Write proxy for field `TACMP1`
pub struct TACMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TACMP1_W<'a> {
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
///Reader of field `TA1RST`
pub type TA1RST_R = crate::R<bool, bool>;
///Write proxy for field `TA1RST`
pub struct TA1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TA1RST_W<'a> {
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
///Reader of field `TA1SET`
pub type TA1SET_R = crate::R<bool, bool>;
///Write proxy for field `TA1SET`
pub struct TA1SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TA1SET_W<'a> {
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
///Reader of field `EXEV10CPT`
pub type EXEV10CPT_R = crate::R<bool, bool>;
///Write proxy for field `EXEV10CPT`
pub struct EXEV10CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV10CPT_W<'a> {
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
///Reader of field `EXEV9CPT`
pub type EXEV9CPT_R = crate::R<bool, bool>;
///Write proxy for field `EXEV9CPT`
pub struct EXEV9CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV9CPT_W<'a> {
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
///Reader of field `EXEV8CPT`
pub type EXEV8CPT_R = crate::R<bool, bool>;
///Write proxy for field `EXEV8CPT`
pub struct EXEV8CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV8CPT_W<'a> {
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
///Reader of field `EXEV7CPT`
pub type EXEV7CPT_R = crate::R<bool, bool>;
///Write proxy for field `EXEV7CPT`
pub struct EXEV7CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV7CPT_W<'a> {
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
///Reader of field `EXEV6CPT`
pub type EXEV6CPT_R = crate::R<bool, bool>;
///Write proxy for field `EXEV6CPT`
pub struct EXEV6CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV6CPT_W<'a> {
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
///Reader of field `EXEV5CPT`
pub type EXEV5CPT_R = crate::R<bool, bool>;
///Write proxy for field `EXEV5CPT`
pub struct EXEV5CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV5CPT_W<'a> {
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
///Reader of field `EXEV4CPT`
pub type EXEV4CPT_R = crate::R<bool, bool>;
///Write proxy for field `EXEV4CPT`
pub struct EXEV4CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV4CPT_W<'a> {
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
///Reader of field `EXEV3CPT`
pub type EXEV3CPT_R = crate::R<bool, bool>;
///Write proxy for field `EXEV3CPT`
pub struct EXEV3CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV3CPT_W<'a> {
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
///Reader of field `EXEV2CPT`
pub type EXEV2CPT_R = crate::R<bool, bool>;
///Write proxy for field `EXEV2CPT`
pub struct EXEV2CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV2CPT_W<'a> {
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
///Reader of field `EXEV1CPT`
pub type EXEV1CPT_R = crate::R<bool, bool>;
///Write proxy for field `EXEV1CPT`
pub struct EXEV1CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV1CPT_W<'a> {
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
///Reader of field `UDPCPT`
pub type UDPCPT_R = crate::R<bool, bool>;
///Write proxy for field `UDPCPT`
pub struct UDPCPT_W<'a> {
    w: &'a mut W,
}
impl<'a> UDPCPT_W<'a> {
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
///Reader of field `SWCPT`
pub type SWCPT_R = crate::R<bool, bool>;
///Write proxy for field `SWCPT`
pub struct SWCPT_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCPT_W<'a> {
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
    ///Bit 31 - Timer E Compare 2
    #[inline(always)]
    pub fn tecmp2(&self) -> TECMP2_R {
        TECMP2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 30 - Timer E Compare 1
    #[inline(always)]
    pub fn tecmp1(&self) -> TECMP1_R {
        TECMP1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 29 - Timer E output 1 Reset
    #[inline(always)]
    pub fn te1rst(&self) -> TE1RST_R {
        TE1RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 28 - Timer E output 1 Set
    #[inline(always)]
    pub fn te1set(&self) -> TE1SET_R {
        TE1SET_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 27 - TFCMP2
    #[inline(always)]
    pub fn tfcmp2(&self) -> TFCMP2_R {
        TFCMP2_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 26 - TFCMP1
    #[inline(always)]
    pub fn tfcmp1(&self) -> TFCMP1_R {
        TFCMP1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 25 - TF1RST
    #[inline(always)]
    pub fn tf1rst(&self) -> TF1RST_R {
        TF1RST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 24 - TF1SET
    #[inline(always)]
    pub fn tf1set(&self) -> TF1SET_R {
        TF1SET_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 23 - Timer C Compare 2
    #[inline(always)]
    pub fn tccmp2(&self) -> TCCMP2_R {
        TCCMP2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 22 - Timer C Compare 1
    #[inline(always)]
    pub fn tccmp1(&self) -> TCCMP1_R {
        TCCMP1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 21 - Timer C output 1 Reset
    #[inline(always)]
    pub fn tc1rst(&self) -> TC1RST_R {
        TC1RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 20 - Timer C output 1 Set
    #[inline(always)]
    pub fn tc1set(&self) -> TC1SET_R {
        TC1SET_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 19 - Timer B Compare 2
    #[inline(always)]
    pub fn tbcmp2(&self) -> TBCMP2_R {
        TBCMP2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 18 - Timer B Compare 1
    #[inline(always)]
    pub fn tbcmp1(&self) -> TBCMP1_R {
        TBCMP1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - Timer B output 1 Reset
    #[inline(always)]
    pub fn tb1rst(&self) -> TB1RST_R {
        TB1RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - Timer B output 1 Set
    #[inline(always)]
    pub fn tb1set(&self) -> TB1SET_R {
        TB1SET_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - Timer A Compare 2
    #[inline(always)]
    pub fn tacmp2(&self) -> TACMP2_R {
        TACMP2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Timer A Compare 1
    #[inline(always)]
    pub fn tacmp1(&self) -> TACMP1_R {
        TACMP1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Timer A output 1 Reset
    #[inline(always)]
    pub fn ta1rst(&self) -> TA1RST_R {
        TA1RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Timer A output 1 Set
    #[inline(always)]
    pub fn ta1set(&self) -> TA1SET_R {
        TA1SET_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - External Event 10 Capture
    #[inline(always)]
    pub fn exev10cpt(&self) -> EXEV10CPT_R {
        EXEV10CPT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - External Event 9 Capture
    #[inline(always)]
    pub fn exev9cpt(&self) -> EXEV9CPT_R {
        EXEV9CPT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - External Event 8 Capture
    #[inline(always)]
    pub fn exev8cpt(&self) -> EXEV8CPT_R {
        EXEV8CPT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - External Event 7 Capture
    #[inline(always)]
    pub fn exev7cpt(&self) -> EXEV7CPT_R {
        EXEV7CPT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - External Event 6 Capture
    #[inline(always)]
    pub fn exev6cpt(&self) -> EXEV6CPT_R {
        EXEV6CPT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - External Event 5 Capture
    #[inline(always)]
    pub fn exev5cpt(&self) -> EXEV5CPT_R {
        EXEV5CPT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - External Event 4 Capture
    #[inline(always)]
    pub fn exev4cpt(&self) -> EXEV4CPT_R {
        EXEV4CPT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - External Event 3 Capture
    #[inline(always)]
    pub fn exev3cpt(&self) -> EXEV3CPT_R {
        EXEV3CPT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - External Event 2 Capture
    #[inline(always)]
    pub fn exev2cpt(&self) -> EXEV2CPT_R {
        EXEV2CPT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - External Event 1 Capture
    #[inline(always)]
    pub fn exev1cpt(&self) -> EXEV1CPT_R {
        EXEV1CPT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Update Capture
    #[inline(always)]
    pub fn udpcpt(&self) -> UDPCPT_R {
        UDPCPT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Software Capture
    #[inline(always)]
    pub fn swcpt(&self) -> SWCPT_R {
        SWCPT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 31 - Timer E Compare 2
    #[inline(always)]
    pub fn tecmp2(&mut self) -> TECMP2_W {
        TECMP2_W { w: self }
    }
    ///Bit 30 - Timer E Compare 1
    #[inline(always)]
    pub fn tecmp1(&mut self) -> TECMP1_W {
        TECMP1_W { w: self }
    }
    ///Bit 29 - Timer E output 1 Reset
    #[inline(always)]
    pub fn te1rst(&mut self) -> TE1RST_W {
        TE1RST_W { w: self }
    }
    ///Bit 28 - Timer E output 1 Set
    #[inline(always)]
    pub fn te1set(&mut self) -> TE1SET_W {
        TE1SET_W { w: self }
    }
    ///Bit 27 - TFCMP2
    #[inline(always)]
    pub fn tfcmp2(&mut self) -> TFCMP2_W {
        TFCMP2_W { w: self }
    }
    ///Bit 26 - TFCMP1
    #[inline(always)]
    pub fn tfcmp1(&mut self) -> TFCMP1_W {
        TFCMP1_W { w: self }
    }
    ///Bit 25 - TF1RST
    #[inline(always)]
    pub fn tf1rst(&mut self) -> TF1RST_W {
        TF1RST_W { w: self }
    }
    ///Bit 24 - TF1SET
    #[inline(always)]
    pub fn tf1set(&mut self) -> TF1SET_W {
        TF1SET_W { w: self }
    }
    ///Bit 23 - Timer C Compare 2
    #[inline(always)]
    pub fn tccmp2(&mut self) -> TCCMP2_W {
        TCCMP2_W { w: self }
    }
    ///Bit 22 - Timer C Compare 1
    #[inline(always)]
    pub fn tccmp1(&mut self) -> TCCMP1_W {
        TCCMP1_W { w: self }
    }
    ///Bit 21 - Timer C output 1 Reset
    #[inline(always)]
    pub fn tc1rst(&mut self) -> TC1RST_W {
        TC1RST_W { w: self }
    }
    ///Bit 20 - Timer C output 1 Set
    #[inline(always)]
    pub fn tc1set(&mut self) -> TC1SET_W {
        TC1SET_W { w: self }
    }
    ///Bit 19 - Timer B Compare 2
    #[inline(always)]
    pub fn tbcmp2(&mut self) -> TBCMP2_W {
        TBCMP2_W { w: self }
    }
    ///Bit 18 - Timer B Compare 1
    #[inline(always)]
    pub fn tbcmp1(&mut self) -> TBCMP1_W {
        TBCMP1_W { w: self }
    }
    ///Bit 17 - Timer B output 1 Reset
    #[inline(always)]
    pub fn tb1rst(&mut self) -> TB1RST_W {
        TB1RST_W { w: self }
    }
    ///Bit 16 - Timer B output 1 Set
    #[inline(always)]
    pub fn tb1set(&mut self) -> TB1SET_W {
        TB1SET_W { w: self }
    }
    ///Bit 15 - Timer A Compare 2
    #[inline(always)]
    pub fn tacmp2(&mut self) -> TACMP2_W {
        TACMP2_W { w: self }
    }
    ///Bit 14 - Timer A Compare 1
    #[inline(always)]
    pub fn tacmp1(&mut self) -> TACMP1_W {
        TACMP1_W { w: self }
    }
    ///Bit 13 - Timer A output 1 Reset
    #[inline(always)]
    pub fn ta1rst(&mut self) -> TA1RST_W {
        TA1RST_W { w: self }
    }
    ///Bit 12 - Timer A output 1 Set
    #[inline(always)]
    pub fn ta1set(&mut self) -> TA1SET_W {
        TA1SET_W { w: self }
    }
    ///Bit 11 - External Event 10 Capture
    #[inline(always)]
    pub fn exev10cpt(&mut self) -> EXEV10CPT_W {
        EXEV10CPT_W { w: self }
    }
    ///Bit 10 - External Event 9 Capture
    #[inline(always)]
    pub fn exev9cpt(&mut self) -> EXEV9CPT_W {
        EXEV9CPT_W { w: self }
    }
    ///Bit 9 - External Event 8 Capture
    #[inline(always)]
    pub fn exev8cpt(&mut self) -> EXEV8CPT_W {
        EXEV8CPT_W { w: self }
    }
    ///Bit 8 - External Event 7 Capture
    #[inline(always)]
    pub fn exev7cpt(&mut self) -> EXEV7CPT_W {
        EXEV7CPT_W { w: self }
    }
    ///Bit 7 - External Event 6 Capture
    #[inline(always)]
    pub fn exev6cpt(&mut self) -> EXEV6CPT_W {
        EXEV6CPT_W { w: self }
    }
    ///Bit 6 - External Event 5 Capture
    #[inline(always)]
    pub fn exev5cpt(&mut self) -> EXEV5CPT_W {
        EXEV5CPT_W { w: self }
    }
    ///Bit 5 - External Event 4 Capture
    #[inline(always)]
    pub fn exev4cpt(&mut self) -> EXEV4CPT_W {
        EXEV4CPT_W { w: self }
    }
    ///Bit 4 - External Event 3 Capture
    #[inline(always)]
    pub fn exev3cpt(&mut self) -> EXEV3CPT_W {
        EXEV3CPT_W { w: self }
    }
    ///Bit 3 - External Event 2 Capture
    #[inline(always)]
    pub fn exev2cpt(&mut self) -> EXEV2CPT_W {
        EXEV2CPT_W { w: self }
    }
    ///Bit 2 - External Event 1 Capture
    #[inline(always)]
    pub fn exev1cpt(&mut self) -> EXEV1CPT_W {
        EXEV1CPT_W { w: self }
    }
    ///Bit 1 - Update Capture
    #[inline(always)]
    pub fn udpcpt(&mut self) -> UDPCPT_W {
        UDPCPT_W { w: self }
    }
    ///Bit 0 - Software Capture
    #[inline(always)]
    pub fn swcpt(&mut self) -> SWCPT_W {
        SWCPT_W { w: self }
    }
}
