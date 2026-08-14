//! An extremely minimal implementation of Codepage 437 encoding.
//!
//! # Features
//! - `default` : `std`
//! - `std` : Link to rust's std library, without this `#[no_std]` is enabled.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std::fmt;

/// The table of characters in Codepage 437.
#[rustfmt::skip]
pub const CP437CHARS: [char; 256] = [
    '\0', '☺',  '☻',  '♥',  '♦',  '♣',  '♠',  '•',  '◘',  '○',  '◙',  '♂',  '♀',  '♪',  '♫',  '☼',
    '►',  '◄',  '↕',  '‼',  '¶',  '§',  '▬',  '↨',  '↑',  '↓',  '→',  '←',  '∟',  '↔',  '▲',  '▼', 
    ' ',  '!',  '"',  '#',  '$',  '%',  '&',  '\'', '(',  ')',  '*',  '+',  ',',  '-',  '.',  '/',
    '0',  '1',  '2',  '3',  '4',  '5',  '6',  '7',  '8',  '9',  ':',  ';',  '<',  '=',  '>',  '?',
    '@',  'A',  'B',  'C',  'D',  'E',  'F',  'G',  'H',  'I',  'J',  'K',  'L',  'M',  'N',  'O',
    'P',  'Q',  'R',  'S',  'T',  'U',  'V',  'W',  'X',  'Y',  'Z',  '[',  '\\', ']',  '^',  '_', 
    '`',  'a',  'b',  'c',  'd',  'e',  'f',  'g',  'h',  'i',  'j',  'k',  'l',  'm',  'n',  'o', 
    'p',  'q',  'r',  's',  't',  'u',  'v',  'w',  'x',  'y',  'z',  '{',  '|',  '}',  '~',  '⌂', 
    'Ç',  'ü',  'é',  'â',  'ä',  'à',  'å',  'ç',  'ê',  'ë',  'è',  'ï',  'î',  'ì',  'Ä',  'Å', 
    'É',  'æ',  'Æ',  'ô',  'ö',  'ò',  'û',  'ù',  'ÿ',  'Ö',  'Ü',  '¢',  '£',  '¥',  '₧',  'ƒ', 
    'á',  'í',  'ó',  'ú',  'ñ',  'Ñ',  'ª',  'º',  '¿',  '⌐',  '¬',  '½',  '¼',  '¡',  '«',  '»', 
    '░',  '▒',  '▓',  '│',  '┤',  '╡',  '╢',  '╖',  '╕',  '╣',  '║',  '╗',  '╝',  '╜',  '╛',  '┐', 
    '└',  '┴',  '┬',  '├',  '─',  '┼',  '╞',  '╟',  '╚',  '╔',  '╩',  '╦',  '╠',  '═',  '╬',  '╧', 
    '╨',  '╤',  '╥',  '╙',  '╘',  '╒',  '╓',  '╫',  '╪',  '┘',  '┌',  '█',  '▄',  '▌',  '▐',  '▀', 
    'α',  'ß',  'Γ',  'π',  'Σ',  'σ',  'µ',  'τ',  'Φ',  'Θ',  'Ω',  'δ',  '∞',  'φ',  'ε',  '∩', 
    '≡',  '±',  '≥',  '≤',  '⌠',  '⌡',  '÷',  '≈',  '°',  '∙',  '·',  '√',  'ⁿ',  '²',  '■',  '\u{a0}'
];

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/// A Codepage 437 character.
///
/// Stored transparently in memory as a single [`u8`].
///
/// # Examples
///
/// A `CP437Char` can be initialized through either [`from_byte`](Self::from_byte) or [`from_byte`](Self::from_char).
///
/// ```
/// use codepage_rs::CP437Char;
///
/// let byt = CP437Char::from_byte(3);
/// let chr = CP437Char::from_char('♥').unwrap();
///
/// assert_eq!(byt, chr);
/// ```
pub struct CP437Char(u8);

impl CP437Char {
    /// The table of characters in Codepage 437.
    pub const CHARS: [char; 256] = CP437CHARS;

