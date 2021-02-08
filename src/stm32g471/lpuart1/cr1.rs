///Reader of register CR1
pub type R = crate::R<u32, super::CR1>;
///Writer for register CR1
pub type W = crate::W<u32, super::CR1>;
///Register CR1 `reset()`'s with value 0
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `RXFFIE`
pub type RXFFIE_R = crate::R<bool, bool>;
///Write proxy for field `RXFFIE`
pub struct RXFFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFFIE_W<'a> {
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
///Reader of field `TXFEIE`
pub type TXFEIE_R = crate::R<bool, bool>;
///Write proxy for field `TXFEIE`
pub struct TXFEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFEIE_W<'a> {
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
///Reader of field `FIFOEN`
pub type FIFOEN_R = crate::R<bool, bool>;
///Write proxy for field `FIFOEN`
pub struct FIFOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOEN_W<'a> {
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
///Reader of field `M1`
pub type M1_R = crate::R<bool, bool>;
///Write proxy for field `M1`
pub struct M1_W<'a> {
    w: &'a mut W,
}
impl<'a> M1_W<'a> {
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
///Reader of field `DEAT4`
pub type DEAT4_R = crate::R<bool, bool>;
///Write proxy for field `DEAT4`
pub struct DEAT4_W<'a> {
    w: &'a mut W,
}
impl<'a> DEAT4_W<'a> {
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
///Reader of field `DEAT3`
pub type DEAT3_R = crate::R<bool, bool>;
///Write proxy for field `DEAT3`
pub struct DEAT3_W<'a> {
    w: &'a mut W,
}
impl<'a> DEAT3_W<'a> {
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
///Reader of field `DEAT2`
pub type DEAT2_R = crate::R<bool, bool>;
///Write proxy for field `DEAT2`
pub struct DEAT2_W<'a> {
    w: &'a mut W,
}
impl<'a> DEAT2_W<'a> {
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
///Reader of field `DEAT1`
pub type DEAT1_R = crate::R<bool, bool>;
///Write proxy for field `DEAT1`
pub struct DEAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DEAT1_W<'a> {
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
///Reader of field `DEAT0`
pub type DEAT0_R = crate::R<bool, bool>;
///Write proxy for field `DEAT0`
pub struct DEAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DEAT0_W<'a> {
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
///Reader of field `DEDT4`
pub type DEDT4_R = crate::R<bool, bool>;
///Write proxy for field `DEDT4`
pub struct DEDT4_W<'a> {
    w: &'a mut W,
}
impl<'a> DEDT4_W<'a> {
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
///Reader of field `DEDT3`
pub type DEDT3_R = crate::R<bool, bool>;
///Write proxy for field `DEDT3`
pub struct DEDT3_W<'a> {
    w: &'a mut W,
}
impl<'a> DEDT3_W<'a> {
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
///Reader of field `DEDT2`
pub type DEDT2_R = crate::R<bool, bool>;
///Write proxy for field `DEDT2`
pub struct DEDT2_W<'a> {
    w: &'a mut W,
}
impl<'a> DEDT2_W<'a> {
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
///Reader of field `DEDT1`
pub type DEDT1_R = crate::R<bool, bool>;
///Write proxy for field `DEDT1`
pub struct DEDT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DEDT1_W<'a> {
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
///Reader of field `DEDT0`
pub type DEDT0_R = crate::R<bool, bool>;
///Write proxy for field `DEDT0`
pub struct DEDT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DEDT0_W<'a> {
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
///Reader of field `CMIE`
pub type CMIE_R = crate::R<bool, bool>;
///Write proxy for field `CMIE`
pub struct CMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMIE_W<'a> {
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
///Reader of field `MME`
pub type MME_R = crate::R<bool, bool>;
///Write proxy for field `MME`
pub struct MME_W<'a> {
    w: &'a mut W,
}
impl<'a> MME_W<'a> {
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
///Reader of field `M0`
pub type M0_R = crate::R<bool, bool>;
///Write proxy for field `M0`
pub struct M0_W<'a> {
    w: &'a mut W,
}
impl<'a> M0_W<'a> {
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
///Reader of field `WAKE`
pub type WAKE_R = crate::R<bool, bool>;
///Write proxy for field `WAKE`
pub struct WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_W<'a> {
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
///Reader of field `PCE`
pub type PCE_R = crate::R<bool, bool>;
///Write proxy for field `PCE`
pub struct PCE_W<'a> {
    w: &'a mut W,
}
impl<'a> PCE_W<'a> {
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
///Reader of field `PS`
pub type PS_R = crate::R<bool, bool>;
///Write proxy for field `PS`
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
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
///Reader of field `PEIE`
pub type PEIE_R = crate::R<bool, bool>;
///Write proxy for field `PEIE`
pub struct PEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIE_W<'a> {
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
///Reader of field `TXEIE`
pub type TXEIE_R = crate::R<bool, bool>;
///Write proxy for field `TXEIE`
pub struct TXEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEIE_W<'a> {
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
///Reader of field `TCIE`
pub type TCIE_R = crate::R<bool, bool>;
///Write proxy for field `TCIE`
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
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
///Reader of field `RXNEIE`
pub type RXNEIE_R = crate::R<bool, bool>;
///Write proxy for field `RXNEIE`
pub struct RXNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNEIE_W<'a> {
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
///Reader of field `IDLEIE`
pub type IDLEIE_R = crate::R<bool, bool>;
///Write proxy for field `IDLEIE`
pub struct IDLEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLEIE_W<'a> {
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
///Reader of field `TE`
pub type TE_R = crate::R<bool, bool>;
///Write proxy for field `TE`
pub struct TE_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_W<'a> {
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
///Reader of field `RE`
pub type RE_R = crate::R<bool, bool>;
///Write proxy for field `RE`
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
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
///Reader of field `UESM`
pub type UESM_R = crate::R<bool, bool>;
///Write proxy for field `UESM`
pub struct UESM_W<'a> {
    w: &'a mut W,
}
impl<'a> UESM_W<'a> {
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
///Reader of field `UE`
pub type UE_R = crate::R<bool, bool>;
///Write proxy for field `UE`
pub struct UE_W<'a> {
    w: &'a mut W,
}
impl<'a> UE_W<'a> {
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
    ///Bit 31 - RXFFIE
    #[inline(always)]
    pub fn rxffie(&self) -> RXFFIE_R {
        RXFFIE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 30 - TXFEIE
    #[inline(always)]
    pub fn txfeie(&self) -> TXFEIE_R {
        TXFEIE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 29 - FIFOEN
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 28 - Word length
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 25 - Driver Enable assertion time
    #[inline(always)]
    pub fn deat4(&self) -> DEAT4_R {
        DEAT4_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 24 - DEAT3
    #[inline(always)]
    pub fn deat3(&self) -> DEAT3_R {
        DEAT3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 23 - DEAT2
    #[inline(always)]
    pub fn deat2(&self) -> DEAT2_R {
        DEAT2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 22 - DEAT1
    #[inline(always)]
    pub fn deat1(&self) -> DEAT1_R {
        DEAT1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 21 - DEAT0
    #[inline(always)]
    pub fn deat0(&self) -> DEAT0_R {
        DEAT0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 20 - Driver Enable de-assertion time
    #[inline(always)]
    pub fn dedt4(&self) -> DEDT4_R {
        DEDT4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 19 - DEDT3
    #[inline(always)]
    pub fn dedt3(&self) -> DEDT3_R {
        DEDT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 18 - DEDT2
    #[inline(always)]
    pub fn dedt2(&self) -> DEDT2_R {
        DEDT2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - DEDT1
    #[inline(always)]
    pub fn dedt1(&self) -> DEDT1_R {
        DEDT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - DEDT0
    #[inline(always)]
    pub fn dedt0(&self) -> DEDT0_R {
        DEDT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 14 - Character match interrupt enable
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Mute mode enable
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Receiver wakeup method
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - interrupt enable
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - USART enable in Stop mode
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - USART enable
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 31 - RXFFIE
    #[inline(always)]
    pub fn rxffie(&mut self) -> RXFFIE_W {
        RXFFIE_W { w: self }
    }
    ///Bit 30 - TXFEIE
    #[inline(always)]
    pub fn txfeie(&mut self) -> TXFEIE_W {
        TXFEIE_W { w: self }
    }
    ///Bit 29 - FIFOEN
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W {
        FIFOEN_W { w: self }
    }
    ///Bit 28 - Word length
    #[inline(always)]
    pub fn m1(&mut self) -> M1_W {
        M1_W { w: self }
    }
    ///Bit 25 - Driver Enable assertion time
    #[inline(always)]
    pub fn deat4(&mut self) -> DEAT4_W {
        DEAT4_W { w: self }
    }
    ///Bit 24 - DEAT3
    #[inline(always)]
    pub fn deat3(&mut self) -> DEAT3_W {
        DEAT3_W { w: self }
    }
    ///Bit 23 - DEAT2
    #[inline(always)]
    pub fn deat2(&mut self) -> DEAT2_W {
        DEAT2_W { w: self }
    }
    ///Bit 22 - DEAT1
    #[inline(always)]
    pub fn deat1(&mut self) -> DEAT1_W {
        DEAT1_W { w: self }
    }
    ///Bit 21 - DEAT0
    #[inline(always)]
    pub fn deat0(&mut self) -> DEAT0_W {
        DEAT0_W { w: self }
    }
    ///Bit 20 - Driver Enable de-assertion time
    #[inline(always)]
    pub fn dedt4(&mut self) -> DEDT4_W {
        DEDT4_W { w: self }
    }
    ///Bit 19 - DEDT3
    #[inline(always)]
    pub fn dedt3(&mut self) -> DEDT3_W {
        DEDT3_W { w: self }
    }
    ///Bit 18 - DEDT2
    #[inline(always)]
    pub fn dedt2(&mut self) -> DEDT2_W {
        DEDT2_W { w: self }
    }
    ///Bit 17 - DEDT1
    #[inline(always)]
    pub fn dedt1(&mut self) -> DEDT1_W {
        DEDT1_W { w: self }
    }
    ///Bit 16 - DEDT0
    #[inline(always)]
    pub fn dedt0(&mut self) -> DEDT0_W {
        DEDT0_W { w: self }
    }
    ///Bit 14 - Character match interrupt enable
    #[inline(always)]
    pub fn cmie(&mut self) -> CMIE_W {
        CMIE_W { w: self }
    }
    ///Bit 13 - Mute mode enable
    #[inline(always)]
    pub fn mme(&mut self) -> MME_W {
        MME_W { w: self }
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m0(&mut self) -> M0_W {
        M0_W { w: self }
    }
    ///Bit 11 - Receiver wakeup method
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W {
        WAKE_W { w: self }
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W {
        PCE_W { w: self }
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W {
        PEIE_W { w: self }
    }
    ///Bit 7 - interrupt enable
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W {
        TXEIE_W { w: self }
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    ///Bit 5 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W {
        RXNEIE_W { w: self }
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W {
        IDLEIE_W { w: self }
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W { w: self }
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    ///Bit 1 - USART enable in Stop mode
    #[inline(always)]
    pub fn uesm(&mut self) -> UESM_W {
        UESM_W { w: self }
    }
    ///Bit 0 - USART enable
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W {
        UE_W { w: self }
    }
}
