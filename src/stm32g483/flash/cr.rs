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
    ///128: Erase Page 128
    PAGE128 = 128,
    ///129: Erase Page 129
    PAGE129 = 129,
    ///130: Erase Page 130
    PAGE130 = 130,
    ///131: Erase Page 131
    PAGE131 = 131,
    ///132: Erase Page 132
    PAGE132 = 132,
    ///133: Erase Page 133
    PAGE133 = 133,
    ///134: Erase Page 134
    PAGE134 = 134,
    ///135: Erase Page 135
    PAGE135 = 135,
    ///136: Erase Page 136
    PAGE136 = 136,
    ///137: Erase Page 137
    PAGE137 = 137,
    ///138: Erase Page 138
    PAGE138 = 138,
    ///139: Erase Page 139
    PAGE139 = 139,
    ///140: Erase Page 140
    PAGE140 = 140,
    ///141: Erase Page 141
    PAGE141 = 141,
    ///142: Erase Page 142
    PAGE142 = 142,
    ///143: Erase Page 143
    PAGE143 = 143,
    ///144: Erase Page 144
    PAGE144 = 144,
    ///145: Erase Page 145
    PAGE145 = 145,
    ///146: Erase Page 146
    PAGE146 = 146,
    ///147: Erase Page 147
    PAGE147 = 147,
    ///148: Erase Page 148
    PAGE148 = 148,
    ///149: Erase Page 149
    PAGE149 = 149,
    ///150: Erase Page 150
    PAGE150 = 150,
    ///151: Erase Page 151
    PAGE151 = 151,
    ///152: Erase Page 152
    PAGE152 = 152,
    ///153: Erase Page 153
    PAGE153 = 153,
    ///154: Erase Page 154
    PAGE154 = 154,
    ///155: Erase Page 155
    PAGE155 = 155,
    ///156: Erase Page 156
    PAGE156 = 156,
    ///157: Erase Page 157
    PAGE157 = 157,
    ///158: Erase Page 158
    PAGE158 = 158,
    ///159: Erase Page 159
    PAGE159 = 159,
    ///160: Erase Page 160
    PAGE160 = 160,
    ///161: Erase Page 161
    PAGE161 = 161,
    ///162: Erase Page 162
    PAGE162 = 162,
    ///163: Erase Page 163
    PAGE163 = 163,
    ///164: Erase Page 164
    PAGE164 = 164,
    ///165: Erase Page 165
    PAGE165 = 165,
    ///166: Erase Page 166
    PAGE166 = 166,
    ///167: Erase Page 167
    PAGE167 = 167,
    ///168: Erase Page 168
    PAGE168 = 168,
    ///169: Erase Page 169
    PAGE169 = 169,
    ///170: Erase Page 170
    PAGE170 = 170,
    ///171: Erase Page 171
    PAGE171 = 171,
    ///172: Erase Page 172
    PAGE172 = 172,
    ///173: Erase Page 173
    PAGE173 = 173,
    ///174: Erase Page 174
    PAGE174 = 174,
    ///175: Erase Page 175
    PAGE175 = 175,
    ///176: Erase Page 176
    PAGE176 = 176,
    ///177: Erase Page 177
    PAGE177 = 177,
    ///178: Erase Page 178
    PAGE178 = 178,
    ///179: Erase Page 179
    PAGE179 = 179,
    ///180: Erase Page 180
    PAGE180 = 180,
    ///181: Erase Page 181
    PAGE181 = 181,
    ///182: Erase Page 182
    PAGE182 = 182,
    ///183: Erase Page 183
    PAGE183 = 183,
    ///184: Erase Page 184
    PAGE184 = 184,
    ///185: Erase Page 185
    PAGE185 = 185,
    ///186: Erase Page 186
    PAGE186 = 186,
    ///187: Erase Page 187
    PAGE187 = 187,
    ///188: Erase Page 188
    PAGE188 = 188,
    ///189: Erase Page 189
    PAGE189 = 189,
    ///190: Erase Page 190
    PAGE190 = 190,
    ///191: Erase Page 191
    PAGE191 = 191,
    ///192: Erase Page 192
    PAGE192 = 192,
    ///193: Erase Page 193
    PAGE193 = 193,
    ///194: Erase Page 194
    PAGE194 = 194,
    ///195: Erase Page 195
    PAGE195 = 195,
    ///196: Erase Page 196
    PAGE196 = 196,
    ///197: Erase Page 197
    PAGE197 = 197,
    ///198: Erase Page 198
    PAGE198 = 198,
    ///199: Erase Page 199
    PAGE199 = 199,
    ///200: Erase Page 200
    PAGE200 = 200,
    ///201: Erase Page 201
    PAGE201 = 201,
    ///202: Erase Page 202
    PAGE202 = 202,
    ///203: Erase Page 203
    PAGE203 = 203,
    ///204: Erase Page 204
    PAGE204 = 204,
    ///205: Erase Page 205
    PAGE205 = 205,
    ///206: Erase Page 206
    PAGE206 = 206,
    ///207: Erase Page 207
    PAGE207 = 207,
    ///208: Erase Page 208
    PAGE208 = 208,
    ///209: Erase Page 209
    PAGE209 = 209,
    ///210: Erase Page 210
    PAGE210 = 210,
    ///211: Erase Page 211
    PAGE211 = 211,
    ///212: Erase Page 212
    PAGE212 = 212,
    ///213: Erase Page 213
    PAGE213 = 213,
    ///214: Erase Page 214
    PAGE214 = 214,
    ///215: Erase Page 215
    PAGE215 = 215,
    ///216: Erase Page 216
    PAGE216 = 216,
    ///217: Erase Page 217
    PAGE217 = 217,
    ///218: Erase Page 218
    PAGE218 = 218,
    ///219: Erase Page 219
    PAGE219 = 219,
    ///220: Erase Page 220
    PAGE220 = 220,
    ///221: Erase Page 221
    PAGE221 = 221,
    ///222: Erase Page 222
    PAGE222 = 222,
    ///223: Erase Page 223
    PAGE223 = 223,
    ///224: Erase Page 224
    PAGE224 = 224,
    ///225: Erase Page 225
    PAGE225 = 225,
    ///226: Erase Page 226
    PAGE226 = 226,
    ///227: Erase Page 227
    PAGE227 = 227,
    ///228: Erase Page 228
    PAGE228 = 228,
    ///229: Erase Page 229
    PAGE229 = 229,
    ///230: Erase Page 230
    PAGE230 = 230,
    ///231: Erase Page 231
    PAGE231 = 231,
    ///232: Erase Page 232
    PAGE232 = 232,
    ///233: Erase Page 233
    PAGE233 = 233,
    ///234: Erase Page 234
    PAGE234 = 234,
    ///235: Erase Page 235
    PAGE235 = 235,
    ///236: Erase Page 236
    PAGE236 = 236,
    ///237: Erase Page 237
    PAGE237 = 237,
    ///238: Erase Page 238
    PAGE238 = 238,
    ///239: Erase Page 239
    PAGE239 = 239,
    ///240: Erase Page 240
    PAGE240 = 240,
    ///241: Erase Page 241
    PAGE241 = 241,
    ///242: Erase Page 242
    PAGE242 = 242,
    ///243: Erase Page 243
    PAGE243 = 243,
    ///244: Erase Page 244
    PAGE244 = 244,
    ///245: Erase Page 245
    PAGE245 = 245,
    ///246: Erase Page 246
    PAGE246 = 246,
    ///247: Erase Page 247
    PAGE247 = 247,
    ///248: Erase Page 248
    PAGE248 = 248,
    ///249: Erase Page 249
    PAGE249 = 249,
    ///250: Erase Page 250
    PAGE250 = 250,
    ///251: Erase Page 251
    PAGE251 = 251,
    ///252: Erase Page 252
    PAGE252 = 252,
    ///253: Erase Page 253
    PAGE253 = 253,
    ///254: Erase Page 254
    PAGE254 = 254,
    ///255: Erase Page 255
    PAGE255 = 255,
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
    pub fn variant(&self) -> crate::Variant<u8, PNB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PNB_A::PAGE0),
            1 => Val(PNB_A::PAGE1),
            2 => Val(PNB_A::PAGE2),
            3 => Val(PNB_A::PAGE3),
            4 => Val(PNB_A::PAGE4),
            5 => Val(PNB_A::PAGE5),
            6 => Val(PNB_A::PAGE6),
            7 => Val(PNB_A::PAGE7),
            8 => Val(PNB_A::PAGE8),
            9 => Val(PNB_A::PAGE9),
            10 => Val(PNB_A::PAGE10),
            11 => Val(PNB_A::PAGE11),
            12 => Val(PNB_A::PAGE12),
            13 => Val(PNB_A::PAGE13),
            14 => Val(PNB_A::PAGE14),
            15 => Val(PNB_A::PAGE15),
            16 => Val(PNB_A::PAGE16),
            17 => Val(PNB_A::PAGE17),
            18 => Val(PNB_A::PAGE18),
            19 => Val(PNB_A::PAGE19),
            20 => Val(PNB_A::PAGE20),
            21 => Val(PNB_A::PAGE21),
            22 => Val(PNB_A::PAGE22),
            23 => Val(PNB_A::PAGE23),
            24 => Val(PNB_A::PAGE24),
            25 => Val(PNB_A::PAGE25),
            26 => Val(PNB_A::PAGE26),
            27 => Val(PNB_A::PAGE27),
            28 => Val(PNB_A::PAGE28),
            29 => Val(PNB_A::PAGE29),
            30 => Val(PNB_A::PAGE30),
            31 => Val(PNB_A::PAGE31),
            32 => Val(PNB_A::PAGE32),
            33 => Val(PNB_A::PAGE33),
            34 => Val(PNB_A::PAGE34),
            35 => Val(PNB_A::PAGE35),
            36 => Val(PNB_A::PAGE36),
            37 => Val(PNB_A::PAGE37),
            38 => Val(PNB_A::PAGE38),
            39 => Val(PNB_A::PAGE39),
            40 => Val(PNB_A::PAGE40),
            41 => Val(PNB_A::PAGE41),
            42 => Val(PNB_A::PAGE42),
            43 => Val(PNB_A::PAGE43),
            44 => Val(PNB_A::PAGE44),
            45 => Val(PNB_A::PAGE45),
            46 => Val(PNB_A::PAGE46),
            47 => Val(PNB_A::PAGE47),
            48 => Val(PNB_A::PAGE48),
            49 => Val(PNB_A::PAGE49),
            50 => Val(PNB_A::PAGE50),
            51 => Val(PNB_A::PAGE51),
            52 => Val(PNB_A::PAGE52),
            53 => Val(PNB_A::PAGE53),
            54 => Val(PNB_A::PAGE54),
            55 => Val(PNB_A::PAGE55),
            56 => Val(PNB_A::PAGE56),
            57 => Val(PNB_A::PAGE57),
            58 => Val(PNB_A::PAGE58),
            59 => Val(PNB_A::PAGE59),
            60 => Val(PNB_A::PAGE60),
            61 => Val(PNB_A::PAGE61),
            62 => Val(PNB_A::PAGE62),
            63 => Val(PNB_A::PAGE63),
            64 => Val(PNB_A::PAGE64),
            65 => Val(PNB_A::PAGE65),
            66 => Val(PNB_A::PAGE66),
            67 => Val(PNB_A::PAGE67),
            68 => Val(PNB_A::PAGE68),
            69 => Val(PNB_A::PAGE69),
            70 => Val(PNB_A::PAGE70),
            71 => Val(PNB_A::PAGE71),
            72 => Val(PNB_A::PAGE72),
            73 => Val(PNB_A::PAGE73),
            74 => Val(PNB_A::PAGE74),
            75 => Val(PNB_A::PAGE75),
            76 => Val(PNB_A::PAGE76),
            77 => Val(PNB_A::PAGE77),
            78 => Val(PNB_A::PAGE78),
            79 => Val(PNB_A::PAGE79),
            80 => Val(PNB_A::PAGE80),
            81 => Val(PNB_A::PAGE81),
            82 => Val(PNB_A::PAGE82),
            83 => Val(PNB_A::PAGE83),
            84 => Val(PNB_A::PAGE84),
            85 => Val(PNB_A::PAGE85),
            86 => Val(PNB_A::PAGE86),
            87 => Val(PNB_A::PAGE87),
            88 => Val(PNB_A::PAGE88),
            89 => Val(PNB_A::PAGE89),
            90 => Val(PNB_A::PAGE90),
            91 => Val(PNB_A::PAGE91),
            92 => Val(PNB_A::PAGE92),
            93 => Val(PNB_A::PAGE93),
            94 => Val(PNB_A::PAGE94),
            95 => Val(PNB_A::PAGE95),
            96 => Val(PNB_A::PAGE96),
            97 => Val(PNB_A::PAGE97),
            98 => Val(PNB_A::PAGE98),
            99 => Val(PNB_A::PAGE99),
            100 => Val(PNB_A::PAGE100),
            101 => Val(PNB_A::PAGE101),
            102 => Val(PNB_A::PAGE102),
            103 => Val(PNB_A::PAGE103),
            104 => Val(PNB_A::PAGE104),
            105 => Val(PNB_A::PAGE105),
            106 => Val(PNB_A::PAGE106),
            107 => Val(PNB_A::PAGE107),
            108 => Val(PNB_A::PAGE108),
            109 => Val(PNB_A::PAGE109),
            110 => Val(PNB_A::PAGE110),
            111 => Val(PNB_A::PAGE111),
            112 => Val(PNB_A::PAGE112),
            113 => Val(PNB_A::PAGE113),
            114 => Val(PNB_A::PAGE114),
            115 => Val(PNB_A::PAGE115),
            116 => Val(PNB_A::PAGE116),
            117 => Val(PNB_A::PAGE117),
            118 => Val(PNB_A::PAGE118),
            119 => Val(PNB_A::PAGE119),
            120 => Val(PNB_A::PAGE120),
            121 => Val(PNB_A::PAGE121),
            122 => Val(PNB_A::PAGE122),
            123 => Val(PNB_A::PAGE123),
            124 => Val(PNB_A::PAGE124),
            125 => Val(PNB_A::PAGE125),
            126 => Val(PNB_A::PAGE126),
            127 => Val(PNB_A::PAGE127),
            128 => Val(PNB_A::PAGE128),
            129 => Val(PNB_A::PAGE129),
            130 => Val(PNB_A::PAGE130),
            131 => Val(PNB_A::PAGE131),
            132 => Val(PNB_A::PAGE132),
            133 => Val(PNB_A::PAGE133),
            134 => Val(PNB_A::PAGE134),
            135 => Val(PNB_A::PAGE135),
            136 => Val(PNB_A::PAGE136),
            137 => Val(PNB_A::PAGE137),
            138 => Val(PNB_A::PAGE138),
            139 => Val(PNB_A::PAGE139),
            140 => Val(PNB_A::PAGE140),
            141 => Val(PNB_A::PAGE141),
            142 => Val(PNB_A::PAGE142),
            143 => Val(PNB_A::PAGE143),
            144 => Val(PNB_A::PAGE144),
            145 => Val(PNB_A::PAGE145),
            146 => Val(PNB_A::PAGE146),
            147 => Val(PNB_A::PAGE147),
            148 => Val(PNB_A::PAGE148),
            149 => Val(PNB_A::PAGE149),
            150 => Val(PNB_A::PAGE150),
            151 => Val(PNB_A::PAGE151),
            152 => Val(PNB_A::PAGE152),
            153 => Val(PNB_A::PAGE153),
            154 => Val(PNB_A::PAGE154),
            155 => Val(PNB_A::PAGE155),
            156 => Val(PNB_A::PAGE156),
            157 => Val(PNB_A::PAGE157),
            158 => Val(PNB_A::PAGE158),
            159 => Val(PNB_A::PAGE159),
            160 => Val(PNB_A::PAGE160),
            161 => Val(PNB_A::PAGE161),
            162 => Val(PNB_A::PAGE162),
            163 => Val(PNB_A::PAGE163),
            164 => Val(PNB_A::PAGE164),
            165 => Val(PNB_A::PAGE165),
            166 => Val(PNB_A::PAGE166),
            167 => Val(PNB_A::PAGE167),
            168 => Val(PNB_A::PAGE168),
            169 => Val(PNB_A::PAGE169),
            170 => Val(PNB_A::PAGE170),
            171 => Val(PNB_A::PAGE171),
            172 => Val(PNB_A::PAGE172),
            173 => Val(PNB_A::PAGE173),
            174 => Val(PNB_A::PAGE174),
            175 => Val(PNB_A::PAGE175),
            176 => Val(PNB_A::PAGE176),
            177 => Val(PNB_A::PAGE177),
            178 => Val(PNB_A::PAGE178),
            179 => Val(PNB_A::PAGE179),
            180 => Val(PNB_A::PAGE180),
            181 => Val(PNB_A::PAGE181),
            182 => Val(PNB_A::PAGE182),
            183 => Val(PNB_A::PAGE183),
            184 => Val(PNB_A::PAGE184),
            185 => Val(PNB_A::PAGE185),
            186 => Val(PNB_A::PAGE186),
            187 => Val(PNB_A::PAGE187),
            188 => Val(PNB_A::PAGE188),
            189 => Val(PNB_A::PAGE189),
            190 => Val(PNB_A::PAGE190),
            191 => Val(PNB_A::PAGE191),
            192 => Val(PNB_A::PAGE192),
            193 => Val(PNB_A::PAGE193),
            194 => Val(PNB_A::PAGE194),
            195 => Val(PNB_A::PAGE195),
            196 => Val(PNB_A::PAGE196),
            197 => Val(PNB_A::PAGE197),
            198 => Val(PNB_A::PAGE198),
            199 => Val(PNB_A::PAGE199),
            200 => Val(PNB_A::PAGE200),
            201 => Val(PNB_A::PAGE201),
            202 => Val(PNB_A::PAGE202),
            203 => Val(PNB_A::PAGE203),
            204 => Val(PNB_A::PAGE204),
            205 => Val(PNB_A::PAGE205),
            206 => Val(PNB_A::PAGE206),
            207 => Val(PNB_A::PAGE207),
            208 => Val(PNB_A::PAGE208),
            209 => Val(PNB_A::PAGE209),
            210 => Val(PNB_A::PAGE210),
            211 => Val(PNB_A::PAGE211),
            212 => Val(PNB_A::PAGE212),
            213 => Val(PNB_A::PAGE213),
            214 => Val(PNB_A::PAGE214),
            215 => Val(PNB_A::PAGE215),
            216 => Val(PNB_A::PAGE216),
            217 => Val(PNB_A::PAGE217),
            218 => Val(PNB_A::PAGE218),
            219 => Val(PNB_A::PAGE219),
            220 => Val(PNB_A::PAGE220),
            221 => Val(PNB_A::PAGE221),
            222 => Val(PNB_A::PAGE222),
            223 => Val(PNB_A::PAGE223),
            224 => Val(PNB_A::PAGE224),
            225 => Val(PNB_A::PAGE225),
            226 => Val(PNB_A::PAGE226),
            227 => Val(PNB_A::PAGE227),
            228 => Val(PNB_A::PAGE228),
            229 => Val(PNB_A::PAGE229),
            230 => Val(PNB_A::PAGE230),
            231 => Val(PNB_A::PAGE231),
            232 => Val(PNB_A::PAGE232),
            233 => Val(PNB_A::PAGE233),
            234 => Val(PNB_A::PAGE234),
            235 => Val(PNB_A::PAGE235),
            236 => Val(PNB_A::PAGE236),
            237 => Val(PNB_A::PAGE237),
            238 => Val(PNB_A::PAGE238),
            239 => Val(PNB_A::PAGE239),
            240 => Val(PNB_A::PAGE240),
            241 => Val(PNB_A::PAGE241),
            242 => Val(PNB_A::PAGE242),
            243 => Val(PNB_A::PAGE243),
            244 => Val(PNB_A::PAGE244),
            245 => Val(PNB_A::PAGE245),
            246 => Val(PNB_A::PAGE246),
            247 => Val(PNB_A::PAGE247),
            248 => Val(PNB_A::PAGE248),
            249 => Val(PNB_A::PAGE249),
            250 => Val(PNB_A::PAGE250),
            251 => Val(PNB_A::PAGE251),
            252 => Val(PNB_A::PAGE252),
            253 => Val(PNB_A::PAGE253),
            254 => Val(PNB_A::PAGE254),
            255 => Val(PNB_A::PAGE255),
            i => Res(i),
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
    ///Checks if the value of the field is `PAGE128`
    #[inline(always)]
    pub fn is_page128(&self) -> bool {
        *self == PNB_A::PAGE128
    }
    ///Checks if the value of the field is `PAGE129`
    #[inline(always)]
    pub fn is_page129(&self) -> bool {
        *self == PNB_A::PAGE129
    }
    ///Checks if the value of the field is `PAGE130`
    #[inline(always)]
    pub fn is_page130(&self) -> bool {
        *self == PNB_A::PAGE130
    }
    ///Checks if the value of the field is `PAGE131`
    #[inline(always)]
    pub fn is_page131(&self) -> bool {
        *self == PNB_A::PAGE131
    }
    ///Checks if the value of the field is `PAGE132`
    #[inline(always)]
    pub fn is_page132(&self) -> bool {
        *self == PNB_A::PAGE132
    }
    ///Checks if the value of the field is `PAGE133`
    #[inline(always)]
    pub fn is_page133(&self) -> bool {
        *self == PNB_A::PAGE133
    }
    ///Checks if the value of the field is `PAGE134`
    #[inline(always)]
    pub fn is_page134(&self) -> bool {
        *self == PNB_A::PAGE134
    }
    ///Checks if the value of the field is `PAGE135`
    #[inline(always)]
    pub fn is_page135(&self) -> bool {
        *self == PNB_A::PAGE135
    }
    ///Checks if the value of the field is `PAGE136`
    #[inline(always)]
    pub fn is_page136(&self) -> bool {
        *self == PNB_A::PAGE136
    }
    ///Checks if the value of the field is `PAGE137`
    #[inline(always)]
    pub fn is_page137(&self) -> bool {
        *self == PNB_A::PAGE137
    }
    ///Checks if the value of the field is `PAGE138`
    #[inline(always)]
    pub fn is_page138(&self) -> bool {
        *self == PNB_A::PAGE138
    }
    ///Checks if the value of the field is `PAGE139`
    #[inline(always)]
    pub fn is_page139(&self) -> bool {
        *self == PNB_A::PAGE139
    }
    ///Checks if the value of the field is `PAGE140`
    #[inline(always)]
    pub fn is_page140(&self) -> bool {
        *self == PNB_A::PAGE140
    }
    ///Checks if the value of the field is `PAGE141`
    #[inline(always)]
    pub fn is_page141(&self) -> bool {
        *self == PNB_A::PAGE141
    }
    ///Checks if the value of the field is `PAGE142`
    #[inline(always)]
    pub fn is_page142(&self) -> bool {
        *self == PNB_A::PAGE142
    }
    ///Checks if the value of the field is `PAGE143`
    #[inline(always)]
    pub fn is_page143(&self) -> bool {
        *self == PNB_A::PAGE143
    }
    ///Checks if the value of the field is `PAGE144`
    #[inline(always)]
    pub fn is_page144(&self) -> bool {
        *self == PNB_A::PAGE144
    }
    ///Checks if the value of the field is `PAGE145`
    #[inline(always)]
    pub fn is_page145(&self) -> bool {
        *self == PNB_A::PAGE145
    }
    ///Checks if the value of the field is `PAGE146`
    #[inline(always)]
    pub fn is_page146(&self) -> bool {
        *self == PNB_A::PAGE146
    }
    ///Checks if the value of the field is `PAGE147`
    #[inline(always)]
    pub fn is_page147(&self) -> bool {
        *self == PNB_A::PAGE147
    }
    ///Checks if the value of the field is `PAGE148`
    #[inline(always)]
    pub fn is_page148(&self) -> bool {
        *self == PNB_A::PAGE148
    }
    ///Checks if the value of the field is `PAGE149`
    #[inline(always)]
    pub fn is_page149(&self) -> bool {
        *self == PNB_A::PAGE149
    }
    ///Checks if the value of the field is `PAGE150`
    #[inline(always)]
    pub fn is_page150(&self) -> bool {
        *self == PNB_A::PAGE150
    }
    ///Checks if the value of the field is `PAGE151`
    #[inline(always)]
    pub fn is_page151(&self) -> bool {
        *self == PNB_A::PAGE151
    }
    ///Checks if the value of the field is `PAGE152`
    #[inline(always)]
    pub fn is_page152(&self) -> bool {
        *self == PNB_A::PAGE152
    }
    ///Checks if the value of the field is `PAGE153`
    #[inline(always)]
    pub fn is_page153(&self) -> bool {
        *self == PNB_A::PAGE153
    }
    ///Checks if the value of the field is `PAGE154`
    #[inline(always)]
    pub fn is_page154(&self) -> bool {
        *self == PNB_A::PAGE154
    }
    ///Checks if the value of the field is `PAGE155`
    #[inline(always)]
    pub fn is_page155(&self) -> bool {
        *self == PNB_A::PAGE155
    }
    ///Checks if the value of the field is `PAGE156`
    #[inline(always)]
    pub fn is_page156(&self) -> bool {
        *self == PNB_A::PAGE156
    }
    ///Checks if the value of the field is `PAGE157`
    #[inline(always)]
    pub fn is_page157(&self) -> bool {
        *self == PNB_A::PAGE157
    }
    ///Checks if the value of the field is `PAGE158`
    #[inline(always)]
    pub fn is_page158(&self) -> bool {
        *self == PNB_A::PAGE158
    }
    ///Checks if the value of the field is `PAGE159`
    #[inline(always)]
    pub fn is_page159(&self) -> bool {
        *self == PNB_A::PAGE159
    }
    ///Checks if the value of the field is `PAGE160`
    #[inline(always)]
    pub fn is_page160(&self) -> bool {
        *self == PNB_A::PAGE160
    }
    ///Checks if the value of the field is `PAGE161`
    #[inline(always)]
    pub fn is_page161(&self) -> bool {
        *self == PNB_A::PAGE161
    }
    ///Checks if the value of the field is `PAGE162`
    #[inline(always)]
    pub fn is_page162(&self) -> bool {
        *self == PNB_A::PAGE162
    }
    ///Checks if the value of the field is `PAGE163`
    #[inline(always)]
    pub fn is_page163(&self) -> bool {
        *self == PNB_A::PAGE163
    }
    ///Checks if the value of the field is `PAGE164`
    #[inline(always)]
    pub fn is_page164(&self) -> bool {
        *self == PNB_A::PAGE164
    }
    ///Checks if the value of the field is `PAGE165`
    #[inline(always)]
    pub fn is_page165(&self) -> bool {
        *self == PNB_A::PAGE165
    }
    ///Checks if the value of the field is `PAGE166`
    #[inline(always)]
    pub fn is_page166(&self) -> bool {
        *self == PNB_A::PAGE166
    }
    ///Checks if the value of the field is `PAGE167`
    #[inline(always)]
    pub fn is_page167(&self) -> bool {
        *self == PNB_A::PAGE167
    }
    ///Checks if the value of the field is `PAGE168`
    #[inline(always)]
    pub fn is_page168(&self) -> bool {
        *self == PNB_A::PAGE168
    }
    ///Checks if the value of the field is `PAGE169`
    #[inline(always)]
    pub fn is_page169(&self) -> bool {
        *self == PNB_A::PAGE169
    }
    ///Checks if the value of the field is `PAGE170`
    #[inline(always)]
    pub fn is_page170(&self) -> bool {
        *self == PNB_A::PAGE170
    }
    ///Checks if the value of the field is `PAGE171`
    #[inline(always)]
    pub fn is_page171(&self) -> bool {
        *self == PNB_A::PAGE171
    }
    ///Checks if the value of the field is `PAGE172`
    #[inline(always)]
    pub fn is_page172(&self) -> bool {
        *self == PNB_A::PAGE172
    }
    ///Checks if the value of the field is `PAGE173`
    #[inline(always)]
    pub fn is_page173(&self) -> bool {
        *self == PNB_A::PAGE173
    }
    ///Checks if the value of the field is `PAGE174`
    #[inline(always)]
    pub fn is_page174(&self) -> bool {
        *self == PNB_A::PAGE174
    }
    ///Checks if the value of the field is `PAGE175`
    #[inline(always)]
    pub fn is_page175(&self) -> bool {
        *self == PNB_A::PAGE175
    }
    ///Checks if the value of the field is `PAGE176`
    #[inline(always)]
    pub fn is_page176(&self) -> bool {
        *self == PNB_A::PAGE176
    }
    ///Checks if the value of the field is `PAGE177`
    #[inline(always)]
    pub fn is_page177(&self) -> bool {
        *self == PNB_A::PAGE177
    }
    ///Checks if the value of the field is `PAGE178`
    #[inline(always)]
    pub fn is_page178(&self) -> bool {
        *self == PNB_A::PAGE178
    }
    ///Checks if the value of the field is `PAGE179`
    #[inline(always)]
    pub fn is_page179(&self) -> bool {
        *self == PNB_A::PAGE179
    }
    ///Checks if the value of the field is `PAGE180`
    #[inline(always)]
    pub fn is_page180(&self) -> bool {
        *self == PNB_A::PAGE180
    }
    ///Checks if the value of the field is `PAGE181`
    #[inline(always)]
    pub fn is_page181(&self) -> bool {
        *self == PNB_A::PAGE181
    }
    ///Checks if the value of the field is `PAGE182`
    #[inline(always)]
    pub fn is_page182(&self) -> bool {
        *self == PNB_A::PAGE182
    }
    ///Checks if the value of the field is `PAGE183`
    #[inline(always)]
    pub fn is_page183(&self) -> bool {
        *self == PNB_A::PAGE183
    }
    ///Checks if the value of the field is `PAGE184`
    #[inline(always)]
    pub fn is_page184(&self) -> bool {
        *self == PNB_A::PAGE184
    }
    ///Checks if the value of the field is `PAGE185`
    #[inline(always)]
    pub fn is_page185(&self) -> bool {
        *self == PNB_A::PAGE185
    }
    ///Checks if the value of the field is `PAGE186`
    #[inline(always)]
    pub fn is_page186(&self) -> bool {
        *self == PNB_A::PAGE186
    }
    ///Checks if the value of the field is `PAGE187`
    #[inline(always)]
    pub fn is_page187(&self) -> bool {
        *self == PNB_A::PAGE187
    }
    ///Checks if the value of the field is `PAGE188`
    #[inline(always)]
    pub fn is_page188(&self) -> bool {
        *self == PNB_A::PAGE188
    }
    ///Checks if the value of the field is `PAGE189`
    #[inline(always)]
    pub fn is_page189(&self) -> bool {
        *self == PNB_A::PAGE189
    }
    ///Checks if the value of the field is `PAGE190`
    #[inline(always)]
    pub fn is_page190(&self) -> bool {
        *self == PNB_A::PAGE190
    }
    ///Checks if the value of the field is `PAGE191`
    #[inline(always)]
    pub fn is_page191(&self) -> bool {
        *self == PNB_A::PAGE191
    }
    ///Checks if the value of the field is `PAGE192`
    #[inline(always)]
    pub fn is_page192(&self) -> bool {
        *self == PNB_A::PAGE192
    }
    ///Checks if the value of the field is `PAGE193`
    #[inline(always)]
    pub fn is_page193(&self) -> bool {
        *self == PNB_A::PAGE193
    }
    ///Checks if the value of the field is `PAGE194`
    #[inline(always)]
    pub fn is_page194(&self) -> bool {
        *self == PNB_A::PAGE194
    }
    ///Checks if the value of the field is `PAGE195`
    #[inline(always)]
    pub fn is_page195(&self) -> bool {
        *self == PNB_A::PAGE195
    }
    ///Checks if the value of the field is `PAGE196`
    #[inline(always)]
    pub fn is_page196(&self) -> bool {
        *self == PNB_A::PAGE196
    }
    ///Checks if the value of the field is `PAGE197`
    #[inline(always)]
    pub fn is_page197(&self) -> bool {
        *self == PNB_A::PAGE197
    }
    ///Checks if the value of the field is `PAGE198`
    #[inline(always)]
    pub fn is_page198(&self) -> bool {
        *self == PNB_A::PAGE198
    }
    ///Checks if the value of the field is `PAGE199`
    #[inline(always)]
    pub fn is_page199(&self) -> bool {
        *self == PNB_A::PAGE199
    }
    ///Checks if the value of the field is `PAGE200`
    #[inline(always)]
    pub fn is_page200(&self) -> bool {
        *self == PNB_A::PAGE200
    }
    ///Checks if the value of the field is `PAGE201`
    #[inline(always)]
    pub fn is_page201(&self) -> bool {
        *self == PNB_A::PAGE201
    }
    ///Checks if the value of the field is `PAGE202`
    #[inline(always)]
    pub fn is_page202(&self) -> bool {
        *self == PNB_A::PAGE202
    }
    ///Checks if the value of the field is `PAGE203`
    #[inline(always)]
    pub fn is_page203(&self) -> bool {
        *self == PNB_A::PAGE203
    }
    ///Checks if the value of the field is `PAGE204`
    #[inline(always)]
    pub fn is_page204(&self) -> bool {
        *self == PNB_A::PAGE204
    }
    ///Checks if the value of the field is `PAGE205`
    #[inline(always)]
    pub fn is_page205(&self) -> bool {
        *self == PNB_A::PAGE205
    }
    ///Checks if the value of the field is `PAGE206`
    #[inline(always)]
    pub fn is_page206(&self) -> bool {
        *self == PNB_A::PAGE206
    }
    ///Checks if the value of the field is `PAGE207`
    #[inline(always)]
    pub fn is_page207(&self) -> bool {
        *self == PNB_A::PAGE207
    }
    ///Checks if the value of the field is `PAGE208`
    #[inline(always)]
    pub fn is_page208(&self) -> bool {
        *self == PNB_A::PAGE208
    }
    ///Checks if the value of the field is `PAGE209`
    #[inline(always)]
    pub fn is_page209(&self) -> bool {
        *self == PNB_A::PAGE209
    }
    ///Checks if the value of the field is `PAGE210`
    #[inline(always)]
    pub fn is_page210(&self) -> bool {
        *self == PNB_A::PAGE210
    }
    ///Checks if the value of the field is `PAGE211`
    #[inline(always)]
    pub fn is_page211(&self) -> bool {
        *self == PNB_A::PAGE211
    }
    ///Checks if the value of the field is `PAGE212`
    #[inline(always)]
    pub fn is_page212(&self) -> bool {
        *self == PNB_A::PAGE212
    }
    ///Checks if the value of the field is `PAGE213`
    #[inline(always)]
    pub fn is_page213(&self) -> bool {
        *self == PNB_A::PAGE213
    }
    ///Checks if the value of the field is `PAGE214`
    #[inline(always)]
    pub fn is_page214(&self) -> bool {
        *self == PNB_A::PAGE214
    }
    ///Checks if the value of the field is `PAGE215`
    #[inline(always)]
    pub fn is_page215(&self) -> bool {
        *self == PNB_A::PAGE215
    }
    ///Checks if the value of the field is `PAGE216`
    #[inline(always)]
    pub fn is_page216(&self) -> bool {
        *self == PNB_A::PAGE216
    }
    ///Checks if the value of the field is `PAGE217`
    #[inline(always)]
    pub fn is_page217(&self) -> bool {
        *self == PNB_A::PAGE217
    }
    ///Checks if the value of the field is `PAGE218`
    #[inline(always)]
    pub fn is_page218(&self) -> bool {
        *self == PNB_A::PAGE218
    }
    ///Checks if the value of the field is `PAGE219`
    #[inline(always)]
    pub fn is_page219(&self) -> bool {
        *self == PNB_A::PAGE219
    }
    ///Checks if the value of the field is `PAGE220`
    #[inline(always)]
    pub fn is_page220(&self) -> bool {
        *self == PNB_A::PAGE220
    }
    ///Checks if the value of the field is `PAGE221`
    #[inline(always)]
    pub fn is_page221(&self) -> bool {
        *self == PNB_A::PAGE221
    }
    ///Checks if the value of the field is `PAGE222`
    #[inline(always)]
    pub fn is_page222(&self) -> bool {
        *self == PNB_A::PAGE222
    }
    ///Checks if the value of the field is `PAGE223`
    #[inline(always)]
    pub fn is_page223(&self) -> bool {
        *self == PNB_A::PAGE223
    }
    ///Checks if the value of the field is `PAGE224`
    #[inline(always)]
    pub fn is_page224(&self) -> bool {
        *self == PNB_A::PAGE224
    }
    ///Checks if the value of the field is `PAGE225`
    #[inline(always)]
    pub fn is_page225(&self) -> bool {
        *self == PNB_A::PAGE225
    }
    ///Checks if the value of the field is `PAGE226`
    #[inline(always)]
    pub fn is_page226(&self) -> bool {
        *self == PNB_A::PAGE226
    }
    ///Checks if the value of the field is `PAGE227`
    #[inline(always)]
    pub fn is_page227(&self) -> bool {
        *self == PNB_A::PAGE227
    }
    ///Checks if the value of the field is `PAGE228`
    #[inline(always)]
    pub fn is_page228(&self) -> bool {
        *self == PNB_A::PAGE228
    }
    ///Checks if the value of the field is `PAGE229`
    #[inline(always)]
    pub fn is_page229(&self) -> bool {
        *self == PNB_A::PAGE229
    }
    ///Checks if the value of the field is `PAGE230`
    #[inline(always)]
    pub fn is_page230(&self) -> bool {
        *self == PNB_A::PAGE230
    }
    ///Checks if the value of the field is `PAGE231`
    #[inline(always)]
    pub fn is_page231(&self) -> bool {
        *self == PNB_A::PAGE231
    }
    ///Checks if the value of the field is `PAGE232`
    #[inline(always)]
    pub fn is_page232(&self) -> bool {
        *self == PNB_A::PAGE232
    }
    ///Checks if the value of the field is `PAGE233`
    #[inline(always)]
    pub fn is_page233(&self) -> bool {
        *self == PNB_A::PAGE233
    }
    ///Checks if the value of the field is `PAGE234`
    #[inline(always)]
    pub fn is_page234(&self) -> bool {
        *self == PNB_A::PAGE234
    }
    ///Checks if the value of the field is `PAGE235`
    #[inline(always)]
    pub fn is_page235(&self) -> bool {
        *self == PNB_A::PAGE235
    }
    ///Checks if the value of the field is `PAGE236`
    #[inline(always)]
    pub fn is_page236(&self) -> bool {
        *self == PNB_A::PAGE236
    }
    ///Checks if the value of the field is `PAGE237`
    #[inline(always)]
    pub fn is_page237(&self) -> bool {
        *self == PNB_A::PAGE237
    }
    ///Checks if the value of the field is `PAGE238`
    #[inline(always)]
    pub fn is_page238(&self) -> bool {
        *self == PNB_A::PAGE238
    }
    ///Checks if the value of the field is `PAGE239`
    #[inline(always)]
    pub fn is_page239(&self) -> bool {
        *self == PNB_A::PAGE239
    }
    ///Checks if the value of the field is `PAGE240`
    #[inline(always)]
    pub fn is_page240(&self) -> bool {
        *self == PNB_A::PAGE240
    }
    ///Checks if the value of the field is `PAGE241`
    #[inline(always)]
    pub fn is_page241(&self) -> bool {
        *self == PNB_A::PAGE241
    }
    ///Checks if the value of the field is `PAGE242`
    #[inline(always)]
    pub fn is_page242(&self) -> bool {
        *self == PNB_A::PAGE242
    }
    ///Checks if the value of the field is `PAGE243`
    #[inline(always)]
    pub fn is_page243(&self) -> bool {
        *self == PNB_A::PAGE243
    }
    ///Checks if the value of the field is `PAGE244`
    #[inline(always)]
    pub fn is_page244(&self) -> bool {
        *self == PNB_A::PAGE244
    }
    ///Checks if the value of the field is `PAGE245`
    #[inline(always)]
    pub fn is_page245(&self) -> bool {
        *self == PNB_A::PAGE245
    }
    ///Checks if the value of the field is `PAGE246`
    #[inline(always)]
    pub fn is_page246(&self) -> bool {
        *self == PNB_A::PAGE246
    }
    ///Checks if the value of the field is `PAGE247`
    #[inline(always)]
    pub fn is_page247(&self) -> bool {
        *self == PNB_A::PAGE247
    }
    ///Checks if the value of the field is `PAGE248`
    #[inline(always)]
    pub fn is_page248(&self) -> bool {
        *self == PNB_A::PAGE248
    }
    ///Checks if the value of the field is `PAGE249`
    #[inline(always)]
    pub fn is_page249(&self) -> bool {
        *self == PNB_A::PAGE249
    }
    ///Checks if the value of the field is `PAGE250`
    #[inline(always)]
    pub fn is_page250(&self) -> bool {
        *self == PNB_A::PAGE250
    }
    ///Checks if the value of the field is `PAGE251`
    #[inline(always)]
    pub fn is_page251(&self) -> bool {
        *self == PNB_A::PAGE251
    }
    ///Checks if the value of the field is `PAGE252`
    #[inline(always)]
    pub fn is_page252(&self) -> bool {
        *self == PNB_A::PAGE252
    }
    ///Checks if the value of the field is `PAGE253`
    #[inline(always)]
    pub fn is_page253(&self) -> bool {
        *self == PNB_A::PAGE253
    }
    ///Checks if the value of the field is `PAGE254`
    #[inline(always)]
    pub fn is_page254(&self) -> bool {
        *self == PNB_A::PAGE254
    }
    ///Checks if the value of the field is `PAGE255`
    #[inline(always)]
    pub fn is_page255(&self) -> bool {
        *self == PNB_A::PAGE255
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
        unsafe { self.bits(variant.into()) }
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
    ///Erase Page 128
    #[inline(always)]
    pub fn page128(self) -> &'a mut W {
        self.variant(PNB_A::PAGE128)
    }
    ///Erase Page 129
    #[inline(always)]
    pub fn page129(self) -> &'a mut W {
        self.variant(PNB_A::PAGE129)
    }
    ///Erase Page 130
    #[inline(always)]
    pub fn page130(self) -> &'a mut W {
        self.variant(PNB_A::PAGE130)
    }
    ///Erase Page 131
    #[inline(always)]
    pub fn page131(self) -> &'a mut W {
        self.variant(PNB_A::PAGE131)
    }
    ///Erase Page 132
    #[inline(always)]
    pub fn page132(self) -> &'a mut W {
        self.variant(PNB_A::PAGE132)
    }
    ///Erase Page 133
    #[inline(always)]
    pub fn page133(self) -> &'a mut W {
        self.variant(PNB_A::PAGE133)
    }
    ///Erase Page 134
    #[inline(always)]
    pub fn page134(self) -> &'a mut W {
        self.variant(PNB_A::PAGE134)
    }
    ///Erase Page 135
    #[inline(always)]
    pub fn page135(self) -> &'a mut W {
        self.variant(PNB_A::PAGE135)
    }
    ///Erase Page 136
    #[inline(always)]
    pub fn page136(self) -> &'a mut W {
        self.variant(PNB_A::PAGE136)
    }
    ///Erase Page 137
    #[inline(always)]
    pub fn page137(self) -> &'a mut W {
        self.variant(PNB_A::PAGE137)
    }
    ///Erase Page 138
    #[inline(always)]
    pub fn page138(self) -> &'a mut W {
        self.variant(PNB_A::PAGE138)
    }
    ///Erase Page 139
    #[inline(always)]
    pub fn page139(self) -> &'a mut W {
        self.variant(PNB_A::PAGE139)
    }
    ///Erase Page 140
    #[inline(always)]
    pub fn page140(self) -> &'a mut W {
        self.variant(PNB_A::PAGE140)
    }
    ///Erase Page 141
    #[inline(always)]
    pub fn page141(self) -> &'a mut W {
        self.variant(PNB_A::PAGE141)
    }
    ///Erase Page 142
    #[inline(always)]
    pub fn page142(self) -> &'a mut W {
        self.variant(PNB_A::PAGE142)
    }
    ///Erase Page 143
    #[inline(always)]
    pub fn page143(self) -> &'a mut W {
        self.variant(PNB_A::PAGE143)
    }
    ///Erase Page 144
    #[inline(always)]
    pub fn page144(self) -> &'a mut W {
        self.variant(PNB_A::PAGE144)
    }
    ///Erase Page 145
    #[inline(always)]
    pub fn page145(self) -> &'a mut W {
        self.variant(PNB_A::PAGE145)
    }
    ///Erase Page 146
    #[inline(always)]
    pub fn page146(self) -> &'a mut W {
        self.variant(PNB_A::PAGE146)
    }
    ///Erase Page 147
    #[inline(always)]
    pub fn page147(self) -> &'a mut W {
        self.variant(PNB_A::PAGE147)
    }
    ///Erase Page 148
    #[inline(always)]
    pub fn page148(self) -> &'a mut W {
        self.variant(PNB_A::PAGE148)
    }
    ///Erase Page 149
    #[inline(always)]
    pub fn page149(self) -> &'a mut W {
        self.variant(PNB_A::PAGE149)
    }
    ///Erase Page 150
    #[inline(always)]
    pub fn page150(self) -> &'a mut W {
        self.variant(PNB_A::PAGE150)
    }
    ///Erase Page 151
    #[inline(always)]
    pub fn page151(self) -> &'a mut W {
        self.variant(PNB_A::PAGE151)
    }
    ///Erase Page 152
    #[inline(always)]
    pub fn page152(self) -> &'a mut W {
        self.variant(PNB_A::PAGE152)
    }
    ///Erase Page 153
    #[inline(always)]
    pub fn page153(self) -> &'a mut W {
        self.variant(PNB_A::PAGE153)
    }
    ///Erase Page 154
    #[inline(always)]
    pub fn page154(self) -> &'a mut W {
        self.variant(PNB_A::PAGE154)
    }
    ///Erase Page 155
    #[inline(always)]
    pub fn page155(self) -> &'a mut W {
        self.variant(PNB_A::PAGE155)
    }
    ///Erase Page 156
    #[inline(always)]
    pub fn page156(self) -> &'a mut W {
        self.variant(PNB_A::PAGE156)
    }
    ///Erase Page 157
    #[inline(always)]
    pub fn page157(self) -> &'a mut W {
        self.variant(PNB_A::PAGE157)
    }
    ///Erase Page 158
    #[inline(always)]
    pub fn page158(self) -> &'a mut W {
        self.variant(PNB_A::PAGE158)
    }
    ///Erase Page 159
    #[inline(always)]
    pub fn page159(self) -> &'a mut W {
        self.variant(PNB_A::PAGE159)
    }
    ///Erase Page 160
    #[inline(always)]
    pub fn page160(self) -> &'a mut W {
        self.variant(PNB_A::PAGE160)
    }
    ///Erase Page 161
    #[inline(always)]
    pub fn page161(self) -> &'a mut W {
        self.variant(PNB_A::PAGE161)
    }
    ///Erase Page 162
    #[inline(always)]
    pub fn page162(self) -> &'a mut W {
        self.variant(PNB_A::PAGE162)
    }
    ///Erase Page 163
    #[inline(always)]
    pub fn page163(self) -> &'a mut W {
        self.variant(PNB_A::PAGE163)
    }
    ///Erase Page 164
    #[inline(always)]
    pub fn page164(self) -> &'a mut W {
        self.variant(PNB_A::PAGE164)
    }
    ///Erase Page 165
    #[inline(always)]
    pub fn page165(self) -> &'a mut W {
        self.variant(PNB_A::PAGE165)
    }
    ///Erase Page 166
    #[inline(always)]
    pub fn page166(self) -> &'a mut W {
        self.variant(PNB_A::PAGE166)
    }
    ///Erase Page 167
    #[inline(always)]
    pub fn page167(self) -> &'a mut W {
        self.variant(PNB_A::PAGE167)
    }
    ///Erase Page 168
    #[inline(always)]
    pub fn page168(self) -> &'a mut W {
        self.variant(PNB_A::PAGE168)
    }
    ///Erase Page 169
    #[inline(always)]
    pub fn page169(self) -> &'a mut W {
        self.variant(PNB_A::PAGE169)
    }
    ///Erase Page 170
    #[inline(always)]
    pub fn page170(self) -> &'a mut W {
        self.variant(PNB_A::PAGE170)
    }
    ///Erase Page 171
    #[inline(always)]
    pub fn page171(self) -> &'a mut W {
        self.variant(PNB_A::PAGE171)
    }
    ///Erase Page 172
    #[inline(always)]
    pub fn page172(self) -> &'a mut W {
        self.variant(PNB_A::PAGE172)
    }
    ///Erase Page 173
    #[inline(always)]
    pub fn page173(self) -> &'a mut W {
        self.variant(PNB_A::PAGE173)
    }
    ///Erase Page 174
    #[inline(always)]
    pub fn page174(self) -> &'a mut W {
        self.variant(PNB_A::PAGE174)
    }
    ///Erase Page 175
    #[inline(always)]
    pub fn page175(self) -> &'a mut W {
        self.variant(PNB_A::PAGE175)
    }
    ///Erase Page 176
    #[inline(always)]
    pub fn page176(self) -> &'a mut W {
        self.variant(PNB_A::PAGE176)
    }
    ///Erase Page 177
    #[inline(always)]
    pub fn page177(self) -> &'a mut W {
        self.variant(PNB_A::PAGE177)
    }
    ///Erase Page 178
    #[inline(always)]
    pub fn page178(self) -> &'a mut W {
        self.variant(PNB_A::PAGE178)
    }
    ///Erase Page 179
    #[inline(always)]
    pub fn page179(self) -> &'a mut W {
        self.variant(PNB_A::PAGE179)
    }
    ///Erase Page 180
    #[inline(always)]
    pub fn page180(self) -> &'a mut W {
        self.variant(PNB_A::PAGE180)
    }
    ///Erase Page 181
    #[inline(always)]
    pub fn page181(self) -> &'a mut W {
        self.variant(PNB_A::PAGE181)
    }
    ///Erase Page 182
    #[inline(always)]
    pub fn page182(self) -> &'a mut W {
        self.variant(PNB_A::PAGE182)
    }
    ///Erase Page 183
    #[inline(always)]
    pub fn page183(self) -> &'a mut W {
        self.variant(PNB_A::PAGE183)
    }
    ///Erase Page 184
    #[inline(always)]
    pub fn page184(self) -> &'a mut W {
        self.variant(PNB_A::PAGE184)
    }
    ///Erase Page 185
    #[inline(always)]
    pub fn page185(self) -> &'a mut W {
        self.variant(PNB_A::PAGE185)
    }
    ///Erase Page 186
    #[inline(always)]
    pub fn page186(self) -> &'a mut W {
        self.variant(PNB_A::PAGE186)
    }
    ///Erase Page 187
    #[inline(always)]
    pub fn page187(self) -> &'a mut W {
        self.variant(PNB_A::PAGE187)
    }
    ///Erase Page 188
    #[inline(always)]
    pub fn page188(self) -> &'a mut W {
        self.variant(PNB_A::PAGE188)
    }
    ///Erase Page 189
    #[inline(always)]
    pub fn page189(self) -> &'a mut W {
        self.variant(PNB_A::PAGE189)
    }
    ///Erase Page 190
    #[inline(always)]
    pub fn page190(self) -> &'a mut W {
        self.variant(PNB_A::PAGE190)
    }
    ///Erase Page 191
    #[inline(always)]
    pub fn page191(self) -> &'a mut W {
        self.variant(PNB_A::PAGE191)
    }
    ///Erase Page 192
    #[inline(always)]
    pub fn page192(self) -> &'a mut W {
        self.variant(PNB_A::PAGE192)
    }
    ///Erase Page 193
    #[inline(always)]
    pub fn page193(self) -> &'a mut W {
        self.variant(PNB_A::PAGE193)
    }
    ///Erase Page 194
    #[inline(always)]
    pub fn page194(self) -> &'a mut W {
        self.variant(PNB_A::PAGE194)
    }
    ///Erase Page 195
    #[inline(always)]
    pub fn page195(self) -> &'a mut W {
        self.variant(PNB_A::PAGE195)
    }
    ///Erase Page 196
    #[inline(always)]
    pub fn page196(self) -> &'a mut W {
        self.variant(PNB_A::PAGE196)
    }
    ///Erase Page 197
    #[inline(always)]
    pub fn page197(self) -> &'a mut W {
        self.variant(PNB_A::PAGE197)
    }
    ///Erase Page 198
    #[inline(always)]
    pub fn page198(self) -> &'a mut W {
        self.variant(PNB_A::PAGE198)
    }
    ///Erase Page 199
    #[inline(always)]
    pub fn page199(self) -> &'a mut W {
        self.variant(PNB_A::PAGE199)
    }
    ///Erase Page 200
    #[inline(always)]
    pub fn page200(self) -> &'a mut W {
        self.variant(PNB_A::PAGE200)
    }
    ///Erase Page 201
    #[inline(always)]
    pub fn page201(self) -> &'a mut W {
        self.variant(PNB_A::PAGE201)
    }
    ///Erase Page 202
    #[inline(always)]
    pub fn page202(self) -> &'a mut W {
        self.variant(PNB_A::PAGE202)
    }
    ///Erase Page 203
    #[inline(always)]
    pub fn page203(self) -> &'a mut W {
        self.variant(PNB_A::PAGE203)
    }
    ///Erase Page 204
    #[inline(always)]
    pub fn page204(self) -> &'a mut W {
        self.variant(PNB_A::PAGE204)
    }
    ///Erase Page 205
    #[inline(always)]
    pub fn page205(self) -> &'a mut W {
        self.variant(PNB_A::PAGE205)
    }
    ///Erase Page 206
    #[inline(always)]
    pub fn page206(self) -> &'a mut W {
        self.variant(PNB_A::PAGE206)
    }
    ///Erase Page 207
    #[inline(always)]
    pub fn page207(self) -> &'a mut W {
        self.variant(PNB_A::PAGE207)
    }
    ///Erase Page 208
    #[inline(always)]
    pub fn page208(self) -> &'a mut W {
        self.variant(PNB_A::PAGE208)
    }
    ///Erase Page 209
    #[inline(always)]
    pub fn page209(self) -> &'a mut W {
        self.variant(PNB_A::PAGE209)
    }
    ///Erase Page 210
    #[inline(always)]
    pub fn page210(self) -> &'a mut W {
        self.variant(PNB_A::PAGE210)
    }
    ///Erase Page 211
    #[inline(always)]
    pub fn page211(self) -> &'a mut W {
        self.variant(PNB_A::PAGE211)
    }
    ///Erase Page 212
    #[inline(always)]
    pub fn page212(self) -> &'a mut W {
        self.variant(PNB_A::PAGE212)
    }
    ///Erase Page 213
    #[inline(always)]
    pub fn page213(self) -> &'a mut W {
        self.variant(PNB_A::PAGE213)
    }
    ///Erase Page 214
    #[inline(always)]
    pub fn page214(self) -> &'a mut W {
        self.variant(PNB_A::PAGE214)
    }
    ///Erase Page 215
    #[inline(always)]
    pub fn page215(self) -> &'a mut W {
        self.variant(PNB_A::PAGE215)
    }
    ///Erase Page 216
    #[inline(always)]
    pub fn page216(self) -> &'a mut W {
        self.variant(PNB_A::PAGE216)
    }
    ///Erase Page 217
    #[inline(always)]
    pub fn page217(self) -> &'a mut W {
        self.variant(PNB_A::PAGE217)
    }
    ///Erase Page 218
    #[inline(always)]
    pub fn page218(self) -> &'a mut W {
        self.variant(PNB_A::PAGE218)
    }
    ///Erase Page 219
    #[inline(always)]
    pub fn page219(self) -> &'a mut W {
        self.variant(PNB_A::PAGE219)
    }
    ///Erase Page 220
    #[inline(always)]
    pub fn page220(self) -> &'a mut W {
        self.variant(PNB_A::PAGE220)
    }
    ///Erase Page 221
    #[inline(always)]
    pub fn page221(self) -> &'a mut W {
        self.variant(PNB_A::PAGE221)
    }
    ///Erase Page 222
    #[inline(always)]
    pub fn page222(self) -> &'a mut W {
        self.variant(PNB_A::PAGE222)
    }
    ///Erase Page 223
    #[inline(always)]
    pub fn page223(self) -> &'a mut W {
        self.variant(PNB_A::PAGE223)
    }
    ///Erase Page 224
    #[inline(always)]
    pub fn page224(self) -> &'a mut W {
        self.variant(PNB_A::PAGE224)
    }
    ///Erase Page 225
    #[inline(always)]
    pub fn page225(self) -> &'a mut W {
        self.variant(PNB_A::PAGE225)
    }
    ///Erase Page 226
    #[inline(always)]
    pub fn page226(self) -> &'a mut W {
        self.variant(PNB_A::PAGE226)
    }
    ///Erase Page 227
    #[inline(always)]
    pub fn page227(self) -> &'a mut W {
        self.variant(PNB_A::PAGE227)
    }
    ///Erase Page 228
    #[inline(always)]
    pub fn page228(self) -> &'a mut W {
        self.variant(PNB_A::PAGE228)
    }
    ///Erase Page 229
    #[inline(always)]
    pub fn page229(self) -> &'a mut W {
        self.variant(PNB_A::PAGE229)
    }
    ///Erase Page 230
    #[inline(always)]
    pub fn page230(self) -> &'a mut W {
        self.variant(PNB_A::PAGE230)
    }
    ///Erase Page 231
    #[inline(always)]
    pub fn page231(self) -> &'a mut W {
        self.variant(PNB_A::PAGE231)
    }
    ///Erase Page 232
    #[inline(always)]
    pub fn page232(self) -> &'a mut W {
        self.variant(PNB_A::PAGE232)
    }
    ///Erase Page 233
    #[inline(always)]
    pub fn page233(self) -> &'a mut W {
        self.variant(PNB_A::PAGE233)
    }
    ///Erase Page 234
    #[inline(always)]
    pub fn page234(self) -> &'a mut W {
        self.variant(PNB_A::PAGE234)
    }
    ///Erase Page 235
    #[inline(always)]
    pub fn page235(self) -> &'a mut W {
        self.variant(PNB_A::PAGE235)
    }
    ///Erase Page 236
    #[inline(always)]
    pub fn page236(self) -> &'a mut W {
        self.variant(PNB_A::PAGE236)
    }
    ///Erase Page 237
    #[inline(always)]
    pub fn page237(self) -> &'a mut W {
        self.variant(PNB_A::PAGE237)
    }
    ///Erase Page 238
    #[inline(always)]
    pub fn page238(self) -> &'a mut W {
        self.variant(PNB_A::PAGE238)
    }
    ///Erase Page 239
    #[inline(always)]
    pub fn page239(self) -> &'a mut W {
        self.variant(PNB_A::PAGE239)
    }
    ///Erase Page 240
    #[inline(always)]
    pub fn page240(self) -> &'a mut W {
        self.variant(PNB_A::PAGE240)
    }
    ///Erase Page 241
    #[inline(always)]
    pub fn page241(self) -> &'a mut W {
        self.variant(PNB_A::PAGE241)
    }
    ///Erase Page 242
    #[inline(always)]
    pub fn page242(self) -> &'a mut W {
        self.variant(PNB_A::PAGE242)
    }
    ///Erase Page 243
    #[inline(always)]
    pub fn page243(self) -> &'a mut W {
        self.variant(PNB_A::PAGE243)
    }
    ///Erase Page 244
    #[inline(always)]
    pub fn page244(self) -> &'a mut W {
        self.variant(PNB_A::PAGE244)
    }
    ///Erase Page 245
    #[inline(always)]
    pub fn page245(self) -> &'a mut W {
        self.variant(PNB_A::PAGE245)
    }
    ///Erase Page 246
    #[inline(always)]
    pub fn page246(self) -> &'a mut W {
        self.variant(PNB_A::PAGE246)
    }
    ///Erase Page 247
    #[inline(always)]
    pub fn page247(self) -> &'a mut W {
        self.variant(PNB_A::PAGE247)
    }
    ///Erase Page 248
    #[inline(always)]
    pub fn page248(self) -> &'a mut W {
        self.variant(PNB_A::PAGE248)
    }
    ///Erase Page 249
    #[inline(always)]
    pub fn page249(self) -> &'a mut W {
        self.variant(PNB_A::PAGE249)
    }
    ///Erase Page 250
    #[inline(always)]
    pub fn page250(self) -> &'a mut W {
        self.variant(PNB_A::PAGE250)
    }
    ///Erase Page 251
    #[inline(always)]
    pub fn page251(self) -> &'a mut W {
        self.variant(PNB_A::PAGE251)
    }
    ///Erase Page 252
    #[inline(always)]
    pub fn page252(self) -> &'a mut W {
        self.variant(PNB_A::PAGE252)
    }
    ///Erase Page 253
    #[inline(always)]
    pub fn page253(self) -> &'a mut W {
        self.variant(PNB_A::PAGE253)
    }
    ///Erase Page 254
    #[inline(always)]
    pub fn page254(self) -> &'a mut W {
        self.variant(PNB_A::PAGE254)
    }
    ///Erase Page 255
    #[inline(always)]
    pub fn page255(self) -> &'a mut W {
        self.variant(PNB_A::PAGE255)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