    #[must_use]
    #[inline]
    /// Constructs a [`CP437Char`] from a [`u8`].
    pub const fn from_byte(val: u8) -> Self {
        Self(val)
    }

    #[must_use]
    #[inline]
    /// Converts a [`CP437Char`] to a [`u8`].
    pub const fn to_byte(self) -> u8 {
        self.0
    }

    #[must_use]
    pub const fn location(self) -> (u8, u8) {
        (self.0 % 16, self.0 / (16))
    }

    #[must_use]
    #[expect(
        clippy::too_many_lines,
        reason = "It's just one massive match statement"
    )]
    /// Attempts to construct a [`CP437Char`] from a [`char`].
    pub const fn from_char(char: char) -> Option<Self> {
        match char {
            '\0' => Some(Self(0)),
            '☺' => Some(Self(1)),
            '☻' => Some(Self(2)),
            '♥' => Some(Self(3)),
            '♦' => Some(Self(4)),
            '♣' => Some(Self(5)),
            '♠' => Some(Self(6)),
            '•' => Some(Self(7)),
            '◘' => Some(Self(8)),
            '○' => Some(Self(9)),
            '◙' => Some(Self(10)),
            '♂' => Some(Self(11)),
            '♀' => Some(Self(12)),
            '♪' => Some(Self(13)),
            '♫' => Some(Self(14)),
            '☼' => Some(Self(15)),
            '►' => Some(Self(16)),
            '◄' => Some(Self(17)),
            '↕' => Some(Self(18)),
            '‼' => Some(Self(19)),
            '¶' => Some(Self(20)),
            '§' => Some(Self(21)),
            '▬' => Some(Self(22)),
            '↨' => Some(Self(23)),
            '↑' => Some(Self(24)),
            '↓' => Some(Self(25)),
            '→' => Some(Self(26)),
            '←' => Some(Self(27)),
            '∟' => Some(Self(28)),
            '↔' => Some(Self(29)),
            '▲' => Some(Self(30)),
            '▼' => Some(Self(31)),
            ' ' => Some(Self(32)),
            '!' => Some(Self(33)),
            '"' => Some(Self(34)),
            '#' => Some(Self(35)),
            '$' => Some(Self(36)),
            '%' => Some(Self(37)),
            '&' => Some(Self(38)),
            '\'' => Some(Self(39)),
            '(' => Some(Self(40)),
            ')' => Some(Self(41)),
            '*' => Some(Self(42)),
            '+' => Some(Self(43)),
            ',' => Some(Self(44)),
            '-' => Some(Self(45)),
            '.' => Some(Self(46)),
            '/' => Some(Self(47)),
            '0' => Some(Self(48)),
            '1' => Some(Self(49)),
            '2' => Some(Self(50)),
            '3' => Some(Self(51)),
            '4' => Some(Self(52)),
            '5' => Some(Self(53)),
            '6' => Some(Self(54)),
            '7' => Some(Self(55)),
            '8' => Some(Self(56)),
            '9' => Some(Self(57)),
            ':' => Some(Self(58)),
            ';' => Some(Self(59)),
            '<' => Some(Self(60)),
            '=' => Some(Self(61)),
            '>' => Some(Self(62)),
            '?' => Some(Self(63)),
            '@' => Some(Self(64)),
            'A' => Some(Self(65)),
            'B' => Some(Self(66)),
            'C' => Some(Self(67)),
            'D' => Some(Self(68)),
            'E' => Some(Self(69)),
            'F' => Some(Self(70)),
            'G' => Some(Self(71)),
            'H' => Some(Self(72)),
            'I' => Some(Self(73)),
            'J' => Some(Self(74)),
            'K' => Some(Self(75)),
            'L' => Some(Self(76)),
            'M' => Some(Self(77)),
            'N' => Some(Self(78)),
            'O' => Some(Self(79)),
            'P' => Some(Self(80)),
            'Q' => Some(Self(81)),
            'R' => Some(Self(82)),
            'S' => Some(Self(83)),
            'T' => Some(Self(84)),
            'U' => Some(Self(85)),
            'V' => Some(Self(86)),
            'W' => Some(Self(87)),
            'X' => Some(Self(88)),
            'Y' => Some(Self(89)),
            'Z' => Some(Self(90)),
            '[' => Some(Self(91)),
            '\\' => Some(Self(92)),
            ']' => Some(Self(93)),
            '^' => Some(Self(94)),
            '_' => Some(Self(95)),
            '`' => Some(Self(96)),
            'a' => Some(Self(97)),
            'b' => Some(Self(98)),
            'c' => Some(Self(99)),
            'd' => Some(Self(100)),
            'e' => Some(Self(101)),
            'f' => Some(Self(102)),
            'g' => Some(Self(103)),
            'h' => Some(Self(104)),
            'i' => Some(Self(105)),
            'j' => Some(Self(106)),
            'k' => Some(Self(107)),
            'l' => Some(Self(108)),
            'm' => Some(Self(109)),
            'n' => Some(Self(110)),
            'o' => Some(Self(111)),
            'p' => Some(Self(112)),
            'q' => Some(Self(113)),
            'r' => Some(Self(114)),
            's' => Some(Self(115)),
            't' => Some(Self(116)),
            'u' => Some(Self(117)),
            'v' => Some(Self(118)),
            'w' => Some(Self(119)),
            'x' => Some(Self(120)),
            'y' => Some(Self(121)),
            'z' => Some(Self(122)),
            '{' => Some(Self(123)),
            '|' => Some(Self(124)),
            '}' => Some(Self(125)),
            '~' => Some(Self(126)),
            '⌂' => Some(Self(127)),
            'Ç' => Some(Self(128)),
            'ü' => Some(Self(129)),
            'é' => Some(Self(130)),
            'â' => Some(Self(131)),
            'ä' => Some(Self(132)),
            'à' => Some(Self(133)),
            'å' => Some(Self(134)),
            'ç' => Some(Self(135)),
            'ê' => Some(Self(136)),
            'ë' => Some(Self(137)),
            'è' => Some(Self(138)),
            'ï' => Some(Self(139)),
            'î' => Some(Self(140)),
            'ì' => Some(Self(141)),
            'Ä' => Some(Self(142)),
            'Å' => Some(Self(143)),
            'É' => Some(Self(144)),
            'æ' => Some(Self(145)),
            'Æ' => Some(Self(146)),
            'ô' => Some(Self(147)),
            'ö' => Some(Self(148)),
            'ò' => Some(Self(149)),
            'û' => Some(Self(150)),
            'ù' => Some(Self(151)),
            'ÿ' => Some(Self(152)),
            'Ö' => Some(Self(153)),
            'Ü' => Some(Self(154)),
            '¢' => Some(Self(155)),
            '£' => Some(Self(156)),
            '¥' => Some(Self(157)),
            '₧' => Some(Self(158)),
            'ƒ' => Some(Self(159)),
            'á' => Some(Self(160)),
            'í' => Some(Self(161)),
            'ó' => Some(Self(162)),
            'ú' => Some(Self(163)),
            'ñ' => Some(Self(164)),
            'Ñ' => Some(Self(165)),
            'ª' => Some(Self(166)),
            'º' => Some(Self(167)),
            '¿' => Some(Self(168)),
            '⌐' => Some(Self(169)),
            '¬' => Some(Self(170)),
            '½' => Some(Self(171)),
            '¼' => Some(Self(172)),
            '¡' => Some(Self(173)),
            '«' => Some(Self(174)),
            '»' => Some(Self(175)),
            '░' => Some(Self(176)),
            '▒' => Some(Self(177)),
            '▓' => Some(Self(178)),
            '│' => Some(Self(179)),
            '┤' => Some(Self(180)),
            '╡' => Some(Self(181)),
            '╢' => Some(Self(182)),
            '╖' => Some(Self(183)),
            '╕' => Some(Self(184)),
            '╣' => Some(Self(185)),
            '║' => Some(Self(186)),
            '╗' => Some(Self(187)),
            '╝' => Some(Self(188)),
            '╜' => Some(Self(189)),
            '╛' => Some(Self(190)),
            '┐' => Some(Self(191)),
            '└' => Some(Self(192)),
            '┴' => Some(Self(193)),
            '┬' => Some(Self(194)),
            '├' => Some(Self(195)),
            '─' => Some(Self(196)),
            '┼' => Some(Self(197)),
            '╞' => Some(Self(198)),
            '╟' => Some(Self(199)),
            '╚' => Some(Self(200)),
            '╔' => Some(Self(201)),
            '╩' => Some(Self(202)),
            '╦' => Some(Self(203)),
            '╠' => Some(Self(204)),
            '═' => Some(Self(205)),
            '╬' => Some(Self(206)),
            '╧' => Some(Self(207)),
            '╨' => Some(Self(208)),
            '╤' => Some(Self(209)),
            '╥' => Some(Self(210)),
            '╙' => Some(Self(211)),
            '╘' => Some(Self(212)),
            '╒' => Some(Self(213)),
            '╓' => Some(Self(214)),
            '╫' => Some(Self(215)),
            '╪' => Some(Self(216)),
            '┘' => Some(Self(217)),
            '┌' => Some(Self(218)),
            '█' => Some(Self(219)),
            '▄' => Some(Self(220)),
            '▌' => Some(Self(221)),
            '▐' => Some(Self(222)),
            '▀' => Some(Self(223)),
            'α' => Some(Self(224)),
            'ß' => Some(Self(225)),
            'Γ' => Some(Self(226)),
            'π' => Some(Self(227)),
            'Σ' => Some(Self(228)),
            'σ' => Some(Self(229)),
            'µ' => Some(Self(230)),
            'τ' => Some(Self(231)),
            'Φ' => Some(Self(232)),
            'Θ' => Some(Self(233)),
            'Ω' => Some(Self(234)),
            'δ' => Some(Self(235)),
            '∞' => Some(Self(236)),
            'φ' => Some(Self(237)),
            'ε' => Some(Self(238)),
            '∩' => Some(Self(239)),
            '≡' => Some(Self(240)),
            '±' => Some(Self(241)),
            '≥' => Some(Self(242)),
            '≤' => Some(Self(243)),
            '⌠' => Some(Self(244)),
            '⌡' => Some(Self(245)),
            '÷' => Some(Self(246)),
            '≈' => Some(Self(247)),
            '°' => Some(Self(248)),
            '∙' => Some(Self(249)),
            '·' => Some(Self(250)),
            '√' => Some(Self(251)),
            'ⁿ' => Some(Self(252)),
            '²' => Some(Self(253)),
            '■' => Some(Self(254)),
            '\u{a0}' => Some(Self(255)),
            _ => None,
        }
    }

    #[must_use]
    #[expect(
        clippy::too_many_lines,
        reason = "It's just one massive match statement"
    )]
    /// Converts a [`CP437Char`] to a [`char`].
    pub const fn to_char(self) -> char {
        match self {
            Self(0) => '\0',
            Self(1) => '☺',
            Self(2) => '☻',
            Self(3) => '♥',
            Self(4) => '♦',
            Self(5) => '♣',
            Self(6) => '♠',
            Self(7) => '•',
            Self(8) => '◘',
            Self(9) => '○',
            Self(10) => '◙',
            Self(11) => '♂',
            Self(12) => '♀',
            Self(13) => '♪',
            Self(14) => '♫',
            Self(15) => '☼',
            Self(16) => '►',
            Self(17) => '◄',
            Self(18) => '↕',
            Self(19) => '‼',
            Self(20) => '¶',
            Self(21) => '§',
            Self(22) => '▬',
            Self(23) => '↨',
            Self(24) => '↑',
            Self(25) => '↓',
            Self(26) => '→',
            Self(27) => '←',
            Self(28) => '∟',
            Self(29) => '↔',
            Self(30) => '▲',
            Self(31) => '▼',
            Self(32) => ' ',
            Self(33) => '!',
            Self(34) => '"',
            Self(35) => '#',
            Self(36) => '$',
            Self(37) => '%',
            Self(38) => '&',
            Self(39) => '\'',
            Self(40) => '(',
            Self(41) => ')',
            Self(42) => '*',
            Self(43) => '+',
            Self(44) => ',',
            Self(45) => '-',
            Self(46) => '.',
            Self(47) => '/',
            Self(48) => '0',
            Self(49) => '1',
            Self(50) => '2',
            Self(51) => '3',
            Self(52) => '4',
            Self(53) => '5',
            Self(54) => '6',
            Self(55) => '7',
            Self(56) => '8',
            Self(57) => '9',
            Self(58) => ':',
            Self(59) => ';',
            Self(60) => '<',
            Self(61) => '=',
            Self(62) => '>',
            Self(63) => '?',
            Self(64) => '@',
            Self(65) => 'A',
            Self(66) => 'B',
            Self(67) => 'C',
            Self(68) => 'D',
            Self(69) => 'E',
            Self(70) => 'F',
            Self(71) => 'G',
            Self(72) => 'H',
            Self(73) => 'I',
            Self(74) => 'J',
            Self(75) => 'K',
            Self(76) => 'L',
            Self(77) => 'M',
            Self(78) => 'N',
            Self(79) => 'O',
            Self(80) => 'P',
            Self(81) => 'Q',
            Self(82) => 'R',
            Self(83) => 'S',
            Self(84) => 'T',
            Self(85) => 'U',
            Self(86) => 'V',
            Self(87) => 'W',
            Self(88) => 'X',
            Self(89) => 'Y',
            Self(90) => 'Z',
            Self(91) => '[',
            Self(92) => '\\',
            Self(93) => ']',
            Self(94) => '^',
            Self(95) => '_',
            Self(96) => '`',
            Self(97) => 'a',
            Self(98) => 'b',
            Self(99) => 'c',
            Self(100) => 'd',
            Self(101) => 'e',
            Self(102) => 'f',
            Self(103) => 'g',
            Self(104) => 'h',
            Self(105) => 'i',
            Self(106) => 'j',
            Self(107) => 'k',
            Self(108) => 'l',
            Self(109) => 'm',
            Self(110) => 'n',
            Self(111) => 'o',
            Self(112) => 'p',
            Self(113) => 'q',
            Self(114) => 'r',
            Self(115) => 's',
            Self(116) => 't',
            Self(117) => 'u',
            Self(118) => 'v',
            Self(119) => 'w',
            Self(120) => 'x',
            Self(121) => 'y',
            Self(122) => 'z',
            Self(123) => '{',
            Self(124) => '|',
            Self(125) => '}',
            Self(126) => '~',
            Self(127) => '⌂',
            Self(128) => 'Ç',
            Self(129) => 'ü',
            Self(130) => 'é',
            Self(131) => 'â',
            Self(132) => 'ä',
            Self(133) => 'à',
            Self(134) => 'å',
            Self(135) => 'ç',
            Self(136) => 'ê',
            Self(137) => 'ë',
            Self(138) => 'è',
            Self(139) => 'ï',
            Self(140) => 'î',
            Self(141) => 'ì',
            Self(142) => 'Ä',
            Self(143) => 'Å',
            Self(144) => 'É',
            Self(145) => 'æ',
            Self(146) => 'Æ',
            Self(147) => 'ô',
            Self(148) => 'ö',
            Self(149) => 'ò',
            Self(150) => 'û',
            Self(151) => 'ù',
            Self(152) => 'ÿ',
            Self(153) => 'Ö',
            Self(154) => 'Ü',
            Self(155) => '¢',
            Self(156) => '£',
            Self(157) => '¥',
            Self(158) => '₧',
            Self(159) => 'ƒ',
            Self(160) => 'á',
            Self(161) => 'í',
            Self(162) => 'ó',
            Self(163) => 'ú',
            Self(164) => 'ñ',
            Self(165) => 'Ñ',
            Self(166) => 'ª',
            Self(167) => 'º',
            Self(168) => '¿',
            Self(169) => '⌐',
            Self(170) => '¬',
            Self(171) => '½',
            Self(172) => '¼',
            Self(173) => '¡',
            Self(174) => '«',
            Self(175) => '»',
            Self(176) => '░',
            Self(177) => '▒',
            Self(178) => '▓',
            Self(179) => '│',
            Self(180) => '┤',
            Self(181) => '╡',
            Self(182) => '╢',
            Self(183) => '╖',
            Self(184) => '╕',
            Self(185) => '╣',
            Self(186) => '║',
            Self(187) => '╗',
            Self(188) => '╝',
            Self(189) => '╜',
            Self(190) => '╛',
            Self(191) => '┐',
            Self(192) => '└',
            Self(193) => '┴',
            Self(194) => '┬',
            Self(195) => '├',
            Self(196) => '─',
            Self(197) => '┼',
            Self(198) => '╞',
            Self(199) => '╟',
            Self(200) => '╚',
            Self(201) => '╔',
            Self(202) => '╩',
            Self(203) => '╦',
            Self(204) => '╠',
            Self(205) => '═',
            Self(206) => '╬',
            Self(207) => '╧',
            Self(208) => '╨',
            Self(209) => '╤',
            Self(210) => '╥',
            Self(211) => '╙',
            Self(212) => '╘',
            Self(213) => '╒',
            Self(214) => '╓',
            Self(215) => '╫',
            Self(216) => '╪',
            Self(217) => '┘',
            Self(218) => '┌',
            Self(219) => '█',
            Self(220) => '▄',
            Self(221) => '▌',
            Self(222) => '▐',
            Self(223) => '▀',
            Self(224) => 'α',
            Self(225) => 'ß',
            Self(226) => 'Γ',
            Self(227) => 'π',
            Self(228) => 'Σ',
            Self(229) => 'σ',
            Self(230) => 'µ',
            Self(231) => 'τ',
            Self(232) => 'Φ',
            Self(233) => 'Θ',
            Self(234) => 'Ω',
            Self(235) => 'δ',
            Self(236) => '∞',
            Self(237) => 'φ',
            Self(238) => 'ε',
            Self(239) => '∩',
            Self(240) => '≡',
            Self(241) => '±',
            Self(242) => '≥',
            Self(243) => '≤',
            Self(244) => '⌠',
            Self(245) => '⌡',
            Self(246) => '÷',
            Self(247) => '≈',
            Self(248) => '°',
            Self(249) => '∙',
            Self(250) => '·',
            Self(251) => '√',
            Self(252) => 'ⁿ',
            Self(253) => '²',
            Self(254) => '■',
            Self(255) => '\u{a0}',
        }
    }
}

