///Reader of register ECR
pub type R = crate::R<u32, super::ECR>;
///Reader of field `TEC`
pub type TEC_R = crate::R<u8, u8>;
///Reader of field `TREC`
pub type TREC_R = crate::R<u8, u8>;
///Reader of field `RP`
pub type RP_R = crate::R<bool, bool>;
///Reader of field `CEL`
pub type CEL_R = crate::R<u8, u8>;
impl R {
    ///Bits 0:7 - TEC
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:14 - TREC
    #[inline(always)]
    pub fn trec(&self) -> TREC_R {
        TREC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 15 - RP
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 16:23 - CEL
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
