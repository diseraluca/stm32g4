///Reader of register CR
pub type R = crate::R<u32, super::CR>;
///Writer for register CR
pub type W = crate::W<u32, super::CR>;
///Register CR `reset()`'s with value 0xc000_0000
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0000
    }
}
///Programming
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PG_A {
    ///0: Flash programming disabled
    DISABLED = 0,
    ///1: Flash programming enabled
    ENABLED = 1,
}
impl From<PG_A> for bool {
    #[inline(always)]
    fn from(variant: PG_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PG`
pub type PG_R = crate::R<bool, PG_A>;
impl PG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PG_A {
        match self.bits {
            false => PG_A::DISABLED,
            true => PG_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PG_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PG_A::ENABLED
    }
}
///Write proxy for field `PG`
pub struct PG_W<'a> {
    w: &'a mut W,
}
impl<'a> PG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Flash programming disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PG_A::DISABLED)
    }
    ///Flash programming enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PG_A::ENABLED)
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
///Page erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER_A {
    ///0: Page erase disabled
    DISABLED = 0,
    ///1: Page erase enabled
    ENABLED = 1,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PER`
pub type PER_R = crate::R<bool, PER_A>;
impl PER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PER_A {
        match self.bits {
            false => PER_A::DISABLED,
            true => PER_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PER_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PER_A::ENABLED
    }
}
///Write proxy for field `PER`
pub struct PER_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Page erase disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PER_A::DISABLED)
    }
    ///Page erase enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PER_A::ENABLED)
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
///Bank 1 Mass erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MER1_A {
    ///1: Erases all bank1 user pages
    MASSERASE = 1,
}
impl From<MER1_A> for bool {
    #[inline(always)]
    fn from(variant: MER1_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MER1`
pub type MER1_R = crate::R<bool, MER1_A>;
impl MER1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MER1_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(MER1_A::MASSERASE),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `MASSERASE`
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER1_A::MASSERASE
    }
}
///Write proxy for field `MER1`
pub struct MER1_W<'a> {
    w: &'a mut W,
}
impl<'a> MER1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MER1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Erases all bank1 user pages
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut W {
        self.variant(MER1_A::MASSERASE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///Page number
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PNB_A {
    ///0: Erase Page 0
    PAGE0 = 0,
    ///1: Erase Page 1
    PAGE1 = 1,
    ///2: Erase Page 2
    PAGE2 = 2,
    ///3: Erase Page 3
    PAGE3 = 3,
    ///4: Erase Page 4
    PAGE4 = 4,
    ///5: Erase Page 5
    PAGE5 = 5,
    ///6: Erase Page 6
    PAGE6 = 6,
    ///7: Erase Page 7
    PAGE7 = 7,
    ///8: Erase Page 8
    PAGE8 = 8,
    ///9: Erase Page 9
    PAGE9 = 9,
    ///10: Erase Page 10
    PAGE10 = 10,
    ///11: Erase Page 11
    PAGE11 = 11,
    ///12: Erase Page 12
    PAGE12 = 12,
    ///13: Erase Page 13
    PAGE13 = 13,
    ///14: Erase Page 14
    PAGE14 = 14,
    ///15: Erase Page 15
    PAGE15 = 15,
    ///16: Erase Page 16
    PAGE16 = 16,
    ///17: Erase Page 17
    PAGE17 = 17,
    ///18: Erase Page 18
    PAGE18 = 18,
    ///19: Erase Page 19
    PAGE19 = 19,
    ///20: Erase Page 20
    PAGE20 = 20,
    ///21: Erase Page 21
    PAGE21 = 21,
    ///22: Erase Page 22
    PAGE22 = 22,
    ///23: Erase Page 23
    PAGE23 = 23,
    ///24: Erase Page 24
    PAGE24 = 24,
    ///25: Erase Page 25
    PAGE25 = 25,
    ///26: Erase Page 26
    PAGE26 = 26,
    ///27: Erase Page 27
    PAGE27 = 27,
    ///28: Erase Page 28
    PAGE28 = 28,
    ///29: Erase Page 29
    PAGE29 = 29,
    ///30: Erase Page 30
    PAGE30 = 30,
    ///31: Erase Page 31
    PAGE31 = 31,
    ///32: Erase Page 32
    PAGE32 = 32,
    ///33: Erase Page 33
    PAGE33 = 33,
    ///34: Erase Page 34
    PAGE34 = 34,
    ///35: Erase Page 35
    PAGE35 = 35,
    ///36: Erase Page 36
    PAGE36 = 36,
    ///37: Erase Page 37
    PAGE37 = 37,
    ///38: Erase Page 38
    PAGE38 = 38,
    ///39: Erase Page 39
    PAGE39 = 39,
    ///40: Erase Page 40
    PAGE40 = 40,
    ///41: Erase Page 41
    PAGE41 = 41,
    ///42: Erase Page 42
    PAGE42 = 42,
    ///43: Erase Page 43
    PAGE43 = 43,
    ///44: Erase Page 44
    PAGE44 = 44,
    ///45: Erase Page 45
    PAGE45 = 45,
    ///46: Erase Page 46
    PAGE46 = 46,
    ///47: Erase Page 47
    PAGE47 = 47,
    ///48: Erase Page 48
    PAGE48 = 48,
    ///49: Erase Page 49
    PAGE49 = 49,
    ///50: Erase Page 50
    PAGE50 = 50,
    ///51: Erase Page 51
    PAGE51 = 51,
    ///52: Erase Page 52
    PAGE52 = 52,
    ///53: Erase Page 53
    PAGE53 = 53,
    ///54: Erase Page 54
    PAGE54 = 54,
    ///55: Erase Page 55
    PAGE55 = 55,
    ///56: Erase Page 56
    PAGE56 = 56,
    ///57: Erase Page 57
    PAGE57 = 57,
    ///58: Erase Page 58
    PAGE58 = 58,
    ///59: Erase Page 59
    PAGE59 = 59,
    ///60: Erase Page 60
    PAGE60 = 60,
    ///61: Erase Page 61
    PAGE61 = 61,
    ///62: Erase Page 62
    PAGE62 = 62,
    ///63: Erase Page 63
    PAGE63 = 63,
    ///64: Erase Page 64
    PAGE64 = 64,
    ///65: Erase Page 65
    PAGE65 = 65,
    ///66: Erase Page 66
    PAGE66 = 66,
    ///67: Erase Page 67
    PAGE67 = 67,
    ///68: Erase Page 68
    PAGE68 = 68,
    ///69: Erase Page 69
    PAGE69 = 69,
    ///70: Erase Page 70
    PAGE70 = 70,
    ///71: Erase Page 71
    PAGE71 = 71,
    ///72: Erase Page 72
    PAGE72 = 72,
    ///73: Erase Page 73
    PAGE73 = 73,
    ///74: Erase Page 74
    PAGE74 = 74,
    ///75: Erase Page 75
    PAGE75 = 75,
    ///76: Erase Page 76
    PAGE76 = 76,
    ///77: Erase Page 77
    PAGE77 = 77,
    ///78: Erase Page 78
    PAGE78 = 78,
    ///79: Erase Page 79
    PAGE79 = 79,
    ///80: Erase Page 80
    PAGE80 = 80,
    ///81: Erase Page 81
    PAGE81 = 81,
    ///82: Erase Page 82
    PAGE82 = 82,
    ///83: Erase Page 83
    PAGE83 = 83,
    ///84: Erase Page 84
    PAGE84 = 84,
    ///85: Erase Page 85
    PAGE85 = 85,
    ///86: Erase Page 86
    PAGE86 = 86,
    ///87: Erase Page 87
    PAGE87 = 87,
    ///88: Erase Page 88
    PAGE88 = 88,
    ///89: Erase Page 89
    PAGE89 = 89,
    ///90: Erase Page 90
    PAGE90 = 90,
    ///91: Erase Page 91
    PAGE91 = 91,
    ///92: Erase Page 92
    PAGE92 = 92,
    ///93: Erase Page 93
    PAGE93 = 93,
    ///94: Erase Page 94
    PAGE94 = 94,
    ///95: Erase Page 95
    PAGE95 = 95,
    ///96: Erase Page 96
    PAGE96 = 96,
    ///97: Erase Page 97
    PAGE97 = 97,
    ///98: Erase Page 98
    PAGE98 = 98,
    ///99: Erase Page 99
    PAGE99 = 99,
    ///100: Erase Page 100
    PAGE100 = 100,
    ///101: Erase Page 101
    PAGE101 = 101,
    ///102: Erase Page 102
    PAGE102 = 102,
    ///103: Erase Page 103
    PAGE103 = 103,
    ///104: Erase Page 104
    PAGE104 = 104,
    ///105: Erase Page 105
    PAGE105 = 105,
    ///106: Erase Page 106
    PAGE106 = 106,
    ///107: Erase Page 107
    PAGE107 = 107,
    ///108: Erase Page 108
    PAGE108 = 108,
    ///109: Erase Page 109
    PAGE109 = 109,
    ///110: Erase Page 110
    PAGE110 = 110,
    ///111: Erase Page 111
    PAGE111 = 111,
    ///112: Erase Page 112
    PAGE112 = 112,
    ///113: Erase Page 113
    PAGE113 = 113,
    ///114: Erase Page 114
    PAGE114 = 114,
    ///115: Erase Page 115
    PAGE115 = 115,
    ///116: Erase Page 116
    PAGE116 = 116,
    ///117: Erase Page 117
    PAGE117 = 117,
    ///118: Erase Page 118
    PAGE118 = 118,
    ///119: Erase Page 119
    PAGE119 = 119,
    ///120: Erase Page 120
    PAGE120 = 120,
    ///121: Erase Page 121
    PAGE121 = 121,
    ///122: Erase Page 122
    PAGE122 = 122,
    ///123: Erase Page 123
    PAGE123 = 123,
    ///124: Erase Page 124
    PAGE124 = 124,
    ///125: Erase Page 125
    PAGE125 = 125,
    ///126: Erase Page 126
    PAGE126 = 126,
    ///127: Erase Page 127
    PAGE127 = 127,
}
impl From<PNB_A> for u8 {
    #[inline(always)]
    fn from(variant: PNB_A) -> Self {
        variant as _
    }
}
///Reader of field `PNB`
pub type PNB_R = crate::R<u8, PNB_A>;
impl PNB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PNB_A {
        match self.bits {
            0 => PNB_A::PAGE0,
            1 => PNB_A::PAGE1,
            2 => PNB_A::PAGE2,
            3 => PNB_A::PAGE3,
            4 => PNB_A::PAGE4,
            5 => PNB_A::PAGE5,
            6 => PNB_A::PAGE6,
            7 => PNB_A::PAGE7,
            8 => PNB_A::PAGE8,
            9 => PNB_A::PAGE9,
            10 => PNB_A::PAGE10,
            11 => PNB_A::PAGE11,
            12 => PNB_A::PAGE12,
            13 => PNB_A::PAGE13,
            14 => PNB_A::PAGE14,
            15 => PNB_A::PAGE15,
            16 => PNB_A::PAGE16,
            17 => PNB_A::PAGE17,
            18 => PNB_A::PAGE18,
            19 => PNB_A::PAGE19,
            20 => PNB_A::PAGE20,
            21 => PNB_A::PAGE21,
            22 => PNB_A::PAGE22,
            23 => PNB_A::PAGE23,
            24 => PNB_A::PAGE24,
            25 => PNB_A::PAGE25,
            26 => PNB_A::PAGE26,
            27 => PNB_A::PAGE27,
            28 => PNB_A::PAGE28,
            29 => PNB_A::PAGE29,
            30 => PNB_A::PAGE30,
            31 => PNB_A::PAGE31,
            32 => PNB_A::PAGE32,
            33 => PNB_A::PAGE33,
            34 => PNB_A::PAGE34,
            35 => PNB_A::PAGE35,
            36 => PNB_A::PAGE36,
            37 => PNB_A::PAGE37,
            38 => PNB_A::PAGE38,
            39 => PNB_A::PAGE39,
            40 => PNB_A::PAGE40,
            41 => PNB_A::PAGE41,
            42 => PNB_A::PAGE42,
            43 => PNB_A::PAGE43,
            44 => PNB_A::PAGE44,
            45 => PNB_A::PAGE45,
            46 => PNB_A::PAGE46,
            47 => PNB_A::PAGE47,
            48 => PNB_A::PAGE48,
            49 => PNB_A::PAGE49,
            50 => PNB_A::PAGE50,
            51 => PNB_A::PAGE51,
            52 => PNB_A::PAGE52,
            53 => PNB_A::PAGE53,
            54 => PNB_A::PAGE54,
            55 => PNB_A::PAGE55,
            56 => PNB_A::PAGE56,
            57 => PNB_A::PAGE57,
            58 => PNB_A::PAGE58,
            59 => PNB_A::PAGE59,
            60 => PNB_A::PAGE60,
            61 => PNB_A::PAGE61,
            62 => PNB_A::PAGE62,
            63 => PNB_A::PAGE63,
            64 => PNB_A::PAGE64,
            65 => PNB_A::PAGE65,
            66 => PNB_A::PAGE66,
            67 => PNB_A::PAGE67,
            68 => PNB_A::PAGE68,
            69 => PNB_A::PAGE69,
            70 => PNB_A::PAGE70,
            71 => PNB_A::PAGE71,
            72 => PNB_A::PAGE72,
            73 => PNB_A::PAGE73,
            74 => PNB_A::PAGE74,
            75 => PNB_A::PAGE75,
            76 => PNB_A::PAGE76,
            77 => PNB_A::PAGE77,
            78 => PNB_A::PAGE78,
            79 => PNB_A::PAGE79,
            80 => PNB_A::PAGE80,
            81 => PNB_A::PAGE81,
            82 => PNB_A::PAGE82,
            83 => PNB_A::PAGE83,
            84 => PNB_A::PAGE84,
            85 => PNB_A::PAGE85,
            86 => PNB_A::PAGE86,
            87 => PNB_A::PAGE87,
            88 => PNB_A::PAGE88,
            89 => PNB_A::PAGE89,
            90 => PNB_A::PAGE90,
            91 => PNB_A::PAGE91,
            92 => PNB_A::PAGE92,
            93 => PNB_A::PAGE93,
            94 => PNB_A::PAGE94,
            95 => PNB_A::PAGE95,
            96 => PNB_A::PAGE96,
            97 => PNB_A::PAGE97,
            98 => PNB_A::PAGE98,
            99 => PNB_A::PAGE99,
            100 => PNB_A::PAGE100,
            101 => PNB_A::PAGE101,
            102 => PNB_A::PAGE102,
            103 => PNB_A::PAGE103,
            104 => PNB_A::PAGE104,
            105 => PNB_A::PAGE105,
            106 => PNB_A::PAGE106,
            107 => PNB_A::PAGE107,
            108 => PNB_A::PAGE108,
            109 => PNB_A::PAGE109,
            110 => PNB_A::PAGE110,
            111 => PNB_A::PAGE111,
            112 => PNB_A::PAGE112,
            113 => PNB_A::PAGE113,
            114 => PNB_A::PAGE114,
            115 => PNB_A::PAGE115,
            116 => PNB_A::PAGE116,
            117 => PNB_A::PAGE117,
            118 => PNB_A::PAGE118,
            119 => PNB_A::PAGE119,
            120 => PNB_A::PAGE120,
            121 => PNB_A::PAGE121,
            122 => PNB_A::PAGE122,
            123 => PNB_A::PAGE123,
            124 => PNB_A::PAGE124,
            125 => PNB_A::PAGE125,
            126 => PNB_A::PAGE126,
            127 => PNB_A::PAGE127,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `PAGE0`
    #[inline(always)]
    pub fn is_page0(&self) -> bool {
        *self == PNB_A::PAGE0
    }
    ///Checks if the value of the field is `PAGE1`
    #[inline(always)]
    pub fn is_page1(&self) -> bool {
        *self == PNB_A::PAGE1
    }
    ///Checks if the value of the field is `PAGE2`
    #[inline(always)]
    pub fn is_page2(&self) -> bool {
        *self == PNB_A::PAGE2
    }
    ///Checks if the value of the field is `PAGE3`
    #[inline(always)]
    pub fn is_page3(&self) -> bool {
        *self == PNB_A::PAGE3
    }
    ///Checks if the value of the field is `PAGE4`
    #[inline(always)]
    pub fn is_page4(&self) -> bool {
        *self == PNB_A::PAGE4
    }
    ///Checks if the value of the field is `PAGE5`
    #[inline(always)]
    pub fn is_page5(&self) -> bool {
        *self == PNB_A::PAGE5
    }
    ///Checks if the value of the field is `PAGE6`
    #[inline(always)]
    pub fn is_page6(&self) -> bool {
        *self == PNB_A::PAGE6
    }
    ///Checks if the value of the field is `PAGE7`
    #[inline(always)]
    pub fn is_page7(&self) -> bool {
        *self == PNB_A::PAGE7
    }
    ///Checks if the value of the field is `PAGE8`
    #[inline(always)]
    pub fn is_page8(&self) -> bool {
        *self == PNB_A::PAGE8
    }
    ///Checks if the value of the field is `PAGE9`
    #[inline(always)]
    pub fn is_page9(&self) -> bool {
        *self == PNB_A::PAGE9
    }
    ///Checks if the value of the field is `PAGE10`
    #[inline(always)]
    pub fn is_page10(&self) -> bool {
        *self == PNB_A::PAGE10
    }
    ///Checks if the value of the field is `PAGE11`
    #[inline(always)]
    pub fn is_page11(&self) -> bool {
        *self == PNB_A::PAGE11
    }
    ///Checks if the value of the field is `PAGE12`
    #[inline(always)]
    pub fn is_page12(&self) -> bool {
        *self == PNB_A::PAGE12
    }
    ///Checks if the value of the field is `PAGE13`
    #[inline(always)]
    pub fn is_page13(&self) -> bool {
        *self == PNB_A::PAGE13
    }
    ///Checks if the value of the field is `PAGE14`
    #[inline(always)]
    pub fn is_page14(&self) -> bool {
        *self == PNB_A::PAGE14
    }
    ///Checks if the value of the field is `PAGE15`
    #[inline(always)]
    pub fn is_page15(&self) -> bool {
        *self == PNB_A::PAGE15
    }
    ///Checks if the value of the field is `PAGE16`
    #[inline(always)]
    pub fn is_page16(&self) -> bool {
        *self == PNB_A::PAGE16
    }
    ///Checks if the value of the field is `PAGE17`
    #[inline(always)]
    pub fn is_page17(&self) -> bool {
        *self == PNB_A::PAGE17
    }
    ///Checks if the value of the field is `PAGE18`
    #[inline(always)]
    pub fn is_page18(&self) -> bool {
        *self == PNB_A::PAGE18
    }
    ///Checks if the value of the field is `PAGE19`
    #[inline(always)]
    pub fn is_page19(&self) -> bool {
        *self == PNB_A::PAGE19
    }
    ///Checks if the value of the field is `PAGE20`
    #[inline(always)]
    pub fn is_page20(&self) -> bool {
        *self == PNB_A::PAGE20
    }
    ///Checks if the value of the field is `PAGE21`
    #[inline(always)]
    pub fn is_page21(&self) -> bool {
        *self == PNB_A::PAGE21
    }
    ///Checks if the value of the field is `PAGE22`
    #[inline(always)]
    pub fn is_page22(&self) -> bool {
        *self == PNB_A::PAGE22
    }
    ///Checks if the value of the field is `PAGE23`
    #[inline(always)]
    pub fn is_page23(&self) -> bool {
        *self == PNB_A::PAGE23
    }
    ///Checks if the value of the field is `PAGE24`
    #[inline(always)]
    pub fn is_page24(&self) -> bool {
        *self == PNB_A::PAGE24
    }
    ///Checks if the value of the field is `PAGE25`
    #[inline(always)]
    pub fn is_page25(&self) -> bool {
        *self == PNB_A::PAGE25
    }
    ///Checks if the value of the field is `PAGE26`
    #[inline(always)]
    pub fn is_page26(&self) -> bool {
        *self == PNB_A::PAGE26
    }
    ///Checks if the value of the field is `PAGE27`
    #[inline(always)]
    pub fn is_page27(&self) -> bool {
        *self == PNB_A::PAGE27
    }
    ///Checks if the value of the field is `PAGE28`
    #[inline(always)]
    pub fn is_page28(&self) -> bool {
        *self == PNB_A::PAGE28
    }
    ///Checks if the value of the field is `PAGE29`
    #[inline(always)]
    pub fn is_page29(&self) -> bool {
        *self == PNB_A::PAGE29
    }
    ///Checks if the value of the field is `PAGE30`
    #[inline(always)]
    pub fn is_page30(&self) -> bool {
        *self == PNB_A::PAGE30
    }
    ///Checks if the value of the field is `PAGE31`
    #[inline(always)]
    pub fn is_page31(&self) -> bool {
        *self == PNB_A::PAGE31
    }
    ///Checks if the value of the field is `PAGE32`
    #[inline(always)]
    pub fn is_page32(&self) -> bool {
        *self == PNB_A::PAGE32
    }
    ///Checks if the value of the field is `PAGE33`
    #[inline(always)]
    pub fn is_page33(&self) -> bool {
        *self == PNB_A::PAGE33
    }
    ///Checks if the value of the field is `PAGE34`
    #[inline(always)]
    pub fn is_page34(&self) -> bool {
        *self == PNB_A::PAGE34
    }
    ///Checks if the value of the field is `PAGE35`
    #[inline(always)]
    pub fn is_page35(&self) -> bool {
        *self == PNB_A::PAGE35
    }
    ///Checks if the value of the field is `PAGE36`
    #[inline(always)]
    pub fn is_page36(&self) -> bool {
        *self == PNB_A::PAGE36
    }
    ///Checks if the value of the field is `PAGE37`
    #[inline(always)]
    pub fn is_page37(&self) -> bool {
        *self == PNB_A::PAGE37
    }
    ///Checks if the value of the field is `PAGE38`
    #[inline(always)]
    pub fn is_page38(&self) -> bool {
        *self == PNB_A::PAGE38
    }
    ///Checks if the value of the field is `PAGE39`
    #[inline(always)]
    pub fn is_page39(&self) -> bool {
        *self == PNB_A::PAGE39
    }
    ///Checks if the value of the field is `PAGE40`
    #[inline(always)]
    pub fn is_page40(&self) -> bool {
        *self == PNB_A::PAGE40
    }
    ///Checks if the value of the field is `PAGE41`
    #[inline(always)]
    pub fn is_page41(&self) -> bool {
        *self == PNB_A::PAGE41
    }
    ///Checks if the value of the field is `PAGE42`
    #[inline(always)]
    pub fn is_page42(&self) -> bool {
        *self == PNB_A::PAGE42
    }
    ///Checks if the value of the field is `PAGE43`
    #[inline(always)]
    pub fn is_page43(&self) -> bool {
        *self == PNB_A::PAGE43
    }
    ///Checks if the value of the field is `PAGE44`
    #[inline(always)]
    pub fn is_page44(&self) -> bool {
        *self == PNB_A::PAGE44
    }
    ///Checks if the value of the field is `PAGE45`
    #[inline(always)]
    pub fn is_page45(&self) -> bool {
        *self == PNB_A::PAGE45
    }
    ///Checks if the value of the field is `PAGE46`
    #[inline(always)]
    pub fn is_page46(&self) -> bool {
        *self == PNB_A::PAGE46
    }
    ///Checks if the value of the field is `PAGE47`
    #[inline(always)]
    pub fn is_page47(&self) -> bool {
        *self == PNB_A::PAGE47
    }
    ///Checks if the value of the field is `PAGE48`
    #[inline(always)]
    pub fn is_page48(&self) -> bool {
        *self == PNB_A::PAGE48
    }
    ///Checks if the value of the field is `PAGE49`
    #[inline(always)]
    pub fn is_page49(&self) -> bool {
        *self == PNB_A::PAGE49
    }
    ///Checks if the value of the field is `PAGE50`
    #[inline(always)]
    pub fn is_page50(&self) -> bool {
        *self == PNB_A::PAGE50
    }
    ///Checks if the value of the field is `PAGE51`
    #[inline(always)]
    pub fn is_page51(&self) -> bool {
        *self == PNB_A::PAGE51
    }
    ///Checks if the value of the field is `PAGE52`
    #[inline(always)]
    pub fn is_page52(&self) -> bool {
        *self == PNB_A::PAGE52
    }
    ///Checks if the value of the field is `PAGE53`
    #[inline(always)]
    pub fn is_page53(&self) -> bool {
        *self == PNB_A::PAGE53
    }
    ///Checks if the value of the field is `PAGE54`
    #[inline(always)]
    pub fn is_page54(&self) -> bool {
        *self == PNB_A::PAGE54
    }
    ///Checks if the value of the field is `PAGE55`
    #[inline(always)]
    pub fn is_page55(&self) -> bool {
        *self == PNB_A::PAGE55
    }
    ///Checks if the value of the field is `PAGE56`
    #[inline(always)]
    pub fn is_page56(&self) -> bool {
        *self == PNB_A::PAGE56
    }
    ///Checks if the value of the field is `PAGE57`
    #[inline(always)]
    pub fn is_page57(&self) -> bool {
        *self == PNB_A::PAGE57
    }
    ///Checks if the value of the field is `PAGE58`
    #[inline(always)]
    pub fn is_page58(&self) -> bool {
        *self == PNB_A::PAGE58
    }
    ///Checks if the value of the field is `PAGE59`
    #[inline(always)]
    pub fn is_page59(&self) -> bool {
        *self == PNB_A::PAGE59
    }
    ///Checks if the value of the field is `PAGE60`
    #[inline(always)]
    pub fn is_page60(&self) -> bool {
        *self == PNB_A::PAGE60
    }
    ///Checks if the value of the field is `PAGE61`
    #[inline(always)]
    pub fn is_page61(&self) -> bool {
        *self == PNB_A::PAGE61
    }
    ///Checks if the value of the field is `PAGE62`
    #[inline(always)]
    pub fn is_page62(&self) -> bool {
        *self == PNB_A::PAGE62
    }
    ///Checks if the value of the field is `PAGE63`
    #[inline(always)]
    pub fn is_page63(&self) -> bool {
        *self == PNB_A::PAGE63
    }
    ///Checks if the value of the field is `PAGE64`
    #[inline(always)]
    pub fn is_page64(&self) -> bool {
        *self == PNB_A::PAGE64
    }
    ///Checks if the value of the field is `PAGE65`
    #[inline(always)]
    pub fn is_page65(&self) -> bool {
        *self == PNB_A::PAGE65
    }
    ///Checks if the value of the field is `PAGE66`
    #[inline(always)]
    pub fn is_page66(&self) -> bool {
        *self == PNB_A::PAGE66
    }
    ///Checks if the value of the field is `PAGE67`
    #[inline(always)]
    pub fn is_page67(&self) -> bool {
        *self == PNB_A::PAGE67
    }
    ///Checks if the value of the field is `PAGE68`
    #[inline(always)]
    pub fn is_page68(&self) -> bool {
        *self == PNB_A::PAGE68
    }
    ///Checks if the value of the field is `PAGE69`
    #[inline(always)]
    pub fn is_page69(&self) -> bool {
        *self == PNB_A::PAGE69
    }
    ///Checks if the value of the field is `PAGE70`
    #[inline(always)]
    pub fn is_page70(&self) -> bool {
        *self == PNB_A::PAGE70
    }
    ///Checks if the value of the field is `PAGE71`
    #[inline(always)]
    pub fn is_page71(&self) -> bool {
        *self == PNB_A::PAGE71
    }
    ///Checks if the value of the field is `PAGE72`
    #[inline(always)]
    pub fn is_page72(&self) -> bool {
        *self == PNB_A::PAGE72
    }
    ///Checks if the value of the field is `PAGE73`
    #[inline(always)]
    pub fn is_page73(&self) -> bool {
        *self == PNB_A::PAGE73
    }
    ///Checks if the value of the field is `PAGE74`
    #[inline(always)]
    pub fn is_page74(&self) -> bool {
        *self == PNB_A::PAGE74
    }
    ///Checks if the value of the field is `PAGE75`
    #[inline(always)]
    pub fn is_page75(&self) -> bool {
        *self == PNB_A::PAGE75
    }
    ///Checks if the value of the field is `PAGE76`
    #[inline(always)]
    pub fn is_page76(&self) -> bool {
        *self == PNB_A::PAGE76
    }
    ///Checks if the value of the field is `PAGE77`
    #[inline(always)]
    pub fn is_page77(&self) -> bool {
        *self == PNB_A::PAGE77
    }
    ///Checks if the value of the field is `PAGE78`
    #[inline(always)]
    pub fn is_page78(&self) -> bool {
        *self == PNB_A::PAGE78
    }
    ///Checks if the value of the field is `PAGE79`
    #[inline(always)]
    pub fn is_page79(&self) -> bool {
        *self == PNB_A::PAGE79
    }
    ///Checks if the value of the field is `PAGE80`
    #[inline(always)]
    pub fn is_page80(&self) -> bool {
        *self == PNB_A::PAGE80
    }
    ///Checks if the value of the field is `PAGE81`
    #[inline(always)]
    pub fn is_page81(&self) -> bool {
        *self == PNB_A::PAGE81
    }
    ///Checks if the value of the field is `PAGE82`
    #[inline(always)]
    pub fn is_page82(&self) -> bool {
        *self == PNB_A::PAGE82
    }
    ///Checks if the value of the field is `PAGE83`
    #[inline(always)]
    pub fn is_page83(&self) -> bool {
        *self == PNB_A::PAGE83
    }
    ///Checks if the value of the field is `PAGE84`
    #[inline(always)]
    pub fn is_page84(&self) -> bool {
        *self == PNB_A::PAGE84
    }
    ///Checks if the value of the field is `PAGE85`
    #[inline(always)]
    pub fn is_page85(&self) -> bool {
        *self == PNB_A::PAGE85
    }
    ///Checks if the value of the field is `PAGE86`
    #[inline(always)]
    pub fn is_page86(&self) -> bool {
        *self == PNB_A::PAGE86
    }
    ///Checks if the value of the field is `PAGE87`
    #[inline(always)]
    pub fn is_page87(&self) -> bool {
        *self == PNB_A::PAGE87
    }
    ///Checks if the value of the field is `PAGE88`
    #[inline(always)]
    pub fn is_page88(&self) -> bool {
        *self == PNB_A::PAGE88
    }
    ///Checks if the value of the field is `PAGE89`
    #[inline(always)]
    pub fn is_page89(&self) -> bool {
        *self == PNB_A::PAGE89
    }
    ///Checks if the value of the field is `PAGE90`
    #[inline(always)]
    pub fn is_page90(&self) -> bool {
        *self == PNB_A::PAGE90
    }
    ///Checks if the value of the field is `PAGE91`
    #[inline(always)]
    pub fn is_page91(&self) -> bool {
        *self == PNB_A::PAGE91
    }
    ///Checks if the value of the field is `PAGE92`
    #[inline(always)]
    pub fn is_page92(&self) -> bool {
        *self == PNB_A::PAGE92
    }
    ///Checks if the value of the field is `PAGE93`
    #[inline(always)]
    pub fn is_page93(&self) -> bool {
        *self == PNB_A::PAGE93
    }
    ///Checks if the value of the field is `PAGE94`
    #[inline(always)]
    pub fn is_page94(&self) -> bool {
        *self == PNB_A::PAGE94
    }
    ///Checks if the value of the field is `PAGE95`
    #[inline(always)]
    pub fn is_page95(&self) -> bool {
        *self == PNB_A::PAGE95
    }
    ///Checks if the value of the field is `PAGE96`
    #[inline(always)]
    pub fn is_page96(&self) -> bool {
        *self == PNB_A::PAGE96
    }
    ///Checks if the value of the field is `PAGE97`
    #[inline(always)]
    pub fn is_page97(&self) -> bool {
        *self == PNB_A::PAGE97
    }
    ///Checks if the value of the field is `PAGE98`
    #[inline(always)]
    pub fn is_page98(&self) -> bool {
        *self == PNB_A::PAGE98
    }
    ///Checks if the value of the field is `PAGE99`
    #[inline(always)]
    pub fn is_page99(&self) -> bool {
        *self == PNB_A::PAGE99
    }
    ///Checks if the value of the field is `PAGE100`
    #[inline(always)]
    pub fn is_page100(&self) -> bool {
        *self == PNB_A::PAGE100
    }
    ///Checks if the value of the field is `PAGE101`
    #[inline(always)]
    pub fn is_page101(&self) -> bool {
        *self == PNB_A::PAGE101
    }
    ///Checks if the value of the field is `PAGE102`
    #[inline(always)]
    pub fn is_page102(&self) -> bool {
        *self == PNB_A::PAGE102
    }
    ///Checks if the value of the field is `PAGE103`
    #[inline(always)]
    pub fn is_page103(&self) -> bool {
        *self == PNB_A::PAGE103
    }
    ///Checks if the value of the field is `PAGE104`
    #[inline(always)]
    pub fn is_page104(&self) -> bool {
        *self == PNB_A::PAGE104
    }
    ///Checks if the value of the field is `PAGE105`
    #[inline(always)]
    pub fn is_page105(&self) -> bool {
        *self == PNB_A::PAGE105
    }
    ///Checks if the value of the field is `PAGE106`
    #[inline(always)]
    pub fn is_page106(&self) -> bool {
        *self == PNB_A::PAGE106
    }
    ///Checks if the value of the field is `PAGE107`
    #[inline(always)]
    pub fn is_page107(&self) -> bool {
        *self == PNB_A::PAGE107
    }
    ///Checks if the value of the field is `PAGE108`
    #[inline(always)]
    pub fn is_page108(&self) -> bool {
        *self == PNB_A::PAGE108
    }
    ///Checks if the value of the field is `PAGE109`
    #[inline(always)]
    pub fn is_page109(&self) -> bool {
        *self == PNB_A::PAGE109
    }
    ///Checks if the value of the field is `PAGE110`
    #[inline(always)]
    pub fn is_page110(&self) -> bool {
        *self == PNB_A::PAGE110
    }
    ///Checks if the value of the field is `PAGE111`
    #[inline(always)]
    pub fn is_page111(&self) -> bool {
        *self == PNB_A::PAGE111
    }
    ///Checks if the value of the field is `PAGE112`
    #[inline(always)]
    pub fn is_page112(&self) -> bool {
        *self == PNB_A::PAGE112
    }
    ///Checks if the value of the field is `PAGE113`
    #[inline(always)]
    pub fn is_page113(&self) -> bool {
        *self == PNB_A::PAGE113
    }
    ///Checks if the value of the field is `PAGE114`
    #[inline(always)]
    pub fn is_page114(&self) -> bool {
        *self == PNB_A::PAGE114
    }
    ///Checks if the value of the field is `PAGE115`
    #[inline(always)]
    pub fn is_page115(&self) -> bool {
        *self == PNB_A::PAGE115
    }
    ///Checks if the value of the field is `PAGE116`
    #[inline(always)]
    pub fn is_page116(&self) -> bool {
        *self == PNB_A::PAGE116
    }
    ///Checks if the value of the field is `PAGE117`
    #[inline(always)]
    pub fn is_page117(&self) -> bool {
        *self == PNB_A::PAGE117
    }
    ///Checks if the value of the field is `PAGE118`
    #[inline(always)]
    pub fn is_page118(&self) -> bool {
        *self == PNB_A::PAGE118
    }
    ///Checks if the value of the field is `PAGE119`
    #[inline(always)]
    pub fn is_page119(&self) -> bool {
        *self == PNB_A::PAGE119
    }
    ///Checks if the value of the field is `PAGE120`
    #[inline(always)]
    pub fn is_page120(&self) -> bool {
        *self == PNB_A::PAGE120
    }
    ///Checks if the value of the field is `PAGE121`
    #[inline(always)]
    pub fn is_page121(&self) -> bool {
        *self == PNB_A::PAGE121
    }
    ///Checks if the value of the field is `PAGE122`
    #[inline(always)]
    pub fn is_page122(&self) -> bool {
        *self == PNB_A::PAGE122
    }
    ///Checks if the value of the field is `PAGE123`
    #[inline(always)]
    pub fn is_page123(&self) -> bool {
        *self == PNB_A::PAGE123
    }
    ///Checks if the value of the field is `PAGE124`
    #[inline(always)]
    pub fn is_page124(&self) -> bool {
        *self == PNB_A::PAGE124
    }
    ///Checks if the value of the field is `PAGE125`
    #[inline(always)]
    pub fn is_page125(&self) -> bool {
        *self == PNB_A::PAGE125
    }
    ///Checks if the value of the field is `PAGE126`
    #[inline(always)]
    pub fn is_page126(&self) -> bool {
        *self == PNB_A::PAGE126
    }
    ///Checks if the value of the field is `PAGE127`
    #[inline(always)]
    pub fn is_page127(&self) -> bool {
        *self == PNB_A::PAGE127
    }
}
///Write proxy for field `PNB`
pub struct PNB_W<'a> {
    w: &'a mut W,
}
impl<'a> PNB_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PNB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Erase Page 0
    #[inline(always)]
    pub fn page0(self) -> &'a mut W {
        self.variant(PNB_A::PAGE0)
    }
    ///Erase Page 1
    #[inline(always)]
    pub fn page1(self) -> &'a mut W {
        self.variant(PNB_A::PAGE1)
    }
    ///Erase Page 2
    #[inline(always)]
    pub fn page2(self) -> &'a mut W {
        self.variant(PNB_A::PAGE2)
    }
    ///Erase Page 3
    #[inline(always)]
    pub fn page3(self) -> &'a mut W {
        self.variant(PNB_A::PAGE3)
    }
    ///Erase Page 4
    #[inline(always)]
    pub fn page4(self) -> &'a mut W {
        self.variant(PNB_A::PAGE4)
    }
    ///Erase Page 5
    #[inline(always)]
    pub fn page5(self) -> &'a mut W {
        self.variant(PNB_A::PAGE5)
    }
    ///Erase Page 6
    #[inline(always)]
    pub fn page6(self) -> &'a mut W {
        self.variant(PNB_A::PAGE6)
    }
    ///Erase Page 7
    #[inline(always)]
    pub fn page7(self) -> &'a mut W {
        self.variant(PNB_A::PAGE7)
    }
    ///Erase Page 8
    #[inline(always)]
    pub fn page8(self) -> &'a mut W {
        self.variant(PNB_A::PAGE8)
    }
    ///Erase Page 9
    #[inline(always)]
    pub fn page9(self) -> &'a mut W {
        self.variant(PNB_A::PAGE9)
    }
    ///Erase Page 10
    #[inline(always)]
    pub fn page10(self) -> &'a mut W {
        self.variant(PNB_A::PAGE10)
    }
    ///Erase Page 11
    #[inline(always)]
    pub fn page11(self) -> &'a mut W {
        self.variant(PNB_A::PAGE11)
    }
    ///Erase Page 12
    #[inline(always)]
    pub fn page12(self) -> &'a mut W {
        self.variant(PNB_A::PAGE12)
    }
    ///Erase Page 13
    #[inline(always)]
    pub fn page13(self) -> &'a mut W {
        self.variant(PNB_A::PAGE13)
    }
    ///Erase Page 14
    #[inline(always)]
    pub fn page14(self) -> &'a mut W {
        self.variant(PNB_A::PAGE14)
    }
    ///Erase Page 15
    #[inline(always)]
    pub fn page15(self) -> &'a mut W {
        self.variant(PNB_A::PAGE15)
    }
    ///Erase Page 16
    #[inline(always)]
    pub fn page16(self) -> &'a mut W {
        self.variant(PNB_A::PAGE16)
    }
    ///Erase Page 17
    #[inline(always)]
    pub fn page17(self) -> &'a mut W {
        self.variant(PNB_A::PAGE17)
    }
    ///Erase Page 18
    #[inline(always)]
    pub fn page18(self) -> &'a mut W {
        self.variant(PNB_A::PAGE18)
    }
    ///Erase Page 19
    #[inline(always)]
    pub fn page19(self) -> &'a mut W {
        self.variant(PNB_A::PAGE19)
    }
    ///Erase Page 20
    #[inline(always)]
    pub fn page20(self) -> &'a mut W {
        self.variant(PNB_A::PAGE20)
    }
    ///Erase Page 21
    #[inline(always)]
    pub fn page21(self) -> &'a mut W {
        self.variant(PNB_A::PAGE21)
    }
    ///Erase Page 22
    #[inline(always)]
    pub fn page22(self) -> &'a mut W {
        self.variant(PNB_A::PAGE22)
    }
    ///Erase Page 23
    #[inline(always)]
    pub fn page23(self) -> &'a mut W {
        self.variant(PNB_A::PAGE23)
    }
    ///Erase Page 24
    #[inline(always)]
    pub fn page24(self) -> &'a mut W {
        self.variant(PNB_A::PAGE24)
    }
    ///Erase Page 25
    #[inline(always)]
    pub fn page25(self) -> &'a mut W {
        self.variant(PNB_A::PAGE25)
    }
    ///Erase Page 26
    #[inline(always)]
    pub fn page26(self) -> &'a mut W {
        self.variant(PNB_A::PAGE26)
    }
    ///Erase Page 27
    #[inline(always)]
    pub fn page27(self) -> &'a mut W {
        self.variant(PNB_A::PAGE27)
    }
    ///Erase Page 28
    #[inline(always)]
    pub fn page28(self) -> &'a mut W {
        self.variant(PNB_A::PAGE28)
    }
    ///Erase Page 29
    #[inline(always)]
    pub fn page29(self) -> &'a mut W {
        self.variant(PNB_A::PAGE29)
    }
    ///Erase Page 30
    #[inline(always)]
    pub fn page30(self) -> &'a mut W {
        self.variant(PNB_A::PAGE30)
    }
    ///Erase Page 31
    #[inline(always)]
    pub fn page31(self) -> &'a mut W {
        self.variant(PNB_A::PAGE31)
    }
    ///Erase Page 32
    #[inline(always)]
    pub fn page32(self) -> &'a mut W {
        self.variant(PNB_A::PAGE32)
    }
    ///Erase Page 33
    #[inline(always)]
    pub fn page33(self) -> &'a mut W {
        self.variant(PNB_A::PAGE33)
    }
    ///Erase Page 34
    #[inline(always)]
    pub fn page34(self) -> &'a mut W {
        self.variant(PNB_A::PAGE34)
    }
    ///Erase Page 35
    #[inline(always)]
    pub fn page35(self) -> &'a mut W {
        self.variant(PNB_A::PAGE35)
    }
    ///Erase Page 36
    #[inline(always)]
    pub fn page36(self) -> &'a mut W {
        self.variant(PNB_A::PAGE36)
    }
    ///Erase Page 37
    #[inline(always)]
    pub fn page37(self) -> &'a mut W {
        self.variant(PNB_A::PAGE37)
    }
    ///Erase Page 38
    #[inline(always)]
    pub fn page38(self) -> &'a mut W {
        self.variant(PNB_A::PAGE38)
    }
    ///Erase Page 39
    #[inline(always)]
    pub fn page39(self) -> &'a mut W {
        self.variant(PNB_A::PAGE39)
    }
    ///Erase Page 40
    #[inline(always)]
    pub fn page40(self) -> &'a mut W {
        self.variant(PNB_A::PAGE40)
    }
    ///Erase Page 41
    #[inline(always)]
    pub fn page41(self) -> &'a mut W {
        self.variant(PNB_A::PAGE41)
    }
    ///Erase Page 42
    #[inline(always)]
    pub fn page42(self) -> &'a mut W {
        self.variant(PNB_A::PAGE42)
    }
    ///Erase Page 43
    #[inline(always)]
    pub fn page43(self) -> &'a mut W {
        self.variant(PNB_A::PAGE43)
    }
    ///Erase Page 44
    #[inline(always)]
    pub fn page44(self) -> &'a mut W {
        self.variant(PNB_A::PAGE44)
    }
    ///Erase Page 45
    #[inline(always)]
    pub fn page45(self) -> &'a mut W {
        self.variant(PNB_A::PAGE45)
    }
    ///Erase Page 46
    #[inline(always)]
    pub fn page46(self) -> &'a mut W {
        self.variant(PNB_A::PAGE46)
    }
    ///Erase Page 47
    #[inline(always)]
    pub fn page47(self) -> &'a mut W {
        self.variant(PNB_A::PAGE47)
    }
    ///Erase Page 48
    #[inline(always)]
    pub fn page48(self) -> &'a mut W {
        self.variant(PNB_A::PAGE48)
    }
    ///Erase Page 49
    #[inline(always)]
    pub fn page49(self) -> &'a mut W {
        self.variant(PNB_A::PAGE49)
    }
    ///Erase Page 50
    #[inline(always)]
    pub fn page50(self) -> &'a mut W {
        self.variant(PNB_A::PAGE50)
    }
    ///Erase Page 51
    #[inline(always)]
    pub fn page51(self) -> &'a mut W {
        self.variant(PNB_A::PAGE51)
    }
    ///Erase Page 52
    #[inline(always)]
    pub fn page52(self) -> &'a mut W {
        self.variant(PNB_A::PAGE52)
    }
    ///Erase Page 53
    #[inline(always)]
    pub fn page53(self) -> &'a mut W {
        self.variant(PNB_A::PAGE53)
    }
    ///Erase Page 54
    #[inline(always)]
    pub fn page54(self) -> &'a mut W {
        self.variant(PNB_A::PAGE54)
    }
    ///Erase Page 55
    #[inline(always)]
    pub fn page55(self) -> &'a mut W {
        self.variant(PNB_A::PAGE55)
    }
    ///Erase Page 56
    #[inline(always)]
    pub fn page56(self) -> &'a mut W {
        self.variant(PNB_A::PAGE56)
    }
    ///Erase Page 57
    #[inline(always)]
    pub fn page57(self) -> &'a mut W {
        self.variant(PNB_A::PAGE57)
    }
    ///Erase Page 58
    #[inline(always)]
    pub fn page58(self) -> &'a mut W {
        self.variant(PNB_A::PAGE58)
    }
    ///Erase Page 59
    #[inline(always)]
    pub fn page59(self) -> &'a mut W {
        self.variant(PNB_A::PAGE59)
    }
    ///Erase Page 60
    #[inline(always)]
    pub fn page60(self) -> &'a mut W {
        self.variant(PNB_A::PAGE60)
    }
    ///Erase Page 61
    #[inline(always)]
    pub fn page61(self) -> &'a mut W {
        self.variant(PNB_A::PAGE61)
    }
    ///Erase Page 62
    #[inline(always)]
    pub fn page62(self) -> &'a mut W {
        self.variant(PNB_A::PAGE62)
    }
    ///Erase Page 63
    #[inline(always)]
    pub fn page63(self) -> &'a mut W {
        self.variant(PNB_A::PAGE63)
    }
    ///Erase Page 64
    #[inline(always)]
    pub fn page64(self) -> &'a mut W {
        self.variant(PNB_A::PAGE64)
    }
    ///Erase Page 65
    #[inline(always)]
    pub fn page65(self) -> &'a mut W {
        self.variant(PNB_A::PAGE65)
    }
    ///Erase Page 66
    #[inline(always)]
    pub fn page66(self) -> &'a mut W {
        self.variant(PNB_A::PAGE66)
    }
    ///Erase Page 67
    #[inline(always)]
    pub fn page67(self) -> &'a mut W {
        self.variant(PNB_A::PAGE67)
    }
    ///Erase Page 68
    #[inline(always)]
    pub fn page68(self) -> &'a mut W {
        self.variant(PNB_A::PAGE68)
    }
    ///Erase Page 69
    #[inline(always)]
    pub fn page69(self) -> &'a mut W {
        self.variant(PNB_A::PAGE69)
    }
    ///Erase Page 70
    #[inline(always)]
    pub fn page70(self) -> &'a mut W {
        self.variant(PNB_A::PAGE70)
    }
    ///Erase Page 71
    #[inline(always)]
    pub fn page71(self) -> &'a mut W {
        self.variant(PNB_A::PAGE71)
    }
    ///Erase Page 72
    #[inline(always)]
    pub fn page72(self) -> &'a mut W {
        self.variant(PNB_A::PAGE72)
    }
    ///Erase Page 73
    #[inline(always)]
    pub fn page73(self) -> &'a mut W {
        self.variant(PNB_A::PAGE73)
    }
    ///Erase Page 74
    #[inline(always)]
    pub fn page74(self) -> &'a mut W {
        self.variant(PNB_A::PAGE74)
    }
    ///Erase Page 75
    #[inline(always)]
    pub fn page75(self) -> &'a mut W {
        self.variant(PNB_A::PAGE75)
    }
    ///Erase Page 76
    #[inline(always)]
    pub fn page76(self) -> &'a mut W {
        self.variant(PNB_A::PAGE76)
    }
    ///Erase Page 77
    #[inline(always)]
    pub fn page77(self) -> &'a mut W {
        self.variant(PNB_A::PAGE77)
    }
    ///Erase Page 78
    #[inline(always)]
    pub fn page78(self) -> &'a mut W {
        self.variant(PNB_A::PAGE78)
    }
    ///Erase Page 79
    #[inline(always)]
    pub fn page79(self) -> &'a mut W {
        self.variant(PNB_A::PAGE79)
    }
    ///Erase Page 80
    #[inline(always)]
    pub fn page80(self) -> &'a mut W {
        self.variant(PNB_A::PAGE80)
    }
    ///Erase Page 81
    #[inline(always)]
    pub fn page81(self) -> &'a mut W {
        self.variant(PNB_A::PAGE81)
    }
    ///Erase Page 82
    #[inline(always)]
    pub fn page82(self) -> &'a mut W {
        self.variant(PNB_A::PAGE82)
    }
    ///Erase Page 83
    #[inline(always)]
    pub fn page83(self) -> &'a mut W {
        self.variant(PNB_A::PAGE83)
    }
    ///Erase Page 84
    #[inline(always)]
    pub fn page84(self) -> &'a mut W {
        self.variant(PNB_A::PAGE84)
    }
    ///Erase Page 85
    #[inline(always)]
    pub fn page85(self) -> &'a mut W {
        self.variant(PNB_A::PAGE85)
    }
    ///Erase Page 86
    #[inline(always)]
    pub fn page86(self) -> &'a mut W {
        self.variant(PNB_A::PAGE86)
    }
    ///Erase Page 87
    #[inline(always)]
    pub fn page87(self) -> &'a mut W {
        self.variant(PNB_A::PAGE87)
    }
    ///Erase Page 88
    #[inline(always)]
    pub fn page88(self) -> &'a mut W {
        self.variant(PNB_A::PAGE88)
    }
    ///Erase Page 89
    #[inline(always)]
    pub fn page89(self) -> &'a mut W {
        self.variant(PNB_A::PAGE89)
    }
    ///Erase Page 90
    #[inline(always)]
    pub fn page90(self) -> &'a mut W {
        self.variant(PNB_A::PAGE90)
    }
    ///Erase Page 91
    #[inline(always)]
    pub fn page91(self) -> &'a mut W {
        self.variant(PNB_A::PAGE91)
    }
    ///Erase Page 92
    #[inline(always)]
    pub fn page92(self) -> &'a mut W {
        self.variant(PNB_A::PAGE92)
    }
    ///Erase Page 93
    #[inline(always)]
    pub fn page93(self) -> &'a mut W {
        self.variant(PNB_A::PAGE93)
    }
    ///Erase Page 94
    #[inline(always)]
    pub fn page94(self) -> &'a mut W {
        self.variant(PNB_A::PAGE94)
    }
    ///Erase Page 95
    #[inline(always)]
    pub fn page95(self) -> &'a mut W {
        self.variant(PNB_A::PAGE95)
    }
    ///Erase Page 96
    #[inline(always)]
    pub fn page96(self) -> &'a mut W {
        self.variant(PNB_A::PAGE96)
    }
    ///Erase Page 97
    #[inline(always)]
    pub fn page97(self) -> &'a mut W {
        self.variant(PNB_A::PAGE97)
    }
    ///Erase Page 98
    #[inline(always)]
    pub fn page98(self) -> &'a mut W {
        self.variant(PNB_A::PAGE98)
    }
    ///Erase Page 99
    #[inline(always)]
    pub fn page99(self) -> &'a mut W {
        self.variant(PNB_A::PAGE99)
    }
    ///Erase Page 100
    #[inline(always)]
    pub fn page100(self) -> &'a mut W {
        self.variant(PNB_A::PAGE100)
    }
    ///Erase Page 101
    #[inline(always)]
    pub fn page101(self) -> &'a mut W {
        self.variant(PNB_A::PAGE101)
    }
    ///Erase Page 102
    #[inline(always)]
    pub fn page102(self) -> &'a mut W {
        self.variant(PNB_A::PAGE102)
    }
    ///Erase Page 103
    #[inline(always)]
    pub fn page103(self) -> &'a mut W {
        self.variant(PNB_A::PAGE103)
    }
    ///Erase Page 104
    #[inline(always)]
    pub fn page104(self) -> &'a mut W {
        self.variant(PNB_A::PAGE104)
    }
    ///Erase Page 105
    #[inline(always)]
    pub fn page105(self) -> &'a mut W {
        self.variant(PNB_A::PAGE105)
    }
    ///Erase Page 106
    #[inline(always)]
    pub fn page106(self) -> &'a mut W {
        self.variant(PNB_A::PAGE106)
    }
    ///Erase Page 107
    #[inline(always)]
    pub fn page107(self) -> &'a mut W {
        self.variant(PNB_A::PAGE107)
    }
    ///Erase Page 108
    #[inline(always)]
    pub fn page108(self) -> &'a mut W {
        self.variant(PNB_A::PAGE108)
    }
    ///Erase Page 109
    #[inline(always)]
    pub fn page109(self) -> &'a mut W {
        self.variant(PNB_A::PAGE109)
    }
    ///Erase Page 110
    #[inline(always)]
    pub fn page110(self) -> &'a mut W {
        self.variant(PNB_A::PAGE110)
    }
    ///Erase Page 111
    #[inline(always)]
    pub fn page111(self) -> &'a mut W {
        self.variant(PNB_A::PAGE111)
    }
    ///Erase Page 112
    #[inline(always)]
    pub fn page112(self) -> &'a mut W {
        self.variant(PNB_A::PAGE112)
    }
    ///Erase Page 113
    #[inline(always)]
    pub fn page113(self) -> &'a mut W {
        self.variant(PNB_A::PAGE113)
    }
    ///Erase Page 114
    #[inline(always)]
    pub fn page114(self) -> &'a mut W {
        self.variant(PNB_A::PAGE114)
    }
    ///Erase Page 115
    #[inline(always)]
    pub fn page115(self) -> &'a mut W {
        self.variant(PNB_A::PAGE115)
    }
    ///Erase Page 116
    #[inline(always)]
    pub fn page116(self) -> &'a mut W {
        self.variant(PNB_A::PAGE116)
    }
    ///Erase Page 117
    #[inline(always)]
    pub fn page117(self) -> &'a mut W {
        self.variant(PNB_A::PAGE117)
    }
    ///Erase Page 118
    #[inline(always)]
    pub fn page118(self) -> &'a mut W {
        self.variant(PNB_A::PAGE118)
    }
    ///Erase Page 119
    #[inline(always)]
    pub fn page119(self) -> &'a mut W {
        self.variant(PNB_A::PAGE119)
    }
    ///Erase Page 120
    #[inline(always)]
    pub fn page120(self) -> &'a mut W {
        self.variant(PNB_A::PAGE120)
    }
    ///Erase Page 121
    #[inline(always)]
    pub fn page121(self) -> &'a mut W {
        self.variant(PNB_A::PAGE121)
    }
    ///Erase Page 122
    #[inline(always)]
    pub fn page122(self) -> &'a mut W {
        self.variant(PNB_A::PAGE122)
    }
    ///Erase Page 123
    #[inline(always)]
    pub fn page123(self) -> &'a mut W {
        self.variant(PNB_A::PAGE123)
    }
    ///Erase Page 124
    #[inline(always)]
    pub fn page124(self) -> &'a mut W {
        self.variant(PNB_A::PAGE124)
    }
    ///Erase Page 125
    #[inline(always)]
    pub fn page125(self) -> &'a mut W {
        self.variant(PNB_A::PAGE125)
    }
    ///Erase Page 126
    #[inline(always)]
    pub fn page126(self) -> &'a mut W {
        self.variant(PNB_A::PAGE126)
    }
    ///Erase Page 127
    #[inline(always)]
    pub fn page127(self) -> &'a mut W {
        self.variant(PNB_A::PAGE127)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 3)) | (((value as u32) & 0x7f) << 3);
        self.w
    }
}
///Start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRT_A {
    ///1: Triggers an erase operation
    START = 1,
}
impl From<STRT_A> for bool {
    #[inline(always)]
    fn from(variant: STRT_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `STRT`
pub type STRT_R = crate::R<bool, STRT_A>;
impl STRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, STRT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(STRT_A::START),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `START`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STRT_A::START
    }
}
///Write proxy for field `STRT`
pub struct STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> STRT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: STRT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Triggers an erase operation
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(STRT_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
///Options modification start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTSTRT_A {
    ///1: Triggers an option modification operation
    START = 1,
}
impl From<OPTSTRT_A> for bool {
    #[inline(always)]
    fn from(variant: OPTSTRT_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OPTSTRT`
pub type OPTSTRT_R = crate::R<bool, OPTSTRT_A>;
impl OPTSTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, OPTSTRT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(OPTSTRT_A::START),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `START`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == OPTSTRT_A::START
    }
}
///Write proxy for field `OPTSTRT`
pub struct OPTSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTSTRT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OPTSTRT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Triggers an option modification operation
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(OPTSTRT_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
///Fast programming
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSTPG_A {
    ///0: Fast programming disabled
    DISABLED = 0,
    ///1: Fast programming enabled
    ENABLED = 1,
}
impl From<FSTPG_A> for bool {
    #[inline(always)]
    fn from(variant: FSTPG_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FSTPG`
pub type FSTPG_R = crate::R<bool, FSTPG_A>;
impl FSTPG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FSTPG_A {
        match self.bits {
            false => FSTPG_A::DISABLED,
            true => FSTPG_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FSTPG_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FSTPG_A::ENABLED
    }
}
///Write proxy for field `FSTPG`
pub struct FSTPG_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTPG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FSTPG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Fast programming disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FSTPG_A::DISABLED)
    }
    ///Fast programming enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FSTPG_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
///End of operation interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOPIE_A {
    ///0: EOP interrupt disabled
    DISABLED = 0,
    ///1: EOP interrupt enabled
    ENABLED = 1,
}
impl From<EOPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOPIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EOPIE`
pub type EOPIE_R = crate::R<bool, EOPIE_A>;
impl EOPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOPIE_A {
        match self.bits {
            false => EOPIE_A::DISABLED,
            true => EOPIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOPIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOPIE_A::ENABLED
    }
}
///Write proxy for field `EOPIE`
pub struct EOPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///EOP interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOPIE_A::DISABLED)
    }
    ///EOP interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOPIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
///Error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    ///0: OPERR error interrupt disabled
    DISABLED = 0,
    ///1: OPERR error interrupt enabled
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ERRIE`
pub type ERRIE_R = crate::R<bool, ERRIE_A>;
impl ERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::ENABLED
    }
}
///Write proxy for field `ERRIE`
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ERRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///OPERR error interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::DISABLED)
    }
    ///OPERR error interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
///PCROP read error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDERRIE_A {
    ///0: Read error interrupt disabled
    DISABLED = 0,
    ///1: Read error interrupt enabled
    ENABLED = 1,
}
impl From<RDERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RDERRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RDERRIE`
pub type RDERRIE_R = crate::R<bool, RDERRIE_A>;
impl RDERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RDERRIE_A {
        match self.bits {
            false => RDERRIE_A::DISABLED,
            true => RDERRIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDERRIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDERRIE_A::ENABLED
    }
}
///Write proxy for field `RDERRIE`
pub struct RDERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDERRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RDERRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Read error interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RDERRIE_A::DISABLED)
    }
    ///Read error interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RDERRIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
