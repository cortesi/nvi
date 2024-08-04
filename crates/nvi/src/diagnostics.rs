//! Diagnostics
//!
//! This module provides structures and functions for working with diagnostics in Neovim.
//! It includes types for configuring how diagnostics are displayed (e.g., virtual text,
//! signs, floating windows) and functions for setting and resetting diagnostics.

use derive_setters::*;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    error::{Error, Result},
    types::Text,
    Client,
};
use crate::Value;

/// Options for getting diagnostics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOpts {
    /// Limit diagnostics to one or more namespaces.
    pub namespace: Option<Vec<i64>>,
    /// Limit diagnostics to those spanning the specified line number.
    pub lnum: Option<u64>,
    /// Filter diagnostics by severity.
    pub severity: Option<SeverityFilter>,
}

/// Options for jumping to diagnostics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpOpts {
    /// Limit diagnostics to one or more namespaces.
    pub namespace: Option<Vec<i64>>,
    /// Filter diagnostics by severity.
    pub severity: Option<SeverityFilter>,
    /// The specific diagnostic to jump to.
    pub diagnostic: Option<Diagnostic>,
    /// The number of diagnostics to move by.
    pub count: Option<i64>,
    /// Cursor position as a (row, col) tuple.
    pub pos: Option<(u64, u64)>,
    /// Whether to loop around file or not.
    pub wrap: Option<bool>,
    /// Whether to open a float after moving.
    pub float: Option<JumpFloat>,
    /// Window ID.
    pub win_id: Option<u64>,
}

/// Position for a floating window.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FloatPos {
    /// Line number.
    Line(u64),
    /// (Row, Column) position.
    Pos(u64, u64),
}

/// Options for floating windows displaying diagnostics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatOpts {
    /// Buffer number to show diagnostics from.
    pub bufnr: Option<u64>,
    /// Limit diagnostics to the given namespace.
    pub namespace: Option<i64>,
    /// Show diagnostics from the whole buffer, current line, or cursor position.
    pub scope: Option<FloatScope>,
    /// Position to use rather than the cursor position.
    pub pos: Option<FloatPos>,
    /// Sort diagnostics by severity.
    pub severity_sort: Option<SeveritySort>,
    /// Filter diagnostics by severity.
    pub severity: Option<SeverityFilter>,
    /// String to use as the header for the floating window.
    pub header: Option<Text>,
    /// Include the diagnostic source in the message.
    pub source: Option<bool>,
    /// Function to format the diagnostic message.
    pub format: Option<String>,
    /// Prefix each diagnostic in the floating window.
    pub prefix: Option<Text>,
    /// Suffix each diagnostic in the floating window.
    pub suffix: Option<Text>,
    /// Unique identifier for the window.
    pub focus_id: Option<String>,
    /// Border style for the floating window.
    pub border: Option<String>,
}

/// Options for adding diagnostics to the location list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetLoclistOpts {
    /// Only add diagnostics from the given namespace.
    pub namespace: Option<i64>,
    /// Window number to set location list for.
    pub winnr: Option<i64>,
    /// Open the location list after setting.
    pub open: Option<bool>,
    /// Title of the location list.
    pub title: Option<String>,
    /// Filter diagnostics by severity.
    pub severity: Option<SeverityFilter>,
}

/// Options for adding diagnostics to the quickfix list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetQfListOpts {
    /// Only add diagnostics from the given namespace.
    pub namespace: Option<i64>,
    /// Open quickfix list after setting.
    pub open: Option<bool>,
    /// Title of quickfix list.
    pub title: Option<String>,
    /// Filter diagnostics by severity.
    pub severity: Option<SeverityFilter>,
}

/// Represents the position of virtual text in the editor.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VirtTextPos {
    Eol,
    Overlay,
    RightAlign,
    Inline,
}

/// Represents a filter for diagnostic severity.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum SeverityFilter {
    Single(Severity),
    Range {
        min: Option<Severity>,
        max: Option<Severity>,
    },
    List(Vec<Severity>),
}

/// Represents the severity levels of diagnostics.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, Default)]
pub enum Severity {
    Error,
    #[default]
    Warn,
    Info,
    Hint,
}

/// Represents the source of virtual text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtualTextSource {
    True,
    False,
    IfMany,
}

