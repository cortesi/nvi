
# Questions


# Features

- Generate help docs. This should now be pretty do-able with the introspection API.
    - Add help docs to introspect()
    - standard "plugin docs" command
- Standard way to control logging in plugins (tracing?)
    - For development, configure tracing to output to screen or file
    - For production, output to a file or neovim notices
- Autocmds should be created within an autocmd group by default
    - When we create the group, we'll set "clear" to clear previous autocmds
- Better execution of lua, with positional replacement of arguments. We could do
  this by using select(offset, ...) to get the arguments, and then assigning
  them to variables to use in the user's code. nvi_1, nvi_2, etc.


# API

- Take opts by reference in nvi_api.


# Design


# nvi.nvim

- nvi.nvim to install, update and run nvi plugins
- should play well with other plugin managers
- only requirement is a working rust install with cargo
- uses 'cargo install' to install and update binaries 
    - cargo install upgrades binaries if a newer version is released


# Demo projects

- nvi-stacks
- nvi-nav



# Dev tools

- nvic live rebuild/reconnect
- nvic run should inject a truss that consumes logs from the client and sends
  them to neovim?
- option to start neovim with the user's config intact
- option to start plugins from installed binary
- lua script to execute on startup (for setting up the environment)
    - neovim vim config file arg enough?
- demos
    - a way to pop up text to tell the user about key bindings, etc
    - non-interactive demos for docs


# Bugs

- We have a set of bugs for certain functions that return Strings. Technically,
  we can get invalid bytes that are not UTF-8, and the Value type can express
  this. However, in our API we always return a String, which will panics in the
  face of invalid bytes. We COULD return a MaybeString, but that puts burden on
  all callers. Let's ponder.
