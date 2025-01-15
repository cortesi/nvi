//! Utilities for defining highlight groups in Neovim.
//!
//! This module provides types and functions for creating and managing Neovim highlight groups
//! with type safety and validation. It supports both direct highlight definitions and links
//! between groups.

use crate::error::Result;
use crate::nvim::opts::SetHl;
use derive_setters::*;
use std::fmt;

use crate::Color;

/// Create a full highlight name by joining a prefix and highlight name with validation.
///
/// Both the prefix and name are validated according to Neovim highlight group naming rules.
pub fn full_name(prefix: &str, name: &str) -> Result<String> {
    check_group_name(prefix)?;
    check_group_name(name)?;
    Ok(format!("{}{}", prefix, name))
}

impl fmt::Display for Hl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fg_str = self.fg.as_ref().map(|c| format!("fg: {}", c.rgb_hex()));
        let bg_str = self.bg.as_ref().map(|c| format!("bg: {}", c.rgb_hex()));
        let bold_str = if self.bold.unwrap_or(false) {
            Some("bold")
        } else {
            None
        };
        let italic_str = if self.italic.unwrap_or(false) {
            Some("italic")
        } else {
            None
        };
        let underline_str = if self.underline.unwrap_or(false) {
            Some("underline")
        } else {
            None
        };
        let reverse_str = if self.reverse.unwrap_or(false) {
            Some("reverse")
        } else {
            None
        };
        let strikethrough_str = if self.strikethrough.unwrap_or(false) {
            Some("strikethrough")
        } else {
            None
        };

        let mut parts: Vec<String> = vec![];

        if let Some(fg) = fg_str {
            parts.push(fg);
        }
        if let Some(bg) = bg_str {
            parts.push(bg);
        }
        if let Some(bold) = bold_str {
            parts.push(bold.into());
        }
        if let Some(italic) = italic_str {
            parts.push(italic.into());
        }
        if let Some(underline) = underline_str {
            parts.push(underline.into());
        }
        if let Some(reverse) = reverse_str {
            parts.push(reverse.into());
        }
        if let Some(strikethrough) = strikethrough_str {
            parts.push(strikethrough.into());
        }

        write!(f, "{}", parts.join(", "))
    }
}

/// Validates that a string is a valid RGB color specification of the form "#xxxxxx".
///
/// The color must start with '#' and be followed by exactly 6 hexadecimal digits.
pub fn validate_color(color: &str) -> Result<()> {
    if !color.starts_with('#') || color.len() != 7 {
        return Err(crate::error::Error::User(format!(
            "Invalid color format '{}': must be '#' followed by 6 hex digits",
            color
        )));
    }
    if !color[1..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(crate::error::Error::User(format!(
            "Invalid color format '{}': must contain only hex digits after '#'",
            color
        )));
    }
    Ok(())
}

/// Validates a highlight group name according to Neovim rules.
/// Group names must consist of ASCII letters, digits, underscores, dots, hyphens, or `@`,
/// and must be no longer than 200 bytes.
///
/// :help group-name
pub(crate) fn check_group_name(name: &str) -> Result<()> {
    if name.len() > 200 {
        return Err(crate::error::Error::User(
            "Highlight group name exceeds 200 bytes".into(),
        ));
    }

    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '-' || c == '@')
    {
        return Err(crate::error::Error::User(
            "Highlight group name contains invalid characters".into(),
        ));
    }

    Ok(())
}

/// A highlight group definition supporting both terminal and GUI attributes.
///
/// This structure represents a subset of Neovim highlight attributes that work consistently
/// across both terminal and GUI environments. It omits legacy attributes like "standout"
/// and GUI-specific features like underdotted text.
#[derive(Debug, Clone, PartialEq, Eq, Setters, Default)]
#[setters(strip_option, into)]
pub struct Hl {
    #[setters(skip)]
    pub fg: Option<Color>,
    #[setters(skip)]
    pub bg: Option<Color>,