impl serde::Serialize for VirtualTextSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            VirtualTextSource::True => serializer.serialize_bool(true),
            VirtualTextSource::False => serializer.serialize_bool(false),
            VirtualTextSource::IfMany => serializer.serialize_str("if_many"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for VirtualTextSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum VirtualTextSourceHelper {
            Bool(bool),
            Str(String),
        }

        match VirtualTextSourceHelper::deserialize(deserializer)? {
            VirtualTextSourceHelper::Bool(true) => Ok(VirtualTextSource::True),
            VirtualTextSourceHelper::Bool(false) => Ok(VirtualTextSource::False),
            VirtualTextSourceHelper::Str(s) if s == "if_many" => Ok(VirtualTextSource::IfMany),
            _ => Err(serde::de::Error::custom(
                "Invalid value for VirtualTextSource",
            )),
        }
    }
}

/// Represents the scope of a floating window.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FloatScope {
    Line,
    Buffer,
    Cursor,
}

/// Configuration options for virtual text in diagnostics.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Setters, Default)]
#[setters(strip_option)]
pub struct VirtualText {
    /// Only show virtual text for diagnostics matching the given severity
    pub severity: Option<SeverityFilter>,
    /// Include the diagnostic source in virtual text
    pub source: Option<VirtualTextSource>,
    /// Amount of empty spaces inserted at the beginning of the virtual text
    pub spacing: Option<u32>,
    /// Prepend diagnostic message with prefix
    pub prefix: Option<String>,
    /// Append diagnostic message with suffix
    pub suffix: Option<String>,
    /// Function to format the diagnostic message
    pub format: Option<String>, // This should be a function in Neovim, but we'll use a string for now
    /// Highlight mode for virtual text
    pub hl_mode: Option<String>,
    /// List of [text, highlight_group] pairs
    pub virt_text: Option<Vec<(String, String)>>,
    /// Position of virtual text
    pub virt_text_pos: Option<VirtTextPos>,
    /// Position of virtual text in window columns
    pub virt_text_win_col: Option<u32>,
    /// Hide virtual text when there is no more space
    pub virt_text_hide: Option<bool>,
}

/// Configuration options for diagnostic signs.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Setters, Default)]
#[setters(strip_option)]
pub struct Signs {
    /// Only show signs for diagnostics matching the given severity
    pub severity: Option<SeverityFilter>,
    /// Base priority to use for signs
    pub priority: Option<u32>,
    /// Mapping of severity to sign text
    pub text: Option<HashMap<Severity, String>>,
    /// Mapping of severity to highlight group for line number
    pub numhl: Option<HashMap<Severity, String>>,
    /// Mapping of severity to highlight group for whole line
    pub linehl: Option<HashMap<Severity, String>>,
}

/// Configuration options for floating windows.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Setters, Default)]
#[setters(strip_option)]
pub struct Float {
    /// Buffer number to show diagnostics from
    pub bufnr: Option<u32>,
    /// Limit diagnostics to the given namespace
    pub namespace: Option<u32>,
    /// Show diagnostics from 'line', 'buffer', or 'cursor'
    pub scope: Option<FloatScope>,
    /// Position to use instead of cursor position
    pub pos: Option<(u32, u32)>,
    /// Sort diagnostics by severity
    pub severity_sort: Option<SeveritySort>,
    /// Filter diagnostics by severity
    pub severity: Option<SeverityFilter>,
    /// String to use as the header for the floating window
    pub header: Option<Text>,
    /// Include the diagnostic source in the message
    pub source: Option<bool>,
    /// Function to format the diagnostic message
    pub format: Option<String>, // This should be a function in Neovim, but we'll use a string for now
    /// Prefix each diagnostic in the floating window
    pub prefix: Option<Text>,
    /// Suffix each diagnostic in the floating window
    pub suffix: Option<Text>,
    /// Unique identifier for the window
    pub focus_id: Option<String>,
    /// Border style for the floating window
    pub border: Option<String>,
}

/// Represents options for jumping to diagnostics with optional floating window.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(variant_size_differences)]
#[allow(clippy::large_enum_variant)]
pub enum JumpFloat {
    True,
    False,
    Options(Float),
}

impl serde::Serialize for JumpFloat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            JumpFloat::True => serializer.serialize_bool(true),
            JumpFloat::False => serializer.serialize_bool(false),
            JumpFloat::Options(float) => float.serialize(serializer),
        }
    }
}

impl<'de> serde::Deserialize<'de> for JumpFloat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        #[allow(variant_size_differences)]
        #[allow(clippy::large_enum_variant)]
        enum JumpFloatHelper {
            Bool(bool),
            Float(Float),
        }

        match JumpFloatHelper::deserialize(deserializer)? {
            JumpFloatHelper::Bool(true) => Ok(JumpFloat::True),
            JumpFloatHelper::Bool(false) => Ok(JumpFloat::False),
            JumpFloatHelper::Float(float) => Ok(JumpFloat::Options(float)),
        }
    }
}

