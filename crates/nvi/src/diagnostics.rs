use derive_setters::*;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{error::Result, types::Text, Client};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VirtTextPos {
    Eol,
    Overlay,
    RightAlign,
    Inline,
}

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

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, Default)]
pub enum Severity {
    Error,
    #[default]
    Warn,
    Info,
    Hint,
}

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

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FloatScope {
    Line,
    Buffer,
    Cursor,
}

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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Setters, Default)]
#[setters(strip_option)]
pub struct SeveritySort {
    /// Reverse sort order
    pub reverse: Option<bool>,
}

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