    // Text attributes omit anachronisms like "standout", and things that are GUI-only like blend,
    // underdotted, etc..
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub reverse: Option<bool>,
    pub strikethrough: Option<bool>,
}

impl Hl {
    /// Creates a new empty highlight definition.
    pub fn new() -> Hl {
        Hl {
            fg: None,
            bg: None,
            bold: None,
            italic: None,
            underline: None,
            reverse: None,
            strikethrough: None,
        }
    }

    /// Sets the foreground color of the highlight.
    ///
    /// The color can be specified as any type that can be converted into a `Color`.
    pub fn fg<T>(mut self, fg: T) -> Result<Self>
    where
        T: TryInto<Color>,
        T::Error: std::fmt::Display,
    {
        self.fg = Some(
            fg.try_into()
                .map_err(|e| crate::error::Error::User(e.to_string()))?,
        );
        Ok(self)
    }

    /// Sets the background color of the highlight.
    ///
    /// The color can be specified as any type that can be converted into a `Color`.
    pub fn bg<T>(mut self, bg: T) -> Result<Self>
    where
        T: TryInto<Color>,
        T::Error: std::fmt::Display,
    {
        self.bg = Some(
            bg.try_into()
                .map_err(|e| crate::error::Error::User(e.to_string()))?,
        );
        Ok(self)
    }

    /// Converts this highlight definition to a Neovim-compatible SetHl structure.
    pub fn to_sethl(&self) -> crate::nvim::opts::SetHl {
        SetHl {
            fg: self.fg.map(|c| c.rgb_hex()),
            bg: self.bg.map(|c| c.rgb_hex()),
            bold: self.bold,
            italic: self.italic,
            underline: self.underline,
            reverse: self.reverse,
            strikethrough: self.strikethrough,
            sp: None,
            blend: None,
            standout: None,
            undercurl: None,
            underdouble: None,
            underdotted: None,
            underdashed: None,
            nocombine: None,
            link: None,
            default: None,
            ctermfg: None,
            ctermbg: None,
            cterm: None,
            force: None,
        }
    }
}

/// A collection of highlight definitions and links.
///
/// This structure manages a set of highlight groups and links between them, allowing
/// for bulk creation and management of related highlights.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Highlights {
    pub highlights: Vec<(String, Hl)>,
    pub links: Vec<(String, String)>,
}

impl Highlights {
    /// Creates a new empty collection of highlights.
    pub fn new() -> Highlights {
        Highlights {
            highlights: Vec::new(),
            links: Vec::new(),
        }
    }

    /// Returns true if this collection contains no highlights or links.
    pub fn is_empty(&self) -> bool {
        self.highlights.is_empty() && self.links.is_empty()
    }

    /// Adds a highlight definition to the collection.
    ///
    /// The highlight name is validated according to Neovim naming rules.
    pub fn hl(mut self, name: &str, h: Hl) -> Self {
        check_group_name(name).expect("Invalid highlight group name");
        self.highlights.push((name.into(), h));
        self
    }

    /// Creates a link from one highlight group to another.
    ///
    /// Both group names are validated according to Neovim naming rules.
    pub fn link(mut self, new_group: &str, existing_group: &str) -> Self {
        check_group_name(new_group).expect("Invalid highlight group name");
        check_group_name(existing_group).expect("Invalid highlight group name");
        self.links.push((new_group.into(), existing_group.into()));
        self
    }

