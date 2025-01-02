use crate::nvim::{types, types::WindowConf};
use crate::{error::Result, Client};

#[derive(Debug)]
pub enum Pos {
    NE,
    N,
    NW,
    E,
    W,
    SE,
    S,
    SW,
    Center,
}

impl Pos {
    /// Calculate window position given dimensions and padding
    ///
    /// # Arguments
    ///
    /// * `enclosing` - (width, height) of the enclosing window
    /// * `target` - (width, height) of the target window to position
    /// * `padding` - padding from the anchor point
    ///
    /// Returns (row, col) coordinates assuming NE anchor
    pub fn win_pos(&self, enclosing: (u64, u64), target: (u64, u64), padding: u64) -> (i64, i64) {
        let (win_width, win_height) = enclosing;
        let (width, height) = target;

        // If target window is larger than enclosing window, position at (0, 0)
        if width > win_width || height > win_height {
            return (0, 0);
        }

        match self {
            Pos::NW => (padding as i64, padding as i64),
            Pos::N => (padding as i64, ((win_width - width) / 2) as i64),
            Pos::NE => (padding as i64, (win_width - width - padding) as i64),
            Pos::W => (((win_height - height) / 2) as i64, padding as i64),
            Pos::E => (
                ((win_height - height) / 2) as i64,
                (win_width - width - padding) as i64,
            ),
            Pos::SW => ((win_height - height - padding) as i64, padding as i64),
            Pos::S => (
                (win_height - height - padding) as i64,
                ((win_width - width) / 2) as i64,
            ),
            Pos::SE => (
                (win_height - height - padding) as i64,
                (win_width - width - padding) as i64,
            ),
            Pos::Center => (
                ((win_height - height) / 2) as i64,
                ((win_width - width) / 2) as i64,
            ),
        }
    }
}

/// Content to be displayed in a window pane.
#[derive(Clone, Debug)]
pub struct Content {
    pub(crate) lines: Vec<String>,
}

impl Content {
    /// Creates new content from a vector of strings.
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }

    /// Creates a blank content area of specified width and height, filled with spaces.
    pub fn blank(width: usize, height: usize) -> Self {
        let line = " ".repeat(width);
        let lines = vec![line; height];
        Self { lines }
    }

    /// Creates a content area of specified width and height with text centered in both dimensions.
    pub fn center(width: usize, height: usize, text: &str) -> Self {
        let text_lines: Vec<&str> = text.lines().collect();
        let text_height = text_lines.len();

        // Calculate vertical padding
        let v_padding = (height.saturating_sub(text_height)) / 2;
        let mut lines = vec![" ".repeat(width); height];

        // Insert text lines in the center
        for (i, text_line) in text_lines.into_iter().enumerate() {
            if v_padding + i >= height {
                break;
            }
            // Calculate horizontal padding for this line
            let h_padding = (width.saturating_sub(text_line.len())) / 2;
            if h_padding + text_line.len() <= width {
                let target_line = &mut lines[v_padding + i];
                target_line.replace_range(h_padding..h_padding + text_line.len(), text_line);
            }
        }

        Self { lines }
    }

    /// Returns the dimensions of the content as (width, height).
    /// Width is the length of the longest line, height is the number of lines.
    pub fn size(&self) -> (usize, usize) {
        let width = self.lines.iter().map(|s| s.len()).max().unwrap_or(0);
        let height = self.lines.len();
        (width, height)
    }
}

/// A window pane, which is a window and a buffer managed in concert. This struct is intended for
/// interface windows, especially floats - that is, windows that aren't used for editing text
/// directly.
#[derive(Clone, Debug)]
pub struct Pane {
    pub window: types::Window,
    pub buffer: types::Buffer,
    pub content: Content,
}

impl Pane {
    /// Creates a new pane builder.
    pub fn builder() -> PaneBuilder {
        PaneBuilder::new()
    }

    /// Destroys the window and buffer, consuming the pane.
    pub async fn destroy(self, client: &Client) -> Result<()> {
        client.nvim.win_close(&self.window, true).await?;
        client
            .nvim
            .buf_delete(&self.buffer, Default::default())
            .await?;
        Ok(())
    }
}

/// Builder for constructing a Pane.
pub struct PaneBuilder {
    border: Option<types::Border>,
    window_conf: Option<WindowConf>,
    win_pos: Option<(types::Window, Pos, u64)>,
    editor_pos: Option<(Pos, u64)>,
}

