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
///Reader of field `ADD4_7`
pub type ADD4_7_R = crate::R<u8, u8>;
///Write proxy for field `ADD4_7`
pub struct ADD4_7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD4_7_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
///Reader of field `ADD0_3`
pub type ADD0_3_R = crate::R<u8, u8>;
///Write proxy for field `ADD0_3`
pub struct ADD0_3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD0_3_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
///Reader of field `RTOEN`
pub type RTOEN_R = crate::R<bool, bool>;
///Write proxy for field `RTOEN`
pub struct RTOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTOEN_W<'a> {
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
///Reader of field `ABRMOD1`
pub type ABRMOD1_R = crate::R<bool, bool>;
///Write proxy for field `ABRMOD1`
pub struct ABRMOD1_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRMOD1_W<'a> {
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
///Reader of field `ABRMOD0`
pub type ABRMOD0_R = crate::R<bool, bool>;
///Write proxy for field `ABRMOD0`
pub struct ABRMOD0_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRMOD0_W<'a> {
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
///Reader of field `ABREN`
pub type ABREN_R = crate::R<bool, bool>;
///Write proxy for field `ABREN`
pub struct ABREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ABREN_W<'a> {
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
///Reader of field `MSBFIRST`
pub type MSBFIRST_R = crate::R<bool, bool>;
///Write proxy for field `MSBFIRST`
pub struct MSBFIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MSBFIRST_W<'a> {
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
///Reader of field `TAINV`
pub type TAINV_R = crate::R<bool, bool>;
///Write proxy for field `TAINV`
pub struct TAINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TAINV_W<'a> {
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
///Reader of field `TXINV`
pub type TXINV_R = crate::R<bool, bool>;
///Write proxy for field `TXINV`
pub struct TXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINV_W<'a> {
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
///Reader of field `RXINV`
pub type RXINV_R = crate::R<bool, bool>;
///Write proxy for field `RXINV`
pub struct RXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINV_W<'a> {
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
///Reader of field `SWAP`
pub type SWAP_R = crate::R<bool, bool>;
///Write proxy for field `SWAP`
pub struct SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_W<'a> {
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
///Reader of field `LINEN`
pub type LINEN_R = crate::R<bool, bool>;
///Write proxy for field `LINEN`
pub struct LINEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LINEN_W<'a> {
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
///Reader of field `STOP`
pub type STOP_R = crate::R<u8, u8>;
///Write proxy for field `STOP`
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
///Reader of field `CLKEN`
pub type CLKEN_R = crate::R<bool, bool>;
///Write proxy for field `CLKEN`
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
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
///Reader of field `CPOL`
pub type CPOL_R = crate::R<bool, bool>;
///Write proxy for field `CPOL`
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
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
///Reader of field `CPHA`
pub type CPHA_R = crate::R<bool, bool>;
///Write proxy for field `CPHA`
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
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
///Reader of field `LBCL`
pub type LBCL_R = crate::R<bool, bool>;
///Write proxy for field `LBCL`
pub struct LBCL_W<'a> {
    w: &'a mut W,
}
impl<'a> LBCL_W<'a> {
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
///Reader of field `LBDIE`
pub type LBDIE_R = crate::R<bool, bool>;
///Write proxy for field `LBDIE`
pub struct LBDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBDIE_W<'a> {
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
///Reader of field `LBDL`
pub type LBDL_R = crate::R<bool, bool>;
///Write proxy for field `LBDL`
pub struct LBDL_W<'a> {
    w: &'a mut W,
}
impl<'a> LBDL_W<'a> {
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
///Reader of field `ADDM7`
pub type ADDM7_R = crate::R<bool, bool>;
///Write proxy for field `ADDM7`
pub struct ADDM7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDM7_W<'a> {
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
///Reader of field `DIS_NSS`
pub type DIS_NSS_R = crate::R<bool, bool>;
///Write proxy for field `DIS_NSS`
pub struct DIS_NSS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_NSS_W<'a> {
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
///Reader of field `SLVEN`
pub type SLVEN_R = crate::R<bool, bool>;
///Write proxy for field `SLVEN`
pub struct SLVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVEN_W<'a> {
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
    ///Bits 28:31 - Address of the USART node
    #[inline(always)]
    pub fn add4_7(&self) -> ADD4_7_R {
        ADD4_7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    ///Bits 24:27 - Address of the USART node
    #[inline(always)]
    pub fn add0_3(&self) -> ADD0_3_R {
        ADD0_3_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 23 - Receiver timeout enable
    #[inline(always)]
    pub fn rtoen(&self) -> RTOEN_R {
        RTOEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 22 - Auto baud rate mode
    #[inline(always)]
    pub fn abrmod1(&self) -> ABRMOD1_R {
        ABRMOD1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 21 - ABRMOD0
    #[inline(always)]
    pub fn abrmod0(&self) -> ABRMOD0_R {
        ABRMOD0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 20 - Auto baud rate enable
    #[inline(always)]
    pub fn abren(&self) -> ABREN_R {
        ABREN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 18 - Binary data inversion
    #[inline(always)]
    pub fn tainv(&self) -> TAINV_R {
        TAINV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - TX pin active level inversion
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - RX pin active level inversion
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - Swap TX/RX pins
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bit 11 - Clock enable
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Clock polarity
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Clock phase
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Last bit clock pulse
    #[inline(always)]
    pub fn lbcl(&self) -> LBCL_R {
        LBCL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - LIN break detection length
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - 7-bit Address Detection/4-bit Address Detection
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - DIS_NSS
    #[inline(always)]
    pub fn dis_nss(&self) -> DIS_NSS_R {
        DIS_NSS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 0 - SLVEN
    #[inline(always)]
    pub fn slven(&self) -> SLVEN_R {
        SLVEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 28:31 - Address of the USART node
    #[inline(always)]
    pub fn add4_7(&mut self) -> ADD4_7_W {
        ADD4_7_W { w: self }
    }
    ///Bits 24:27 - Address of the USART node
    #[inline(always)]
    pub fn add0_3(&mut self) -> ADD0_3_W {
        ADD0_3_W { w: self }
    }
    ///Bit 23 - Receiver timeout enable
    #[inline(always)]
    pub fn rtoen(&mut self) -> RTOEN_W {
        RTOEN_W { w: self }
    }
    ///Bit 22 - Auto baud rate mode
    #[inline(always)]
    pub fn abrmod1(&mut self) -> ABRMOD1_W {
        ABRMOD1_W { w: self }
    }
    ///Bit 21 - ABRMOD0
    #[inline(always)]
    pub fn abrmod0(&mut self) -> ABRMOD0_W {
        ABRMOD0_W { w: self }
    }
    ///Bit 20 - Auto baud rate enable
    #[inline(always)]
    pub fn abren(&mut self) -> ABREN_W {
        ABREN_W { w: self }
    }
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MSBFIRST_W {
        MSBFIRST_W { w: self }
    }
    ///Bit 18 - Binary data inversion
    #[inline(always)]
    pub fn tainv(&mut self) -> TAINV_W {
        TAINV_W { w: self }
    }
    ///Bit 17 - TX pin active level inversion
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W {
        TXINV_W { w: self }
    }
    ///Bit 16 - RX pin active level inversion
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W {
        RXINV_W { w: self }
    }
    ///Bit 15 - Swap TX/RX pins
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W {
        SWAP_W { w: self }
    }
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    pub fn linen(&mut self) -> LINEN_W {
        LINEN_W { w: self }
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    ///Bit 11 - Clock enable
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    ///Bit 10 - Clock polarity
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    ///Bit 9 - Clock phase
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    ///Bit 8 - Last bit clock pulse
    #[inline(always)]
    pub fn lbcl(&mut self) -> LBCL_W {
        LBCL_W { w: self }
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    pub fn lbdie(&mut self) -> LBDIE_W {
        LBDIE_W { w: self }
    }
    ///Bit 5 - LIN break detection length
    #[inline(always)]
    pub fn lbdl(&mut self) -> LBDL_W {
        LBDL_W { w: self }
    }
    ///Bit 4 - 7-bit Address Detection/4-bit Address Detection
    #[inline(always)]
    pub fn addm7(&mut self) -> ADDM7_W {
        ADDM7_W { w: self }
    }
    ///Bit 3 - DIS_NSS
    #[inline(always)]
    pub fn dis_nss(&mut self) -> DIS_NSS_W {
        DIS_NSS_W { w: self }
    }
    ///Bit 0 - SLVEN
    #[inline(always)]
    pub fn slven(&mut self) -> SLVEN_W {
        SLVEN_W { w: self }
    }
}