    /// Creates all highlights and links in the collection.
    ///
    /// The client's name is prepended to all highlight group names to provide namespacing.
    /// This allows the same highlight definitions to be created with different prefixes.
    pub async fn create(&self, client: &crate::Client) -> crate::error::Result<()> {
        let ns_id = 0; // Use the default namespace

        // Create highlights
        for (name, opts) in &self.highlights {
            client
                .nvim
                .set_hl(ns_id, &client.hl_name(name)?, opts.to_sethl())
                .await?;
        }

        // Create links
        for (new_group, existing_group) in &self.links {
            client
                .nvim
                .set_hl(
                    ns_id,
                    &client.hl_name(new_group)?,
                    SetHl {
                        link: Some(existing_group.to_string()),
                        ..Default::default()
                    },
                )
                .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::NviTest;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_validate_color() {
        // Valid colors
        assert!(validate_color("#000000").is_ok());
        assert!(validate_color("#FFFFFF").is_ok());
        assert!(validate_color("#ff00ff").is_ok());
        assert!(validate_color("#1a2b3c").is_ok());

        // Invalid colors
        let err = validate_color("000000").unwrap_err().to_string(); // missing #
        assert!(err.contains("must be '#' followed by"));

        let err = validate_color("#00000").unwrap_err().to_string(); // too short
        assert!(err.contains("must be '#' followed by"));

        let err = validate_color("#0000000").unwrap_err().to_string(); // too long
        assert!(err.contains("must be '#' followed by"));

        let err = validate_color("#gggggg").unwrap_err().to_string(); // invalid hex
        assert!(err.contains("must contain only hex digits"));

        let err = validate_color("#00000g").unwrap_err().to_string(); // invalid hex
        assert!(err.contains("must contain only hex digits"));

        let err = validate_color("#invalid").unwrap_err().to_string(); // invalid hex
        assert!(err.contains("must be '#' followed by"));
    }

    #[test]
    fn test_check_group_name() {
        // Valid names
        assert!(check_group_name("Normal").is_ok());
        assert!(check_group_name("Test_Group").is_ok());
        assert!(check_group_name("Test.Group").is_ok());
        assert!(check_group_name("Test-Group").is_ok());
        assert!(check_group_name("@text").is_ok());

        // Invalid names
        assert!(check_group_name("Test Group").is_err()); // space
        assert!(check_group_name("Test*Group").is_err()); // special char
        assert!(check_group_name("Test💡Group").is_err()); // unicode
        assert!(check_group_name(&"a".repeat(201)).is_err()); // too long
    }

    #[test]
    fn test_group_new() {
        let g = Hl::new().bold(true).italic(true);
        assert_eq!(g.fg, None);
        assert_eq!(g.bold, Some(true));
        assert_eq!(g.italic, Some(true));
        assert_eq!(g.underline, None);

        let _ = Highlights::new()
            .hl("foo", Hl::new().bold(true).italic(true))
            .hl(
                "bar",
                Hl::new().fg("#ff0000").unwrap().bg("#0000ff").unwrap(),
            )
            .link("foo", "bar");
    }

    #[test]
    fn test_invalid_color() {
        let hl = Hl::new().fg("invalid");
        assert!(hl.is_err());

        let hl = Hl::new().bg("invalid");
        assert!(hl.is_err());
    }

    #[tokio::test]
    async fn test_highlight_creation() {
        let test = NviTest::builder().run().await.unwrap();
        let highlights = Highlights::new()
            .hl("TestHl", Hl::new().fg("#ff0000").unwrap().bold(true))
            .link("TestLink", "TestHl");

        highlights.create(&test.client).await.unwrap();

        // Get the highlight definitions
        let hl: std::collections::HashMap<String, crate::Value> = test
            .client
            .nvim
            .get_hl(0, std::collections::HashMap::new())
            .await
            .unwrap();

        // Check the highlight group
        let test_hl = hl.get("test_TestHl").unwrap().as_map().unwrap();
        test_hl
            .iter()
            .find(|(k, _)| k.as_str().unwrap() == "fg")
            .unwrap();
        test_hl
            .iter()
            .find(|(k, _)| k.as_str().unwrap() == "bold")
            .unwrap();

        // Check the link
        let test_link = hl.get("test_TestLink").unwrap().as_map().unwrap();
        let link = &test_link
            .iter()
            .find(|(k, _)| k.as_str().unwrap() == "link")
            .unwrap()
            .1;
        assert_eq!(link.as_str().unwrap(), "TestHl");

        test.finish().await.unwrap();
    }
}

