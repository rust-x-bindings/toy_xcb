
use std::ops::{BitAnd, BitOr, BitXor};


pub const MODS_CTRL_MASK  : u8 = 0x01;
pub const MODS_SHIFT_MASK : u8 = 0x02;
pub const MODS_META_MASK  : u8 = 0x04;
pub const MODS_ALT_MASK   : u8 = 0x08;
pub const MODS_SUPER_MASK : u8 = 0x10;
pub const MODS_KEY_MASK   : u8 = 0x1f;

pub const MODS_LEFT_MASK  : u8 = 0x20;
pub const MODS_RIGHT_MASK : u8 = 0x40;
pub const MODS_SIDE_MASK  : u8 = 0x60;

pub const MODS_LEFT_CTRL  : u8 = MODS_LEFT_MASK | MODS_CTRL_MASK;
pub const MODS_LEFT_SHIFT : u8 = MODS_LEFT_MASK | MODS_SHIFT_MASK;
pub const MODS_LEFT_META  : u8 = MODS_LEFT_MASK | MODS_META_MASK;
pub const MODS_LEFT_ALT   : u8 = MODS_LEFT_MASK | MODS_ALT_MASK;
pub const MODS_LEFT_SUPER : u8 = MODS_LEFT_MASK | MODS_SUPER_MASK;

pub const MODS_RIGHT_CTRL : u8 = MODS_RIGHT_MASK | MODS_CTRL_MASK;
pub const MODS_RIGHT_SHIFT: u8 = MODS_RIGHT_MASK | MODS_SHIFT_MASK;
pub const MODS_RIGHT_META : u8 = MODS_RIGHT_MASK | MODS_META_MASK;
pub const MODS_RIGHT_ALT  : u8 = MODS_RIGHT_MASK | MODS_ALT_MASK;
pub const MODS_RIGHT_SUPER: u8 = MODS_RIGHT_MASK | MODS_SUPER_MASK;

pub const MODS_CTRL       : u8 = MODS_LEFT_CTRL  | MODS_RIGHT_CTRL;
pub const MODS_SHIFT      : u8 = MODS_LEFT_SHIFT | MODS_RIGHT_SHIFT;
pub const MODS_META       : u8 = MODS_LEFT_META  | MODS_RIGHT_META;
pub const MODS_ALT        : u8 = MODS_LEFT_ALT   | MODS_RIGHT_ALT;
pub const MODS_SUPER      : u8 = MODS_LEFT_SUPER | MODS_RIGHT_SUPER;



#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct Mods {
    fields: u8
}

impl Mods {

    pub fn new(fields: u8) -> Mods {
        Mods { fields: fields & (MODS_KEY_MASK | MODS_SIDE_MASK) }
    }

    pub fn fields(&self) -> u8 {
        self.fields
    }

    pub fn is_left(&self) -> bool {
        (self.fields & MODS_LEFT_MASK) != 0
    }
    pub fn is_right(&self) -> bool {
        (self.fields & MODS_RIGHT_MASK) != 0
    }

    pub fn has_ctrl(&self) -> bool {
        (self.fields & MODS_CTRL_MASK) != 0
    }
    pub fn has_shift(&self) -> bool {
        (self.fields & MODS_SHIFT_MASK) != 0
    }
    pub fn has_meta(&self) -> bool {
        (self.fields & MODS_META_MASK) != 0
    }
    pub fn has_alt(&self) -> bool {
        (self.fields & MODS_ALT_MASK) != 0
    }
    pub fn has_super(&self) -> bool {
        (self.fields & MODS_SUPER_MASK) != 0
    }


    pub fn has_all(&self, fields: u8) -> bool {
        let fields = fields & MODS_KEY_MASK;
        (self.fields & fields) == fields
    }
    pub fn has_any(&self, fields: u8) -> bool {
        let fields = fields & MODS_KEY_MASK;
        (self.fields & fields) != 0
    }
    pub fn has_none(&self, fields: u8) -> bool {
        let fields = fields & MODS_KEY_MASK;
        (self.fields & fields) == 0
    }
}

impl PartialEq<u8> for Mods {
    fn eq(&self, rhs: &u8) -> bool {
        self.fields == *rhs
    }
}

impl PartialEq<Mods> for u8 {
    fn eq (&self, rhs: &Mods) -> bool {
        *self == rhs.fields
    }
}

