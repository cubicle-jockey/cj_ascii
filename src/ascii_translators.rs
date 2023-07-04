use crate::ascii_consts::*;

/// Returns the character represented by the ASCII ordinal value.
#[inline(always)]
pub const fn ascii_ord_to_char(o: u8) -> char {
    o as char
}

/// Returns the character represented by the ASCII ordinal value.
#[inline(always)]
pub const fn ascii_ord_ref_to_char(o: &u8) -> char {
    ascii_ord_to_char(*o)
}

/// Returns the ASCII ordinal value represented by the character, or `None` if the character is not ASCII.
#[inline(always)]
pub const fn char_to_ascii_ord(c: char) -> Option<u8> {
    match c {
        '\u{0000}' => Some(NULL),
        '\u{0001}' => Some(SOH),
        '\u{0002}' => Some(STX),
        '\u{0003}' => Some(ETX),
        '\u{0004}' => Some(EOT),
        '\u{0005}' => Some(ENQ),
        '\u{0006}' => Some(ACK),
        '\u{0007}' => Some(BEL),
        '\u{0008}' => Some(BS),
        '\u{0009}' => Some(HT),
        '\u{000A}' => Some(LF),
        '\u{000B}' => Some(VT),
        '\u{000C}' => Some(FF),
        '\u{000D}' => Some(CR),
        '\u{000E}' => Some(SO),
        '\u{000F}' => Some(SI),
        '\u{0010}' => Some(DLE),
        '\u{0011}' => Some(DC1),
        '\u{0012}' => Some(DC2),
        '\u{0013}' => Some(DC3),
        '\u{0014}' => Some(DC4),
        '\u{0015}' => Some(NAK),
        '\u{0016}' => Some(SYN),
        '\u{0017}' => Some(ETB),
        '\u{0018}' => Some(CAN),
        '\u{0019}' => Some(EM),
        '\u{001A}' => Some(SUB),
        '\u{001B}' => Some(ESC),
        '\u{001C}' => Some(FS),
        '\u{001D}' => Some(GS),
        '\u{001E}' => Some(RS),
        '\u{001F}' => Some(US),
        '\u{0020}' => Some(SPACE),
        '\u{0021}' => Some(EXCLAMATION_MARK),
        '\u{0022}' => Some(QUOTATION_MARK),
        '\u{0023}' => Some(NUMBER_SIGN),
        '\u{0024}' => Some(DOLLAR_SIGN),
        '\u{0025}' => Some(PERCENT_SIGN),
        '\u{0026}' => Some(AMPERSAND),
        '\u{0027}' => Some(APOSTROPHE),
        '\u{0028}' => Some(LEFT_PARENTHESIS),
        '\u{0029}' => Some(RIGHT_PARENTHESIS),
        '\u{002A}' => Some(ASTERISK),
        '\u{002B}' => Some(PLUS_SIGN),
        '\u{002C}' => Some(COMMA),
        '\u{002D}' => Some(HYPHEN_MINUS),
        '\u{002E}' => Some(FULL_STOP),
        '\u{002F}' => Some(SOLIDUS),
        '\u{0030}' => Some(DIGIT_ZERO),
        '\u{0031}' => Some(DIGIT_ONE),
        '\u{0032}' => Some(DIGIT_TWO),
        '\u{0033}' => Some(DIGIT_THREE),
        '\u{0034}' => Some(DIGIT_FOUR),
        '\u{0035}' => Some(DIGIT_FIVE),
        '\u{0036}' => Some(DIGIT_SIX),
        '\u{0037}' => Some(DIGIT_SEVEN),
        '\u{0038}' => Some(DIGIT_EIGHT),
        '\u{0039}' => Some(DIGIT_NINE),
        '\u{003A}' => Some(COLON),
        '\u{003B}' => Some(SEMICOLON),
        '\u{003C}' => Some(LESS_THAN_SIGN),
        '\u{003D}' => Some(EQUALS_SIGN),
        '\u{003E}' => Some(GREATER_THAN_SIGN),
        '\u{003F}' => Some(QUESTION_MARK),
        '\u{0040}' => Some(COMMERCIAL_AT),
        '\u{0041}' => Some(LATIN_CAPITAL_LETTER_A),
        '\u{0042}' => Some(LATIN_CAPITAL_LETTER_B),
        '\u{0043}' => Some(LATIN_CAPITAL_LETTER_C),
        '\u{0044}' => Some(LATIN_CAPITAL_LETTER_D),
        '\u{0045}' => Some(LATIN_CAPITAL_LETTER_E),
        '\u{0046}' => Some(LATIN_CAPITAL_LETTER_F),
        '\u{0047}' => Some(LATIN_CAPITAL_LETTER_G),
        '\u{0048}' => Some(LATIN_CAPITAL_LETTER_H),
        '\u{0049}' => Some(LATIN_CAPITAL_LETTER_I),
        '\u{004A}' => Some(LATIN_CAPITAL_LETTER_J),
        '\u{004B}' => Some(LATIN_CAPITAL_LETTER_K),
        '\u{004C}' => Some(LATIN_CAPITAL_LETTER_L),
        '\u{004D}' => Some(LATIN_CAPITAL_LETTER_M),
        '\u{004E}' => Some(LATIN_CAPITAL_LETTER_N),
        '\u{004F}' => Some(LATIN_CAPITAL_LETTER_O),
        '\u{0050}' => Some(LATIN_CAPITAL_LETTER_P),
        '\u{0051}' => Some(LATIN_CAPITAL_LETTER_Q),
        '\u{0052}' => Some(LATIN_CAPITAL_LETTER_R),
        '\u{0053}' => Some(LATIN_CAPITAL_LETTER_S),
        '\u{0054}' => Some(LATIN_CAPITAL_LETTER_T),
        '\u{0055}' => Some(LATIN_CAPITAL_LETTER_U),
        '\u{0056}' => Some(LATIN_CAPITAL_LETTER_V),
        '\u{0057}' => Some(LATIN_CAPITAL_LETTER_W),
        '\u{0058}' => Some(LATIN_CAPITAL_LETTER_X),
        '\u{0059}' => Some(LATIN_CAPITAL_LETTER_Y),
        '\u{005A}' => Some(LATIN_CAPITAL_LETTER_Z),
        '\u{005B}' => Some(LEFT_SQUARE_BRACKET),
        '\u{005C}' => Some(REVERSE_SOLIDUS),
        '\u{005D}' => Some(RIGHT_SQUARE_BRACKET),
        '\u{005E}' => Some(CIRCUMFLEX_ACCENT),
        '\u{005F}' => Some(LOW_LINE),
        '\u{0060}' => Some(GRAVE_ACCENT),
        '\u{0061}' => Some(LATIN_SMALL_LETTER_A),
        '\u{0062}' => Some(LATIN_SMALL_LETTER_B),
        '\u{0063}' => Some(LATIN_SMALL_LETTER_C),
        '\u{0064}' => Some(LATIN_SMALL_LETTER_D),
        '\u{0065}' => Some(LATIN_SMALL_LETTER_E),
        '\u{0066}' => Some(LATIN_SMALL_LETTER_F),
        '\u{0067}' => Some(LATIN_SMALL_LETTER_G),
        '\u{0068}' => Some(LATIN_SMALL_LETTER_H),
        '\u{0069}' => Some(LATIN_SMALL_LETTER_I),
        '\u{006A}' => Some(LATIN_SMALL_LETTER_J),
        '\u{006B}' => Some(LATIN_SMALL_LETTER_K),
        '\u{006C}' => Some(LATIN_SMALL_LETTER_L),
        '\u{006D}' => Some(LATIN_SMALL_LETTER_M),
        '\u{006E}' => Some(LATIN_SMALL_LETTER_N),
        '\u{006F}' => Some(LATIN_SMALL_LETTER_O),
        '\u{0070}' => Some(LATIN_SMALL_LETTER_P),
        '\u{0071}' => Some(LATIN_SMALL_LETTER_Q),
        '\u{0072}' => Some(LATIN_SMALL_LETTER_R),
        '\u{0073}' => Some(LATIN_SMALL_LETTER_S),
        '\u{0074}' => Some(LATIN_SMALL_LETTER_T),
        '\u{0075}' => Some(LATIN_SMALL_LETTER_U),
        '\u{0076}' => Some(LATIN_SMALL_LETTER_V),
        '\u{0077}' => Some(LATIN_SMALL_LETTER_W),
        '\u{0078}' => Some(LATIN_SMALL_LETTER_X),
        '\u{0079}' => Some(LATIN_SMALL_LETTER_Y),
        '\u{007A}' => Some(LATIN_SMALL_LETTER_Z),
        '\u{007B}' => Some(LEFT_CURLY_BRACKET),
        '\u{007C}' => Some(VERTICAL_LINE),
        '\u{007D}' => Some(RIGHT_CURLY_BRACKET),
        '\u{007E}' => Some(TILDE),
        '\u{007F}' => Some(DEL),
        '\u{0080}' => Some(EXT_128),
        '\u{0081}' => Some(EXT_129),
        '\u{0082}' => Some(EXT_130),
        '\u{0083}' => Some(EXT_131),
        '\u{0084}' => Some(EXT_132),
        '\u{0085}' => Some(EXT_133),
        '\u{0086}' => Some(EXT_134),
        '\u{0087}' => Some(EXT_135),
        '\u{0088}' => Some(EXT_136),
        '\u{0089}' => Some(EXT_137),
        '\u{008A}' => Some(EXT_138),
        '\u{008B}' => Some(EXT_139),
        '\u{008C}' => Some(EXT_140),
        '\u{008D}' => Some(EXT_141),
        '\u{008E}' => Some(EXT_142),
        '\u{008F}' => Some(EXT_143),
        '\u{0090}' => Some(EXT_144),
        '\u{0091}' => Some(EXT_145),
        '\u{0092}' => Some(EXT_146),
        '\u{0093}' => Some(EXT_147),
        '\u{0094}' => Some(EXT_148),
        '\u{0095}' => Some(EXT_149),
        '\u{0096}' => Some(EXT_150),
        '\u{0097}' => Some(EXT_151),
        '\u{0098}' => Some(EXT_152),
        '\u{0099}' => Some(EXT_153),
        '\u{009A}' => Some(EXT_154),
        '\u{009B}' => Some(EXT_155),
        '\u{009C}' => Some(EXT_156),
        '\u{009D}' => Some(EXT_157),
        '\u{009E}' => Some(EXT_158),
        '\u{009F}' => Some(EXT_159),
        '\u{00A0}' => Some(NO_BREAK_SPACE),
        '\u{00A1}' => Some(INVERTED_EXCLAMATION_MARK),
        '\u{00A2}' => Some(CENT_SIGN),
        '\u{00A3}' => Some(POUND_SIGN),
        '\u{00A4}' => Some(CURRENCY_SIGN),
        '\u{00A5}' => Some(YEN_SIGN),
        '\u{00A6}' => Some(BROKEN_BAR),
        '\u{00A7}' => Some(SECTION_SIGN),
        '\u{00A8}' => Some(DIAERESIS),
        '\u{00A9}' => Some(COPYRIGHT_SIGN),
        '\u{00AA}' => Some(FEMININE_ORDINAL_INDICATOR),
        '\u{00AB}' => Some(LEFT_POINTING_DOUBLE_ANGLE_QUOTATION_MARK),
        '\u{00AC}' => Some(NOT_SIGN),
        '\u{00AD}' => Some(SHY),
        '\u{00AE}' => Some(REGISTERED_SIGN),
        '\u{00AF}' => Some(MACRON),
        '\u{00B0}' => Some(DEGREE_SIGN),
        '\u{00B1}' => Some(PLUS_MINUS_SIGN),
        '\u{00B2}' => Some(SUPERSCRIPT_TWO),
        '\u{00B3}' => Some(SUPERSCRIPT_THREE),
        '\u{00B4}' => Some(ACUTE_ACCENT),
        '\u{00B5}' => Some(MICRO_SIGN),
        '\u{00B6}' => Some(PILCROW_SIGN),
        '\u{00B7}' => Some(MIDDLE_DOT),
        '\u{00B8}' => Some(CEDILLA),
        '\u{00B9}' => Some(SUPERSCRIPT_ONE),
        '\u{00BA}' => Some(MASCULINE_ORDINAL_INDICATOR),
        '\u{00BB}' => Some(RIGHT_POINTING_DOUBLE_ANGLE_QUOTATION_MARK),
        '\u{00BC}' => Some(VULGAR_FRACTION_ONE_QUARTER),
        '\u{00BD}' => Some(VULGAR_FRACTION_ONE_HALF),
        '\u{00BE}' => Some(VULGAR_FRACTION_THREE_QUARTERS),
        '\u{00BF}' => Some(INVERTED_QUESTION_MARK),
        '\u{00C0}' => Some(LATIN_CAPITAL_LETTER_A_WITH_GRAVE),
        '\u{00C1}' => Some(LATIN_CAPITAL_LETTER_A_WITH_ACUTE),
        '\u{00C2}' => Some(LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX),
        '\u{00C3}' => Some(LATIN_CAPITAL_LETTER_A_WITH_TILDE),
        '\u{00C4}' => Some(LATIN_CAPITAL_LETTER_A_WITH_DIAERESIS),
        '\u{00C5}' => Some(LATIN_CAPITAL_LETTER_A_WITH_RING_ABOVE),
        '\u{00C6}' => Some(LATIN_CAPITAL_LETTER_AE),
        '\u{00C7}' => Some(LATIN_CAPITAL_LETTER_C_WITH_CEDILLA),
        '\u{00C8}' => Some(LATIN_CAPITAL_LETTER_E_WITH_GRAVE),
        '\u{00C9}' => Some(LATIN_CAPITAL_LETTER_E_WITH_ACUTE),
        '\u{00CA}' => Some(LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX),
        '\u{00CB}' => Some(LATIN_CAPITAL_LETTER_E_WITH_DIAERESIS),
        '\u{00CC}' => Some(LATIN_CAPITAL_LETTER_I_WITH_GRAVE),
        '\u{00CD}' => Some(LATIN_CAPITAL_LETTER_I_WITH_ACUTE),
        '\u{00CE}' => Some(LATIN_CAPITAL_LETTER_I_WITH_CIRCUMFLEX),
        '\u{00CF}' => Some(LATIN_CAPITAL_LETTER_I_WITH_DIAERESIS),
        '\u{00D0}' => Some(LATIN_CAPITAL_LETTER_ETH),
        '\u{00D1}' => Some(LATIN_CAPITAL_LETTER_N_WITH_TILDE),
        '\u{00D2}' => Some(LATIN_CAPITAL_LETTER_O_WITH_GRAVE),
        '\u{00D3}' => Some(LATIN_CAPITAL_LETTER_O_WITH_ACUTE),
        '\u{00D4}' => Some(LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX),
        '\u{00D5}' => Some(LATIN_CAPITAL_LETTER_O_WITH_TILDE),
        '\u{00D6}' => Some(LATIN_CAPITAL_LETTER_O_WITH_DIAERESIS),
        '\u{00D7}' => Some(MULTIPLICATION_SIGN),
        '\u{00D8}' => Some(LATIN_CAPITAL_LETTER_O_WITH_STROKE),
        '\u{00D9}' => Some(LATIN_CAPITAL_LETTER_U_WITH_GRAVE),
        '\u{00DA}' => Some(LATIN_CAPITAL_LETTER_U_WITH_ACUTE),
        '\u{00DB}' => Some(LATIN_CAPITAL_LETTER_U_WITH_CIRCUMFLEX),
        '\u{00DC}' => Some(LATIN_CAPITAL_LETTER_U_WITH_DIAERESIS),
        '\u{00DD}' => Some(LATIN_CAPITAL_LETTER_Y_WITH_ACUTE),
        '\u{00DE}' => Some(LATIN_CAPITAL_LETTER_THORN),
        '\u{00DF}' => Some(LATIN_SMALL_LETTER_SHARP_S),
        '\u{00E0}' => Some(LATIN_SMALL_LETTER_A_WITH_GRAVE),
        '\u{00E1}' => Some(LATIN_SMALL_LETTER_A_WITH_ACUTE),
        '\u{00E2}' => Some(LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX),
        '\u{00E3}' => Some(LATIN_SMALL_LETTER_A_WITH_TILDE),
        '\u{00E4}' => Some(LATIN_SMALL_LETTER_A_WITH_DIAERESIS),
        '\u{00E5}' => Some(LATIN_SMALL_LETTER_A_WITH_RING_ABOVE),
        '\u{00E6}' => Some(LATIN_SMALL_LETTER_AE),
        '\u{00E7}' => Some(LATIN_SMALL_LETTER_C_WITH_CEDILLA),
        '\u{00E8}' => Some(LATIN_SMALL_LETTER_E_WITH_GRAVE),
        '\u{00E9}' => Some(LATIN_SMALL_LETTER_E_WITH_ACUTE),
        '\u{00EA}' => Some(LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX),
        '\u{00EB}' => Some(LATIN_SMALL_LETTER_E_WITH_DIAERESIS),
        '\u{00EC}' => Some(LATIN_SMALL_LETTER_I_WITH_GRAVE),
        '\u{00ED}' => Some(LATIN_SMALL_LETTER_I_WITH_ACUTE),
        '\u{00EE}' => Some(LATIN_SMALL_LETTER_I_WITH_CIRCUMFLEX),
        '\u{00EF}' => Some(LATIN_SMALL_LETTER_I_WITH_DIAERESIS),
        '\u{00F0}' => Some(LATIN_SMALL_LETTER_ETH),
        '\u{00F1}' => Some(LATIN_SMALL_LETTER_N_WITH_TILDE),
        '\u{00F2}' => Some(LATIN_SMALL_LETTER_O_WITH_GRAVE),
        '\u{00F3}' => Some(LATIN_SMALL_LETTER_O_WITH_ACUTE),
        '\u{00F4}' => Some(LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX),
        '\u{00F5}' => Some(LATIN_SMALL_LETTER_O_WITH_TILDE),
        '\u{00F6}' => Some(LATIN_SMALL_LETTER_O_WITH_DIAERESIS),
        '\u{00F7}' => Some(DIVISION_SIGN),
        '\u{00F8}' => Some(LATIN_SMALL_LETTER_O_WITH_STROKE),
        '\u{00F9}' => Some(LATIN_SMALL_LETTER_U_WITH_GRAVE),
        '\u{00FA}' => Some(LATIN_SMALL_LETTER_U_WITH_ACUTE),
        '\u{00FB}' => Some(LATIN_SMALL_LETTER_U_WITH_CIRCUMFLEX),
        '\u{00FC}' => Some(LATIN_SMALL_LETTER_U_WITH_DIAERESIS),
        '\u{00FD}' => Some(LATIN_SMALL_LETTER_Y_WITH_ACUTE),
        '\u{00FE}' => Some(LATIN_SMALL_LETTER_THORN),
        '\u{00FF}' => Some(LATIN_SMALL_LETTER_Y_WITH_DIAERESIS),
        _ => None,
    }
}