///Force the option byte loading
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OBL_LAUNCH_A {
    ///0: Option byte loading complete
    DISABLED = 0,
    ///1: Option byte loading requested
    ENABLED = 1,
}
impl From<OBL_LAUNCH_A> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCH_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OBL_LAUNCH`
pub type OBL_LAUNCH_R = crate::R<bool, OBL_LAUNCH_A>;
impl OBL_LAUNCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OBL_LAUNCH_A {
        match self.bits {
            false => OBL_LAUNCH_A::DISABLED,
            true => OBL_LAUNCH_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OBL_LAUNCH_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OBL_LAUNCH_A::ENABLED
    }
}
///Write proxy for field `OBL_LAUNCH`
pub struct OBL_LAUNCH_W<'a> {
    w: &'a mut W,
}
impl<'a> OBL_LAUNCH_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OBL_LAUNCH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Option byte loading complete
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OBL_LAUNCH_A::DISABLED)
    }
    ///Option byte loading requested
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OBL_LAUNCH_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
///SEC_PROT1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_PROT1_A {
    ///0: Securable area 1 is unlocked
    UNLOCKED = 0,
    ///1: Securable area 1 is locked
    LOCKED = 1,
}
impl From<SEC_PROT1_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_PROT1_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SEC_PROT1`
pub type SEC_PROT1_R = crate::R<bool, SEC_PROT1_A>;
impl SEC_PROT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SEC_PROT1_A {
        match self.bits {
            false => SEC_PROT1_A::UNLOCKED,
            true => SEC_PROT1_A::LOCKED,
        }
    }
    ///Checks if the value of the field is `UNLOCKED`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == SEC_PROT1_A::UNLOCKED
    }
    ///Checks if the value of the field is `LOCKED`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == SEC_PROT1_A::LOCKED
    }
}
///Write proxy for field `SEC_PROT1`
pub struct SEC_PROT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_PROT1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SEC_PROT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Securable area 1 is unlocked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(SEC_PROT1_A::UNLOCKED)
    }
    ///Securable area 1 is locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(SEC_PROT1_A::LOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
