//! A compendium of types for working with the Neovim msgrpc API
use derive_setters::*;
use serde_derive::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes, NoneAsEmptyString};

use super::opts;
use crate::{client, error::Result};

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
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename = "_ExtStruct")]
pub struct Window(#[serde_as(as = "(_, Bytes)")] (i8, Vec<u8>));

impl std::fmt::Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Window({})", u64::from(self.clone()))
    }
}

impl std::fmt::Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Window({})", u64::from(self.clone()))
    }
}

impl Window {
    pub fn current() -> Self {
        Window((WINDOW_EXT_TYPE, vec![0, 0, 0, 0]))
    }

    pub async fn winhl(&self, c: &client::Client, highlights: Vec<(String, String)>) -> Result<()> {
        let hl_string = highlights
            .into_iter()
            .map(|(from, to)| format!("{}:{}", from, to))
            .collect::<Vec<_>>()
            .join(",");
        self.set(c, "winhl", hl_string).await
    }

    /// Set an option on this window
    pub async fn set<T: serde::Serialize>(
        &self,
        c: &client::Client,
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
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename = "_ExtStruct")]
pub struct TabPage(#[serde_as(as = "(_, Bytes)")] (i8, Vec<u8>));

impl std::fmt::Debug for TabPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TabPage({})", u64::from(self.clone()))
    }
}

impl std::fmt::Display for TabPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TabPage({})", u64::from(self.clone()))
    }
}

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