impl BitAnd for Mods {
    type Output = Mods;
    fn bitand(self, rhs: Mods) -> Mods {
        Mods { fields: self.fields & rhs.fields }
    }
}

impl BitOr for Mods {
    type Output = Mods;
    fn bitor(self, rhs: Mods) -> Mods {
        Mods { fields: self.fields | rhs.fields }
    }
}

impl BitXor for Mods {
    type Output = Mods;
    fn bitxor(self, rhs: Mods) -> Mods {
        Mods { fields: self.fields ^ rhs.fields }
    }
}



// values are from USB HID table
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Code {

    None                = 0,
    ErrorRollOver       = 1,
    POSTFail            = 2,
    ErrorUndefined      = 3,
    A                   = 4,
    B                   = 5,
    C                   = 6,
    D                   = 7,
    E                   = 8,
    F                   = 9,
    G                   = 10,
    H                   = 11,
    I                   = 12,
    J                   = 13,
    K                   = 14,
    L                   = 15,
    M                   = 16,
    N                   = 17,
    O                   = 18,
    P                   = 19,
    Q                   = 20,
    R                   = 21,
    S                   = 22,
    T                   = 23,
    U                   = 24,
    V                   = 25,
    W                   = 26,
    X                   = 27,
    Y                   = 28,
    Z                   = 29,
    N1                  = 30,
    N2                  = 31,
    N3                  = 32,
    N4                  = 33,
    N5                  = 34,
    N6                  = 35,
    N7                  = 36,
    N8                  = 37,
    N9                  = 38,
    N0                  = 39,
    Enter               = 40,
    Escape              = 41,
    Backspace           = 42,
    Tab                 = 43,
    Space               = 44,
    Minus               = 45,
    Equals              = 46,
    LeftBracket         = 47,
    RightBracket        = 48,
    Backslash           = 49,
    UK_Hash             = 50,
    Semicolon           = 51,
    Quote               = 52,
    Grave               = 53,
    Comma               = 54,
    Period              = 55,
    Slash               = 56,
    CapsLock            = 57,
    F1                  = 58,
    F2                  = 59,
    F3                  = 60,
    F4                  = 61,
    F5                  = 62,
    F6                  = 63,
    F7                  = 64,
    F8                  = 65,
    F9                  = 66,
    F10                 = 67,
    F11                 = 68,
    F12                 = 69,
    PrintScreen         = 70,
    ScrollLock          = 71,
    Pause               = 72,
    Insert              = 73,
    Home                = 74,
    PageUp              = 75,
    Delete              = 76,
    End                 = 77,
    PageDown            = 78,
    Right               = 79,
    Left                = 80,
    Down                = 81,
    Up                  = 82,
    KP_NumLock          = 83,
    KP_Divide           = 84,
    KP_Multiply         = 85,
    KP_Subtract         = 86,
    KP_Add              = 87,
    KP_Enter            = 88,
    KP_1                = 89,
    KP_2                = 90,
    KP_3                = 91,
    KP_4                = 92,
    KP_5                = 93,
    KP_6                = 94,
    KP_7                = 95,
    KP_8                = 96,
    KP_9                = 97,
    KP_0                = 98,
    KP_Period           = 99,
    UK_Backslash        = 100,
    KP_Equal            = 103,
    F13                 = 104,
    F14                 = 105,
    F15                 = 106,
    F16                 = 107,
    F17                 = 108,
    F18                 = 109,
    F19                 = 110,
    F20                 = 111,
    F21                 = 112,
    F22                 = 113,
    F23                 = 114,
    F24                 = 115,
    Execute             = 116,
    Help                = 117,
    Menu                = 118,
    Select              = 119,
    Stop                = 120,
    Again               = 121,
    Undo                = 122,
    Cut                 = 123,
    Copy                = 124,
    Paste               = 125,
    Find                = 126,
    Mute                = 127,
    VolumeUp            = 128,
    VolumeDown          = 129,
    LockingCapsLock     = 130,
    LockingNumLock      = 131,
    LockingScrollLock   = 132,
    KP_Comma            = 133,
    KP_EqualSign        = 134,
    International1      = 135,
    International2      = 136,
    International3      = 137,
    International4      = 138,
    International5      = 139,
    International6      = 140,
    International7      = 141,
    International8      = 142,
    International9      = 143,
    LANG1               = 144,  // Hangul / English toggle
    LANG2               = 145,  // Hanja conversion
    LANG3               = 146,  // Katakana
    LANG4               = 147,  // Hiragana
    LANG5               = 148,  // Zenkaku/Hankaku
    LANG6               = 149,
    LANG7               = 150,
    LANG8               = 151,
    LANG9               = 152,
    AltErase            = 153,
    SysReq              = 154,
    Cancel              = 155,
    Clear               = 156,
    Prior               = 157,
    Return              = 158,
    Separator           = 159,
    Out                 = 160,
    Oper                = 161,
    ClearAgain          = 162,
    CrSelProps          = 163,
    ExSel               = 164,

    KP_00               = 176,
    KP_000              = 177,
    ThousandsSep        = 178,
    DecimalSep          = 179,
    CurrencyUnit        = 180,
    CurrencySubUnit     = 181,
    KP_LeftParent       = 182,
    KP_RightParent      = 183,
    KP_LeftCurly        = 184,
    KP_RightCurly       = 185,
    KP_Tab              = 186,
    KP_Backspace        = 187,
    KP_A                = 188,
    KP_B                = 189,
    KP_C                = 190,
    KP_D                = 191,
    KP_E                = 192,
    KP_F                = 193,
    KP_XOR              = 194,
    KP_Pow              = 195,
    KP_Percent          = 196,
    KP_LeftAngle        = 197,
    KP_RightAngle       = 198,
    KP_BitAnd           = 199,
    KP_LogicAnd         = 200,
    KP_BitOr            = 201,
    KP_LogicOr          = 202,
    KP_Colon            = 203,
    KP_Hash             = 204,
    KP_Space            = 205,
    KP_At               = 206,
    KP_Not              = 207,
    KP_MemStore         = 208,
    KP_MemRecall        = 209,
    KP_MemClear         = 210,
    KP_MemAdd           = 211,
    KP_MemSubtract      = 212,
    KP_MemMultiply      = 213,
    KP_MemDivide        = 214,
    KP_PlusMinus        = 215,
    KP_Clear            = 216,
    KP_ClearEntry       = 217,
    KP_Binary           = 218,
    KP_Octal            = 219,
    KP_Decimal          = 220,
    KP_Hexadecimal      = 221,

    LeftCtrl            = 224,
    LeftShift           = 225,
    LeftAlt             = 226,
    LeftSuper           = 227,
    RightCtrl           = 228,
    RightShift          = 229,
    RightAlt            = 230,
    RightSuper          = 231,

    Unknown             = 255
}