///Options Lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTLOCK_A {
    ///0: FLASH_CR register options are unlocked
    UNLOCKED = 0,
    ///1: FLASH_CR register options are locked
    LOCKED = 1,
}
impl From<OPTLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCK_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OPTLOCK`
pub type OPTLOCK_R = crate::R<bool, OPTLOCK_A>;
impl OPTLOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPTLOCK_A {
        match self.bits {
            false => OPTLOCK_A::UNLOCKED,
            true => OPTLOCK_A::LOCKED,
        }
    }
    ///Checks if the value of the field is `UNLOCKED`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == OPTLOCK_A::UNLOCKED
    }
    ///Checks if the value of the field is `LOCKED`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == OPTLOCK_A::LOCKED
    }
}
///Write proxy for field `OPTLOCK`
pub struct OPTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTLOCK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OPTLOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///FLASH_CR register options are unlocked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(OPTLOCK_A::UNLOCKED)
    }
    ///FLASH_CR register options are locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(OPTLOCK_A::LOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
///FLASH_CR Lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    ///0: FLASH_CR register is unlocked
    UNLOCKED = 0,
    ///1: FLASH_CR register is locked
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LOCK`
pub type LOCK_R = crate::R<bool, LOCK_A>;
impl LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    ///Checks if the value of the field is `UNLOCKED`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    ///Checks if the value of the field is `LOCKED`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
///Write proxy for field `LOCK`
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///FLASH_CR register is unlocked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    ///FLASH_CR register is locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
///Bank erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKER_A {
    ///0: Bank 1 is selected for page erase
    BANK1 = 0,
    ///1: Bank 2 is selected for page erase
    BANK2 = 1,
}
impl From<BKER_A> for bool {
    #[inline(always)]
    fn from(variant: BKER_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `BKER`
pub type BKER_R = crate::R<bool, BKER_A>;
impl BKER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKER_A {
        match self.bits {
            false => BKER_A::BANK1,
            true => BKER_A::BANK2,
        }
    }
    ///Checks if the value of the field is `BANK1`
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BKER_A::BANK1
    }
    ///Checks if the value of the field is `BANK2`
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BKER_A::BANK2
    }
}
///Write proxy for field `BKER`
pub struct BKER_W<'a> {
    w: &'a mut W,
}
impl<'a> BKER_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BKER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Bank 1 is selected for page erase
    #[inline(always)]
    pub fn bank1(self) -> &'a mut W {
        self.variant(BKER_A::BANK1)
    }
    ///Bank 2 is selected for page erase
    #[inline(always)]
    pub fn bank2(self) -> &'a mut W {
        self.variant(BKER_A::BANK2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
///Bank 2 Mass erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MER2_A {
    ///1: Triggers bank 2 mass erase
    START = 1,
}
impl From<MER2_A> for bool {
    #[inline(always)]
    fn from(variant: MER2_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MER2`
pub type MER2_R = crate::R<bool, MER2_A>;
impl MER2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MER2_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(MER2_A::START),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `START`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == MER2_A::START
    }
}
///Write proxy for field `MER2`
pub struct MER2_W<'a> {
    w: &'a mut W,
}
impl<'a> MER2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MER2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Triggers bank 2 mass erase
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(MER2_A::START)
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
///Securable memory area protection bit for bank 2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_PROT2_A {
    ///0: Securable area 2 is unlocked
    UNLOCKED = 0,
    ///1: Securable area 2 is locked
    LOCKED = 1,
}
impl From<SEC_PROT2_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_PROT2_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SEC_PROT2`
pub type SEC_PROT2_R = crate::R<bool, SEC_PROT2_A>;
impl SEC_PROT2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SEC_PROT2_A {
        match self.bits {
            false => SEC_PROT2_A::UNLOCKED,
            true => SEC_PROT2_A::LOCKED,
        }
    }
    ///Checks if the value of the field is `UNLOCKED`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == SEC_PROT2_A::UNLOCKED
    }
    ///Checks if the value of the field is `LOCKED`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == SEC_PROT2_A::LOCKED
    }
}
///Write proxy for field `SEC_PROT2`
pub struct SEC_PROT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_PROT2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SEC_PROT2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Securable area 2 is unlocked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(SEC_PROT2_A::UNLOCKED)
    }
    ///Securable area 2 is locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(SEC_PROT2_A::LOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Page erase
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Bank 1 Mass erase
    #[inline(always)]
    pub fn mer1(&self) -> MER1_R {
        MER1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bits 3:9 - Page number
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Options modification start
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - Fast programming
    #[inline(always)]
    pub fn fstpg(&self) -> FSTPG_R {
        FSTPG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 24 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - PCROP read error interrupt enable
    #[inline(always)]
    pub fn rderrie(&self) -> RDERRIE_R {
        RDERRIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - Force the option byte loading
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 28 - SEC_PROT1
    #[inline(always)]
    pub fn sec_prot1(&self) -> SEC_PROT1_R {
        SEC_PROT1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 30 - Options Lock
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - FLASH_CR Lock
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 11 - Bank erase
    #[inline(always)]
    pub fn bker(&self) -> BKER_R {
        BKER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 15 - Bank 2 Mass erase
    #[inline(always)]
    pub fn mer2(&self) -> MER2_R {
        MER2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 29 - Securable memory area protection bit for bank 2
    #[inline(always)]
    pub fn sec_prot2(&self) -> SEC_PROT2_R {
        SEC_PROT2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W {
        PG_W { w: self }
    }
    ///Bit 1 - Page erase
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W { w: self }
    }
    ///Bit 2 - Bank 1 Mass erase
    #[inline(always)]
    pub fn mer1(&mut self) -> MER1_W {
        MER1_W { w: self }
    }
    ///Bits 3:9 - Page number
    #[inline(always)]
    pub fn pnb(&mut self) -> PNB_W {
        PNB_W { w: self }
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W {
        STRT_W { w: self }
    }
    ///Bit 17 - Options modification start
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W {
        OPTSTRT_W { w: self }
    }
    ///Bit 18 - Fast programming
    #[inline(always)]
    pub fn fstpg(&mut self) -> FSTPG_W {
        FSTPG_W { w: self }
    }
    ///Bit 24 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W {
        EOPIE_W { w: self }
    }
    ///Bit 25 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    ///Bit 26 - PCROP read error interrupt enable
    #[inline(always)]
    pub fn rderrie(&mut self) -> RDERRIE_W {
        RDERRIE_W { w: self }
    }
    ///Bit 27 - Force the option byte loading
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W {
        OBL_LAUNCH_W { w: self }
    }
    ///Bit 28 - SEC_PROT1
    #[inline(always)]
    pub fn sec_prot1(&mut self) -> SEC_PROT1_W {
        SEC_PROT1_W { w: self }
    }
    ///Bit 30 - Options Lock
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W {
        OPTLOCK_W { w: self }
    }
    ///Bit 31 - FLASH_CR Lock
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    ///Bit 11 - Bank erase
    #[inline(always)]
    pub fn bker(&mut self) -> BKER_W {
        BKER_W { w: self }
    }
    ///Bit 15 - Bank 2 Mass erase
    #[inline(always)]
    pub fn mer2(&mut self) -> MER2_W {
        MER2_W { w: self }
    }
    ///Bit 29 - Securable memory area protection bit for bank 2
    #[inline(always)]
    pub fn sec_prot2(&mut self) -> SEC_PROT2_W {
        SEC_PROT2_W { w: self }
    }
}
