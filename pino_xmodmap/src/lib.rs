//! Tiny xmodmap parsing library
//!
//! ```rust
//! use pino_xmodmap::{KeyTable, Modifier, KeySym};
//!
//! fn main() {
//!     let xmodmap = KeyTable::new().unwrap();
//!     let a_key = xmodmap.get_key(KeySym::KEY_a).unwrap();
//! }
//! ```

pub use std::str::FromStr;
use std::{collections::HashMap, fmt, process::Command};

/// Each possible modifier key combination
///
/// These corresponds to each column in the .Xmodmap file
#[derive(std::cmp::PartialEq, std::cmp::Eq, std::hash::Hash, Clone)]
pub enum Modifier {
    Key,
    ShiftKey,
    ModeSwitchKey,
    ModeSwitchShiftKey,
    ISOLevel3ShiftKey,
    ISOLevel3ShiftShiftKey,
}

/// Key code as referenced by xmodmap
pub type KeyCode = u8;

/// Combination of a modifier and a keycode
pub type Key = (Modifier, KeyCode);

/// Master table of conversions between key and key sym
pub struct KeyTable {
    key_to_keysym: HashMap<Key, KeySym>,
    keysym_to_key: HashMap<KeySym, Key>,
}

/// Xmodmap related errors
#[derive(Debug)]
pub enum Error {
    /// Missing xmodmap executable
    XmodmapRunError,
    /// Xmodmap file was malformed
    InvalidFormat,
    /// Key code does not exist
    NonExistentKeyCode,
    /// Key sym does not exist
    NonExistentKeySym,
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::XmodmapRunError => write!(
                f,
                "could not run xmodmap command, do you have it installed?"
            ),
            Error::InvalidFormat => write!(f, "invalid xmodmap format"),
            Error::NonExistentKeyCode => write!(f, "non-existent keycode"),
            Error::NonExistentKeySym => write!(f, "non-existent keysym"),
        }
    }
}

impl KeyTable {
    // requires that user has xmodmap program installed
    /// Reads from xmodmap file and populates keytable
    pub fn new() -> Result<Self, Error> {
        let mut key_to_keysym: HashMap<Key, KeySym> = HashMap::new();
        let mut keysym_to_key: HashMap<KeySym, Key> = HashMap::new();

        let output = Command::new("xmodmap")
            .arg("-pke")
            .output()
            .or(Err(Error::XmodmapRunError))?;
        let raw_xmodmap = String::from_utf8(output.stdout).or(Err(Error::XmodmapRunError))?;

        for l in raw_xmodmap.lines() {
            let mut split = l.split_ascii_whitespace();

            assert_eq!(Some("keycode"), split.next());
            let keycode = split
                .next()
                .ok_or(Error::InvalidFormat)?
                .parse::<u8>()
                .or(Err(Error::InvalidFormat))?;
            assert_eq!(Some("="), split.next());

            // TODO handle case where next() fails in a better way
            let a = KeySym::from_str(split.next().unwrap_or("")).unwrap_or(KeySym::KEY_NONE);
            let b = KeySym::from_str(split.next().unwrap_or("")).unwrap_or(KeySym::KEY_NONE);
            key_to_keysym.insert((Modifier::Key, keycode), a.clone());
            keysym_to_key.insert(a, (Modifier::Key, keycode));
            key_to_keysym.insert((Modifier::ShiftKey, keycode), b.clone());
            keysym_to_key.insert(b, (Modifier::Key, keycode));
        }

        Ok(KeyTable {
            key_to_keysym,
            keysym_to_key,
        })
    }

    /// Query a keysym
    pub fn get_keysym(&self, modifier: Modifier, code: KeyCode) -> Result<KeySym, Error> {
        match self.key_to_keysym.get(&(modifier, code)) {
            Some(k) => Ok(k.clone()),
            None => Err(Error::NonExistentKeyCode),
        }
    }

    /// Query a keycode
    pub fn get_key(&self, keysym: KeySym) -> Result<Key, Error> {
        match self.keysym_to_key.get(&keysym) {
            Some(k) => Ok(k.clone()),
            None => Err(Error::NonExistentKeySym),
        }
    }
}

