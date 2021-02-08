///Reader of register ENDN
pub type R = crate::R<u32, super::ENDN>;
///Reader of field `ETV`
pub type ETV_R = crate::R<u32, u32>;
impl R {
    ///Bits 0:31 - ETV
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
