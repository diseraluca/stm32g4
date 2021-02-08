///Reader of register TOCV
pub type R = crate::R<u32, super::TOCV>;
///Reader of field `TOC`
pub type TOC_R = crate::R<u16, u16>;
impl R {
    ///Bits 0:15 - TOC
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
    }
}
