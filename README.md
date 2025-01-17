
# nvi - Neovim plugins in Rust

- Generated bindings to the Neovim API, type-safe to the extent the API metadata allows
- An ergonomic way to define plugins with request endpoints and auto commands
- A UI library of useful user interface abstractions
- The [nvirun.nvim] plugin to install, update and run nvi plugins. Plugins are
  installed and updated using `cargo` - the only requirement on the user is to
  have working Rust installation.
- Helpers for testing plugins, including connecting plugins to live headless
  neovim instance.
- Automatic generation of plugin documentation, including vimdoc 
- An API for building plugin demos, used for testing during development,
  generating interactive examples and demonstrating plugin features to users.


