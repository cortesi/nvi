use crate::error::Result;
use crate::nvim::opts::SetHl;
use derive_setters::*;

use crate::Color;

/// Create a full highlight name by joining a prefix and highlight name with validation.
pub fn full_name(prefix: &str, name: &str) -> Result<String> {
    check_group_name(prefix)?;
    check_group_name(name)?;
    Ok(format!("{}{}", prefix, name))
}

/// Validates that a string is a valid RGB color specification of the form "#xxxxxx"
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
pub fn check_group_name(name: &str) -> Result<()> {
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

/// We support the subset of highlight attributes that are common to both terminal and GUI, and
/// omit anachronisms like "standout".
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

    pub fn fg(mut self, fg: &str) -> Result<Self> {
        validate_color(fg)?;
        self.fg = Some(fg.try_into().map_err(crate::error::Error::User)?);
        Ok(self)
    }

    pub fn bg(mut self, bg: &str) -> Result<Self> {
        validate_color(bg)?;
        self.bg = Some(bg.try_into().map_err(crate::error::Error::User)?);
        Ok(self)
    }

    /// Convert to opts::SetHl, copying only the fields that are present in both structs
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Highlights {
    pub highlights: Vec<(String, Hl)>,
    pub links: Vec<(String, String)>,
}

impl Highlights {
    pub fn new() -> Highlights {
        Highlights {
            highlights: Vec::new(),
            links: Vec::new(),
        }
    }

    pub fn hl(mut self, name: &str, h: Hl) -> Self {
        check_group_name(name).expect("Invalid highlight group name");
        self.highlights.push((name.into(), h));
        self
    }

    pub fn link(mut self, new_group: &str, existing_group: &str) -> Self {
        check_group_name(new_group).expect("Invalid highlight group name");
        check_group_name(existing_group).expect("Invalid highlight group name");
        self.links.push((new_group.into(), existing_group.into()));
        self
    }

    /// Create all highlights and links in this collection
    ///
    /// `prefix` is prepended to all highlight group names. This is useful when
    /// the same highlight definitions need to be created with different namespaces.
    pub async fn create(&self, client: &crate::Client, prefix: &str) -> crate::error::Result<()> {
        let ns_id = 0; // Use the default namespace

        // Create highlights
        for (name, opts) in &self.highlights {
            let full = full_name(prefix, name)?;
            client.nvim.set_hl(ns_id, &full, opts.to_sethl()).await?;
        }

        // Create links
        for (new_group, existing_group) in &self.links {
            let full = full_name(prefix, new_group)?;
            client
                .nvim
                .set_hl(
                    ns_id,
                    &full,
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
    #[should_panic(
        expected = "Invalid color format 'invalid': must be '#' followed by 6 hex digits"
    )]
    fn test_invalid_fg_color() {
        let hl = Hl::new().fg("invalid").unwrap();
        hl.to_sethl();
    }

    #[test]
    #[should_panic(
        expected = "Invalid color format 'invalid': must be '#' followed by 6 hex digits"
    )]
    fn test_invalid_bg_color() {
        let hl = Hl::new().bg("invalid").unwrap();
        hl.to_sethl();
    }

    #[tokio::test]
    async fn test_highlight_creation() {
        let test = NviTest::builder().run().await.unwrap();
        let highlights = Highlights::new()
            .hl("TestHl", Hl::new().fg("#ff0000").unwrap().bold(true))
            .link("TestLink", "TestHl");

        highlights.create(&test.client, "test_").await.unwrap();

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