/// Configuration options for jumping to diagnostics.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Setters, Default)]
#[setters(strip_option)]
pub struct Jump {
    /// Whether to open float when jumping
    pub float: Option<JumpFloat>,
    /// Whether to wrap around file
    pub wrap: Option<bool>,
    /// Filter diagnostics by severity when jumping
    pub severity: Option<SeverityFilter>,
}

/// Configuration options for sorting diagnostics by severity.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Setters, Default)]
#[setters(strip_option)]
pub struct SeveritySort {
    /// Reverse sort order
    pub reverse: Option<bool>,
}

/// Main configuration options for diagnostics.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Setters, Default)]
#[setters(strip_option)]
pub struct DiagnosticOpts {
    /// Options for virtual text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_text: Option<VirtualText>,
    /// Options for diagnostic signs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signs: Option<Signs>,
    /// Options for floating windows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float: Option<Float>,
    /// Update diagnostics in Insert mode
    pub update_in_insert: bool,
    /// Sort diagnostics by severity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_sort: Option<SeveritySort>,
    /// Options for jumping to diagnostics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump: Option<Jump>,
}

/// Represents a single diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, Setters)]
#[setters(strip_option)]
pub struct Diagnostic {
    /// Buffer number
    pub bufnr: u64,
    /// The starting line of the diagnostic (0-indexed)
    pub lnum: u64,
    /// The final line of the diagnostic (0-indexed)
    pub end_lnum: u64,
    /// The starting column of the diagnostic (0-indexed)
    pub col: u64,
    /// The final column of the diagnostic (0-indexed)
    pub end_col: u64,
    /// The severity of the diagnostic
    pub severity: Severity,
    /// The diagnostic text
    pub message: String,
    /// The source of the diagnostic
    pub source: String,
    /// The diagnostic code
    pub code: String,
    /// Arbitrary data plugins can add
    pub user_data: Option<crate::Value>,
    /// The namespace of the diagnostic
    pub namespace: u64,
}

/// Configure diagnostic options globally or for a specific namespace.
///
/// * `opts` - The diagnostic options to set.
/// * `namespace` - Optional namespace. If None, configures global options.
pub async fn diagnostic_config<T>(
    c: &mut Client,
    opts: DiagnosticOpts,
    namespace: Option<i64>,
) -> Result<()> {
    let namespace = namespace.unwrap_or(-1);
    c.nvim
        .exec_lua(
            "vim.diagnostic.config(...)",
            vec![
                serde_rmpv::to_value(&opts)?,
                serde_rmpv::to_value(&namespace)?,
            ],
        )
        .await?;
    Ok(())
}

/// Get current diagnostics for a buffer.
///
/// * `bufnr` - Optional buffer number. If None, gets diagnostics for all buffers.
/// * `opts` - Optional settings to filter the diagnostics.
///
/// Returns a vector of Diagnostic structs.
pub async fn diagnostic_get<T>(
    c: &mut Client,
    bufnr: Option<T>,
    opts: Option<GetOpts>,
) -> Result<Vec<Diagnostic>>
where
    T: Into<u64>,
{
    let bufnr: Option<u64> = bufnr.map(Into::into);
    let result: Vec<Diagnostic> = c
        .nvim
        .exec_lua(
            "return vim.diagnostic.get(...)",
            vec![serde_rmpv::to_value(&bufnr)?, serde_rmpv::to_value(&opts)?],
        )
        .await?
        .as_array()
        .ok_or_else(|| Error::User("Expected array".into()))?
        .iter()
        .map(|v| serde_rmpv::from_value::<Diagnostic>(v).unwrap())
        .collect::<Vec<_>>();
    Ok(result)
}

/// Hide currently displayed diagnostics.
///
/// * `namespace` - Optional namespace to hide diagnostics for.
/// * `bufnr` - Optional buffer number to hide diagnostics in.
pub async fn diagnostic_hide(
    c: &mut Client,
    namespace: Option<i64>,
    bufnr: Option<u64>,
) -> Result<()> {
    c.nvim
        .exec_lua(
            "vim.diagnostic.hide(...)",
            vec![
                serde_rmpv::to_value(&namespace)?,
                serde_rmpv::to_value(&bufnr)?,
            ],
        )
        .await?;
    Ok(())
}