/// Returns the ASCII ordinal value represented by the character, or `None` if the character is not ASCII.
#[inline(always)]
pub const fn char_ref_to_ascii_ord(c: &char) -> Option<u8> {
    char_to_ascii_ord(*c)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ascii_ord_to_char() {
        assert_eq!(ascii_ord_to_char(NULL), '\u{0000}');
        assert_eq!(ascii_ord_to_char(SOH), '\u{0001}');
        assert_eq!(ascii_ord_to_char(STX), '\u{0002}');
        assert_eq!(ascii_ord_to_char(ETX), '\u{0003}');
        assert_eq!(ascii_ord_to_char(EOT), '\u{0004}');
        assert_eq!(ascii_ord_to_char(ENQ), '\u{0005}');
        assert_eq!(ascii_ord_to_char(ACK), '\u{0006}');
        assert_eq!(ascii_ord_to_char(BEL), '\u{0007}');
        assert_eq!(ascii_ord_to_char(BS), '\u{0008}');
        assert_eq!(ascii_ord_to_char(HT), '\u{0009}');
        assert_eq!(ascii_ord_to_char(LF), '\u{000A}');
        assert_eq!(ascii_ord_to_char(VT), '\u{000B}');
        assert_eq!(ascii_ord_to_char(FF), '\u{000C}');
        assert_eq!(ascii_ord_to_char(CR), '\u{000D}');
        assert_eq!(ascii_ord_to_char(SO), '\u{000E}');
        assert_eq!(ascii_ord_to_char(SI), '\u{000F}');
        assert_eq!(ascii_ord_to_char(DLE), '\u{0010}');
        assert_eq!(ascii_ord_to_char(DC1), '\u{0011}');
        assert_eq!(ascii_ord_to_char(DC2), '\u{0012}');
        assert_eq!(ascii_ord_to_char(DC3), '\u{0013}');
        assert_eq!(ascii_ord_to_char(DC4), '\u{0014}');
        assert_eq!(ascii_ord_to_char(NAK), '\u{0015}');
        assert_eq!(ascii_ord_to_char(SYN), '\u{0016}');
        assert_eq!(ascii_ord_to_char(ETB), '\u{0017}');
        assert_eq!(ascii_ord_to_char(CAN), '\u{0018}');
        assert_eq!(ascii_ord_to_char(EM), '\u{0019}');
        assert_eq!(ascii_ord_to_char(SUB), '\u{001A}');
        assert_eq!(ascii_ord_to_char(ESC), '\u{001B}');
        assert_eq!(ascii_ord_to_char(FS), '\u{001C}');
        assert_eq!(ascii_ord_to_char(GS), '\u{001D}');
        assert_eq!(ascii_ord_to_char(RS), '\u{001E}');
        assert_eq!(ascii_ord_to_char(US), '\u{001F}');
        assert_eq!(ascii_ord_to_char(SPACE), '\u{0020}');
        assert_eq!(ascii_ord_to_char(EXCLAMATION_MARK), '\u{0021}');
        assert_eq!(ascii_ord_to_char(QUOTATION_MARK), '\u{0022}');
        assert_eq!(ascii_ord_to_char(NUMBER_SIGN), '\u{0023}');
        assert_eq!(ascii_ord_to_char(DOLLAR_SIGN), '\u{0024}');
        assert_eq!(ascii_ord_to_char(PERCENT_SIGN), '\u{0025}');
        assert_eq!(ascii_ord_to_char(AMPERSAND), '\u{0026}');
        assert_eq!(ascii_ord_to_char(APOSTROPHE), '\u{0027}');
        assert_eq!(ascii_ord_to_char(LEFT_PARENTHESIS), '\u{0028}');
        assert_eq!(ascii_ord_to_char(RIGHT_PARENTHESIS), '\u{0029}');
        assert_eq!(ascii_ord_to_char(ASTERISK), '\u{002A}');
        assert_eq!(ascii_ord_to_char(PLUS_SIGN), '\u{002B}');
        assert_eq!(ascii_ord_to_char(COMMA), '\u{002C}');
        assert_eq!(ascii_ord_to_char(HYPHEN_MINUS), '\u{002D}');
        assert_eq!(ascii_ord_to_char(FULL_STOP), '\u{002E}');
        assert_eq!(ascii_ord_to_char(SOLIDUS), '\u{002F}');
        assert_eq!(ascii_ord_to_char(DIGIT_ZERO), '\u{0030}');
        assert_eq!(ascii_ord_to_char(DIGIT_ONE), '\u{0031}');
        assert_eq!(ascii_ord_to_char(DIGIT_TWO), '\u{0032}');
        assert_eq!(ascii_ord_to_char(DIGIT_THREE), '\u{0033}');
        assert_eq!(ascii_ord_to_char(DIGIT_FOUR), '\u{0034}');
        assert_eq!(ascii_ord_to_char(DIGIT_FIVE), '\u{0035}');
        assert_eq!(ascii_ord_to_char(DIGIT_SIX), '\u{0036}');
        assert_eq!(ascii_ord_to_char(DIGIT_SEVEN), '\u{0037}');
        assert_eq!(ascii_ord_to_char(DIGIT_EIGHT), '\u{0038}');
        assert_eq!(ascii_ord_to_char(DIGIT_NINE), '\u{0039}');
        assert_eq!(ascii_ord_to_char(COLON), '\u{003A}');
        assert_eq!(ascii_ord_to_char(SEMICOLON), '\u{003B}');
        assert_eq!(ascii_ord_to_char(LESS_THAN_SIGN), '\u{003C}');
        assert_eq!(ascii_ord_to_char(EQUALS_SIGN), '\u{003D}');
        assert_eq!(ascii_ord_to_char(GREATER_THAN_SIGN), '\u{003E}');
        assert_eq!(ascii_ord_to_char(QUESTION_MARK), '\u{003F}');
        assert_eq!(ascii_ord_to_char(COMMERCIAL_AT), '\u{0040}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_A), '\u{0041}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_B), '\u{0042}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_C), '\u{0043}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_D), '\u{0044}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_E), '\u{0045}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_F), '\u{0046}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_G), '\u{0047}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_H), '\u{0048}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_I), '\u{0049}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_J), '\u{004A}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_K), '\u{004B}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_L), '\u{004C}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_M), '\u{004D}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_N), '\u{004E}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_O), '\u{004F}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_P), '\u{0050}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_Q), '\u{0051}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_R), '\u{0052}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_S), '\u{0053}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_T), '\u{0054}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_U), '\u{0055}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_V), '\u{0056}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_W), '\u{0057}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_X), '\u{0058}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_Y), '\u{0059}');
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_Z), '\u{005A}');
        assert_eq!(ascii_ord_to_char(LEFT_SQUARE_BRACKET), '\u{005B}');
        assert_eq!(ascii_ord_to_char(REVERSE_SOLIDUS), '\u{005C}');
        assert_eq!(ascii_ord_to_char(RIGHT_SQUARE_BRACKET), '\u{005D}');
        assert_eq!(ascii_ord_to_char(CIRCUMFLEX_ACCENT), '\u{005E}');
        assert_eq!(ascii_ord_to_char(LOW_LINE), '\u{005F}');
        assert_eq!(ascii_ord_to_char(GRAVE_ACCENT), '\u{0060}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_A), '\u{0061}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_B), '\u{0062}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_C), '\u{0063}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_D), '\u{0064}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_E), '\u{0065}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_F), '\u{0066}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_G), '\u{0067}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_H), '\u{0068}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_I), '\u{0069}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_J), '\u{006A}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_K), '\u{006B}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_L), '\u{006C}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_M), '\u{006D}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_N), '\u{006E}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_O), '\u{006F}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_P), '\u{0070}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_Q), '\u{0071}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_R), '\u{0072}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_S), '\u{0073}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_T), '\u{0074}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_U), '\u{0075}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_V), '\u{0076}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_W), '\u{0077}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_X), '\u{0078}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_Y), '\u{0079}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_Z), '\u{007A}');
        assert_eq!(ascii_ord_to_char(LEFT_CURLY_BRACKET), '\u{007B}');
        assert_eq!(ascii_ord_to_char(VERTICAL_LINE), '\u{007C}');
        assert_eq!(ascii_ord_to_char(RIGHT_CURLY_BRACKET), '\u{007D}');
        assert_eq!(ascii_ord_to_char(TILDE), '\u{007E}');
        assert_eq!(ascii_ord_to_char(DEL), '\u{007F}');
        assert_eq!(ascii_ord_to_char(EXT_128), '\u{0080}');
        assert_eq!(ascii_ord_to_char(EXT_129), '\u{0081}');
        assert_eq!(ascii_ord_to_char(EXT_130), '\u{0082}');
        assert_eq!(ascii_ord_to_char(EXT_131), '\u{0083}');
        assert_eq!(ascii_ord_to_char(EXT_132), '\u{0084}');
        assert_eq!(ascii_ord_to_char(EXT_133), '\u{0085}');
        assert_eq!(ascii_ord_to_char(EXT_134), '\u{0086}');
        assert_eq!(ascii_ord_to_char(EXT_135), '\u{0087}');
        assert_eq!(ascii_ord_to_char(EXT_136), '\u{0088}');
        assert_eq!(ascii_ord_to_char(EXT_137), '\u{0089}');
        assert_eq!(ascii_ord_to_char(EXT_138), '\u{008A}');
        assert_eq!(ascii_ord_to_char(EXT_139), '\u{008B}');
        assert_eq!(ascii_ord_to_char(EXT_140), '\u{008C}');
        assert_eq!(ascii_ord_to_char(EXT_141), '\u{008D}');
        assert_eq!(ascii_ord_to_char(EXT_142), '\u{008E}');
        assert_eq!(ascii_ord_to_char(EXT_143), '\u{008F}');
        assert_eq!(ascii_ord_to_char(EXT_144), '\u{0090}');
        assert_eq!(ascii_ord_to_char(EXT_145), '\u{0091}');
        assert_eq!(ascii_ord_to_char(EXT_146), '\u{0092}');
        assert_eq!(ascii_ord_to_char(EXT_147), '\u{0093}');
        assert_eq!(ascii_ord_to_char(EXT_148), '\u{0094}');
        assert_eq!(ascii_ord_to_char(EXT_149), '\u{0095}');
        assert_eq!(ascii_ord_to_char(EXT_150), '\u{0096}');
        assert_eq!(ascii_ord_to_char(EXT_151), '\u{0097}');
        assert_eq!(ascii_ord_to_char(EXT_152), '\u{0098}');
        assert_eq!(ascii_ord_to_char(EXT_153), '\u{0099}');
        assert_eq!(ascii_ord_to_char(EXT_154), '\u{009A}');
        assert_eq!(ascii_ord_to_char(EXT_155), '\u{009B}');
        assert_eq!(ascii_ord_to_char(EXT_156), '\u{009C}');
        assert_eq!(ascii_ord_to_char(EXT_157), '\u{009D}');
        assert_eq!(ascii_ord_to_char(EXT_158), '\u{009E}');
        assert_eq!(ascii_ord_to_char(EXT_159), '\u{009F}');
        assert_eq!(ascii_ord_to_char(NO_BREAK_SPACE), '\u{00A0}');
        assert_eq!(ascii_ord_to_char(INVERTED_EXCLAMATION_MARK), '\u{00A1}');
        assert_eq!(ascii_ord_to_char(CENT_SIGN), '\u{00A2}');
        assert_eq!(ascii_ord_to_char(POUND_SIGN), '\u{00A3}');
        assert_eq!(ascii_ord_to_char(CURRENCY_SIGN), '\u{00A4}');
        assert_eq!(ascii_ord_to_char(YEN_SIGN), '\u{00A5}');
        assert_eq!(ascii_ord_to_char(BROKEN_BAR), '\u{00A6}');
        assert_eq!(ascii_ord_to_char(SECTION_SIGN), '\u{00A7}');
        assert_eq!(ascii_ord_to_char(DIAERESIS), '\u{00A8}');
        assert_eq!(ascii_ord_to_char(COPYRIGHT_SIGN), '\u{00A9}');
        assert_eq!(ascii_ord_to_char(FEMININE_ORDINAL_INDICATOR), '\u{00AA}');
        assert_eq!(
            ascii_ord_to_char(LEFT_POINTING_DOUBLE_ANGLE_QUOTATION_MARK),
            '\u{00AB}'
        );
        assert_eq!(ascii_ord_to_char(NOT_SIGN), '\u{00AC}');
        assert_eq!(ascii_ord_to_char(SHY), '\u{00AD}');
        assert_eq!(ascii_ord_to_char(REGISTERED_SIGN), '\u{00AE}');
        assert_eq!(ascii_ord_to_char(MACRON), '\u{00AF}');
        assert_eq!(ascii_ord_to_char(DEGREE_SIGN), '\u{00B0}');
        assert_eq!(ascii_ord_to_char(PLUS_MINUS_SIGN), '\u{00B1}');
        assert_eq!(ascii_ord_to_char(SUPERSCRIPT_TWO), '\u{00B2}');
        assert_eq!(ascii_ord_to_char(SUPERSCRIPT_THREE), '\u{00B3}');
        assert_eq!(ascii_ord_to_char(ACUTE_ACCENT), '\u{00B4}');
        assert_eq!(ascii_ord_to_char(MICRO_SIGN), '\u{00B5}');
        assert_eq!(ascii_ord_to_char(PILCROW_SIGN), '\u{00B6}');
        assert_eq!(ascii_ord_to_char(MIDDLE_DOT), '\u{00B7}');
        assert_eq!(ascii_ord_to_char(CEDILLA), '\u{00B8}');
        assert_eq!(ascii_ord_to_char(SUPERSCRIPT_ONE), '\u{00B9}');
        assert_eq!(ascii_ord_to_char(MASCULINE_ORDINAL_INDICATOR), '\u{00BA}');
        assert_eq!(
            ascii_ord_to_char(RIGHT_POINTING_DOUBLE_ANGLE_QUOTATION_MARK),
            '\u{00BB}'
        );
        assert_eq!(ascii_ord_to_char(VULGAR_FRACTION_ONE_QUARTER), '\u{00BC}');
        assert_eq!(ascii_ord_to_char(VULGAR_FRACTION_ONE_HALF), '\u{00BD}');
        assert_eq!(
            ascii_ord_to_char(VULGAR_FRACTION_THREE_QUARTERS),
            '\u{00BE}'
        );
        assert_eq!(ascii_ord_to_char(INVERTED_QUESTION_MARK), '\u{00BF}');
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_A_WITH_GRAVE),
            '\u{00C0}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_A_WITH_ACUTE),
            '\u{00C1}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX),
            '\u{00C2}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_A_WITH_TILDE),
            '\u{00C3}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_A_WITH_DIAERESIS),
            '\u{00C4}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_A_WITH_RING_ABOVE),
            '\u{00C5}'
        );
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_AE), '\u{00C6}');
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_C_WITH_CEDILLA),
            '\u{00C7}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_E_WITH_GRAVE),
            '\u{00C8}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_E_WITH_ACUTE),
            '\u{00C9}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX),
            '\u{00CA}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_E_WITH_DIAERESIS),
            '\u{00CB}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_I_WITH_GRAVE),
            '\u{00CC}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_I_WITH_ACUTE),
            '\u{00CD}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_I_WITH_CIRCUMFLEX),
            '\u{00CE}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_I_WITH_DIAERESIS),
            '\u{00CF}'
        );
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_ETH), '\u{00D0}');
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_N_WITH_TILDE),
            '\u{00D1}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_O_WITH_GRAVE),
            '\u{00D2}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_O_WITH_ACUTE),
            '\u{00D3}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX),
            '\u{00D4}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_O_WITH_TILDE),
            '\u{00D5}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_O_WITH_DIAERESIS),
            '\u{00D6}'
        );
        assert_eq!(ascii_ord_to_char(MULTIPLICATION_SIGN), '\u{00D7}');
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_O_WITH_STROKE),
            '\u{00D8}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_U_WITH_GRAVE),
            '\u{00D9}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_U_WITH_ACUTE),
            '\u{00DA}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_U_WITH_CIRCUMFLEX),
            '\u{00DB}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_U_WITH_DIAERESIS),
            '\u{00DC}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_CAPITAL_LETTER_Y_WITH_ACUTE),
            '\u{00DD}'
        );
        assert_eq!(ascii_ord_to_char(LATIN_CAPITAL_LETTER_THORN), '\u{00DE}');
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_SHARP_S), '\u{00DF}');
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_A_WITH_GRAVE),
            '\u{00E0}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_A_WITH_ACUTE),
            '\u{00E1}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX),
            '\u{00E2}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_A_WITH_TILDE),
            '\u{00E3}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_A_WITH_DIAERESIS),
            '\u{00E4}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_A_WITH_RING_ABOVE),
            '\u{00E5}'
        );
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_AE), '\u{00E6}');
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_C_WITH_CEDILLA),
            '\u{00E7}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_E_WITH_GRAVE),
            '\u{00E8}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_E_WITH_ACUTE),
            '\u{00E9}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX),
            '\u{00EA}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_E_WITH_DIAERESIS),
            '\u{00EB}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_I_WITH_GRAVE),
            '\u{00EC}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_I_WITH_ACUTE),
            '\u{00ED}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_I_WITH_CIRCUMFLEX),
            '\u{00EE}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_I_WITH_DIAERESIS),
            '\u{00EF}'
        );
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_ETH), '\u{00F0}');
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_N_WITH_TILDE),
            '\u{00F1}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_O_WITH_GRAVE),
            '\u{00F2}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_O_WITH_ACUTE),
            '\u{00F3}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX),
            '\u{00F4}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_O_WITH_TILDE),
            '\u{00F5}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_O_WITH_DIAERESIS),
            '\u{00F6}'
        );
        assert_eq!(ascii_ord_to_char(DIVISION_SIGN), '\u{00F7}');
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_O_WITH_STROKE),
            '\u{00F8}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_U_WITH_GRAVE),
            '\u{00F9}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_U_WITH_ACUTE),
            '\u{00FA}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_U_WITH_CIRCUMFLEX),
            '\u{00FB}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_U_WITH_DIAERESIS),
            '\u{00FC}'
        );
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_Y_WITH_ACUTE),
            '\u{00FD}'
        );
        assert_eq!(ascii_ord_to_char(LATIN_SMALL_LETTER_THORN), '\u{00FE}');
        assert_eq!(
            ascii_ord_to_char(LATIN_SMALL_LETTER_Y_WITH_DIAERESIS),
            '\u{00FF}'
        );
    }
}
