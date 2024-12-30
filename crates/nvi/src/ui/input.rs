//
/// Our strategy is to use:
///
/// - getchar to get a character
/// - use getcharmod to get the modifiers
///
/// See:
/// :help key-notation
use std::fmt;

pub enum Mod {
    /// Shift modifier (2)
    Shift,
    /// Control modifier (4)
    Control,
    /// Alt/Meta modifier (8)
    Alt,
    /// Meta modifier when different from Alt (16)
    Meta,
    /// Command (Mac) or Super key (128)
    Super,
    /// Mouse double click (32)
    DClick,
    /// Mouse triple click (64)
    TClick,
    /// Mouse quadruple click (96 = 32 + 64)
    QClick,
}

impl Mod {
    fn to_prefix(&self) -> &'static str {
        match self {
            Mod::Shift => "S-",
            Mod::Control => "C-",
            Mod::Alt => "M-",
            Mod::Meta => "T-",
            Mod::Super => "D-",
            // Mouse clicks don't have prefixes in key notation
            Mod::DClick | Mod::TClick | Mod::QClick => "",
        }
    }
}

pub enum Keys {
    // Special keys
    Nul,
    BS,
    Tab,
    NL,
    CR,
    Return,
    Enter,
    Esc,
    Space,
    Lt,
    Bslash,
    Bar,
    Del,
    CSI,
    EOL,
    Ignore,
    NOP,

    // Cursor keys
    Up,
    Down,
    Left,
    Right,

    // Function keys
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

    // Navigation keys
    Help,
    Undo,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,

    // Keypad keys
    KUp,
    KDown,
    KLeft,
    KRight,
    KHome,
    KEnd,
    KOrigin,
    KPageUp,
    KPageDown,
    KDel,
    KPlus,
    KMinus,
    KMultiply,
    KDivide,
    KPoint,
    KComma,
    KEqual,
    KEnter,
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,

    // Mouse
    LeftMouse,
    RightMouse,
    MiddleMouse,

    // Regular character
    Char(char),
}

impl Keys {
    fn name(&self) -> String {
        match self {
            Keys::Nul => "Nul".to_string(),
            Keys::BS => "BS".to_string(),
            Keys::Tab => "Tab".to_string(),
            Keys::NL => "NL".to_string(),
            Keys::CR => "CR".to_string(),
            Keys::Return => "Return".to_string(),
            Keys::Enter => "Enter".to_string(),
            Keys::Esc => "Esc".to_string(),
            Keys::Space => "Space".to_string(),
            Keys::Lt => "lt".to_string(),
            Keys::Bslash => "Bslash".to_string(),
            Keys::Bar => "Bar".to_string(),
            Keys::Del => "Del".to_string(),
            Keys::CSI => "CSI".to_string(),
            Keys::EOL => "EOL".to_string(),
            Keys::Ignore => "Ignore".to_string(),
            Keys::NOP => "NOP".to_string(),
            Keys::Up => "Up".to_string(),
            Keys::Down => "Down".to_string(),
            Keys::Left => "Left".to_string(),
            Keys::Right => "Right".to_string(),
            Keys::F1 => "F1".to_string(),
            Keys::F2 => "F2".to_string(),
            Keys::F3 => "F3".to_string(),
            Keys::F4 => "F4".to_string(),
            Keys::F5 => "F5".to_string(),
            Keys::F6 => "F6".to_string(),
            Keys::F7 => "F7".to_string(),
            Keys::F8 => "F8".to_string(),
            Keys::F9 => "F9".to_string(),
            Keys::F10 => "F10".to_string(),
            Keys::F11 => "F11".to_string(),
            Keys::F12 => "F12".to_string(),
            Keys::Help => "Help".to_string(),
            Keys::Undo => "Undo".to_string(),
            Keys::Insert => "Insert".to_string(),
            Keys::Home => "Home".to_string(),
            Keys::End => "End".to_string(),
            Keys::PageUp => "PageUp".to_string(),
            Keys::PageDown => "PageDown".to_string(),
            Keys::KUp => "kUp".to_string(),
            Keys::KDown => "kDown".to_string(),
            Keys::KLeft => "kLeft".to_string(),
            Keys::KRight => "kRight".to_string(),
            Keys::KHome => "kHome".to_string(),
            Keys::KEnd => "kEnd".to_string(),
            Keys::KOrigin => "kOrigin".to_string(),
            Keys::KPageUp => "kPageUp".to_string(),
            Keys::KPageDown => "kPageDown".to_string(),
            Keys::KDel => "kDel".to_string(),
            Keys::KPlus => "kPlus".to_string(),
            Keys::KMinus => "kMinus".to_string(),
            Keys::KMultiply => "kMultiply".to_string(),
            Keys::KDivide => "kDivide".to_string(),
            Keys::KPoint => "kPoint".to_string(),
            Keys::KComma => "kComma".to_string(),
            Keys::KEqual => "kEqual".to_string(),
            Keys::KEnter => "kEnter".to_string(),
            Keys::K0 => "k0".to_string(),
            Keys::K1 => "k1".to_string(),
            Keys::K2 => "k2".to_string(),
            Keys::K3 => "k3".to_string(),
            Keys::K4 => "k4".to_string(),
            Keys::K5 => "k5".to_string(),
            Keys::K6 => "k6".to_string(),
            Keys::K7 => "k7".to_string(),
            Keys::K8 => "k8".to_string(),
            Keys::K9 => "k9".to_string(),
            Keys::LeftMouse => "LeftMouse".to_string(),
            Keys::RightMouse => "RightMouse".to_string(),
            Keys::MiddleMouse => "MiddleMouse".to_string(),
            Keys::Char(c) => c.to_string(),
        }
    }
}

pub struct KeyPress {
    pub modifers: Vec<Mod>,
    pub key: Keys,

    /// The string representation as returned by nvim
    pub raw: String,
}

impl fmt::Display for KeyPress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modifers.is_empty() {
            match &self.key {
                Keys::Char(c) => write!(f, "{}", c),
                _ => write!(f, "<{}>", self.key.name()),
            }
        } else {
            write!(f, "<")?;
            for modifier in &self.modifers {
                write!(f, "{}", modifier.to_prefix())?;
            }
            write!(f, "{}>", self.key.name())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_char() {
        let key = KeyPress {
            modifers: vec![],
            key: Keys::Char('a'),
            raw: "a".to_string(),
        };
        assert_eq!(format!("{}", key), "a");
    }

    #[test]
    fn test_uppercase_char() {
        let key = KeyPress {
            modifers: vec![],
            key: Keys::Char('A'),
            raw: "A".to_string(),
        };
        assert_eq!(format!("{}", key), "A");
    }

    #[test]
    fn test_ctrl_a() {
        let key = KeyPress {
            modifers: vec![Mod::Control],
            key: Keys::Char('a'),
            raw: "<C-a>".to_string(),
        };
        assert_eq!(format!("{}", key), "<C-a>");
    }
}
