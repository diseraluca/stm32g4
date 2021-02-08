///Reader of register CIFR
pub type R = crate::R<u32, super::CIFR>;
///Reader of field `LSIRDYF`
pub type LSIRDYF_R = crate::R<bool, bool>;
///Reader of field `LSERDYF`
pub type LSERDYF_R = crate::R<bool, bool>;
///Reader of field `HSIRDYF`
pub type HSIRDYF_R = crate::R<bool, bool>;
///Reader of field `HSERDYF`
pub type HSERDYF_R = crate::R<bool, bool>;
///Reader of field `PLLRDYF`
pub type PLLRDYF_R = crate::R<bool, bool>;
///Reader of field `CSSF`
pub type CSSF_R = crate::R<bool, bool>;
///Reader of field `LSECSSF`
pub type LSECSSF_R = crate::R<bool, bool>;
///Reader of field `HSI48RDYF`
pub type HSI48RDYF_R = crate::R<bool, bool>;
impl R {
    ///Bit 0 - LSI ready interrupt flag
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - LSE ready interrupt flag
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 3 - HSI ready interrupt flag
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - HSE ready interrupt flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 8 - Clock security system interrupt flag
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - LSE Clock security system interrupt flag
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - HSI48 ready interrupt flag
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
