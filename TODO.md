
# Questions

- Extract an nvi-ui library?


# Features

- Generate help docs. This should now be pretty do-able with the introspection API.
    - Add help docs to introspect()
    - standard "plugin docs" command
- Standard way to control logging in plugins (tracing?)
    - For development, configure tracing to output to screen or file
    - For production, output to a file or neovim notices
- Autocmds should be created within an autocmd group by default
    - When we create the group, we'll set "clear" to clear previous autocmds

# Design


# nvi.nvim

- nvi.nvim to install, update and run nvi plugins
- should play well with other plugin managers
- only requirement is a working rust install with cargo
- uses 'cargo install' to install and update binaries 
    - cargo install upgrades binaries if a newer version is released
- To do this, we'd derive NviPlugin for Arc<Mutex<Plugin>>



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
- idea: a standard way for tools to specify demos
    - demos start developer's defined config for initialization
    - has a way to pop up text to tell the user about key bindings, etc
    - useful both for interactive testing by developers and for users
    - bundle demos along with the compiled plugin
    - non-interactive demos for docs?