/// Each lower case key sym
pub static ALL_LOWER_CASE: &[KeySym] = &[
    KeySym::KEY_a,
    KeySym::KEY_b,
    KeySym::KEY_c,
    KeySym::KEY_d,
    KeySym::KEY_e,
    KeySym::KEY_f,
    KeySym::KEY_g,
    KeySym::KEY_h,
    KeySym::KEY_i,
    KeySym::KEY_j,
    KeySym::KEY_k,
    KeySym::KEY_l,
    KeySym::KEY_m,
    KeySym::KEY_n,
    KeySym::KEY_o,
    KeySym::KEY_p,
    KeySym::KEY_q,
    KeySym::KEY_r,
    KeySym::KEY_s,
    KeySym::KEY_t,
    KeySym::KEY_u,
    KeySym::KEY_v,
    KeySym::KEY_w,
    KeySym::KEY_x,
    KeySym::KEY_y,
    KeySym::KEY_z,
];

/// Each upper case key sym
pub static ALL_UPPER_CASE: &[KeySym] = &[
    KeySym::KEY_A,
    KeySym::KEY_B,
    KeySym::KEY_C,
    KeySym::KEY_D,
    KeySym::KEY_E,
    KeySym::KEY_F,
    KeySym::KEY_G,
    KeySym::KEY_H,
    KeySym::KEY_I,
    KeySym::KEY_J,
    KeySym::KEY_K,
    KeySym::KEY_L,
    KeySym::KEY_M,
    KeySym::KEY_N,
    KeySym::KEY_O,
    KeySym::KEY_P,
    KeySym::KEY_Q,
    KeySym::KEY_R,
    KeySym::KEY_S,
    KeySym::KEY_T,
    KeySym::KEY_U,
    KeySym::KEY_V,
    KeySym::KEY_W,
    KeySym::KEY_X,
    KeySym::KEY_Y,
    KeySym::KEY_Z,
];

/// Enum for each possible key sym
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum KeySym {
    KEY_NONE,
    KEY_a,
    KEY_b,
    KEY_c,
    KEY_d,
    KEY_e,
    KEY_f,
    KEY_g,
    KEY_h,
    KEY_i,
    KEY_j,
    KEY_k,
    KEY_l,
    KEY_m,
    KEY_n,
    KEY_o,
    KEY_p,
    KEY_q,
    KEY_r,
    KEY_s,
    KEY_t,
    KEY_u,
    KEY_v,
    KEY_w,
    KEY_x,
    KEY_y,
    KEY_z,
    KEY_A,
    KEY_B,
    KEY_C,
    KEY_D,
    KEY_E,
    KEY_F,
    KEY_G,
    KEY_H,
    KEY_I,
    KEY_J,
    KEY_K,
    KEY_L,
    KEY_M,
    KEY_N,
    KEY_O,
    KEY_P,
    KEY_Q,
    KEY_R,
    KEY_S,
    KEY_T,
    KEY_U,
    KEY_V,
    KEY_W,
    KEY_X,
    KEY_Y,
    KEY_Z,
    KEY_SPACE,
    KEY_RETURN,
    KEY_BACKSPACE,
    KEY_TAB,
    KEY_ESCAPE,
    KEY_GRAVE,
    KEY_TILDE,
    KEY_0,
    KEY_1,
    KEY_2,
    KEY_3,
    KEY_4,
    KEY_5,
    KEY_6,
    KEY_7,
    KEY_8,
    KEY_9,
    KEY_EXCLAMATION,
    KEY_AT,
    KEY_NUMBERSIGN,
    KEY_DOLLAR,
    KEY_PERCENT,
    KEY_CIRCUM,
    KEY_AMPERSAND,
    KEY_ASTERISK,
    KEY_PARENLEFT,
    KEY_PARENRIGHT,
    KEY_MINUS,
    KEY_UNDERSCORE,
    KEY_PLUS,
    KEY_EQUAL,
    KEY_BRACKETLEFT,
    KEY_BRACKETRIGHT,
    KEY_BRACELEFT,
    KEY_BRACERIGHT,
    KEY_SEMICOLON,
    KEY_COLON,
    KEY_APOSTROPHE,
    KEY_DOUBLEQUOTE,
    KEY_BACKSLASH,
    KEY_BAR,
    KEY_COMMA,
    KEY_LESS,
    KEY_PERIOD,
    KEY_GREATER,
    KEY_SLASH,
    KEY_QUESTION,
    KEY_F1,
    KEY_F2,
    KEY_F3,
    KEY_F4,
    KEY_F5,
    KEY_F6,
    KEY_F7,
    KEY_F8,
    KEY_F9,
    KEY_F10,
    KEY_F11,
    KEY_F12,
}

impl FromStr for KeySym {
    type Err = ();

