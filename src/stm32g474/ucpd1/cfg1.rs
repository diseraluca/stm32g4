///Reader of register CFG1
pub type R = crate::R<u32, super::CFG1>;
///Writer for register CFG1
pub type W = crate::W<u32, super::CFG1>;
///Register CFG1 `reset()`'s with value 0
impl crate::ResetValue for super::CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `HBITCLKDIV`
pub type HBITCLKDIV_R = crate::R<u8, u8>;
///Write proxy for field `HBITCLKDIV`
pub struct HBITCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HBITCLKDIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
///Reader of field `IFRGAP`
pub type IFRGAP_R = crate::R<u8, u8>;
///Write proxy for field `IFRGAP`
pub struct IFRGAP_W<'a> {
    w: &'a mut W,
}
impl<'a> IFRGAP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
///Reader of field `TRANSWIN`
pub type TRANSWIN_R = crate::R<u8, u8>;
///Write proxy for field `TRANSWIN`
pub struct TRANSWIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSWIN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
///Reader of field `PSC_USBPDCLK`
pub type PSC_USBPDCLK_R = crate::R<u8, u8>;
///Write proxy for field `PSC_USBPDCLK`
pub struct PSC_USBPDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_USBPDCLK_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
///Reader of field `RXORDSETEN`
pub type RXORDSETEN_R = crate::R<u16, u16>;
///Write proxy for field `RXORDSETEN`
pub struct RXORDSETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXORDSETEN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 20)) | (((value as u32) & 0x01ff) << 20);
        self.w
    }
}
///Reader of field `TXDMAEN`
pub type TXDMAEN_R = crate::R<bool, bool>;
///Write proxy for field `TXDMAEN`
pub struct TXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAEN_W<'a> {
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
///Reader of field `RXDMAEN`
pub type RXDMAEN_R = crate::R<bool, bool>;
///Write proxy for field `RXDMAEN`
pub struct RXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAEN_W<'a> {
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
///Reader of field `UCPDEN`
pub type UCPDEN_R = crate::R<bool, bool>;
///Write proxy for field `UCPDEN`
pub struct UCPDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPDEN_W<'a> {
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
impl R {
    ///Bits 0:5 - HBITCLKDIV
    #[inline(always)]
    pub fn hbitclkdiv(&self) -> HBITCLKDIV_R {
        HBITCLKDIV_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:10 - IFRGAP
    #[inline(always)]
    pub fn ifrgap(&self) -> IFRGAP_R {
        IFRGAP_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 11:15 - TRANSWIN
    #[inline(always)]
    pub fn transwin(&self) -> TRANSWIN_R {
        TRANSWIN_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    ///Bits 17:19 - PSC_USBPDCLK
    #[inline(always)]
    pub fn psc_usbpdclk(&self) -> PSC_USBPDCLK_R {
        PSC_USBPDCLK_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    ///Bits 20:28 - RXORDSETEN
    #[inline(always)]
    pub fn rxordseten(&self) -> RXORDSETEN_R {
        RXORDSETEN_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    ///Bit 29 - TXDMAEN
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - RXDMAEN
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - UCPDEN
    #[inline(always)]
    pub fn ucpden(&self) -> UCPDEN_R {
        UCPDEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:5 - HBITCLKDIV
    #[inline(always)]
    pub fn hbitclkdiv(&mut self) -> HBITCLKDIV_W {
        HBITCLKDIV_W { w: self }
    }
    ///Bits 6:10 - IFRGAP
    #[inline(always)]
    pub fn ifrgap(&mut self) -> IFRGAP_W {
        IFRGAP_W { w: self }
    }
    ///Bits 11:15 - TRANSWIN
    #[inline(always)]
    pub fn transwin(&mut self) -> TRANSWIN_W {
        TRANSWIN_W { w: self }
    }
    ///Bits 17:19 - PSC_USBPDCLK
    #[inline(always)]
    pub fn psc_usbpdclk(&mut self) -> PSC_USBPDCLK_W {
        PSC_USBPDCLK_W { w: self }
    }
    ///Bits 20:28 - RXORDSETEN
    #[inline(always)]
    pub fn rxordseten(&mut self) -> RXORDSETEN_W {
        RXORDSETEN_W { w: self }
    }
    ///Bit 29 - TXDMAEN
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W {
        TXDMAEN_W { w: self }
    }
    ///Bit 30 - RXDMAEN
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    ///Bit 31 - UCPDEN
    #[inline(always)]
    pub fn ucpden(&mut self) -> UCPDEN_W {
        UCPDEN_W { w: self }
    }
}
