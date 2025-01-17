
# Questions

- At the moment, the "highlights" and "connected" methods are weird - we derive
  them if they are in the impl block for our macro, but it's error prone since
  the names are not constrained by a trait. It feels like maybe these should be
  in a separate trait that is explicitly implemented? This would be more
  idiomatic, but also add implementation complexity.
- Highights - at the moment we generate highilght names like "my_pluginNormal".
  Wonder if "my_plugin_Normal" would be better?
- Take opts by reference in nvi_api?
- What should live on Client and what shouldn't? For instance, many input
  functions could naturally live on Client. 


# Features

- Evolve pane::Text to be much more powerful, and have operations that make creating visual interfaces easier
- Standard way to control logging in plugins (tracing?)
    - For development, configure tracing to output to screen or file
    - For production, output to a file or neovim notices
- Autocmds should be created within an autocmd group by default
    - When we create the group, we'll set "clear" to clear previous autocmds
- Better execution of lua, with positional replacement of arguments. We could do
  this by using select(offset, ...) to get the arguments, and then assigning
  them to variables to use in the user's code. nvi_1, nvi_2, etc.
- At the moment we're tied to neovim nightly for protocol generation, although all our
  unit tests pass on neovim stable. We should:
    - Specify in docs that we need nightly for dev
    - Detect if protocol generation is happening on stable and fail
    - Work unit testing on nightly and stable into our CI somehow


# Plugin docs

- Specify that the first line of plugin doc string is the "short doc" used in
  vimdoc rendering and plugin listings.
- Vim help output format 
    - https://github.com/google/vimdoc
- Wrap text to a width - a default for non-terminal output, and terminal width
  for terminal output
- A docs field for highlights?


# Docs

- nvi manual
- Parser for Vim help format would be super useful, not just to extract the
  complete docs for our API, but also as a general service to the community.
  The current docs just suck.


# API Design

- Nicer way to do equality checks against KeyPress
- Consolidate our Error enum, and evaluate whether we need more variants
- We have a confusing situation where Client can be managing a plugin (in
  plugin code) or not (in demos). We should probalby make this distinct.


# nvi.nvim

- nvi.nvim to install, update and run nvi plugins
- should play well with other plugin managers
- only requirement is a working rust install with cargo
- uses 'cargo install' to install and update binaries 
    - cargo install upgrades binaries if a newer version is released


# Demo projects

- nvi-stacks
- built-in demo project, which will also be useful for dev


# Dev tools

- live rebuild/reconnect
- run should inject a truss that consumes logs from the client and sends
  them to neovim?
- demos
    - a way to pop up text to tell the user about key bindings, etc
    - this could just be a ui modal pane, without specific features related to demo


# Bugs

- We have a set of bugs for certain functions that return Strings. Technically,
  we can get invalid bytes that are not UTF-8, and the Value type can express
  this. However, in our API we always return a String, which will panics in the
  face of invalid bytes. We COULD return a MaybeString, but that puts burden on
  all callers. Let's ponder.
