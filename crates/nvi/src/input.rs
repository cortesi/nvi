//! This module does the surprisingly complicated job of bringing type safety and order to the
//! chaos of Vim keypresses.
//!
//! - We have a standard structure for reprsenting key presses - a key and a set of modifiers.
//! - Our standard structure has a uniform string representation that mirrors Vim's key notation.
//! - We go to considerable lengths to make sure that functions for reading and writing input are
//!   symmetric - that is,if you write keypresses with feedkeys, you can be sure to get them back
//!   in the same format with getchar (modulo normalization).
//! - We normalize control characters into their corresponding KeyPress representation, e.g.
//!   Control-A is represented as <C-A>, not as \x01.
//! - We normalize the case of control characters, so that <C-a> is the same as <C-A>.
//!
//! The string format for keypresses is:
//!
//! - <key>
//! - <modifier-key>
//!
//! The following modifiers are supported:
//!
//! - C: Control
//! - S: Shift
//! - M: Alt/Meta
//! - T: Meta
//! - D: Super
//!
//! For mouse events, the format is:
//!
//! - <LeftMouse>
//! - <RightMouse>
//! - <MiddleMouse>
//!
//! With modifiers:
//!
//! - DClick: Double click
//! - TClick: Triple click
//! - QClick: Quadruple click
//!
//! See:
//! :help key-notation

use std::fmt::{self, Write};

use crate::{error::Error, error::Result, lua, lua_exec, Client, Value};

/// A modifier key.
#[derive(Debug, PartialEq, Clone)]
pub enum Mod {
    /// Shift modifier
    Shift,
    /// Control modifier
    Control,
    /// Alt/Meta modifier
    Alt,
    /// Meta modifier when different from Alt
    Meta,
    /// Command (Mac) or Super key
    Super,
    /// Mouse double click
    DClick,
    /// Mouse triple click
    TClick,
    /// Mouse quadruple click
    QClick,
}

impl Mod {
    /// Creates a vector of Mod from a charmod number.
    pub fn from_charmod(charmod: u8) -> Vec<Mod> {
        let mut mods = Vec::new();

        if charmod & 2 != 0 {
            mods.push(Mod::Shift);
        }
        if charmod & 4 != 0 {
            mods.push(Mod::Control);
        }
        if charmod & 8 != 0 {
            mods.push(Mod::Alt);
        }
        if charmod & 16 != 0 {
            mods.push(Mod::Meta);
        }
        if charmod & 32 != 0 {
            mods.push(Mod::DClick);
        }
        if charmod & 64 != 0 {
            mods.push(Mod::TClick);
        }
        if charmod & 96 == 96 {
            mods.push(Mod::QClick);
        }
        if charmod & 128 != 0 {
            mods.push(Mod::Super);
        }

        mods
    }
}

impl Mod {
    /// Returns the numeric value of the modifier.
    pub fn num(&self) -> u8 {
        match self {
            Mod::Shift => 2,
            Mod::Control => 4,
            Mod::Alt => 8,
            Mod::Meta => 16,
            Mod::Super => 128,
            Mod::DClick => 32,
            Mod::TClick => 64,
            Mod::QClick => 96,
        }
    }

    /// Returns the prefix representation of the modifier.
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

/// A non-modifier key.
#[derive(Debug, PartialEq, Clone, strum::Display, strum::EnumString)]
#[strum(ascii_case_insensitive)]
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
    /// Returns the official VIM name of the key.
    pub fn name(&self) -> String {
        match self {
            Keys::Char(c) => c.to_string(),
            other => {
                let s = other.to_string();
                if s.len() > 1 && s.starts_with('K') {
                    format!("k{}", &s[1..])
                } else {
                    s
                }
            }
        }
    }

    /// Parse a key name into a Keys variant.
    pub fn from_name(name: &str) -> Result<Self, Error> {
        name.parse::<Keys>()
            .map_err(|_| Error::User(format!("Invalid key name: {}", name)))
    }
}

/// A key press event, including a modifier and a key.
#[derive(Debug, PartialEq, Clone)]
pub struct KeyPress {
    pub modifers: Vec<Mod>,
    pub key: Keys,
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
            match &self.key {
                Keys::Char(c) => write!(f, "{}>", c),
                _ => write!(f, "{}>", self.key.name()),
            }
        }
    }
}

impl KeyPress {
    /// Normalizes control characters into their corresponding KeyPress representation
    fn normalise(&self) -> KeyPress {
        match (&self.key, &self.modifers) {
            // Control character (ASCII 1-26)
            (Keys::Char(c), _) if *c as u32 <= 26 => KeyPress {
                modifers: vec![Mod::Control],
                key: Keys::Char((*c as u8 + b'A' - 1) as char),
            },
            // Lowercase control combination
            (Keys::Char(c), mods) if mods.contains(&Mod::Control) && c.is_ascii_lowercase() => {
                KeyPress {
                    modifers: self.modifers.clone(),
                    key: Keys::Char(c.to_ascii_uppercase()),
                }
            }
            _ => self.clone(),
        }
    }

    /// Constructs a KeyPress object from a given Lua string.
    fn from_lua_with_mods(modifiers: Vec<Mod>, value: &str) -> Result<Self, Error> {
        let raw = value.to_string();
        // Decode the value here, which should be in the format of <mod-key>
        // or just a single character without modifiers.
        if raw.starts_with('<') && raw.ends_with('>') {
            let parts: Vec<&str> = raw
                .trim_start_matches('<')
                .trim_end_matches('>')
                .split('-')
                .collect();
            if let Some((&key, _)) = parts.split_last() {
                let key = if key.len() == 1 {
                    Keys::Char(key.chars().next().unwrap())
                } else {
                    Keys::from_name(key)?
                };

                return Ok(KeyPress {
                    modifers: modifiers,
                    key,
                }
                .normalise());
            }
        } else if raw.len() == 1 {
            return Ok(KeyPress {
                modifers: Vec::new(),
                key: Keys::Char(raw.chars().next().unwrap()),
            }
            .normalise());
        }
        Err(Error::User(format!("Failed to parse keypress: {:?}", raw)))
    }
}