pub const SYM_CONTROL_MASK     :isize = 0x8000_0000;
pub const SYM_KP_MASK          :isize = 0x4000_0000;
pub const SYM_MEDIA_MASK       :isize = 0x2000_0000;
pub const SYM_MODS_MASK        :isize = 0x0080_0000;
pub const SYM_CTRL_MASK        :isize = 0x0001_0000;
pub const SYM_SHIFT_MASK       :isize = 0x0002_0000;
pub const SYM_META_MASK        :isize = 0x0004_0000;
pub const SYM_ALT_MASK         :isize = 0x0008_0000;
pub const SYM_SUPER_MASK       :isize = 0x0010_0000;
pub const SYM_LEFT_MASK        :isize = 0x0020_0000;
pub const SYM_RIGHT_MASK       :isize = 0x0040_0000;
pub const SYM_LATIN1_SMALL_MASK:isize = 0x0000_0020;


#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Sym {

    None                    = 0,

    Unknown                 = 0x0000_ffdf,


    Escape                  = SYM_CONTROL_MASK | 1,
    Tab,
    LeftTab,
    Backspace,
    Return,
    Delete,
    SysRq,
    Pause,
    Clear,

    CapsLock,
    NumLock,
    ScrollLock,

    Left,
    Up,
    Right,
    Down,
    PageUp,
    PageDown,
    Home,
    End,

    Print,
    Insert,
    Menu,
    Help,
    Break,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    KP_Enter                = SYM_KP_MASK | 1,
    KP_Delete,
    KP_Home,
    KP_Begin,
    KP_End,
    KP_PageUp,
    KP_PageDown,
    KP_Up,
    KP_Down,
    KP_Left,
    KP_Right,
    KP_Equal,
    KP_Multiply,
    KP_Add,
    KP_Divide,
    KP_Subtract,
    KP_Decimal,
    KP_Separator,

    KP_0,
    KP_1,
    KP_2,
    KP_3,
    KP_4,
    KP_6,
    KP_7,
    KP_8,
    KP_9,



    dead_grave              = SYM_MODS_MASK | 1,
    dead_acute,
    dead_circumflex,
    dead_tilde,
    dead_macron,
    dead_breve,
    dead_abovedot,
    dead_diaeresis,
    dead_abovering,
    dead_doubleacute,
    dead_caron,
    dead_cedilla,
    dead_ogonek,
    dead_iota,
    dead_voiced_sound,
    dead_semivoiced_sound,
    dead_belowdot,
    dead_hook,
    dead_horn,
    dead_stroke,
    dead_abovecomma,
    dead_abovereversedcomma,
    dead_doublegrave,
    dead_belowring,
    dead_belowmacron,
    dead_belowcircumflex,
    dead_belowtilde,
    dead_belowbreve,
    dead_belowdiaeresis,
    dead_invertedbreve,
    dead_belowcomma,
    dead_currency,

