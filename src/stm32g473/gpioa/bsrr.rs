///Writer for register BSRR
pub type W = crate::W<u32, super::BSRR>;
///Register BSRR `reset()`'s with value 0
impl crate::ResetValue for super::BSRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Write proxy for field `BR15`
pub struct BR15_W<'a> {
    w: &'a mut W,
}
impl<'a> BR15_W<'a> {
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
///Write proxy for field `BR14`
pub struct BR14_W<'a> {
    w: &'a mut W,
}
impl<'a> BR14_W<'a> {
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
///Write proxy for field `BR13`
pub struct BR13_W<'a> {
    w: &'a mut W,
}
impl<'a> BR13_W<'a> {
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
///Write proxy for field `BR12`
pub struct BR12_W<'a> {
    w: &'a mut W,
}
impl<'a> BR12_W<'a> {
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
///Write proxy for field `BR11`
pub struct BR11_W<'a> {
    w: &'a mut W,
}
impl<'a> BR11_W<'a> {
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
///Write proxy for field `BR10`
pub struct BR10_W<'a> {
    w: &'a mut W,
}
impl<'a> BR10_W<'a> {
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
///Write proxy for field `BR9`
pub struct BR9_W<'a> {
    w: &'a mut W,
}
impl<'a> BR9_W<'a> {
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
///Write proxy for field `BR8`
pub struct BR8_W<'a> {
    w: &'a mut W,
}
impl<'a> BR8_W<'a> {
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
///Write proxy for field `BR7`
pub struct BR7_W<'a> {
    w: &'a mut W,
}
impl<'a> BR7_W<'a> {
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
///Write proxy for field `BR6`
pub struct BR6_W<'a> {
    w: &'a mut W,
}
impl<'a> BR6_W<'a> {
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
///Write proxy for field `BR5`
pub struct BR5_W<'a> {
    w: &'a mut W,
}
impl<'a> BR5_W<'a> {
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
///Write proxy for field `BR4`
pub struct BR4_W<'a> {
    w: &'a mut W,
}
impl<'a> BR4_W<'a> {
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
///Write proxy for field `BR3`
pub struct BR3_W<'a> {
    w: &'a mut W,
}
impl<'a> BR3_W<'a> {
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
///Write proxy for field `BR2`
pub struct BR2_W<'a> {
    w: &'a mut W,
}
impl<'a> BR2_W<'a> {
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
///Write proxy for field `BR1`
pub struct BR1_W<'a> {
    w: &'a mut W,
}
impl<'a> BR1_W<'a> {
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
///Write proxy for field `BR0`
pub struct BR0_W<'a> {
    w: &'a mut W,
}
impl<'a> BR0_W<'a> {
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
///Write proxy for field `BS15`
pub struct BS15_W<'a> {
    w: &'a mut W,
}
impl<'a> BS15_W<'a> {
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
///Write proxy for field `BS14`
pub struct BS14_W<'a> {
    w: &'a mut W,
}
impl<'a> BS14_W<'a> {
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
///Write proxy for field `BS13`
pub struct BS13_W<'a> {
    w: &'a mut W,
}
impl<'a> BS13_W<'a> {
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
///Write proxy for field `BS12`
pub struct BS12_W<'a> {
    w: &'a mut W,
}
impl<'a> BS12_W<'a> {
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
///Write proxy for field `BS11`
pub struct BS11_W<'a> {
    w: &'a mut W,
}
impl<'a> BS11_W<'a> {
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
///Write proxy for field `BS10`
pub struct BS10_W<'a> {
    w: &'a mut W,
}
impl<'a> BS10_W<'a> {
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
///Write proxy for field `BS9`
pub struct BS9_W<'a> {
    w: &'a mut W,
}
impl<'a> BS9_W<'a> {
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
///Write proxy for field `BS8`
pub struct BS8_W<'a> {
    w: &'a mut W,
}
impl<'a> BS8_W<'a> {
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
///Write proxy for field `BS7`
pub struct BS7_W<'a> {
    w: &'a mut W,
}
impl<'a> BS7_W<'a> {
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
///Write proxy for field `BS6`
pub struct BS6_W<'a> {
    w: &'a mut W,
}
impl<'a> BS6_W<'a> {
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
///Write proxy for field `BS5`
pub struct BS5_W<'a> {
    w: &'a mut W,
}
impl<'a> BS5_W<'a> {
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
///Write proxy for field `BS4`
pub struct BS4_W<'a> {
    w: &'a mut W,
}
impl<'a> BS4_W<'a> {
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
///Write proxy for field `BS3`
pub struct BS3_W<'a> {
    w: &'a mut W,
}
impl<'a> BS3_W<'a> {
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
///Write proxy for field `BS2`
pub struct BS2_W<'a> {
    w: &'a mut W,
}
impl<'a> BS2_W<'a> {
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
///Write proxy for field `BS1`
pub struct BS1_W<'a> {
    w: &'a mut W,
}
impl<'a> BS1_W<'a> {
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
///Write proxy for field `BS0`
pub struct BS0_W<'a> {
    w: &'a mut W,
}
impl<'a> BS0_W<'a> {
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
impl W {
    ///Bit 31 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W {
        BR15_W { w: self }
    }
    ///Bit 30 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W {
        BR14_W { w: self }
    }
    ///Bit 29 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W {
        BR13_W { w: self }
    }
    ///Bit 28 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br12(&mut self) -> BR12_W {
        BR12_W { w: self }
    }
    ///Bit 27 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br11(&mut self) -> BR11_W {
        BR11_W { w: self }
    }
    ///Bit 26 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br10(&mut self) -> BR10_W {
        BR10_W { w: self }
    }
    ///Bit 25 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br9(&mut self) -> BR9_W {
        BR9_W { w: self }
    }
    ///Bit 24 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W {
        BR8_W { w: self }
    }
    ///Bit 23 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W {
        BR7_W { w: self }
    }
    ///Bit 22 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W {
        BR6_W { w: self }
    }
    ///Bit 21 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W {
        BR5_W { w: self }
    }
    ///Bit 20 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W {
        BR4_W { w: self }
    }
    ///Bit 19 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W {
        BR3_W { w: self }
    }
    ///Bit 18 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W {
        BR2_W { w: self }
    }
    ///Bit 17 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W {
        BR1_W { w: self }
    }
    ///Bit 16 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W {
        BR0_W { w: self }
    }
    ///Bit 15 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs15(&mut self) -> BS15_W {
        BS15_W { w: self }
    }
    ///Bit 14 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs14(&mut self) -> BS14_W {
        BS14_W { w: self }
    }
    ///Bit 13 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs13(&mut self) -> BS13_W {
        BS13_W { w: self }
    }
    ///Bit 12 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs12(&mut self) -> BS12_W {
        BS12_W { w: self }
    }
    ///Bit 11 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs11(&mut self) -> BS11_W {
        BS11_W { w: self }
    }
    ///Bit 10 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs10(&mut self) -> BS10_W {
        BS10_W { w: self }
    }
    ///Bit 9 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs9(&mut self) -> BS9_W {
        BS9_W { w: self }
    }
    ///Bit 8 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs8(&mut self) -> BS8_W {
        BS8_W { w: self }
    }
    ///Bit 7 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs7(&mut self) -> BS7_W {
        BS7_W { w: self }
    }
    ///Bit 6 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs6(&mut self) -> BS6_W {
        BS6_W { w: self }
    }
    ///Bit 5 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs5(&mut self) -> BS5_W {
        BS5_W { w: self }
    }
    ///Bit 4 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs4(&mut self) -> BS4_W {
        BS4_W { w: self }
    }
    ///Bit 3 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs3(&mut self) -> BS3_W {
        BS3_W { w: self }
    }
    ///Bit 2 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs2(&mut self) -> BS2_W {
        BS2_W { w: self }
    }
    ///Bit 1 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs1(&mut self) -> BS1_W {
        BS1_W { w: self }
    }
    ///Bit 0 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs0(&mut self) -> BS0_W {
        BS0_W { w: self }
    }
}