/// Get a single keypress from the client.
pub async fn get_keypress(client: &Client) -> Result<KeyPress, Error> {
    let lua_code = r#"
        local char = vim.fn.getcharstr()
        local charmod = vim.fn.getcharmod()
        return {charmod, char}
    "#;

    match lua_exec!(client, lua_code).await? {
        Value::Array(arr) if arr.len() == 2 => {
            if let Value::Integer(charmod) = &arr[0] {
                let modifiers = if let Some(ch) = charmod.as_u64() {
                    Mod::from_charmod(ch as u8)
                } else {
                    return Err(Error::User(
                        "Failed to interpret charmod as u64".to_string(),
                    ));
                };

                match &arr[1] {
                    Value::String(s) => KeyPress::from_lua_with_mods(
                        modifiers,
                        s.as_str().expect("Lua string conversion failed"),
                    ),
                    Value::Binary(bytes) => {
                        // For binary data, we need to use vim.fn.keytrans to get the key notation
                        // Pass the raw bytes directly to keytrans
                        let lua_keytrans = format!(
                            "return vim.fn.keytrans('{}')",
                            bytes.iter().fold(String::new(), |mut acc, &b| {
                                let _ = write!(acc, "\\x{:02x}", b);
                                acc
                            })
                        );
                        match lua!(client, &lua_keytrans).await? {
                            Value::String(s) => KeyPress::from_lua_with_mods(
                                modifiers,
                                s.as_str().expect("Lua string conversion failed"),
                            ),
                            _ => Err(Error::User("keytrans did not return a string".to_string())),
                        }
                    }
                    _ => Err(Error::User(
                        "Unexpected type for keypress value".to_string(),
                    )),
                }
            } else {
                Err(Error::User(
                    "Unexpected types in Lua return value".to_string(),
                ))
            }
        }
        _ => Err(Error::User(
            "Unexpected return type from lua execution".to_string(),
        )),
    }
}

/// Feed a string of keys to the client. Special keys should be escaped in
/// the standard vim way, e.g: "\<C-a>" for Control-A. Valid modifiers are:
pub async fn feedkeys(client: &Client, keys: &str) -> Result<()> {
    let lua_code = format!(
        r#"
            vim.fn.feedkeys(vim.api.nvim_eval("\"{}\""))
        "#,
        lua::escape_str(keys),
    );
    match lua_exec!(client, &lua_code).await {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::User(format!("Failed to feedkeys: {}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::NviTest;

    #[test]
    fn test_mods_from_charmod() {
        assert_eq!(Mod::from_charmod(0), vec![]);
        assert_eq!(Mod::from_charmod(2), vec![Mod::Shift]);
        assert_eq!(Mod::from_charmod(12), vec![Mod::Control, Mod::Alt]);
        assert_eq!(
            Mod::from_charmod(96),
            vec![Mod::DClick, Mod::TClick, Mod::QClick]
        );
        assert_eq!(Mod::from_charmod(128), vec![Mod::Super]);
    }

    #[test]
    fn test_key_name_roundtrip() {
        let test_cases = vec![
            "Tab",
            "Enter",
            "Esc",
            "Space",
            "Up",
            "Down",
            "Left",
            "Right",
            "F1",
            "F12",
            "Home",
            "End",
            "PageUp",
            "KUp",
            "KDown",
            "K0",
            "K9",
            "LeftMouse",
        ];

        for name in test_cases {
            let key = Keys::from_name(name).unwrap();
            assert_eq!(key.name().to_lowercase(), name.to_lowercase());
        }
    }

    #[test]
    fn test_key_press_display() {
        let test_cases = vec![
            (
                KeyPress {
                    modifers: vec![],
                    key: Keys::Char('a'),
                },
                "a",
            ),
            (
                KeyPress {
                    modifers: vec![],
                    key: Keys::Char('A'),
                },
                "A",
            ),
            (
                KeyPress {
                    modifers: vec![Mod::Control],
                    key: Keys::Char('a'),
                },
                "<C-a>",
            ),
        ];

        for (key, expected) in test_cases {
            assert_eq!(format!("{}", key), expected);
        }
    }

    #[tokio::test]
    async fn test_input() {
        let test_cases = vec![
            ("a", "a"),
            ("A", "A"),
            (r"\<S-b>", "B"),
            (r"\<S-B>", "B"),
            (r"\<C-A>", "<C-A>"),
            (r"\<C-a>", "<C-A>"),
            (r"\<M-Left>", "<M-Left>"),
        ];
        let test = NviTest::builder()
            .log_level(tracing::Level::DEBUG)
            .run()
            .await
            .unwrap();

        for (input, expected) in test_cases {
            let client1 = test.client.clone();
            let client2 = client1.clone();
            let test_input = input.to_string();
            let handle = tokio::spawn(async move {
                loop {
                    feedkeys(&client2, &test_input).await.unwrap();
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }
            });
            let key = get_keypress(&client1).await.unwrap();
            println!("Got key: {:?}", key);
            handle.abort();
            let _ = handle.await;

            assert_eq!(format!("{}", key), expected);
        }
        test.finish().await.unwrap();
    }
}
