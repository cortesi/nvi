use crate::nvim::types::{Buffer, Window};
pub mod win;

/// A Pane is a user interface window, which tracks both a neovim window and the underlying buffer
/// it displays together.
pub struct Pane {
    pub window: Window,
    pub buffer: Buffer,
}