/// Display diagnostics for the given namespace and buffer.
///
/// * `namespace` - Optional namespace to show diagnostics for.
/// * `bufnr` - Optional buffer number to show diagnostics in.
/// * `diagnostics` - Optional vector of Diagnostic structs to display.
/// * `opts` - Optional display options.
pub async fn diagnostic_show<T>(
    c: &mut Client,
    namespace: Option<i64>,
    bufnr: Option<T>,
    diagnostics: Option<Vec<Diagnostic>>,
    opts: Option<DiagnosticOpts>,
) -> Result<()>
where
    T: Into<u64>,
{
    let bufnr: Option<u64> = bufnr.map(Into::into);
    c.nvim
        .exec_lua(
            "vim.diagnostic.show(...)",
            vec![
                serde_rmpv::to_value(&namespace)?,
                serde_rmpv::to_value(&bufnr)?,
                serde_rmpv::to_value(&diagnostics)?,
                serde_rmpv::to_value(&opts)?,
            ],
        )
        .await?;
    Ok(())
}

/// Get the next diagnostic closest to the cursor position.
///
/// * `opts` - Optional jump options.
pub async fn diagnostic_get_next(
    c: &mut Client,
    opts: Option<JumpOpts>,
) -> Result<Option<Diagnostic>> {
    let result: Value = c
        .nvim
        .exec_lua(
            "return vim.diagnostic.get_next(...)",
            vec![serde_rmpv::to_value(&opts)?],
        )
        .await?;

    match &result {
        Value::Nil => Ok(None),
        Value::Map(_) => {
            let diagnostic: Diagnostic = serde_rmpv::from_value(&result)?;
            Ok(Some(diagnostic))
        }
        _ => Err(Error::User(
            "Unexpected return type from diagnostic_get_next".to_string(),
        )),
    }
}

/// Get the previous diagnostic closest to the cursor position.
///
/// * `opts` - Optional jump options.
///
/// Returns an Option<Diagnostic>. None if no previous diagnostic is found.
pub async fn diagnostic_get_prev(
    c: &mut Client,
    opts: Option<JumpOpts>,
) -> Result<Option<Diagnostic>> {
    let result: Value = c
        .nvim
        .exec_lua(
            "return vim.diagnostic.get_prev(...)",
            vec![serde_rmpv::to_value(&opts)?],
        )
        .await?;

    match &result {
        Value::Nil => Ok(None),
        Value::Map(_) => {
            let diagnostic: Diagnostic = serde_rmpv::from_value(&result)?;
            Ok(Some(diagnostic))
        }
        _ => Err(Error::User(
            "Unexpected return type from diagnostic_get_prev".to_string(),
        )),
    }
}

/// Show diagnostics in a floating window.
///
/// * `opts` - Optional float options.
///
/// Returns a tuple of two Option<u64>: (float_bufnr, winid).
pub async fn diagnostic_open_float(
    c: &mut Client,
    opts: Option<FloatOpts>,
) -> Result<(Option<u64>, Option<u64>)> {
    let result: Value = c
        .nvim
        .exec_lua(
            "return vim.diagnostic.open_float(...)",
            vec![serde_rmpv::to_value(&opts)?],
        )
        .await?;

    match &result {
        Value::Array(arr) if arr.len() == 2 => {
            let float_bufnr: Option<u64> = match &arr[0] {
                Value::Integer(n) => Some(
                    n.as_u64()
                        .ok_or(Error::User("Invalid float_bufnr".to_string()))?,
                ),
                Value::Nil => None,
                _ => return Err(Error::User("Unexpected type for float_bufnr".to_string())),
            };
            let winid: Option<u64> = match &arr[1] {
                Value::Integer(n) => {
                    Some(n.as_u64().ok_or(Error::User("Invalid winid".to_string()))?)
                }
                Value::Nil => None,
                _ => return Err(Error::User("Unexpected type for winid".to_string())),
            };
            Ok((float_bufnr, winid))
        }
        _ => Err(Error::User(
            "Unexpected return type from diagnostic_open_float".to_string(),
        )),
    }
}

/// Add buffer diagnostics to the location list.
///
/// * `opts` - Options for setting the location list.
pub async fn diagnostic_setloclist(c: &mut Client, opts: SetLoclistOpts) -> Result<()> {
    c.nvim
        .exec_lua(
            "vim.diagnostic.setloclist(...)",
            vec![serde_rmpv::to_value(&opts)?],
        )
        .await?;
    Ok(())
}

/// Add all diagnostics to the quickfix list.
///
/// * `opts` - Options for setting the quickfix list.
pub async fn diagnostic_setqflist(c: &mut Client, opts: SetQfListOpts) -> Result<()> {
    c.nvim
        .exec_lua(
            "vim.diagnostic.setqflist(...)",
            vec![serde_rmpv::to_value(&opts)?],
        )
        .await?;
    Ok(())
}