    /// From Xmodmap entry to KeySym
    fn from_str(input: &str) -> Result<KeySym, Self::Err> {
        match input {
            "a" => Ok(KeySym::KEY_a),
            "b" => Ok(KeySym::KEY_b),
            "c" => Ok(KeySym::KEY_c),
            "d" => Ok(KeySym::KEY_d),
            "e" => Ok(KeySym::KEY_e),
            "f" => Ok(KeySym::KEY_f),
            "g" => Ok(KeySym::KEY_g),
            "h" => Ok(KeySym::KEY_h),
            "i" => Ok(KeySym::KEY_i),
            "j" => Ok(KeySym::KEY_j),
            "k" => Ok(KeySym::KEY_k),
            "l" => Ok(KeySym::KEY_l),
            "m" => Ok(KeySym::KEY_m),
            "n" => Ok(KeySym::KEY_n),
            "o" => Ok(KeySym::KEY_o),
            "p" => Ok(KeySym::KEY_p),
            "q" => Ok(KeySym::KEY_q),
            "r" => Ok(KeySym::KEY_r),
            "s" => Ok(KeySym::KEY_s),
            "t" => Ok(KeySym::KEY_t),
            "u" => Ok(KeySym::KEY_u),
            "v" => Ok(KeySym::KEY_v),
            "w" => Ok(KeySym::KEY_w),
            "x" => Ok(KeySym::KEY_x),
            "y" => Ok(KeySym::KEY_y),
            "z" => Ok(KeySym::KEY_z),
            "A" => Ok(KeySym::KEY_A),
            "B" => Ok(KeySym::KEY_B),
            "C" => Ok(KeySym::KEY_C),
            "D" => Ok(KeySym::KEY_D),
            "E" => Ok(KeySym::KEY_E),
            "F" => Ok(KeySym::KEY_F),
            "G" => Ok(KeySym::KEY_G),
            "H" => Ok(KeySym::KEY_H),
            "I" => Ok(KeySym::KEY_I),
            "J" => Ok(KeySym::KEY_J),
            "K" => Ok(KeySym::KEY_K),
            "L" => Ok(KeySym::KEY_L),
            "M" => Ok(KeySym::KEY_M),
            "N" => Ok(KeySym::KEY_N),
            "O" => Ok(KeySym::KEY_O),
            "P" => Ok(KeySym::KEY_P),
            "Q" => Ok(KeySym::KEY_Q),
            "R" => Ok(KeySym::KEY_R),
            "S" => Ok(KeySym::KEY_S),
            "T" => Ok(KeySym::KEY_T),
            "U" => Ok(KeySym::KEY_U),
            "V" => Ok(KeySym::KEY_V),
            "W" => Ok(KeySym::KEY_W),
            "X" => Ok(KeySym::KEY_X),
            "Y" => Ok(KeySym::KEY_Y),
            "Z" => Ok(KeySym::KEY_Z),
            "0" => Ok(KeySym::KEY_0),
            "1" => Ok(KeySym::KEY_1),
            "2" => Ok(KeySym::KEY_2),
            "3" => Ok(KeySym::KEY_3),
            "4" => Ok(KeySym::KEY_4),
            "5" => Ok(KeySym::KEY_5),
            "6" => Ok(KeySym::KEY_6),
            "7" => Ok(KeySym::KEY_7),
            "8" => Ok(KeySym::KEY_8),
            "9" => Ok(KeySym::KEY_9),
            "exclam" => Ok(KeySym::KEY_EXCLAMATION),
            "at" => Ok(KeySym::KEY_AT),
            "numbersign" => Ok(KeySym::KEY_NUMBERSIGN),
            "dollar" => Ok(KeySym::KEY_DOLLAR),
            "percent" => Ok(KeySym::KEY_PERCENT),
            "asciicircum" => Ok(KeySym::KEY_CIRCUM),
            "ampersand" => Ok(KeySym::KEY_AMPERSAND),
            "asterisk" => Ok(KeySym::KEY_ASTERISK),
            "parenleft" => Ok(KeySym::KEY_PARENLEFT),
            "parenright" => Ok(KeySym::KEY_PARENRIGHT),
            "minus" => Ok(KeySym::KEY_MINUS),
            "underscore" => Ok(KeySym::KEY_UNDERSCORE),
            "plus" => Ok(KeySym::KEY_PLUS),
            "equal" => Ok(KeySym::KEY_EQUAL),
            "bracketleft" => Ok(KeySym::KEY_BRACKETLEFT),
            "bracketright" => Ok(KeySym::KEY_BRACKETRIGHT),
            "braceleft" => Ok(KeySym::KEY_BRACELEFT),
            "braceright" => Ok(KeySym::KEY_BRACERIGHT),
            "semicolon" => Ok(KeySym::KEY_SEMICOLON),
            "colon" => Ok(KeySym::KEY_COLON),
            "apostrophe" => Ok(KeySym::KEY_APOSTROPHE),
            "quotedbl" => Ok(KeySym::KEY_DOUBLEQUOTE),
            "backslash" => Ok(KeySym::KEY_BACKSLASH),
            "bar" => Ok(KeySym::KEY_BAR),
            "comma" => Ok(KeySym::KEY_COMMA),
            "less" => Ok(KeySym::KEY_LESS),
            "period" => Ok(KeySym::KEY_PERIOD),
            "greater" => Ok(KeySym::KEY_GREATER),
            "slash" => Ok(KeySym::KEY_SLASH),
            "question" => Ok(KeySym::KEY_QUESTION),
            "grave" => Ok(KeySym::KEY_GRAVE),
            "asciitilde" => Ok(KeySym::KEY_TILDE),
            "space" => Ok(KeySym::KEY_SPACE),
            "Return" => Ok(KeySym::KEY_RETURN),
            "BackSpace" => Ok(KeySym::KEY_BACKSPACE),
            "Tab" => Ok(KeySym::KEY_TAB),
            "Escape" => Ok(KeySym::KEY_ESCAPE),
            "NoSymbol" => Ok(KeySym::KEY_NONE),
            "F1" => Ok(KeySym::KEY_F1),
            "F2" => Ok(KeySym::KEY_F2),
            "F3" => Ok(KeySym::KEY_F3),
            "F4" => Ok(KeySym::KEY_F4),
            "F5" => Ok(KeySym::KEY_F5),
            "F6" => Ok(KeySym::KEY_F6),
            "F7" => Ok(KeySym::KEY_F7),
            "F8" => Ok(KeySym::KEY_F8),
            "F9" => Ok(KeySym::KEY_F9),
            "F10" => Ok(KeySym::KEY_F10),
            "F11" => Ok(KeySym::KEY_F11),
            "F12" => Ok(KeySym::KEY_F12),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for KeySym {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ' ' => Ok(KeySym::KEY_SPACE),
            'a' => Ok(KeySym::KEY_a),
            'b' => Ok(KeySym::KEY_b),
            'c' => Ok(KeySym::KEY_c),
            'd' => Ok(KeySym::KEY_d),
            'e' => Ok(KeySym::KEY_e),
            'f' => Ok(KeySym::KEY_f),
            'g' => Ok(KeySym::KEY_g),
            'h' => Ok(KeySym::KEY_h),
            'i' => Ok(KeySym::KEY_i),
            'j' => Ok(KeySym::KEY_j),
            'k' => Ok(KeySym::KEY_k),
            'l' => Ok(KeySym::KEY_l),
            'm' => Ok(KeySym::KEY_m),
            'n' => Ok(KeySym::KEY_n),
            'o' => Ok(KeySym::KEY_o),
            'p' => Ok(KeySym::KEY_p),
            'q' => Ok(KeySym::KEY_q),
            'r' => Ok(KeySym::KEY_r),
            's' => Ok(KeySym::KEY_s),
            't' => Ok(KeySym::KEY_t),
            'u' => Ok(KeySym::KEY_u),
            'v' => Ok(KeySym::KEY_v),
            'w' => Ok(KeySym::KEY_w),
            'x' => Ok(KeySym::KEY_x),
            'y' => Ok(KeySym::KEY_y),
            'z' => Ok(KeySym::KEY_z),
            'A' => Ok(KeySym::KEY_A),
            'B' => Ok(KeySym::KEY_B),
            'C' => Ok(KeySym::KEY_C),
            'D' => Ok(KeySym::KEY_D),
            'E' => Ok(KeySym::KEY_E),
            'F' => Ok(KeySym::KEY_F),
            'G' => Ok(KeySym::KEY_G),
            'H' => Ok(KeySym::KEY_H),
            'I' => Ok(KeySym::KEY_I),
            'J' => Ok(KeySym::KEY_J),
            'K' => Ok(KeySym::KEY_K),
            'L' => Ok(KeySym::KEY_L),
            'M' => Ok(KeySym::KEY_M),
            'N' => Ok(KeySym::KEY_N),
            'O' => Ok(KeySym::KEY_O),
            'P' => Ok(KeySym::KEY_P),
            'Q' => Ok(KeySym::KEY_Q),
            'R' => Ok(KeySym::KEY_R),
            'S' => Ok(KeySym::KEY_S),
            'T' => Ok(KeySym::KEY_T),
            'U' => Ok(KeySym::KEY_U),
            'V' => Ok(KeySym::KEY_V),
            'W' => Ok(KeySym::KEY_W),
            'X' => Ok(KeySym::KEY_X),
            'Y' => Ok(KeySym::KEY_Y),
            'Z' => Ok(KeySym::KEY_Z),
            '0' => Ok(KeySym::KEY_0),
            '1' => Ok(KeySym::KEY_1),
            '2' => Ok(KeySym::KEY_2),
            '3' => Ok(KeySym::KEY_3),
            '4' => Ok(KeySym::KEY_4),
            '5' => Ok(KeySym::KEY_5),
            '6' => Ok(KeySym::KEY_6),
            '7' => Ok(KeySym::KEY_7),
            '8' => Ok(KeySym::KEY_8),
            '9' => Ok(KeySym::KEY_9),
            '!' => Ok(KeySym::KEY_EXCLAMATION),
            '@' => Ok(KeySym::KEY_AT),
            '#' => Ok(KeySym::KEY_NUMBERSIGN),
            '$' => Ok(KeySym::KEY_DOLLAR),
            '%' => Ok(KeySym::KEY_PERCENT),
            '^' => Ok(KeySym::KEY_CIRCUM),
            '&' => Ok(KeySym::KEY_AMPERSAND),
            '*' => Ok(KeySym::KEY_ASTERISK),
            '(' => Ok(KeySym::KEY_PARENLEFT),
            ')' => Ok(KeySym::KEY_PARENRIGHT),
            '-' => Ok(KeySym::KEY_MINUS),
            '_' => Ok(KeySym::KEY_UNDERSCORE),
            '+' => Ok(KeySym::KEY_PLUS),
            '=' => Ok(KeySym::KEY_EQUAL),
            '[' => Ok(KeySym::KEY_BRACKETLEFT),
            ']' => Ok(KeySym::KEY_BRACKETRIGHT),
            '{' => Ok(KeySym::KEY_BRACELEFT),
            '}' => Ok(KeySym::KEY_BRACERIGHT),
            ';' => Ok(KeySym::KEY_SEMICOLON),
            ':' => Ok(KeySym::KEY_COLON),
            '\'' => Ok(KeySym::KEY_APOSTROPHE),
            '"' => Ok(KeySym::KEY_DOUBLEQUOTE),
            '\\' => Ok(KeySym::KEY_BACKSLASH),
            '|' => Ok(KeySym::KEY_BAR),
            ',' => Ok(KeySym::KEY_COMMA),
            '<' => Ok(KeySym::KEY_LESS),
            '.' => Ok(KeySym::KEY_PERIOD),
            '>' => Ok(KeySym::KEY_GREATER),
            '/' => Ok(KeySym::KEY_SLASH),
            '?' => Ok(KeySym::KEY_QUESTION),
            '`' => Ok(KeySym::KEY_GRAVE),
            '~' => Ok(KeySym::KEY_TILDE),
            _ => return Err(()),
        }
    }
}

impl TryFrom<KeySym> for char {
    type Error = ();
    fn try_from(value: KeySym) -> Result<Self, Self::Error> {
        let key = match value {
            KeySym::KEY_BACKSPACE => 0x08,
            KeySym::KEY_TAB => 0x09,
            KeySym::KEY_ESCAPE => 0x1b,
            KeySym::KEY_SPACE => 0x20,
            KeySym::KEY_EXCLAMATION => 0x21,
            KeySym::KEY_DOUBLEQUOTE => 0x22,
            KeySym::KEY_NUMBERSIGN => 0x23,
            KeySym::KEY_DOLLAR => 0x24,
            KeySym::KEY_PERCENT => 0x25,
            KeySym::KEY_AMPERSAND => 0x26,
            KeySym::KEY_APOSTROPHE => 0x27,
            KeySym::KEY_PARENLEFT => 0x28,
            KeySym::KEY_PARENRIGHT => 0x29,
            KeySym::KEY_ASTERISK => 0x2a,
            KeySym::KEY_PLUS => 0x2b,
            KeySym::KEY_COMMA => 0x2c,
            KeySym::KEY_MINUS => 0x2d,
            KeySym::KEY_PERIOD => 0x2e,
            KeySym::KEY_SLASH => 0x2f,
            KeySym::KEY_0 => 0x30,
            KeySym::KEY_1 => 0x31,
            KeySym::KEY_2 => 0x32,
            KeySym::KEY_3 => 0x33,
            KeySym::KEY_4 => 0x34,
            KeySym::KEY_5 => 0x35,
            KeySym::KEY_6 => 0x36,
            KeySym::KEY_7 => 0x37,
            KeySym::KEY_8 => 0x38,
            KeySym::KEY_9 => 0x39,
            KeySym::KEY_COLON => 0x3a,
            KeySym::KEY_SEMICOLON => 0x3b,
            KeySym::KEY_LESS => 0x3c,
            KeySym::KEY_EQUAL => 0x3d,
            KeySym::KEY_GREATER => 0x3e,
            KeySym::KEY_QUESTION => 0x3f,
            KeySym::KEY_AT => 0x40,
            KeySym::KEY_A => 0x41,
            KeySym::KEY_B => 0x42,
            KeySym::KEY_C => 0x43,
            KeySym::KEY_D => 0x44,
            KeySym::KEY_E => 0x45,
            KeySym::KEY_F => 0x46,
            KeySym::KEY_G => 0x47,
            KeySym::KEY_H => 0x48,
            KeySym::KEY_I => 0x49,
            KeySym::KEY_J => 0x4a,
            KeySym::KEY_K => 0x4b,
            KeySym::KEY_L => 0x4c,
            KeySym::KEY_M => 0x4d,
            KeySym::KEY_N => 0x4e,
            KeySym::KEY_O => 0x4f,
            KeySym::KEY_P => 0x50,
            KeySym::KEY_Q => 0x51,
            KeySym::KEY_R => 0x52,
            KeySym::KEY_S => 0x53,
            KeySym::KEY_T => 0x54,
            KeySym::KEY_U => 0x55,
            KeySym::KEY_V => 0x56,
            KeySym::KEY_W => 0x57,
            KeySym::KEY_X => 0x58,
            KeySym::KEY_Y => 0x59,
            KeySym::KEY_Z => 0x5a,
            KeySym::KEY_a => 0x61,
            KeySym::KEY_b => 0x62,
            KeySym::KEY_c => 0x63,
            KeySym::KEY_d => 0x64,
            KeySym::KEY_e => 0x65,
            KeySym::KEY_f => 0x66,
            KeySym::KEY_g => 0x67,
            KeySym::KEY_h => 0x68,
            KeySym::KEY_i => 0x69,
            KeySym::KEY_j => 0x6a,
            KeySym::KEY_k => 0x6b,
            KeySym::KEY_l => 0x6c,
            KeySym::KEY_m => 0x6d,
            KeySym::KEY_n => 0x6e,
            KeySym::KEY_o => 0x6f,
            KeySym::KEY_p => 0x70,
            KeySym::KEY_q => 0x71,
            KeySym::KEY_r => 0x72,
            KeySym::KEY_s => 0x73,
            KeySym::KEY_t => 0x74,
            KeySym::KEY_u => 0x75,
            KeySym::KEY_v => 0x76,
            KeySym::KEY_w => 0x77,
            KeySym::KEY_x => 0x78,
            KeySym::KEY_y => 0x79,
            KeySym::KEY_z => 0x7a,
            KeySym::KEY_BRACELEFT => 0x7b,
            KeySym::KEY_BAR => 0x7c,
            KeySym::KEY_BRACERIGHT => 0x7d,
            KeySym::KEY_TILDE => 0x7e,
            _ => return Err(()),
        };

        char::from_u32(key).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::KeySym;

    #[test]
    fn str_to_keysym() {
        assert_eq!(KeySym::from_str("a"), Ok(KeySym::KEY_a));
        assert_eq!(KeySym::from_str("asciitilde"), Ok(KeySym::KEY_TILDE));
    }

    #[test]
    fn char_to_keysym() {
        assert_eq!(KeySym::try_from('a'), Ok(KeySym::KEY_a));
        assert_eq!(KeySym::try_from('~'), Ok(KeySym::KEY_TILDE));
    }

    #[test]
    fn keysym_to_char() {
        assert_eq!(char::try_from(KeySym::KEY_a), Ok('a'));
        assert_eq!(char::try_from(KeySym::KEY_TILDE), Ok('~'));
    }
}