impl PaneBuilder {
    /// Creates a new pane builder.
    fn new() -> Self {
        Self {
            border: None,
            window_conf: None,
            win_pos: None,
            editor_pos: None,
        }
    }

    /// Sets a base window configuration. Other builder methods will override these settings.
    pub fn with_window_conf(mut self, conf: WindowConf) -> Self {
        self.window_conf = Some(conf);
        self
    }

    /// Sets the border style for the pane.
    pub fn with_border(mut self, border: types::Border) -> Self {
        self.border = Some(border);
        self
    }

    /// Positions the pane relative to another window.
    pub fn with_win_pos(mut self, win: types::Window, pos: Pos, padding: u64) -> Self {
        self.win_pos = Some((win, pos, padding));
        self
    }

    /// Positions the pane relative to the editor window.
    pub fn with_editor_pos(mut self, pos: Pos, padding: u64) -> Self {
        self.editor_pos = Some((pos, padding));
        self
    }

    /// Builds the pane with the configured options, creating the underlying buffer and window.
    pub async fn build(self, client: &Client, content: Content) -> Result<Pane> {
        let buffer = client.nvim.create_buf(false, true).await?;

        // Set the buffer content
        client
            .nvim
            .buf_set_lines(&buffer, 0, -1, false, content.lines.clone())
            .await?;

        let mut conf = self.window_conf.unwrap_or_default();

        if conf.style.is_none() {
            conf.style = Some("minimal".to_string());
        }
        if let Some(ref border) = self.border {
            conf = conf.border(border.clone());
        }

        let width = conf.width.unwrap_or(content.size().0 as u64);
        let height = conf.height.unwrap_or(content.size().1 as u64);
        conf.width = Some(width);
        conf.height = Some(height);

        // Handle window positioning if specified
        if let Some((win, pos, padding)) = self.win_pos {
            // Get the target window dimensions
            let win_width = client.nvim.win_get_width(&win).await? as u64;
            let win_height = client.nvim.win_get_height(&win).await? as u64;

            conf = conf.relative(types::Relative::Win).win(win);

            let (row, col) = pos.win_pos((win_width, win_height), (width, height), padding);

            conf = conf.row(row).col(col);
        } else if let Some((pos, padding)) = self.editor_pos {
            conf = conf.relative(types::Relative::Editor);

            // Get the editor dimensions using &o_columns and &o_lines
            let editor_width: u64 = client
                .nvim
                .get_option_value("columns", Default::default())
                .await?;
            let editor_height: u64 = client
                .nvim
                .get_option_value("lines", Default::default())
                .await?;

            let (row, col) = pos.win_pos((editor_width, editor_height), (width, height), padding);

            conf = conf.row(row).col(col);
        } else {
            conf = conf.relative(types::Relative::Editor).row(0).col(0);
        }

        let window = client.nvim.open_win(&buffer, true, conf).await?;
        Ok(Pane {
            window,
            buffer,
            content,
        })
    }
}

