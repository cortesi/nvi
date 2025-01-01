use crate::nvim::{types, types::WindowConf};
use crate::{error::Result, Client};

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

/// Content to be displayed in a window pane.
pub struct Content {
    lines: Vec<String>,
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
}

impl PaneBuilder {
    /// Creates a new pane builder.
    fn new() -> Self {
        Self {
            border: None,
            width: None,
            height: None,
            window_conf: None,
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

    /// Builds the pane with the configured options, creating the underlying buffer and window.
    pub async fn build(self, client: &Client, content: Content) -> Result<Pane> {
        let buffer = client.nvim.create_buf(false, true).await?;

        let mut conf = self
            .window_conf
            .unwrap_or_default()
            .relative(types::Relative::Editor);
        conf.style = Some("minimal".to_string());
        if let Some(ref border) = self.border {
            conf = conf.border(border.clone());
        }
        if let Some(width) = self.width {
            conf = conf.width(width);
        }
        if let Some(height) = self.height {
            conf = conf.height(height);
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::test::NviTest;
//
//     #[tokio::test]
//     async fn test_pane_creation() {
//         let test = NviTest::builder().run().await.unwrap();
//         let content = Content::new(vec!["test".to_string()]);
//         let pane = Pane::builder()
//             .with_width(10)
//             .with_height(1)
//             .build(&test.client, content)
//             .await
//             .unwrap();
//
//         assert!(test.client.nvim.win_is_valid(&pane.window).await.unwrap());
//         assert!(test.client.nvim.buf_is_valid(&pane.buffer).await.unwrap());
//
//         pane.destroy().await.unwrap();
//         test.finish().await.unwrap();
//     }
// }
