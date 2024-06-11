/* A compendium of types for working with the Neovim msgrpc API */
use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

pub const BUFFER_EXT_TYPE: i8 = 0;
pub const WINDOW_EXT_TYPE: i8 = 1;
pub const TABPAGE_EXT_TYPE: i8 = 2;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Buffer(#[serde_as(as = "(_, Bytes)")] (i8, Vec<u8>));

impl Buffer {
    pub fn current() -> Self {
        Buffer((BUFFER_EXT_TYPE, vec![0, 0, 0, 0]))
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Window(#[serde_as(as = "(_, Bytes)")] (i8, Vec<u8>));

impl Window {
    pub fn current() -> Self {
        Window((WINDOW_EXT_TYPE, vec![0, 0, 0, 0]))
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TabPage(#[serde_as(as = "(_, Bytes)")] (i8, Vec<u8>));

impl TabPage {
    pub fn current() -> Self {
        TabPage((TABPAGE_EXT_TYPE, vec![0, 0, 0, 0]))
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Group {
    Name(String),
    Id(u64),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Builder, Default)]
#[builder(setter(strip_option), default)]
pub struct CreateAutocmdOpts {
    /// Autocommand group name or ID to match against.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// Pattern to match literally
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<Vec<String>>,
    /// Buffer number for buffer-local autocommands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer: Option<u64>,
    /// Description for docs and troubleshooting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// Vimscript function name to call when the event is triggered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<String>,
    /// Vim command to execute when the event is triggered. Can't be used with callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// Run the command only once
    #[serde(skip_serializing_if = "Option::is_none")]
    pub once: Option<bool>,
    /// Run nested autocommands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Builder, Default)]
#[builder(setter(strip_option), default)]
pub struct ExecAutocmdsOpts {
    /// Autocommand group name or ID to match against.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// Pattern to match literally
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<Vec<String>>,
    /// Buffer number for buffer-local autocommands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer: Option<u64>,
    /// Process the modeline after the autocommands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modeline: Option<bool>,
    /// Data to send to event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AutocmdEvent {
    pub id: u64,
    pub event: Event,
    pub group: Option<u64>,
    pub matches: Option<Vec<String>>,
    pub buf: u64,
    pub file: String,
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use rmpv::Value;
    use serde_rmpv::{from_value, to_value};

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
        let serialized = to_value(&group).unwrap();
        assert_eq!(serialized, Value::String("test".to_string().into()));
        let g2: Group = from_value(&serialized).unwrap();
        assert_eq!(group, g2);

        let group = Group::Id(5);
        let serialized = to_value(&group).unwrap();
        assert_eq!(serialized, Value::Integer(5.into()));
        let g2: Group = from_value(&serialized).unwrap();
        assert_eq!(group, g2);
    }
}
