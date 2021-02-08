///Reader of register JDR1
pub type R = crate::R<u32, super::JDR1>;
///Reader of field `JDATA`
pub type JDATA_R = crate::R<u16, u16>;
impl R {
    ///Bits 0:15 - Injected data
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
}
