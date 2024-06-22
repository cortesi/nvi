/* A compendium of types for working with the Neovim msgrpc API */
use derive_setters::*;
use serde_derive::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes, NoneAsEmptyString};

use crate::{client, error::Result, opts};

pub const BUFFER_EXT_TYPE: i8 = 0;
pub const WINDOW_EXT_TYPE: i8 = 1;
pub const TABPAGE_EXT_TYPE: i8 = 2;

fn u8_array_to_u64(bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .rev()
        .fold(0u64, |acc, &b| (acc << 8) | b as u64)
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename = "_ExtStruct")]
pub struct Buffer(#[serde_as(as = "(_, Bytes)")] (i8, Vec<u8>));

impl Buffer {
    pub fn current() -> Self {
        Buffer((BUFFER_EXT_TYPE, vec![0, 0, 0, 0]))
    }

    /// Set an option on this buffer
    pub async fn set<T: serde::Serialize>(
        &self,
        c: &mut client::Client,
        name: &str,
        value: T,
    ) -> Result<()> {
        c.nvim
            .set_option_value(
                name,
                value,
                opts::SetOptionValue::default().buf(self.clone()),
            )
            .await
    }
}

impl From<Buffer> for u64 {
    fn from(val: Buffer) -> Self {
        let (_, bytes) = val.0;
        u8_array_to_u64(&bytes)
    }
}

impl From<u64> for Buffer {
    fn from(value: u64) -> Self {
        let bytes = value.to_le_bytes().to_vec();
        Buffer((BUFFER_EXT_TYPE, bytes))
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename = "_ExtStruct")]
pub struct Window(#[serde_as(as = "(_, Bytes)")] (i8, Vec<u8>));

impl Window {
    pub fn current() -> Self {
        Window((WINDOW_EXT_TYPE, vec![0, 0, 0, 0]))
    }

    /// Set an option on this window
    pub async fn set<T: serde::Serialize>(
        &self,
        c: &mut client::Client,
        name: &str,
        value: T,
    ) -> Result<()> {
        c.nvim
            .set_option_value(
                name,
                value,
                opts::SetOptionValue::default().win(self.clone()),
            )
            .await
    }
}

impl From<Window> for u64 {
    fn from(val: Window) -> Self {
        let (_, bytes) = val.0;
        u8_array_to_u64(&bytes)
    }
}

impl From<u64> for Window {
    fn from(value: u64) -> Self {
        let bytes = value.to_le_bytes().to_vec();
        Window((WINDOW_EXT_TYPE, bytes))
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename = "_ExtStruct")]
pub struct TabPage(#[serde_as(as = "(_, Bytes)")] (i8, Vec<u8>));

impl TabPage {
    pub fn current() -> Self {
        TabPage((TABPAGE_EXT_TYPE, vec![0, 0, 0, 0]))
    }
}

impl From<TabPage> for u64 {
    fn from(val: TabPage) -> Self {
        let (_, bytes) = val.0;
        u8_array_to_u64(&bytes)
    }
}

impl From<u64> for TabPage {
    fn from(value: u64) -> Self {
        let bytes = value.to_le_bytes().to_vec();
        TabPage((TABPAGE_EXT_TYPE, bytes))
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct APIVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub api_level: u64,
    pub api_compatible: u64,
    pub build: String,
    pub prerelease: bool,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct ApiInfo {
    pub version: APIVersion,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ChanInfo {
    pub id: u64,
    pub argv: Option<String>,
    pub stream: String,
    pub mode: String,
    pub pty: Option<String>,
    // FIXME: Docs aren't clear on what the return types of these two are.
    //pub buffer: Option<String>,
    //pub client: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AutocmdEvent {
    /// The AutoCommand ID, as returned by `nvim_create_autocmd`
    pub id: u64,
    /// The triggered event
    pub event: Event,
    /// AutoCommand group, if any
    pub group: Option<u64>,
    /// The pattern that triggered the event, expanded from <amatch>
    pub matches: Option<Vec<String>>,
    /// The buffer, from <abuf>
    pub buf: u64,
    /// The file, from <afile>
    pub file: String,
    /// Data passed by nvim_exec_autocmds
    pub data: Option<crate::Value>,
}

/// Autocommand events. See here for documentation: https://neovim.io/doc/user/autocmd.html#autocmd-events
#[derive(
    Debug, Clone, PartialEq, Eq, strum::Display, strum::EnumString, Deserialize, Serialize,
)]
pub enum Event {
    BufAdd,
    BufDelete,
    BufEnter,
    BufFilePost,
    BufFilePre,
    BufHidden,
    BufLeave,
    BufNew,
    BufModifiedSet,
    BufNewFile,
    BufRead,
    BufReadPost,
    BufWinLeave,
    BufWipeout,
    BufWrite,
    BufWritePre,
    BufWritePost,
    BufWriteCmd,
    ChanInfo,
    ChanOpen,
    CmdUndefined,
    CmdlineChanged,
    CmdlineEnter,
    CmdlineLeave,
    CmdwinEnter,
    CmdwinLeave,
    ColorScheme,
    ColorSchemePre,
    CompleteChanged,
    CompleteDonePre,
    CompleteDone,
    CursorHold,
    CursorHoldI,
    CursorMoved,
    CursorMovedI,
    DiffUpdated,
    DirChanged,
    DirChangedPre,
    ExitPre,
    FileAppendCmd,
    FileAppendPost,
    FileAppendPre,
    FileChangedRO,
    FileChangedShell,
    FileChangedShellPost,
    FileReadCmd,
    FileReadPost,
    FileReadPre,
    FileType,
    FileWriteCmd,
    FileWritePost,
    FileWritePre,
    FilterReadPost,
    FilterReadPre,
    FilterWritePost,
    FilterWritePre,
    FocusGained,
    FocusLost,
    FuncUndefined,
    UIEnter,
    UILeave,
    InsertChange,
    InsertCharPre,
    InsertEnter,
    InsertLeavePre,
    InsertLeave,
    MenuPopup,
    ModeChanged,
    OptionSet,
    QuickFixCmdPre,
    QuickFixCmdPost,
    QuitPre,
    RemoteReply,
    SearchWrapped,
    RecordingEnter,
    RecordingLeave,
    SafeState,
    SessionLoadPost,
    SessionWritePost,
    ShellCmdPost,
    Signal,
    ShellFilterPost,
    SourcePre,
    SourcePost,
    SourceCmd,
    SpellFileMissing,
    StdinReadPost,
    StdinReadPre,
    SwapExists,
    Syntax,
    TabEnter,
    TabLeave,
    TabNew,
    TabNewEntered,
    TabClosed,
    TermOpen,
    TermEnter,
    TermLeave,
    TermClose,
    TermRequest,
    TermResponse,
    TextChanged,
    TextChangedI,
    TextChangedP,
    TextChangedT,
    TextYankPost,
    User,
    UserGettingBored,
    VimEnter,
    VimLeave,
    VimLeavePre,
    VimResized,
    VimResume,
    VimSuspend,
    WinClosed,
    WinEnter,
    WinLeave,
    WinNew,
    WinScrolled,
    WinResized,
}

/// A group specification, used in many command options. Groups can be specified as either a string
/// name, or as a numeric ID.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Group {
    Name(String),
    Id(u64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl LogLevel {
    pub fn to_u64(&self) -> u64 {
        match self {
            LogLevel::Trace => 0,
            LogLevel::Debug => 1,
            LogLevel::Info => 2,
            LogLevel::Warn => 3,
            LogLevel::Error => 4,
        }
    }
}

#[derive(
    Debug, Clone, Deserialize, Serialize, PartialEq, Eq, strum::EnumString, strum::Display,
)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Relative {
    Editor,
    Win,
    Cursor,
    Mouse,
}

#[derive(
    Debug, Clone, Deserialize, Serialize, PartialEq, Eq, strum::EnumString, strum::Display,
)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Split {
    Left,
    Right,
    Above,
    Below,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Anchor {
    NW,
    NE,
    SW,
    SE,
}

/// A group specification, used in many command options. Groups can be specified as either a string
/// name, or as a numeric ID.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Text {
    /// A plain string, with a default highlight group
    Plain(String),
    /// A sequence of (text, highlight) tuples
    Highlights(Vec<(String, String)>),
}

/// A group specification, used in many command options. Groups can be specified as either a string
/// name, or as a numeric ID.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum Border {
    None,
    Single,
    Double,
    Rounded,
    Solid,
    Shadow,
    Array(Vec<String>),
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Setters, Default)]
#[setters(strip_option)]
pub struct WindowConf {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "NoneAsEmptyString")]
    pub relative: Option<Relative>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win: Option<Window>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bufpos: Option<(u64, u64)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub col: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focusable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zinc: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<Border>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_pos: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noautocmd: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "NoneAsEmptyString")]
    pub split: Option<Split>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmpv::Value;
    use serde_rmpv::from_value;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_deser_windowconf() {
        // Verify that we can deserialize with all missing fields
        let ret: WindowConf = serde_rmpv::from_value(&Value::Map(vec![])).unwrap();
        assert!(ret.split.is_none());

        // Test that untagged enums deserialize correctly
        let v = Value::Map(vec![
            (Value::from("split"), Value::from("left")),
            (Value::from("title"), Value::from("text")),
            (
                Value::from("footer"),
                Value::Array(vec![Value::Array(vec![
                    Value::from("one"),
                    Value::from("two"),
                ])]),
            ),
        ]);
        let ret: WindowConf = serde_rmpv::from_value(&v).unwrap();
        assert_eq!(ret.split, Some(Split::Left));
        assert_eq!(ret.title, Some(Text::Plain("text".to_string())));
        assert_eq!(
            ret.footer,
            Some(Text::Highlights(vec![(
                "one".to_string(),
                "two".to_string()
            )]))
        );

        // Don't just directly test against the serialization format here, because maps are arrays
        // and order may differ.
        let v2 = serde_rmpv::from_value(&serde_rmpv::to_value(&ret).unwrap()).unwrap();
        assert_eq!(ret, v2);
    }

    #[test]
    fn test_deser_event() {
        let v: Event = from_value(&Value::from("User")).unwrap();
        assert_eq!(v, Event::User);
    }

    #[test]
    fn test_deser_autocmdevent() {
        let evt = Value::Map(vec![
            (Value::from("id"), Value::from(1)),
            (Value::from("event"), Value::from("User")),
            (Value::from("group"), Value::from(1)),
            (
                Value::from("matches"),
                Value::Array(vec![Value::from("*.rs")]),
            ),
            (Value::from("buf"), Value::from(1)),
            (Value::from("file"), Value::from("file")),
            (Value::from("data"), Value::from("data")),
        ]);

        let expected = AutocmdEvent {
            id: 1,
            event: Event::User,
            group: Some(1),
            matches: Some(vec!["*.rs".to_string()]),
            buf: 1,
            file: "file".into(),
            data: Some(rmpv::Value::from("data")),
        };

        let ret: AutocmdEvent = from_value(&evt).unwrap();
        assert_eq!(ret, expected);
    }

    #[test]
    fn test_serialize_group() {
        let group = Group::Name("test".to_string());
        let serialized = serde_rmpv::to_value(&group).unwrap();
        assert_eq!(serialized, Value::String("test".to_string().into()));
        let g2: Group = from_value(&serialized).unwrap();
        assert_eq!(group, g2);

        let group = Group::Id(5);
        let serialized = serde_rmpv::to_value(&group).unwrap();
        assert_eq!(serialized, Value::Integer(5.into()));
        let g2: Group = from_value(&serialized).unwrap();
        assert_eq!(group, g2);
    }
}
