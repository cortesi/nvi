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

    /// Returns the dimensions of the content as (width, height).
    /// Width is the length of the longest line, height is the number of lines.
    pub fn size(&self) -> (usize, usize) {
        let width = self.lines.iter().map(|s| s.len()).max().unwrap_or(0);
        let height = self.lines.len();
        (width, height)
    }
}

/// A window pane.
pub struct Pane {
    pub window: types::Window,
    pub buffer: types::Buffer,
    pub border: Option<types::Border>,
    pub content: Content,
    pub width: Option<u64>,
    pub height: Option<u64>,
    client: Client,
}

impl Pane {
    /// Creates a new pane builder.
    pub fn builder() -> PaneBuilder {
        PaneBuilder::new()
    }

    /// Returns the dimensions of the pane as (width, height).
    /// If width or height is not explicitly set, uses the content dimensions.
    pub fn size(&self) -> (u64, u64) {
        let (content_width, content_height) = self.content.size();
        let width = self.width.unwrap_or(content_width as u64);
        let height = self.height.unwrap_or(content_height as u64);
        (width, height)
    }

    /// Destroys the window and buffer, consuming the pane.
    pub async fn destroy(self) -> Result<()> {
        self.client.nvim.win_close(&self.window, true).await?;
        self.client
            .nvim
            .buf_delete(&self.buffer, Default::default())
            .await?;
        Ok(())
    }
}

/// Builder for constructing a Pane.
pub struct PaneBuilder {
    border: Option<types::Border>,
    width: Option<u64>,
    height: Option<u64>,
    window_conf: Option<WindowConf>,
    win_pos: Option<(types::Window, Pos, u64)>,
    editor_pos: Option<(Pos, u64)>,
}

impl PaneBuilder {
    /// Creates a new pane builder.
    fn new() -> Self {
        Self {
            border: None,
            width: None,
            height: None,
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

    /// Sets the width of the pane.
    pub fn with_width(mut self, width: u64) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the height of the pane.
    pub fn with_height(mut self, height: u64) -> Self {
        self.height = Some(height);
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

        let mut conf = self.window_conf.unwrap_or_else(|| WindowConf {
            style: Some("minimal".to_string()),
            relative: Some(types::Relative::Editor),
            row: Some(0),
            col: Some(0),
            ..Default::default()
        });

        // Ensure we have a style set
        if conf.style.is_none() {
            conf.style = Some("minimal".to_string());
        }
        if let Some(ref border) = self.border {
            conf = conf.border(border.clone());
        }
        if let Some(width) = self.width {
            conf = conf.width(width);
        }
        if let Some(height) = self.height {
            conf = conf.height(height);
        }

        // Handle window positioning if specified
        if let Some((win, pos, padding)) = self.win_pos {
            // Get the target window dimensions
            let win_width = client.nvim.win_get_width(&win).await? as u64;
            let win_height = client.nvim.win_get_height(&win).await? as u64;

            conf = conf.relative(types::Relative::Win).win(win);

            // Get our own dimensions
            let width = conf.width.unwrap_or(content.size().0 as u64);
            let height = conf.height.unwrap_or(content.size().1 as u64);

            let (row, col) = pos.win_pos((win_width, win_height), (width, height), padding);

            conf = conf.row(row).col(col);
        } else if let Some((pos, padding)) = self.editor_pos {
            conf = conf.relative(types::Relative::Editor);

            // Get the editor dimensions using &o_columns and &o_lines
            let editor_width = client
                .nvim
                .get_option_value("columns", Default::default())
                .await?
                .as_u64()
                .unwrap();
            let editor_height = client
                .nvim
                .get_option_value("lines", Default::default())
                .await?
                .as_u64()
                .unwrap();

            // Get our own dimensions
            let width = conf.width.unwrap_or(content.size().0 as u64);
            let height = conf.height.unwrap_or(content.size().1 as u64);

            let (row, col) = pos.win_pos((editor_width, editor_height), (width, height), padding);

            conf = conf.row(row).col(col);
        } else {
            conf = conf.relative(types::Relative::Editor).row(0).col(0);
        }

        let window = client.nvim.open_win(&buffer, true, conf).await?;

        Ok(Pane {
            window,
            buffer,
            border: self.border,
            content,
            width: self.width,
            height: self.height,
            client: client.clone(),
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

    // #[tokio::test]
    // async fn test_pane_creation() {
    //     let test = NviTest::builder().run().await.unwrap();
    //     let content = Content::new(vec!["test".to_string()]);
    //
    //     let pane = Pane::builder()
    //         .with_width(10)
    //         .with_height(1)
    //         .build(&test.client, content)
    //         .await
    //         .unwrap();
    //
    //     assert!(test.client.nvim.win_is_valid(&pane.window).await.unwrap());
    //     assert!(test.client.nvim.buf_is_valid(&pane.buffer).await.unwrap());
    //
    //     pane.destroy().await.unwrap();
    //     test.finish().await.unwrap();
    // }
}