/// Sets diagnostics for a specific buffer and namespace.
///
/// * `namespace` - The diagnostic namespace.
/// * `bufnr` - Buffer number to set diagnostics for.
/// * `diagnostics` - Vector of Diagnostic structs to set.
/// * `opts` - Display options for the diagnostics.
pub async fn diagnostic_set<T>(
    c: &mut Client,
    namespace: i64,
    bufnr: T,
    diagnostics: Vec<Diagnostic>,
    opts: DiagnosticOpts,
) -> Result<()>
where
    T: Into<u64>,
{
    let bufnr: u64 = bufnr.into();
    c.nvim
        .exec_lua(
            "vim.diagnostic.set(...)",
            vec![
                serde_rmpv::to_value(&namespace)?,
                serde_rmpv::to_value(&bufnr)?,
                serde_rmpv::to_value(&diagnostics)?,
                serde_rmpv::to_value(&opts)?,
            ],
        )
        .await?;
    Ok(())
}

/// Resets diagnostics for a specific buffer and namespace.
///
/// * `namespace` - The diagnostic namespace to reset.
/// * `bufnr` - Buffer number to reset diagnostics for.
pub async fn diagnostic_reset(c: &mut Client, namespace: i64, bufnr: u64) -> Result<()> {
    c.nvim
        .exec_lua(
            "vim.diagnostic.reset(...)",
            vec![namespace.into(), bufnr.into()],
        )
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_idempotence<T>(value: T)
    where
        T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq,
    {
        let serialized = serde_rmpv::to_value(&value).unwrap();
        let deserialized: T = serde_rmpv::from_value(&serialized).unwrap();
        let reserialized = serde_rmpv::to_value(&deserialized).unwrap();

        assert_eq!(value, deserialized);
        assert_eq!(serialized, reserialized);
    }

    #[test]
    fn test_virt_text_pos_idempotence() {
        test_idempotence(VirtTextPos::Eol);
        test_idempotence(VirtTextPos::Overlay);
        test_idempotence(VirtTextPos::RightAlign);
        test_idempotence(VirtTextPos::Inline);
    }

    #[test]
    fn test_severity_filter_idempotence() {
        test_idempotence(SeverityFilter::Single(Severity::Error));
        test_idempotence(SeverityFilter::Range {
            min: Some(Severity::Warn),
            max: Some(Severity::Info),
        });
        test_idempotence(SeverityFilter::List(vec![Severity::Error, Severity::Hint]));
        test_idempotence(SeverityFilter::Single(Severity::Error));
    }

    #[test]
    fn test_virtual_text_source_idempotence() {
        test_idempotence(VirtualTextSource::True);
        test_idempotence(VirtualTextSource::False);
        test_idempotence(VirtualTextSource::IfMany);
    }

    #[test]
    fn test_virtual_text_idempotence() {
        let vt = VirtualText {
            severity: Some(SeverityFilter::Single(Severity::Warn)),
            source: Some(VirtualTextSource::IfMany),
            spacing: Some(2),
            prefix: Some("Prefix".to_string()),
            suffix: Some("Suffix".to_string()),
            format: Some("Format".to_string()),
            hl_mode: Some("HlMode".to_string()),
            virt_text: Some(vec![("Text".to_string(), "Group".to_string())]),
            virt_text_pos: Some(VirtTextPos::Eol),
            virt_text_win_col: Some(10),
            virt_text_hide: Some(true),
        };
        test_idempotence(vt);
    }

    #[test]
    fn test_diagnostic_opts_idempotence() {
        let opts = DiagnosticOpts {
            virtual_text: Some(VirtualText::default()),
            signs: Some(Signs::default()),
            float: Some(Float::default()),
            update_in_insert: true,
            severity_sort: Some(SeveritySort {
                reverse: Some(true),
            }),
            jump: Some(Jump {
                float: Some(JumpFloat::Options(Float::default())),
                wrap: Some(true),
                severity: Some(SeverityFilter::Single(Severity::Error)),
            }),
        };
        test_idempotence(opts);
    }

    #[test]
    fn test_diagnostic_idempotence() {
        let diagnostic = Diagnostic {
            bufnr: 1,
            lnum: 10,
            end_lnum: 11,
            col: 5,
            end_col: 15,
            severity: Severity::Error,
            message: "Test diagnostic".to_string(),
            source: "test_source".to_string(),
            code: "E001".to_string(),
            user_data: None,
            namespace: 0,
        };
        test_idempotence(diagnostic);
    }
}
