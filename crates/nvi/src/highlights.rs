use crate::nvim::opts::SetHl;
use derive_setters::*;

/// We support the subset of highlight attributes that are common to both terminal and GUI, and
/// omit anachronisms like "standout".
#[derive(Debug, Clone, PartialEq, Eq, Setters, Default)]
#[setters(strip_option, into)]
pub struct Hl {
    pub fg: Option<String>,
    pub bg: Option<String>,

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

    /// Convert to opts::SetHl, copying only the fields that are present in both structs
    pub fn to_sethl(&self) -> crate::nvim::opts::SetHl {
        SetHl {
            fg: self.fg.clone(),
            bg: self.bg.clone(),
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

    pub fn hl(&mut self, name: &str, h: Hl) -> &mut Self {
        self.highlights.push((name.into(), h));
        self
    }

    pub fn link(&mut self, new_group: &str, existing_group: &str) -> &mut Self {
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
            let full_name = format!("{}{}", prefix, name);
            client
                .nvim
                .set_hl(ns_id, &full_name, opts.to_sethl())
                .await?;
        }

        // Create links
        for (new_group, existing_group) in &self.links {
            let full_name = format!("{}{}", prefix, new_group);
            let full_target = format!("{}{}", prefix, existing_group);
            client
                .nvim
                .set_hl(
                    ns_id,
                    &full_name,
                    SetHl {
                        link: Some(full_target),
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

    #[test]
    fn test_group_new() {
        let g = Hl::new().bold(true).italic(true);
        assert_eq!(g.fg, None);
        assert_eq!(g.bold, Some(true));
        assert_eq!(g.italic, Some(true));
        assert_eq!(g.underline, None);

        let _ = Highlights::new()
            .hl("foo", Hl::new().bold(true).italic(true))
            .hl("bar", Hl::new().fg("red").bg("blue"))
            .link("foo", "bar");
    }
}