/* extra dead elements for German T3 layout */
    dead_lowline,
    dead_aboveverticalline,
    dead_belowverticalline,
    dead_longsolidusoverlay,

/* dead vowels for universal syllable entry */
    dead_a,
    dead_A,
    dead_e,
    dead_E,
    dead_i,
    dead_I,
    dead_o,
    dead_O,
    dead_u,
    dead_U,
    dead_small_schwa,
    dead_capital_schwa,

    ModeSwitch,

    LeftCtrl                = SYM_CTRL_MASK  | SYM_LEFT_MASK  | SYM_MODS_MASK,
    RightCtrl               = SYM_CTRL_MASK  | SYM_RIGHT_MASK | SYM_MODS_MASK,
    LeftShift               = SYM_SHIFT_MASK | SYM_LEFT_MASK  | SYM_MODS_MASK,
    RightShift              = SYM_SHIFT_MASK | SYM_RIGHT_MASK | SYM_MODS_MASK,
    LeftMeta                = SYM_META_MASK  | SYM_LEFT_MASK  | SYM_MODS_MASK,
    RightMeta               = SYM_META_MASK  | SYM_RIGHT_MASK | SYM_MODS_MASK,
    LeftAlt                 = SYM_ALT_MASK   | SYM_LEFT_MASK  | SYM_MODS_MASK,
    RightAlt                = SYM_ALT_MASK   | SYM_RIGHT_MASK | SYM_MODS_MASK,
    LeftSuper               = SYM_SUPER_MASK | SYM_LEFT_MASK  | SYM_MODS_MASK,
    RightSuper              = SYM_SUPER_MASK | SYM_RIGHT_MASK | SYM_MODS_MASK,

    Ctrl                    = SYM_CTRL_MASK  | SYM_LEFT_MASK  | SYM_RIGHT_MASK | SYM_MODS_MASK,
    Shift                   = SYM_SHIFT_MASK | SYM_LEFT_MASK  | SYM_RIGHT_MASK | SYM_MODS_MASK,
    Meta                    = SYM_META_MASK  | SYM_LEFT_MASK  | SYM_RIGHT_MASK | SYM_MODS_MASK,
    Alt                     = SYM_ALT_MASK   | SYM_LEFT_MASK  | SYM_RIGHT_MASK | SYM_MODS_MASK,
    Super                   = SYM_SUPER_MASK | SYM_LEFT_MASK  | SYM_RIGHT_MASK | SYM_MODS_MASK,

    //AltGr                 = SYM_ALT_MASK   | SYM_RIGHT_MASK | SYM_MODS_MASK,

    /*
     * Latin 1
     * (ISO/IEC 8859-1 = Unicode U+0020..U+00FF)
     * Byte 3 = 0
     */
    space                   = 0x0000_0020,  /* U+0020 SPACE */
    exclam                  = 0x0000_0021,  /* U+0021 EXCLAMATION MARK */
    quotedbl                = 0x0000_0022,  /* U+0022 QUOTATION MARK */
    numbersign              = 0x0000_0023,  /* U+0023 NUMBER SIGN */
    dollar                  = 0x0000_0024,  /* U+0024 DOLLAR SIGN */
    percent                 = 0x0000_0025,  /* U+0025 PERCENT SIGN */
    ampersand               = 0x0000_0026,  /* U+0026 AMPERSAND */
    apostrophe              = 0x0000_0027,  /* U+0027 APOSTROPHE */
    parenleft               = 0x0000_0028,  /* U+0028 LEFT PARENTHESIS */
    parenright              = 0x0000_0029,  /* U+0029 RIGHT PARENTHESIS */
    asterisk                = 0x0000_002a,  /* U+002A ASTERISK */
    plus                    = 0x0000_002b,  /* U+002B PLUS SIGN */
    comma                   = 0x0000_002c,  /* U+002C COMMA */
    minus                   = 0x0000_002d,  /* U+002D HYPHEN-MINUS */
    period                  = 0x0000_002e,  /* U+002E FULL STOP */
    slash                   = 0x0000_002f,  /* U+002F SOLIDUS */
    D0                      = 0x0000_0030,  /* U+0030 DIGIT ZERO */
    D1                      = 0x0000_0031,  /* U+0031 DIGIT ONE */
    D2                      = 0x0000_0032,  /* U+0032 DIGIT TWO */
    D3                      = 0x0000_0033,  /* U+0033 DIGIT THREE */
    D4                      = 0x0000_0034,  /* U+0034 DIGIT FOUR */
    D5                      = 0x0000_0035,  /* U+0035 DIGIT FIVE */
    D6                      = 0x0000_0036,  /* U+0036 DIGIT SIX */
    D7                      = 0x0000_0037,  /* U+0037 DIGIT SEVEN */
    D8                      = 0x0000_0038,  /* U+0038 DIGIT EIGHT */
    D9                      = 0x0000_0039,  /* U+0039 DIGIT NINE */
    colon                   = 0x0000_003a,  /* U+003A COLON */
    semicolon               = 0x0000_003b,  /* U+003B SEMICOLON */
    less                    = 0x0000_003c,  /* U+003C LESS-THAN SIGN */
    equal                   = 0x0000_003d,  /* U+003D EQUALS SIGN */
    greater                 = 0x0000_003e,  /* U+003E GREATER-THAN SIGN */
    question                = 0x0000_003f,  /* U+003F QUESTION MARK */
    at                      = 0x0000_0040,  /* U+0040 COMMERCIAL AT */
    A                       = 0x0000_0041,  /* U+0041 LATIN CAPITAL LETTER A */
    B                       = 0x0000_0042,  /* U+0042 LATIN CAPITAL LETTER B */
    C                       = 0x0000_0043,  /* U+0043 LATIN CAPITAL LETTER C */
    D                       = 0x0000_0044,  /* U+0044 LATIN CAPITAL LETTER D */
    E                       = 0x0000_0045,  /* U+0045 LATIN CAPITAL LETTER E */
    F                       = 0x0000_0046,  /* U+0046 LATIN CAPITAL LETTER F */
    G                       = 0x0000_0047,  /* U+0047 LATIN CAPITAL LETTER G */
    H                       = 0x0000_0048,  /* U+0048 LATIN CAPITAL LETTER H */
    I                       = 0x0000_0049,  /* U+0049 LATIN CAPITAL LETTER I */
    J                       = 0x0000_004a,  /* U+004A LATIN CAPITAL LETTER J */
    K                       = 0x0000_004b,  /* U+004B LATIN CAPITAL LETTER K */
    L                       = 0x0000_004c,  /* U+004C LATIN CAPITAL LETTER L */
    M                       = 0x0000_004d,  /* U+004D LATIN CAPITAL LETTER M */
    N                       = 0x0000_004e,  /* U+004E LATIN CAPITAL LETTER N */
    O                       = 0x0000_004f,  /* U+004F LATIN CAPITAL LETTER O */
    P                       = 0x0000_0050,  /* U+0050 LATIN CAPITAL LETTER P */
    Q                       = 0x0000_0051,  /* U+0051 LATIN CAPITAL LETTER Q */
    R                       = 0x0000_0052,  /* U+0052 LATIN CAPITAL LETTER R */
    S                       = 0x0000_0053,  /* U+0053 LATIN CAPITAL LETTER S */
    T                       = 0x0000_0054,  /* U+0054 LATIN CAPITAL LETTER T */
    U                       = 0x0000_0055,  /* U+0055 LATIN CAPITAL LETTER U */
    V                       = 0x0000_0056,  /* U+0056 LATIN CAPITAL LETTER V */
    W                       = 0x0000_0057,  /* U+0057 LATIN CAPITAL LETTER W */
    X                       = 0x0000_0058,  /* U+0058 LATIN CAPITAL LETTER X */
    Y                       = 0x0000_0059,  /* U+0059 LATIN CAPITAL LETTER Y */
    Z                       = 0x0000_005a,  /* U+005A LATIN CAPITAL LETTER Z */
    bracketleft             = 0x0000_005b,  /* U+005B LEFT SQUARE BRACKET */
    backslash               = 0x0000_005c,  /* U+005C REVERSE SOLIDUS */
    bracketright            = 0x0000_005d,  /* U+005D RIGHT SQUARE BRACKET */
    asciicircum             = 0x0000_005e,  /* U+005E CIRCUMFLEX ACCENT */
    underscore              = 0x0000_005f,  /* U+005F LOW LINE */
    grave                   = 0x0000_0060,  /* U+0060 GRAVE ACCENT */
    //a                     = 0x0000_0061,  /* U+0061 LATIN SMALL LETTER A */
    //b                     = 0x0000_0062,  /* U+0062 LATIN SMALL LETTER B */
    //c                     = 0x0000_0063,  /* U+0063 LATIN SMALL LETTER C */
    //d                     = 0x0000_0064,  /* U+0064 LATIN SMALL LETTER D */
    //e                     = 0x0000_0065,  /* U+0065 LATIN SMALL LETTER E */
    //f                     = 0x0000_0066,  /* U+0066 LATIN SMALL LETTER F */
    //g                     = 0x0000_0067,  /* U+0067 LATIN SMALL LETTER G */
    //h                     = 0x0000_0068,  /* U+0068 LATIN SMALL LETTER H */
    //i                     = 0x0000_0069,  /* U+0069 LATIN SMALL LETTER I */
    //j                     = 0x0000_006a,  /* U+006A LATIN SMALL LETTER J */
    //k                     = 0x0000_006b,  /* U+006B LATIN SMALL LETTER K */
    //l                     = 0x0000_006c,  /* U+006C LATIN SMALL LETTER L */
    //m                     = 0x0000_006d,  /* U+006D LATIN SMALL LETTER M */
    //n                     = 0x0000_006e,  /* U+006E LATIN SMALL LETTER N */
    //o                     = 0x0000_006f,  /* U+006F LATIN SMALL LETTER O */
    //p                     = 0x0000_0070,  /* U+0070 LATIN SMALL LETTER P */
    //q                     = 0x0000_0071,  /* U+0071 LATIN SMALL LETTER Q */
    //r                     = 0x0000_0072,  /* U+0072 LATIN SMALL LETTER R */
    //s                     = 0x0000_0073,  /* U+0073 LATIN SMALL LETTER S */
    //t                     = 0x0000_0074,  /* U+0074 LATIN SMALL LETTER T */
    //u                     = 0x0000_0075,  /* U+0075 LATIN SMALL LETTER U */
    //v                     = 0x0000_0076,  /* U+0076 LATIN SMALL LETTER V */
    //w                     = 0x0000_0077,  /* U+0077 LATIN SMALL LETTER W */
    //x                     = 0x0000_0078,  /* U+0078 LATIN SMALL LETTER X */
    //y                     = 0x0000_0079,  /* U+0079 LATIN SMALL LETTER Y */
    //z                     = 0x0000_007a,  /* U+007A LATIN SMALL LETTER Z */
    braceleft               = 0x0000_007b,  /* U+007B LEFT CURLY BRACKET */
    bar                     = 0x0000_007c,  /* U+007C VERTICAL LINE */
    braceright              = 0x0000_007d,  /* U+007D RIGHT CURLY BRACKET */
    asciitilde              = 0x0000_007e,  /* U+007E TILDE */


    //// dead keys (X keycode - 0xED00 to avoid the conflict)
    //Dead_Grave          = 0x0100_1250,
    //Dead_Acute          = 0x0100_1251,
    //Dead_Circumflex     = 0x0100_1252,
    //Dead_Tilde          = 0x0100_1253,
    //Dead_Macron         = 0x0100_1254,
    //Dead_Breve          = 0x0100_1255,
    //Dead_Abovedot       = 0x0100_1256,
    //Dead_Diaeresis      = 0x0100_1257,
    //Dead_Abovering      = 0x0100_1258,
    //Dead_Doubleacute    = 0x0100_1259,
    //Dead_Caron          = 0x0100_125a,
    //Dead_Cedilla        = 0x0100_125b,
    //Dead_Ogonek         = 0x0100_125c,
    //Dead_Iota           = 0x0100_125d,
    //Dead_Voiced_Sound   = 0x0100_125e,
    //Dead_Semivoiced_Sound = 0x0100_125f,
    //Dead_Belowdot       = 0x0100_1260,
    //Dead_Hook           = 0x0100_1261,
    //Dead_Horn           = 0x0100_1262,

    Back                    = SYM_MEDIA_MASK | 1,
    Forward,
    Stop,
    Refresh,
    VolumeDown,
    VolumeMute,
    VolumeUp,
    BassBoost,
    BassUp,
    BassDown,
    TrebleUp,
    TrebleDown,
    MediaPlay,
    MediaStop,
    MediaPrevious,
    MediaNext,
    MediaRecord,
    MediaPause,
    MediaTogglePlayPause,
    HomePage,
    Favorites,
    Search,
    Standby,
    OpenUrl,
    MyComputer,
    LaunchMail,
    LaunchMedia,
    Launch0,
    Launch1,
    Launch2,
    Launch3,
    Launch4,
    Launch5,
    Launch6,
    Launch7,
    Launch8,
    Launch9,
    LaunchA,
    LaunchB,
    LaunchC,
    LaunchD,
    LaunchE,
    LaunchF,
    MonBrightnessUp,
    MonBrightnessDown,
    KeyboardLightOnOff,
    KeyboardBrightnessUp,
    KeyboardBrightnessDown,
    PowerOff,
    WakeUp,
    Eject,
    ScreenSaver,
    WWW,
    Memo,
    LightBulb,
    Shop,
    History,
    AddFavorite,
    HotLinks,
    BrightnessAdjust,
    Finance,
    Community,
    AudioRewind, // Media rewind
    BackForward,
    ApplicationLeft,
    ApplicationRight,
    Book,
    CD,
    Calculator,
    ToDoList,
    ClearGrab,
    Close,
    Copy,
    Cut,
    Display, // Output switch key
    DOS,
    Documents,
    Excel,
    Explorer,
    Game,
    Go,
    iTouch,
    LogOff,
    Market,
    Meeting,
    MenuKB,
    MenuPB,
    MySites,
    News,
    OfficeHome,
    Option,
    Paste,
    Phone,
    Calendar,
    Reply,
    Reload,
    RotateWindows,
    RotationPB,
    RotationKB,
    Save,
    Send,
    Spell,
    SplitScreen,
    Support,
    TaskPane,
    Terminal,
    Tools,
    Travel,
    Video,
    Word,
    Xfer,
    ZoomIn,
    ZoomOut,
    Away,
    Messenger,
    WebCam,
    MailForward,
    Pictures,
    Music,
    Battery,
    Bluetooth,
    WLAN,
    UWB,
    AudioForward, // Media fast-forward
    AudioRepeat, // Toggle repeat mode
    AudioRandomPlay, // Toggle shuffle mode
    Subtitle,
    AudioCycleTrack,
    Time,
    Hibernate,
    View,
    TopMenu,
    PowerDown,
    Suspend,
    ContrastAdjust,

    LaunchG,
    LaunchH,

    TouchpadToggle,
    TouchpadOn,
    TouchpadOff,

    MicMute,

    Red,
    Green,
    Yellow,
    Blue,

    ChannelUp,
    ChannelDown,

    Guide,
    Info,
    Settings,

    MicVolumeUp,
    MicVolumeDown,

    New,
    Open,
    Find,
    Undo,
    Redo,

    MediaLast,

    // Keypad navigation keys
    Select,
    Yes,
    No,

    // Newer misc keys
    Cancel,
    Printer,
    Execute,
    Sleep,
    Play, // Not the same as MediaPlay
    Zoom,
    //Jisho, // IME: Dictionary key
    //Oyayubi_Left, // IME: Left Oyayubi key
    //Oyayubi_Right, // IME: Right Oyayubi key
    Exit,

    // Device keys
    Context1,
    Context2,
    Context3,
    Context4,
    Call,      // set absolute state to in a call (do not toggle state)
    Hangup,    // set absolute state to hang up (do not toggle state)
    Flip,
    ToggleCallHangup, // a toggle key for answering, or hanging up, based on current call state
    VoiceDial,
    LastNumberRedial,

    Camera,
    CameraFocus,


}
