///Reader of register KEYR7
pub type R = crate::R<u32, super::KEYR7>;
///Writer for register KEYR7
pub type W = crate::W<u32, super::KEYR7>;
///Register KEYR7 `reset()`'s with value 0
impl crate::ResetValue for super::KEYR7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `KEY`
pub type KEY_R = crate::R<u32, u32>;
///Write proxy for field `KEY`
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - AES key
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - AES key
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