impl From<CP437Char> for char {
    fn from(value: CP437Char) -> Self {
        value.to_char()
    }
}

impl TryFrom<char> for CP437Char {
    type Error = ();

    fn try_from(value: char) -> Result<Self, ()> {
        Self::from_char(value).ok_or(())
    }
}

#[cfg(feature = "std")]
impl fmt::Display for CP437Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

/// An extension trait for [`char`] and other types that behave similar to [`char`].
pub trait CP437CharExt {
    /// Checks if the character is in CP437.
    fn is_cp437(&self) -> bool;

    /// Converts a character to a byte in CP437, if it would be valid.
    ///
    /// This should always return [`None`] if [`is_cp437`](`Self::is_cp437`) returns `false`.
    fn to_cp437_byte(self) -> Option<u8>;

    /// Casts a character to a [`CP437Char`], if it would be valid.
    ///
    /// This should always return [`None`] if [`is_cp437`](`Self::is_cp437`) returns `false`.
    fn to_cp437(self) -> Option<CP437Char>;
}

impl CP437CharExt for char {
    fn is_cp437(&self) -> bool {
        CP437CHARS.contains(self)
    }

    fn to_cp437_byte(self) -> Option<u8> {
        self.to_cp437().map(CP437Char::to_byte)
    }

    fn to_cp437(self) -> Option<CP437Char> {
        CP437Char::from_char(self)
    }
}

// I don't know who would want this, but I'll provide it regardless.
impl CP437CharExt for CP437Char {
    fn is_cp437(&self) -> bool {
        true
    }

    fn to_cp437_byte(self) -> Option<u8> {
        Some(self.to_byte())
    }

    fn to_cp437(self) -> Option<CP437Char> {
        Some(self)
    }
}