/// Autocommand events. See here for documentation:
/// https://neovim.io/doc/user/autocmd.html#autocmd-events
#[derive(
    Debug, Clone, PartialEq, Eq, strum::Display, strum::EnumString, Deserialize, Serialize,
)]
pub enum Event {
    /// After adding a new buffer or existing unlisted buffer to the buffer list
    BufAdd,
    /// Before deleting a buffer from the buffer list
    BufDelete,
    /// After entering (visiting, switching-to) a new or existing buffer
    BufEnter,
    /// After changing the name of the current buffer with `:file` or `:saveas`
    BufFilePost,
    /// Before changing the name of the current buffer with `:file` or `:saveas`
    BufFilePre,
    /// Before a buffer becomes hidden
    BufHidden,
    /// Before leaving to another buffer
    BufLeave,
    /// After creating a new buffer or renaming an existing buffer
    BufNew,
    /// After the `modified` value of a buffer has been changed
    BufModifiedSet,
    /// Before reading a file into a buffer
    BufReadPre,
    /// When starting to edit a file that doesn't exist
    BufNewFile,
    /// When starting to edit a new buffer, after reading the file into the buffer
    BufRead,
    /// After reading a file into a buffer
    BufReadPost,
    /// Before a buffer is removed from a window
    BufWinLeave,
    /// Before completely deleting a buffer
    BufWipeout,
    /// Before writing the whole buffer to a file
    BufWrite,
    /// Before writing the whole buffer to a file
    BufWritePre,
    /// After writing the whole buffer to a file
    BufWritePost,
    /// Before writing the whole buffer to a file (should do the writing)
    BufWriteCmd,
    /// Before unloading a buffer, when the text in the buffer is going to be freed
    BufUnload,
    /// State of channel changed, for instance the client of a RPC channel described itself
    ChanInfo,
    /// Just after a channel was opened
    ChanOpen,
    /// When a user command is used but it isn't defined
    CmdUndefined,
    /// After a change was made to the text inside command line
    CmdlineChanged,
    /// After entering the command-line
    CmdlineEnter,
    /// Before leaving the command-line
    CmdlineLeave,
    /// After entering the command-line window
    CmdwinEnter,
    /// Before leaving the command-line window
    CmdwinLeave,
    /// After loading a color scheme
    ColorScheme,
    /// Before loading a color scheme
    ColorSchemePre,
    /// After each time the Insert mode completion menu changed
    CompleteChanged,
    /// After Insert mode completion is done, before clearing completion info
    CompleteDonePre,
    /// After Insert mode completion is done
    CompleteDone,
    /// When the user doesn't press a key for the time specified with `updatetime`
    CursorHold,
    /// Like `CursorHold`, but in Insert mode
    CursorHoldI,
    /// After the cursor was moved in Normal or Visual mode
    CursorMoved,
    /// After the cursor was moved in Insert mode
    CursorMovedI,
    /// After diffs have been updated
    DiffUpdated,
    /// After the current directory was changed
    DirChanged,
    /// When the current directory is going to be changed
    DirChangedPre,
    /// When using `:quit`, `:wq` in a way it makes Vim exit
    ExitPre,
    /// Before appending to a file
    FileAppendCmd,
    /// After appending to a file
    FileAppendPost,
    /// Before appending to a file
    FileAppendPre,
    /// Before making the first change to a read-only file
    FileChangedRO,
    /// When Vim notices that the modification time of a file has changed
    FileChangedShell,
    /// After handling a file that was changed outside of Vim
    FileChangedShellPost,
    /// Before reading a file with a `:read` command
    FileReadCmd,
    /// After reading a file with a `:read` command
    FileReadPost,
    /// Before reading a file with a `:read` command
    FileReadPre,
    /// When the `filetype` option has been set
    FileType,
    /// Before writing to a file, when not writing the whole buffer
    FileWriteCmd,
    /// After writing to a file, when not writing the whole buffer
    FileWritePost,
    /// Before writing to a file, when not writing the whole buffer
    FileWritePre,
    /// After reading a file from a filter command
    FilterReadPost,
    /// Before reading a file from a filter command
    FilterReadPre,
    /// After writing a file for a filter command
    FilterWritePost,
    /// Before writing a file for a filter command
    FilterWritePre,
    /// Nvim got focus
    FocusGained,
    /// Nvim lost focus
    FocusLost,
    /// When a user function is used but it isn't defined
    FuncUndefined,
    /// After a UI connects via `nvim_ui_attach()`
    UIEnter,
    /// After a UI disconnects from Nvim
    UILeave,
    /// When typing `<Insert>` while in Insert or Replace mode
    InsertChange,
    /// When a character is typed in Insert mode, before inserting the char
    InsertCharPre,
    /// Just before starting Insert mode
    InsertEnter,
    /// Just before leaving Insert mode
    InsertLeavePre,
    /// Just after leaving Insert mode
    InsertLeave,
    /// Just before showing the popup menu (under the right mouse button)
    MenuPopup,
    /// After changing the mode
    ModeChanged,
    /// After setting an option
    OptionSet,
    /// Before a quickfix command is run
    QuickFixCmdPre,
    /// After a quickfix command is run
    QuickFixCmdPost,
    /// When using `:quit`, `:wq` or `:qall`, before deciding whether it closes the current window
    QuitPre,
    /// When a reply from a Vim that functions as server was received
    RemoteReply,
    /// After making a search with `n` or `N` if the search wraps around the document
    SearchWrapped,
    /// When a macro starts recording
    RecordingEnter,
    /// When a macro stops recording
    RecordingLeave,
    /// When nothing is pending, going to wait for the user to type a character
    SafeState,
    /// After loading the session file created using `:mksession`
    SessionLoadPost,
    /// After writing a session file by calling `:mksession`
    SessionWritePost,
    /// After executing a shell command with `:!cmd`, `:make` and `:grep`
    ShellCmdPost,
    /// After Nvim receives a signal
    Signal,
    /// After executing a shell command with `:{range}!cmd`, `:w !cmd` or `:r !cmd`
    ShellFilterPost,
    /// Before sourcing a Vimscript/Lua file
    SourcePre,
    /// After sourcing a Vimscript/Lua file
    SourcePost,
    /// When sourcing a Vimscript/Lua file
    SourceCmd,
    /// When trying to load a spell checking file and it can't be found
    SpellFileMissing,
    /// During startup, after reading from stdin into the buffer
    StdinReadPost,
    /// During startup, before reading from stdin into the buffer
    StdinReadPre,
    /// Detected an existing swap file when starting to edit a file
    SwapExists,
    /// When the `syntax` option has been set
    Syntax,
    /// Just after entering a tab page
    TabEnter,
    /// Just before leaving a tab page
    TabLeave,
    /// When creating a new tab page
    TabNew,
    /// After entering a new tab page
    TabNewEntered,
    /// After closing a tab page
    TabClosed,
    /// When a terminal job is starting
    TermOpen,
    /// After entering Terminal-mode
    TermEnter,
    /// After leaving Terminal-mode
    TermLeave,
    /// When a terminal job ends
    TermClose,
    /// When a `:terminal` child process emits an OSC or DCS sequence
    TermRequest,
    /// When Nvim receives an OSC or DCS response from the host terminal
    TermResponse,
    /// After a change was made to the text in the current buffer in Normal mode
    TextChanged,
    /// After a change was made to the text in the current buffer in Insert mode
    TextChangedI,
    /// After a change was made to the text in the current buffer in Insert mode, only when the popup menu is visible
    TextChangedP,
    /// After a change was made to the text in the current buffer in Terminal-mode
    TextChangedT,
    /// Just after a yank or deleting command
    TextYankPost,
    /// Not executed automatically. Use `:doautocmd` to trigger this
    User,
    /// When the user presses the same key 42 times. Just kidding! :-)
    UserGettingBored,
    /// After doing all the startup stuff, including loading vimrc files
    VimEnter,
    /// Before exiting Vim, just after writing the `.shada` file
    VimLeave,
    /// Before exiting Vim, just before writing the `.shada` file
    VimLeavePre,
    /// After the Vim window was resized
    VimResized,
    /// After Nvim resumes from suspend state
    VimResume,
    /// Before Nvim enters suspend state
    VimSuspend,
    /// When closing a window, just before it is removed from the window layout
    WinClosed,
    /// After entering another window
    WinEnter,
    /// Before leaving a window
    WinLeave,
    /// When a new window was created
    WinNew,
    /// After any window in the current tab page scrolled the text
    WinScrolled,
    /// After a window in the current tab page changed width or height
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

impl Group {
    pub fn to_lua_arg(&self) -> String {
        match self {
            Group::Name(s) => s.clone(),
            Group::Id(i) => format!("'{}'", i),
        }
    }
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

/// Plain text, or a sequence of highlights.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Text {
    /// A plain string, with a default highlight group
    Plain(String),
    /// A sequence of (text, highlight) tuples
    Highlights(Vec<(String, String)>),
}

/// A border specification.
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

/// Window configuration options.
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
    fn test_event_from_str() {
        use std::str::FromStr;
        assert_eq!(Event::from_str("User").unwrap(), Event::User);
        assert_eq!(Event::from_str("BufAdd").unwrap(), Event::BufAdd);
        assert_eq!(Event::from_str("ColorScheme").unwrap(), Event::ColorScheme);
        assert!(Event::from_str("InvalidEvent").is_err());
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
