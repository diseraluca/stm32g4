///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Writer for register SR
pub type W = crate::W<u32, super::SR>;
///Register SR `reset()`'s with value 0
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///End of operation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOP_A {
    ///0: No EOP event occurred
    NOEVENT = 0,
    ///1: EOP event occurred
    EVENT = 1,
}
impl From<EOP_A> for bool {
    #[inline(always)]
    fn from(variant: EOP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EOP`
pub type EOP_R = crate::R<bool, EOP_A>;
impl EOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOP_A {
        match self.bits {
            false => EOP_A::NOEVENT,
            true => EOP_A::EVENT,
        }
    }
    ///Checks if the value of the field is `NOEVENT`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == EOP_A::NOEVENT
    }
    ///Checks if the value of the field is `EVENT`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == EOP_A::EVENT
    }
}
///Write proxy for field `EOP`
pub struct EOP_W<'a> {
    w: &'a mut W,
}
impl<'a> EOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No EOP event occurred
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(EOP_A::NOEVENT)
    }
    ///EOP event occurred
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(EOP_A::EVENT)
    }
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
///Operation error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPERR_A {
    ///0: No operation error occurred
    NOERROR = 0,
    ///1: Operation error occurred
    ERROR = 1,
}
impl From<OPERR_A> for bool {
    #[inline(always)]
    fn from(variant: OPERR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OPERR`
pub type OPERR_R = crate::R<bool, OPERR_A>;
impl OPERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPERR_A {
        match self.bits {
            false => OPERR_A::NOERROR,
            true => OPERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OPERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == OPERR_A::ERROR
    }
}
///Write proxy for field `OPERR`
pub struct OPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OPERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No operation error occurred
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(OPERR_A::NOERROR)
    }
    ///Operation error occurred
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(OPERR_A::ERROR)
    }
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
///Programming error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROGERR_A {
    ///0: No programming error occurred
    NOERROR = 0,
    ///1: Programming error occurred
    ERROR = 1,
}
impl From<PROGERR_A> for bool {
    #[inline(always)]
    fn from(variant: PROGERR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PROGERR`
pub type PROGERR_R = crate::R<bool, PROGERR_A>;
impl PROGERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PROGERR_A {
        match self.bits {
            false => PROGERR_A::NOERROR,
            true => PROGERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PROGERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PROGERR_A::ERROR
    }
}
///Write proxy for field `PROGERR`
pub struct PROGERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PROGERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No programming error occurred
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(PROGERR_A::NOERROR)
    }
    ///Programming error occurred
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(PROGERR_A::ERROR)
    }
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
///Write protected error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRPERR_A {
    ///0: No write protection error occurred
    NOERROR = 0,
    ///1: Write protection error occurred
    ERROR = 1,
}
impl From<WRPERR_A> for bool {
    #[inline(always)]
    fn from(variant: WRPERR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `WRPERR`
pub type WRPERR_R = crate::R<bool, WRPERR_A>;
impl WRPERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WRPERR_A {
        match self.bits {
            false => WRPERR_A::NOERROR,
            true => WRPERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRPERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRPERR_A::ERROR
    }
}
///Write proxy for field `WRPERR`
pub struct WRPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WRPERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WRPERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No write protection error occurred
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(WRPERR_A::NOERROR)
    }
    ///Write protection error occurred
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(WRPERR_A::ERROR)
    }
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
///Programming alignment error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGAERR_A {
    ///0: No programming alignment error occurred
    NOERROR = 0,
    ///1: Programming alignment error occurred
    ERROR = 1,
}
impl From<PGAERR_A> for bool {
    #[inline(always)]
    fn from(variant: PGAERR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PGAERR`
pub type PGAERR_R = crate::R<bool, PGAERR_A>;
impl PGAERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PGAERR_A {
        match self.bits {
            false => PGAERR_A::NOERROR,
            true => PGAERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGAERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGAERR_A::ERROR
    }
}
///Write proxy for field `PGAERR`
pub struct PGAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PGAERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PGAERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No programming alignment error occurred
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(PGAERR_A::NOERROR)
    }
    ///Programming alignment error occurred
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(PGAERR_A::ERROR)
    }
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
///Size error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZERR_A {
    ///0: No size error occurred
    NOERROR = 0,
    ///1: Size error occurred
    ERROR = 1,
}
impl From<SIZERR_A> for bool {
    #[inline(always)]
    fn from(variant: SIZERR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SIZERR`
pub type SIZERR_R = crate::R<bool, SIZERR_A>;
impl SIZERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SIZERR_A {
        match self.bits {
            false => SIZERR_A::NOERROR,
            true => SIZERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == SIZERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SIZERR_A::ERROR
    }
}
///Write proxy for field `SIZERR`
pub struct SIZERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SIZERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No size error occurred
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(SIZERR_A::NOERROR)
    }
    ///Size error occurred
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(SIZERR_A::ERROR)
    }
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
///Programming sequence error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGSERR_A {
    ///0: No programming sequence error occurred
    NOERROR = 0,
    ///1: Programming sequence error occurred
    ERROR = 1,
}
impl From<PGSERR_A> for bool {
    #[inline(always)]
    fn from(variant: PGSERR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PGSERR`
pub type PGSERR_R = crate::R<bool, PGSERR_A>;
impl PGSERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PGSERR_A {
        match self.bits {
            false => PGSERR_A::NOERROR,
            true => PGSERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGSERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGSERR_A::ERROR
    }
}
///Write proxy for field `PGSERR`
pub struct PGSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PGSERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PGSERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No programming sequence error occurred
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(PGSERR_A::NOERROR)
    }
    ///Programming sequence error occurred
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(PGSERR_A::ERROR)
    }
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
///Fast programming data miss error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MISERR_A {
    ///0: No fast programming data miss error occurred
    NOERROR = 0,
    ///1: Fast programming data miss error occurred
    ERROR = 1,
}
impl From<MISERR_A> for bool {
    #[inline(always)]
    fn from(variant: MISERR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MISERR`
pub type MISERR_R = crate::R<bool, MISERR_A>;
impl MISERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MISERR_A {
        match self.bits {
            false => MISERR_A::NOERROR,
            true => MISERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == MISERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == MISERR_A::ERROR
    }
}
///Write proxy for field `MISERR`
pub struct MISERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MISERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MISERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No fast programming data miss error occurred
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(MISERR_A::NOERROR)
    }
    ///Fast programming data miss error occurred
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(MISERR_A::ERROR)
    }
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
///Fast programming error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FASTERR_A {
    ///0: No fast programming error occurred
    NOERROR = 0,
    ///1: Fast programming error occurred
    ERROR = 1,
}
impl From<FASTERR_A> for bool {
    #[inline(always)]
    fn from(variant: FASTERR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FASTERR`
pub type FASTERR_R = crate::R<bool, FASTERR_A>;
impl FASTERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FASTERR_A {
        match self.bits {
            false => FASTERR_A::NOERROR,
            true => FASTERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FASTERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FASTERR_A::ERROR
    }
}
///Write proxy for field `FASTERR`
pub struct FASTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FASTERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No fast programming error occurred
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(FASTERR_A::NOERROR)
    }
    ///Fast programming error occurred
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(FASTERR_A::ERROR)
    }
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
///PCROP read error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDERR_A {
    ///0: No read error occurred
    NOERROR = 0,
    ///1: Read error occurred
    ERROR = 1,
}
impl From<RDERR_A> for bool {
    #[inline(always)]
    fn from(variant: RDERR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RDERR`
pub type RDERR_R = crate::R<bool, RDERR_A>;
impl RDERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RDERR_A {
        match self.bits {
            false => RDERR_A::NOERROR,
            true => RDERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RDERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RDERR_A::ERROR
    }
}
///Write proxy for field `RDERR`
pub struct RDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RDERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RDERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No read error occurred
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(RDERR_A::NOERROR)
    }
    ///Read error occurred
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(RDERR_A::ERROR)
    }
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
///Option validity error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTVERR_A {
    ///0: No option validity error occurred
    NOERROR = 0,
    ///1: Option validity error occurred
    ERROR = 1,
}
impl From<OPTVERR_A> for bool {
    #[inline(always)]
    fn from(variant: OPTVERR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OPTVERR`
pub type OPTVERR_R = crate::R<bool, OPTVERR_A>;
impl OPTVERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPTVERR_A {
        match self.bits {
            false => OPTVERR_A::NOERROR,
            true => OPTVERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OPTVERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == OPTVERR_A::ERROR
    }
}
///Write proxy for field `OPTVERR`
pub struct OPTVERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTVERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OPTVERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No option validity error occurred
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(OPTVERR_A::NOERROR)
    }
    ///Option validity error occurred
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(OPTVERR_A::ERROR)
    }
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
///Busy
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSY_A {
    ///0: No write/reset operation is in progress and no error occurred
    INACTIVE = 0,
    ///1: Write/reset operation is in progress or an error occurred
    ACTIVE = 1,
}
impl From<BSY_A> for bool {
    #[inline(always)]
    fn from(variant: BSY_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `BSY`
pub type BSY_R = crate::R<bool, BSY_A>;
impl BSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BSY_A {
        match self.bits {
            false => BSY_A::INACTIVE,
            true => BSY_A::ACTIVE,
        }
    }
    ///Checks if the value of the field is `INACTIVE`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BSY_A::INACTIVE
    }
    ///Checks if the value of the field is `ACTIVE`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BSY_A::ACTIVE
    }
}
impl R {
    ///Bit 0 - End of operation
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 3 - Programming error
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Write protected error
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Size error
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Fast programming data miss error
    #[inline(always)]
    pub fn miserr(&self) -> MISERR_R {
        MISERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Fast programming error
    #[inline(always)]
    pub fn fasterr(&self) -> FASTERR_R {
        FASTERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 14 - PCROP read error
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Option validity error
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Busy
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - End of operation
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W {
        EOP_W { w: self }
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    pub fn operr(&mut self) -> OPERR_W {
        OPERR_W { w: self }
    }
    ///Bit 3 - Programming error
    #[inline(always)]
    pub fn progerr(&mut self) -> PROGERR_W {
        PROGERR_W { w: self }
    }
    ///Bit 4 - Write protected error
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W {
        WRPERR_W { w: self }
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&mut self) -> PGAERR_W {
        PGAERR_W { w: self }
    }
    ///Bit 6 - Size error
    #[inline(always)]
    pub fn sizerr(&mut self) -> SIZERR_W {
        SIZERR_W { w: self }
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    pub fn pgserr(&mut self) -> PGSERR_W {
        PGSERR_W { w: self }
    }
    ///Bit 8 - Fast programming data miss error
    #[inline(always)]
    pub fn miserr(&mut self) -> MISERR_W {
        MISERR_W { w: self }
    }
    ///Bit 9 - Fast programming error
    #[inline(always)]
    pub fn fasterr(&mut self) -> FASTERR_W {
        FASTERR_W { w: self }
    }
    ///Bit 14 - PCROP read error
    #[inline(always)]
    pub fn rderr(&mut self) -> RDERR_W {
        RDERR_W { w: self }
    }
    ///Bit 15 - Option validity error
    #[inline(always)]
    pub fn optverr(&mut self) -> OPTVERR_W {
        OPTVERR_W { w: self }
    }
}