impl Default for PaneBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::NviTest;

    /// A test case for window positioning
    struct WinPosCase {
        /// Description of the test case
        desc: &'static str,
        /// Size of the enclosing window (width, height)
        enclosing: (u64, u64),
        /// Size of the target window (width, height)
        target: (u64, u64),
        /// Padding between windows
        padding: u64,
        /// Expected positions for each Pos variant
        positions: Vec<(Pos, (i64, i64))>,
    }

    #[test]
    fn test_content_center() {
        let tests = vec![
            ("single line", 4, 3, "hi", vec!["    ", " hi ", "    "]),
            (
                "multi line",
                6,
                5,
                "hi\nbye",
                vec!["      ", "  hi  ", " bye  ", "      ", "      "],
            ),
            ("exact fit", 2, 1, "hi", vec!["hi"]),
        ];

        for (name, width, height, text, expected) in tests {
            let content = Content::center(width, height, text);
            assert_eq!(content.lines, expected, "test case: {}", name);
            assert_eq!(content.size(), (width, height), "size check: {}", name);
        }
    }

    #[test]
    fn test_win_pos() {
        let test_cases = vec![
            WinPosCase {
                desc: "Oversized window (15x15) in 10x10 enclosing window",
                enclosing: (10, 10),
                target: (15, 15),
                padding: 0,
                positions: vec![
                    (Pos::NW, (0, 0)), // all positions should return (0, 0)
                    (Pos::N, (0, 0)),
                    (Pos::NE, (0, 0)),
                    (Pos::W, (0, 0)),
                    (Pos::E, (0, 0)),
                    (Pos::SW, (0, 0)),
                    (Pos::S, (0, 0)),
                    (Pos::SE, (0, 0)),
                    (Pos::Center, (0, 0)),
                ],
            },
            WinPosCase {
                desc: "Small window (2x2) in 10x10 enclosing window",
                enclosing: (10, 10),
                target: (2, 2),
                padding: 0,
                positions: vec![
                    (Pos::NW, (0, 0)),     // top left
                    (Pos::N, (0, 4)),      // top center
                    (Pos::NE, (0, 8)),     // top right
                    (Pos::W, (4, 0)),      // middle left
                    (Pos::E, (4, 8)),      // middle right
                    (Pos::SW, (8, 0)),     // bottom left
                    (Pos::S, (8, 4)),      // bottom center
                    (Pos::SE, (8, 8)),     // bottom right
                    (Pos::Center, (4, 4)), // center
                ],
            },
            WinPosCase {
                desc: "Large window (10x10) in 20x20 enclosing window",
                enclosing: (20, 20),
                target: (10, 10),
                padding: 0,
                positions: vec![
                    (Pos::NW, (0, 0)),     // top left
                    (Pos::N, (0, 5)),      // top center: (20-10)/2 = 5
                    (Pos::NE, (0, 10)),    // top right: 20-10
                    (Pos::W, (5, 0)),      // middle left
                    (Pos::E, (5, 10)),     // middle right
                    (Pos::SW, (10, 0)),    // bottom left
                    (Pos::S, (10, 5)),     // bottom center
                    (Pos::SE, (10, 10)),   // bottom right
                    (Pos::Center, (5, 5)), // center
                ],
            },
            WinPosCase {
                desc: "Small window (2x2) with padding=1",
                enclosing: (10, 10),
                target: (2, 2),
                padding: 1,
                positions: vec![
                    (Pos::NW, (1, 1)),     // top left
                    (Pos::N, (1, 4)),      // top center
                    (Pos::NE, (1, 7)),     // top right
                    (Pos::W, (4, 1)),      // middle left
                    (Pos::E, (4, 7)),      // middle right
                    (Pos::SW, (7, 1)),     // bottom left
                    (Pos::S, (7, 4)),      // bottom center
                    (Pos::SE, (7, 7)),     // bottom right
                    (Pos::Center, (4, 4)), // center
                ],
            },
            WinPosCase {
                desc: "Small window (2x2) with padding=3",
                enclosing: (10, 10),
                target: (2, 2),
                padding: 3,
                positions: vec![
                    (Pos::NW, (3, 3)),     // top left
                    (Pos::N, (3, 4)),      // top center
                    (Pos::NE, (3, 5)),     // top right
                    (Pos::W, (4, 3)),      // middle left
                    (Pos::E, (4, 5)),      // middle right
                    (Pos::SW, (5, 3)),     // bottom left
                    (Pos::S, (5, 4)),      // bottom center
                    (Pos::SE, (5, 5)),     // bottom right
                    (Pos::Center, (4, 4)), // center
                ],
            },
        ];

        for case in test_cases {
            for (pos, expected) in case.positions {
                assert_eq!(
                    pos.win_pos(case.enclosing, case.target, case.padding),
                    expected,
                    "{}: enclosing: {:?}, target: {:?}, padding: {}, pos: {:?}",
                    case.desc,
                    case.enclosing,
                    case.target,
                    case.padding,
                    pos
                );
            }
        }
    }

    #[tokio::test]
    async fn test_pane_creation() {
        let test = NviTest::builder().run().await.unwrap();
        let content = Content::new(vec!["test".to_string()]);

        let pane = Pane::builder()
            .with_editor_pos(Pos::Center, 0)
            .build(&test.client, content)
            .await
            .unwrap();

        assert!(test.client.nvim.win_is_valid(&pane.window).await.unwrap());
        assert!(test.client.nvim.buf_is_valid(&pane.buffer).await.unwrap());

        pane.destroy(&test.client).await.unwrap();
        test.finish().await.unwrap();
    }
}
