pub const DOCS: &[(&str, &str)] = &[
    (
        "nvim__complete_set",
        "
        EXPERIMENTAL: this API may change in the future.

        Sets info for the completion item at the given index. If the info text was 
        shown in a window, returns the window and buffer ids, or empty dict if not
        shown.
        ",
    ),
    (
        "nvim__fs_watch",
        "Registers a recursive (filesystem) watcher.",
    ),
    (
        "nvim__get_lib",
        "Gets the paths contained in the runtimepath variable.",
    ),
    (
        "nvim_buf_add_highlight",
        "
        Adds a highlight to buffer.

        Useful for plugins that dynamically generate highlights to a buffer (like
        a semantic highlighter or linter). The function adds a single highlight to
        a buffer. Unlike `matchaddpos()` highlights follow changes to line
        numbering (as lines are inserted/removed above the highlighted line), like
        signs and marks do.

        Namespaces are used for batch deletion/updating of a set of highlights. To
        create a namespace, use `nvim_create_namespace()` which returns a
        namespace id. Pass it in to this function as ns_id to add highlights to
        the namespace. All highlights in the same namespace can then be cleared
        with single call to `nvim_buf_clear_namespace()`. If the highlight never
        will be deleted by an API call, pass ns_id = -1.

        As a shorthand, ns_id = 0 can be used to create a new namespace for the
        highlight, the allocated id is then returned. If hl_group is the empty
        string no highlight is added, but a new ns_id is still returned. This is
        supported for backwards compatibility, new code should use
        `nvim_create_namespace()` to create a new empty namespace.
        ",
    ),
    (
        "nvim_buf_attach",
        "
        Activates buffer-update events on a channel, or as Lua callbacks.
        ",
    ),
    (
        "nvim_buf_call",
        "
        Call a function with buffer as temporary current buffer.

        This temporarily switches current buffer to buffer. If the current
        window already shows buffer, the window is not switched. If a window
        inside the current tabpage (including a float) already shows the buffer,
        then one of these windows will be set as current window temporarily.
        Otherwise a temporary scratch window (called the autocmd window for
        historical reasons) will be used.

        This is useful e.g. to call Vimscript functions that only work with the
        current buffer/window currently, like `termopen()`.
        ",
    ),
    (
        "nvim_buf_clear_namespace",
        "
        Clears namespaced objects (highlights, extmarks, virtual text) from a
        region.

        Lines are 0-indexed. `api-indexing` To clear the namespace in the entire
        buffer, specify line_start=0 and line_end=-1.
        ",
    ),
    (
        "nvim_buf_del_extmark",
        "
        Removes an extmark.
        ",
    ),
    ("nvim__get_runtime", "Find files in runtime directories."),
    (
        "nvim__id",
        "Returns object given as argument.

        This API function is used for testing. One should not rely on its presence
        in plugins.",
    ),
    (
        "nvim__id_array",
        "Returns array given as argument.

        This API function is used for testing. One should not rely on its presence
        in plugins.",
    ),
    (
        "nvim__id_dict",
        "Returns dict given as argument.

        This API function is used for testing. One should not rely on its presence
        in plugins.",
    ),
    (
        "nvim__id_float",
        "Returns floating-point value given as argument.

        This API function is used for testing. One should not rely on its presence
        in plugins.",
    ),
    (
        "nvim__inspect_cell",
        "NB: if your UI does not use hlstate, this will not return hlstate first time.",
    ),
    (
        "nvim__invalidate_glyph_cache",
        "For testing. The condition in schar_cache_clear_if_full is hard to reach,
        so this function can be used to force a cache clear in a test.",
    ),
    (
        "nvim__ns_get",
        "EXPERIMENTAL: this API will change in the future.

        Get the properties for namespace.",
    ),
    (
        "nvim__ns_set",
        "EXPERIMENTAL: this API will change in the future.
        
        Set some properties for namespace.",
    ),
    (
        "nvim__redraw",
        "EXPERIMENTAL: this API will change in the future.

        Instruct Nvim to redraw various components.",
    ),
    ("nvim__stats", "Gets internal stats."),
    (
        "nvim_buf_attach",
        "
            Activates buffer-update events on a channel, or as Lua callbacks.
        ",
    ),
    (
        "nvim_buf_add_highlight",
        "Adds a highlight to buffer.

        Useful for plugins that dynamically generate highlights to a buffer (like
        a semantic highlighter or linter). The function adds a single highlight to
        a buffer. Unlike |matchaddpos()| highlights follow changes to line
        numbering (as lines are inserted/removed above the highlighted line), like
        signs and marks do.

        Namespaces are used for batch deletion/updating of a set of highlights. To
        create a namespace, use |nvim_create_namespace()| which returns a
        namespace id. Pass it in to this function as ns_id to add highlights to
        the namespace. All highlights in the same namespace can then be cleared
        with single call to |nvim_buf_clear_namespace()|. If the highlight never
        will be deleted by an API call, pass ns_id = -1.

        As a shorthand, ns_id = 0 can be used to create a new namespace for the
        highlight, the allocated id is then returned. If hl_group is the empty
        string no highlight is added, but a new ns_id is still returned. This is
        supported for backwards compatibility, new code should use
        |nvim_create_namespace()| to create a new empty namespace.",
    ),
    (
        "nvim_buf_call",
        "
            Call a function with buffer as temporary current buffer.

            This temporarily switches current buffer to buffer. If the current
            window already shows buffer, the window is not switched. If a window
            inside the current tabpage (including a float) already shows the buffer,
            then one of these windows will be set as current window temporarily.
            Otherwise a temporary scratch window (called the autocmd window for
            historical reasons) will be used.

            This is useful e.g. to call Vimscript functions that only work with the
            current buffer/window currently, like |termopen()|.
        ",
    ),
    (
        "nvim_buf_clear_namespace",
        "
            Clears |namespace|d objects (highlights, |extmarks|, virtual text) from a
            region.

            Lines are 0-indexed. |api-indexing| To clear the namespace in the entire
            buffer, specify line_start=0 and line_end=-1.
        ",
    ),
    (
        "nvim_buf_del_extmark",
        "
            Removes an |extmark|.
        ",
    ),
    (
        "nvim_buf_del_keymap",
        "
            Unmaps a buffer-local |mapping| for the given mode.
        ",
    ),
    (
        "nvim_buf_del_mark",
        "
            Deletes a named mark in the buffer. See |mark-motions|.

            Note: only deletes marks set in the buffer, if the mark is not set in the
            buffer it will return false.
        ",
    ),
    (
        "nvim_buf_del_var",
        "
            Removes a buffer-scoped (b:) variable
        ",
    ),
    (
        "nvim_buf_delete",
        "
            Deletes the buffer. See |:bwipeout|
        ",
    ),
    (
        "nvim_buf_detach",
        "
            Deactivates buffer-update events on the channel.
        ",
    ),
    (
        "nvim_buf_get_changedtick",
        "
            Gets a changed tick of a buffer
        ",
    ),
    (
        "nvim_buf_get_commands",
        "
            Gets a map of buffer-local |user-commands|.
        ",
    ),
    (
        "nvim_buf_get_extmark_by_id",
        "
            Gets the position (0-indexed) of an |extmark|.
        ",
    ),
    (
        "nvim_buf_get_extmarks",
        "
            Gets |extmarks| in traversal order from a |charwise| region defined by
            buffer positions (inclusive, 0-indexed |api-indexing|).

            Region can be given as (row,col) tuples, or valid extmark ids (whose
            positions define the bounds). 0 and -1 are understood as (0,0) and (-1,-1)
            respectively.

            If end is less than start, traversal works backwards. (Useful with
            limit, to get the first marks prior to a given position.)

            Note: when using extmark ranges (marks with a end_row/end_col position)
            the overlap option might be useful. Otherwise only the start position of
            an extmark will be considered.

            Note: legacy signs placed through the |:sign| commands are implemented as
            extmarks and will show up here. Their details array will contain a
            sign_name field.
        ",
    ),
    (
        "nvim_buf_get_keymap",
        "
            Gets a list of buffer-local |mapping| definitions.
        ",
    ),
    (
        "nvim_buf_get_lines",
        "
            Gets a line-range from the buffer.

            Indexing is zero-based, end-exclusive. Negative indices are interpreted as
            length+1+index: -1 refers to the index past the end. So to get the last
            element use start=-2 and end=-1.

            Out-of-bounds indices are clamped to the nearest valid value, unless
            strict_indexing is set.
        ",
    ),
    (
        "nvim_buf_get_mark",
        "
            Returns a (row,col) tuple representing the position of the named mark.
            End of line column position is returned as |v:maxcol| (big number).
            See |mark-motions|.

            Marks are (1,0)-indexed. |api-indexing|
        ",
    ),
    (
        "nvim_buf_get_name",
        "
            Gets the full file name for the buffer
        ",
    ),
    (
        "nvim_buf_get_offset",
        "
            Returns the byte offset of a line (0-indexed). |api-indexing|

            Line 1 (index=0) has offset 0. UTF-8 bytes are counted. EOL is one byte.
            fileformat and fileencoding are ignored. The line index just after the
            last line gives the total byte-count of the buffer. A final EOL byte is
            counted if it would be written, see eol.

            Unlike |line2byte()|, throws error for out-of-bounds indexing. Returns -1
            for unloaded buffer.
        ",
    ),
    (
        "nvim_buf_get_text",
        "
            Gets a range from the buffer.

            This differs from |nvim_buf_get_lines()| in that it allows retrieving only
            portions of a line.

            Indexing is zero-based. Row indices are end-inclusive, and column indices
            are end-exclusive.

            Prefer |nvim_buf_get_lines()| when retrieving entire lines.
        ",
    ),
    (
        "nvim_buf_get_var",
        "
            Gets a buffer-scoped (b:) variable.
        ",
    ),
    (
        "nvim_buf_is_loaded",
        "
            Checks if a buffer is valid and loaded. See |api-buffer| for more info
            about unloaded buffers.
        ",
    ),
    (
        "nvim_buf_is_valid",
        "
            Checks if a buffer is valid.

            Note: Even if a buffer is valid it may have been unloaded. See |api-buffer|
            for more info about unloaded buffers.
        ",
    ),
    (
        "nvim_buf_line_count",
        "
            Returns the number of lines in the given buffer.
        ",
    ),
    (
        "nvim_buf_set_keymap",
        "
            Sets a buffer-local |mapping| for the given mode.
        ",
    ),
    (
        "nvim_buf_set_lines",
        "
            Sets (replaces) a line-range in the buffer.

            Indexing is zero-based, end-exclusive. Negative indices are interpreted as
            length+1+index: -1 refers to the index past the end. So to change or
            delete the last element use start=-2 and end=-1.

            To insert lines at a given index, set start and end to the same index.
            To delete a range of lines, set replacement to an empty array.

            Out-of-bounds indices are clamped to the nearest valid value, unless
            strict_indexing is set.
        ",
    ),
    (
        "nvim_buf_set_mark",
        "
        Sets a named mark in the given buffer, all marks are allowed
        file/uppercase, visual, last change, etc. See mark-motions.

        Marks are (1,0)-indexed. api-indexing

        Note: Passing 0 as line deletes the mark
        ",
    ),
    (
        "nvim_buf_set_name",
        "
        Sets the full file name for a buffer, like :file_f
        ",
    ),
    (
        "nvim_buf_set_text",
        "
        Sets (replaces) a range in the buffer

        This is recommended over nvim_buf_set_lines() when only modifying parts
        of a line, as extmarks will be preserved on non-modified parts of the
        touched lines.

        Indexing is zero-based. Row indices are end-inclusive, and column indices
        are end-exclusive.

        To insert text at a given (row, column) location, use
        start_row = end_row = row and start_col = end_col = col. To delete the
        text in a range, use replacement = {}.

        Note: Prefer nvim_buf_set_lines() (for performance) to add or delete
        entire lines.
        Note: Prefer nvim_paste() or nvim_put() to insert (instead of replace)
        text at cursor.
        ",
    ),
    (
        "nvim_buf_set_var",
        "
        Sets a buffer-scoped (b:) variable
        ",
    ),
    (
        "nvim_chan_send",
        "
        Send data to channel. For a job, it writes it to the stdin of the
        process. For the stdio channel `channel-stdio`, it writes to Nvim's
        stdout. For an internal terminal instance (`nvim_open_term()`) it writes
        directly to terminal output. See `channel-bytes` for more information.

        This function writes raw data, not RPC messages. If the channel was
        created with rpc=true then the channel expects RPC messages, use
        `vim.rpcnotify()` and `vim.rpcrequest()` instead.
        ",
    ),
    (
        "nvim_clear_autocmds",
        "
        Clears all autocommands selected by {opts}. To delete autocmds see
        `nvim_del_autocmd()`.
        ",
    ),
    (
        "nvim_cmd",
        "
        Executes an Ex command.

        Unlike `nvim_command()` this command takes a structured Dict instead of a
        String. This allows for easier construction and manipulation of an Ex
        command. This also allows for things such as having spaces inside a
        command argument, expanding filenames in a command that otherwise does not
        expand filenames, etc. Command arguments may also be Number, Boolean or
        String.

        The first argument may also be used instead of count for commands that
        support it in order to make their usage simpler. For example, instead of
        `vim.cmd.bdelete{ count = 2 }`, you may do `vim.cmd.bdelete(2)`.

        On execution error: fails with Vimscript error, updates v:errmsg.
        ",
    ),
    (
        "nvim_command",
        "
        Executes an Ex command.

        On execution error: fails with Vimscript error, updates v:errmsg.

        Prefer `nvim_cmd()` or `nvim_exec2()` instead. To modify an Ex command in
        a structured way before executing it, modify the result of
        `nvim_parse_cmd()` then pass it to `nvim_cmd()`.
        ",
    ),
    (
        "nvim_create_augroup",
        "
        Create or get an autocommand group autocmd-groups.
        ",
    ),
    (
        "nvim_create_autocmd",
        "
        Creates an autocommand event handler, defined by callback (Lua
        function or Vimscript function name string) or command (Ex command
        string).

        Note: pattern is NOT automatically expanded (unlike with :autocmd),
        thus names like $HOME and ~ must be expanded explicitly.
        ",
    ),
    (
        "nvim_create_buf",
        "
        Creates a new, empty, unnamed buffer.
        ",
    ),
    (
        "nvim_create_namespace",
        "
        Creates a new namespace or gets an existing one.

        Namespaces are used for buffer highlights and virtual text, see
        nvim_buf_add_highlight() and nvim_buf_set_extmark().

        Namespaces can be named or anonymous. If name matches an existing
        namespace, the associated id is returned. If name is an empty string a
        new, anonymous namespace is created.
        ",
    ),
    (
        "nvim_create_user_command",
        "
        Creates a global user-commands command.
        ",
    ),
    (
        "nvim_del_augroup_by_id",
        "
        Delete an autocommand group by id.

        To get a group id one can use nvim_get_autocmds().

        NOTE: behavior differs from :augroup-delete. When deleting a group,
        autocommands contained in this group will also be deleted and cleared.
        This group will no longer exist.
        ",
    ),
    (
        "nvim_del_augroup_by_name",
        "
        Delete an autocommand group by name.

        NOTE: behavior differs from :augroup-delete. When deleting a group,
        autocommands contained in this group will also be deleted and cleared.
        This group will no longer exist.
        ",
    ),
    (
        "nvim_del_autocmd",
        "
        Deletes an autocommand by id.
        ",
    ),
    (
        "nvim_del_current_line",
        "
        Deletes the current line.
        ",
    ),
    (
        "nvim_del_keymap",
        "
        Unmaps a global mapping for the given mode.

        To unmap a buffer-local mapping, use nvim_buf_del_keymap().
        ",
    ),
    (
        "nvim_del_mark",
        "
        Deletes an uppercase/file named mark. See mark-motions.

        Note: Lowercase name (or other buffer-local mark) is an error.
        ",
    ),
    (
        "nvim_del_user_command",
        "
        Delete a user-defined command.
        ",
    ),
    (
        "nvim_del_var",
        "
        Removes a global (g:) variable.
        ",
    ),
    (
        "nvim_echo",
        "
        Echo a message.
        ",
    ),
    (
        "nvim_err_write",
        "
        Writes a message to the Vim error buffer. Does not append \\n, the
        message is buffered (will not display) until a linefeed is written.
        ",
    ),
    (
        "nvim_err_writeln",
        "
        Writes a message to the Vim error buffer. Appends \\n, so the buffer is
        flushed (and displayed).
        ",
    ),
    (
        "nvim_eval",
        "
        Evaluates a Vimscript expression. Dicts and Lists are recursively
        expanded.

        On execution error: fails with Vimscript error, updates v:errmsg.
        ",
    ),
    (
        "nvim_eval_statusline",
        "
        Evaluates statusline string.
        ",
    ),
    (
        "nvim_exec2",
        "
            Executes Vimscript (multiline block of Ex commands), like anonymous
            |:source|.

            Unlike |nvim_command()| this function supports heredocs, script-scope
            (s:), etc.

            On execution error: fails with Vimscript error, updates v:errmsg.
        ",
    ),
    (
        "nvim_exec_lua",
        "
            Execute Lua code. Parameters (if any) are available as ... inside the
            chunk. The chunk can return a value.

            Only statements are executed. To evaluate an expression, prefix it with
            return: return my_function(...)
        ",
    ),
    (
        "nvim_feedkeys",
        "
            Sends input-keys to Nvim, subject to various quirks controlled by mode
            flags. This is a blocking call, unlike |nvim_input()|.

            On execution error: does not fail, but updates v:errmsg.

            To input sequences like <C-o> use |nvim_replace_termcodes()| (typically
            with escape_ks=false) to replace |keycodes|, then pass the result to
            nvim_feedkeys().
        ",
    ),
    (
        "nvim_get_all_options_info",
        "
            Gets the option information for all options.

            The dict has the full option names as keys and option metadata dicts as
            detailed at |nvim_get_option_info2()|.
        ",
    ),
    (
        "nvim_get_api_info",
        "
            Returns a 2-tuple (Array), where item 0 is the current channel id and item
            1 is the |api-metadata| map (Dict).
        ",
    ),
    (
        "nvim_get_chan_info",
        "
            Gets information about a channel.
        ",
    ),
    (
        "nvim_get_color_by_name",
        "
            Returns the 24-bit RGB value of a |nvim_get_color_map()| color name or
            #rrggbb hexadecimal string.
        ",
    ),
    (
        "nvim_get_color_map",
        "
            Returns a map of color names and RGB values.

            Keys are color names (e.g. Aqua) and values are 24-bit RGB color values
            (e.g. 65535).
        ",
    ),
    (
        "nvim_get_commands",
        "
            Gets a map of global (non-buffer-local) Ex commands.

            Currently only |user-commands| are supported, not builtin Ex commands.
        ",
    ),
    (
        "nvim_get_context",
        "
            Gets a map of the current editor state.
        ",
    ),
    (
        "nvim_get_current_buf",
        "
            Gets the current buffer.
        ",
    ),
    (
        "nvim_get_current_line",
        "
            Gets the current line.
        ",
    ),
    (
        "nvim_get_current_tabpage",
        "
            Gets the current tabpage.
        ",
    ),
    (
        "nvim_get_current_win",
        "
            Gets the current window.
        ",
    ),
    (
        "nvim_get_hl",
        "
            Gets all or specific highlight groups in a namespace.

            Note: When the link attribute is defined in the highlight definition map,
            other attributes will not be taking effect (see |:hi-link|).
        ",
    ),
    (
        "nvim_get_hl_id_by_name",
        "
            Gets a highlight group by name

            similar to |hlID()|, but allocates a new ID if not present.
        ",
    ),
    (
        "nvim_get_hl_ns",
        "
            Gets the active highlight namespace.
        ",
    ),
    (
        "nvim_get_keymap",
        "
            Gets a list of global (non-buffer-local) |mapping| definitions.
        ",
    ),
    (
        "nvim_get_mark",
        "
            Returns a (row, col, buffer, buffername) tuple representing the position
            of the uppercase/file named mark. End of line column position is
            returned as |v:maxcol| (big number). See |mark-motions|.

            Marks are (1,0)-indexed. |api-indexing|

            Note: Lowercase name (or other buffer-local mark) is an error.
        ",
    ),
    (
        "nvim_get_mode",
        "
            Gets the current mode. |mode()| blocking is true if Nvim is waiting for
            input.
        ",
    ),
    (
        "nvim_get_namespaces",
        "
            Gets existing, non-anonymous |namespace|s.
        ",
    ),
    (
        "nvim_get_option_info2",
        "
            Gets the option information for one option from arbitrary buffer or window
        ",
    ),
    (
        "nvim_get_option_value",
        "
            Gets the value of an option. The behavior of this function matches that of
            |:set|: the local value of an option is returned if it exists; otherwise,
            the global value is returned. Local values always correspond to the
            current buffer or window, unless buf or win is set in {opts}.
        ",
    ),
    (
        "nvim_get_proc",
        "
            Gets info describing process pid.
        ",
    ),
    (
        "nvim_get_proc_children",
        "
            Gets the immediate children of process pid.
        ",
    ),
    (
        "nvim_get_runtime_file",
        "
            Finds files in runtime directories, in runtimepath order.

            name can contain wildcards. For example
            nvim_get_runtime_file(colors/*.{vim,lua}, true) will return all color
            scheme files. Always use forward slashes (/) in the search pattern for
            subdirectories regardless of platform.

            It is not an error to not find any files. An empty array is returned then.
        ",
    ),
    (
        "nvim_get_var",
        "
            Gets a global (g:) variable.
        ",
    ),
    (
        "nvim_get_vvar",
        "
            Gets a v: variable.
        ",
    ),
    (
        "nvim_input",
        "
            Queues raw user-input. Unlike |nvim_feedkeys()|, this uses a low-level
            input buffer and the call is non-blocking (input is processed
            asynchronously by the eventloop).

            To input blocks of text, |nvim_paste()| is much faster and should be
            preferred.

            On execution error: does not fail, but updates v:errmsg.

            Note: |keycodes| like <CR> are translated, so < is special. To input a
            literal <, send <LT>.

            Note: For mouse events use |nvim_input_mouse()|. The pseudokey form
            <LeftMouse><col,row> is deprecated since |api-level| 6.
        ",
    ),
    (
        "nvim_input_mouse",
        "
            Send mouse event from GUI.

            Non-blocking: does not wait on any result, but queues the event to be
            processed soon by the event loop.

            Note: Currently this does not support scripting multiple mouse events by
            calling it multiple times in a loop: the intermediate mouse positions
            will be ignored. It should be used to implement real-time mouse input
            in a GUI. The deprecated pseudokey form (<LeftMouse><col,row>) of
            |nvim_input()| has the same limitation.
        ",
    ),
    (
        "nvim_list_bufs",
        "
            Gets the current list of buffer handles

            Includes unlisted (unloaded/deleted) buffers, like :ls!. Use
            |nvim_buf_is_loaded()| to check if a buffer is loaded.
        ",
    ),
    (
        "nvim_list_chans",
        "
            Get information about all open channels.
        ",
    ),
    (
        "nvim_list_runtime_paths",
        "
            Gets the paths contained in |runtime-search-path|.
        ",
    ),
    (
        "nvim_list_tabpages",
        "
            Gets the current list of tabpage handles.
        ",
    ),
    (
        "nvim_list_uis",
        "
            Gets a list of dictionaries representing attached UIs.
        ",
    ),
    (
        "nvim_list_wins",
        "
            Gets the current list of window handles.
        ",
    ),
    (
        "nvim_load_context",
        "
            Sets the current editor state from the given |context| map.
        ",
    ),
    (
        "nvim_notify",
        "
            Notify the user with a message.

            Relays the call to vim.notify . By default forwards your message in the
            echo area but can be overridden to trigger desktop notifications.
        ",
    ),
    (
        "nvim_open_term",
        "
            Open a terminal instance in a buffer

            By default (and currently the only option) the terminal will not be
            connected to an external process. Instead, input sent on the channel will
            be echoed directly by the terminal. This is useful to display ANSI
            terminal sequences returned as part of a rpc message, or similar.

            Note: to directly initiate the terminal using the right size, display the
            buffer in a configured window before calling this. For instance, for a
            floating display, first create an empty buffer using |nvim_create_buf()|,
            then display it using |nvim_open_win()|, and then call this function. Then
            |nvim_chan_send()| can be called immediately to process sequences in a
            virtual terminal having the intended size.
        ",
    ),
    (
        "nvim_open_win",
        "
            Opens a new split window, or a floating window if relative is specified,
            or an external window (managed by the UI) if external is specified.

            Floats are windows that are drawn above the split layout, at some anchor
            position in some other window. Floats can be drawn internally or by
            external GUI with the |ui-multigrid| extension. External windows are only
            supported with multigrid GUIs, and are displayed as separate top-level
            windows.

            For a general overview of floats, see |api-floatwin|.

            The width and height of the new window must be specified when opening
            a floating window, but are optional for normal windows.

            If relative and external are omitted, a normal split window is
            created. The win property determines which window will be split. If no
            win is provided or win == 0, a window will be created adjacent to the
            current window. If -1 is provided, a top-level split will be created.
            vertical and split are only valid for normal windows, and are used to
            control split direction. For vertical, the exact direction is determined
            by splitright and splitbelow. Split windows cannot have
            bufpos/row/col/border/title/footer properties.

            With relative=editor (row=0,col=0) refers to the top-left corner of the
            screen-grid and (row=Lines-1,col=Columns-1) refers to the bottom-right
            corner. Fractional values are allowed, but the builtin implementation
            (used by non-multigrid UIs) will always round down to nearest integer.

            Out-of-bounds values, and configurations that make the float not fit
            inside the main editor, are allowed. The builtin implementation truncates
            values so floats are fully within the main screen grid. External GUIs
            could let floats hover outside of the main window like a tooltip, but this
            should not be used to specify arbitrary WM screen positions.
        ",
    ),
    (
        "nvim_out_write",
        "
            Writes a message to the Vim output buffer. Does not append \\n, the
            message is buffered (will not display) until a linefeed is written.
        ",
    ),
    (
        "nvim_paste",
        "
            Pastes at cursor (in any mode), and sets redo so dot (|.|) will repeat
            the input. UIs call this to implement paste, but it is also intended for
            use by scripts to input large, dot-repeatable blocks of text (as opposed
            to |nvim_input()| which is subject to mappings/events and is thus much
            slower).

            Invokes the |vim.paste()| handler, which handles each mode appropriately.

            Errors (nomodifiable, vim.paste() failure, …) are reflected in err
            but do not affect the return value (which is strictly decided by
            vim.paste()). On error or cancel, subsequent calls are ignored
            (drained) until the next paste is initiated (phase 1 or -1).
        ",
    ),
    (
        "nvim_parse_cmd",
        "
            Parse command line.

            Does not check the validity of command arguments.
        ",
    ),
    (
        "nvim_parse_expression",
        "
            Parse a Vimscript expression.
        ",
    ),
    (
        "nvim_put",
        "
            Puts text at cursor, in any mode. For dot-repeatable input, use
            |nvim_paste()|.

            Compare |:put| and |p| which are always linewise.
        ",
    ),
    (
        "nvim_replace_termcodes",
        "
            Replaces terminal codes and |keycodes| (<CR>, <Esc>, ...) in a string with
            the internal representation.
        ",
    ),
    (
        "nvim_select_popupmenu_item",
        "
            Selects an item in the completion popup menu.

            If neither |ins-completion| nor |cmdline-completion| popup menu is active
            this API call is silently ignored. Useful for an external UI using
            |ui-popupmenu| to control the popup menu with the mouse. Can also be used
            in a mapping; use <Cmd> |:map-cmd| or a Lua mapping to ensure the mapping
            does not end completion mode.
        ",
    ),
    (
        "nvim_set_client_info",
        "
            Self-identifies the client.

            The client/plugin/application should call this after connecting, to
            provide hints about its identity and purpose, for debugging and
            orchestration.

            Can be called more than once; the caller should merge old info if
            appropriate. Example: library first identifies the channel, then a plugin
            using that library later identifies itself.

            Note: Something is better than nothing. You do not need to include all the
            fields.
        ",
    ),
    (
        "nvim_set_current_buf",
        "
            Sets the current buffer.
        ",
    ),
    (
        "nvim_set_current_dir",
        "
            Changes the global working directory.
        ",
    ),
    (
        "nvim_set_current_line",
        "
            Sets the current line.
        ",
    ),
    (
        "nvim_set_current_tabpage",
        "
            Sets the current tabpage.
        ",
    ),
    (
        "nvim_set_current_win",
        "
            Sets the current window.
        ",
    ),
    (
        "nvim_set_decoration_provider",
        "
            Set or change decoration provider for a |namespace|

            This is a very general purpose interface for having Lua callbacks being
            triggered during the redraw code.

            The expected usage is to set |extmarks| for the currently redrawn buffer.
            |nvim_buf_set_extmark()| can be called to add marks on a per-window or
            per-lines basis. Use the ephemeral key to only use the mark for the
            current screen redraw (the callback will be called again for the next
            redraw).

            Note: this function should not be called often. Rather, the callbacks
            themselves can be used to throttle unneeded callbacks. the on_start
            callback can return false to disable the provider until the next redraw.
            Similarly, return false in on_win will skip the on_line calls for
            that window (but any extmarks set in on_win will still be used). A
            plugin managing multiple sources of decoration should ideally only set one
            provider, and merge the sources internally. You can use multiple ns_id
            for the extmarks set/modified inside the callback anyway.

            Note: doing anything other than setting extmarks is considered
            experimental. Doing things like changing options are not explicitly
            forbidden, but is likely to have unexpected consequences (such as 100% CPU
            consumption). Doing vim.rpcnotify should be OK, but vim.rpcrequest is
            quite dubious for the moment.

            Note: It is not allowed to remove or update extmarks in on_line
            callbacks.
        ",
    ),
    (
        "nvim_set_hl",
        "
            Sets a highlight group.

            Note: Unlike the :highlight command which can update a highlight group,
            this function completely replaces the definition. For example:
            nvim_set_hl(0, Visual, {}) will clear the highlight group 'Visual'.

            Note: The fg and bg keys also accept the string values fg or bg
            which act as aliases to the corresponding foreground and background
            values of the Normal group. If the Normal group has not been defined,
            using these values results in an error.

            Note: If link is used in combination with other attributes; only the
            link will take effect (see |:hi-link|).
        ",
    ),
    (
        "nvim_set_hl_ns",
        "
            Set active namespace for highlights defined with |nvim_set_hl()|. This can
            be set for a single window, see |nvim_win_set_hl_ns()|.
        ",
    ),
    (
        "nvim_set_hl_ns_fast",
        "
            Set active namespace for highlights defined with |nvim_set_hl()| while
            redrawing.

            This function meant to be called while redrawing, primarily from
            |nvim_set_decoration_provider()| on_win and on_line callbacks, which are
            allowed to change the namespace during a redraw cycle.
        ",
    ),
    (
        "nvim_set_keymap",
        "
            Sets a global |mapping| for the given mode.

            To set a buffer-local mapping, use |nvim_buf_set_keymap()|.

            Unlike |:map|, leading/trailing whitespace is accepted as part of the
            {lhs} or {rhs}. Empty {rhs} is <Nop>. |keycodes| are replaced as usual.
        ",
    ),
    (
        "nvim_set_option_value",
        "
            Sets the value of an option. The behavior of this function matches that of
            |:set|: for global-local options, both the global and local value are set
            unless otherwise specified with {scope}.

            Note the options {win} and {buf} cannot be used together.
        ",
    ),
    (
        "nvim_set_var",
        "
            Sets a global (g:) variable
        ",
    ),
    (
        "nvim_set_vvar",
        "
            Sets a v: variable, if it is not readonly.
        ",
    ),
    (
        "nvim_strwidth",
        "
            Calculates the number of display cells occupied by text. Control
            characters including <Tab> count as one cell.
        ",
    ),
    (
        "nvim_tabpage_del_var",
        "
            Removes a tab-scoped (t:) variable
        ",
    ),
    (
        "nvim_tabpage_get_number",
        "
            Gets the tabpage number
        ",
    ),
    (
        "nvim_tabpage_get_var",
        "
            Gets a tab-scoped (t:) variable
        ",
    ),
    (
        "nvim_tabpage_get_win",
        "
            Gets the current window in a tabpage
        ",
    ),
    (
        "nvim_tabpage_is_valid",
        "
            Checks if a tabpage is valid
        ",
    ),
    (
        "nvim_tabpage_list_wins",
        "
            Gets the windows in a tabpage
        ",
    ),
    (
        "nvim_tabpage_set_var",
        "
            Sets a tab-scoped (t:) variable
        ",
    ),
    (
        "nvim_tabpage_set_win",
        "
            Sets the current window in a tabpage
        ",
    ),
    (
        "nvim_ui_attach",
        "
            Activates UI events on the channel.

            Entry point of all UI clients. Allows |--embed| to continue startup.
            Implies that the client is ready to show the UI. Adds the client to the
            list of UIs. |nvim_list_uis()|

            Note: If multiple UI clients are attached, the global screen dimensions
            degrade to the smallest client. E.g. if client A requests 80x40 but
            client B requests 200x100, the global screen has size 80x40.
        ",
    ),
    (
        "nvim_ui_detach",
        "
            Deactivates UI events on the channel.

            Removes the client from the list of UIs. |nvim_list_uis()|
        ",
    ),
    (
        "nvim_ui_pum_set_bounds",
        "
            Tells Nvim the geometry of the popupmenu, to align floating windows with
            an external popup menu.

            Note that this method is not to be confused with
            |nvim_ui_pum_set_height()|, which sets the number of visible items in the
            popup menu, while this function sets the bounding box of the popup menu,
            including visual elements such as borders and sliders. Floats need not use
            the same font size, nor be anchored to exact grid corners, so one can set
            floating-point numbers to the popup menu geometry.
        ",
    ),
    (
        "nvim_ui_pum_set_height",
        "
            Tells Nvim the number of elements displaying in the popupmenu, to decide
            <PageUp> and <PageDown> movement.
        ",
    ),
    (
        "nvim_ui_set_focus",
        "
            Tells the nvim server if focus was gained or lost by the GUI
        ",
    ),
    (
        "nvim_ui_set_option",
        "
            Set a UI option.
        ",
    ),
    (
        "nvim_ui_term_event",
        "
            Tells Nvim when a terminal event has occurred

            The following terminal events are supported:
            * termresponse: The terminal sent an OSC or DCS response sequence to
              Nvim. The payload is the received response. Sets |v:termresponse| and
              fires |TermResponse|.
        ",
    ),
    (
        "nvim_ui_try_resize",
        "
            Try to resize the UI.
        ",
    ),
    (
        "nvim_ui_try_resize_grid",
        "
            Tell Nvim to resize a grid. Triggers a grid_resize event with the
            requested grid size or the maximum size if it exceeds size limits.

            On invalid grid handle, fails with error.
        ",
    ),
    (
        "nvim_win_call",
        "
            Calls a function with window as temporary current window.
        ",
    ),
    (
        "nvim_win_close",
        "
            Closes the window (like |:close| with a |window-ID|).
        ",
    ),
    (
        "nvim_win_del_var",
        "
            Removes a window-scoped (w:) variable
        ",
    ),
    (
        "nvim_win_get_buf",
        "
            Gets the current buffer in a window
        ",
    ),
    (
        "nvim_win_get_cursor",
        "
            Gets the (1,0)-indexed, buffer-relative cursor position for a given window
            (different windows showing the same buffer have independent cursor
            positions).
        ",
    ),
    (
        "nvim_win_get_height",
        "
            Gets the window height
        ",
    ),
    (
        "nvim_win_get_number",
        "
            Gets the window number
        ",
    ),
    (
        "nvim_win_get_position",
        "
            Gets the window position in display cells. First position is zero.
        ",
    ),
    (
        "nvim_win_get_tabpage",
        "
            Gets the window tabpage
        ",
    ),
    (
        "nvim_win_get_var",
        "
            Gets a window-scoped (w:) variable
        ",
    ),
    (
        "nvim_win_get_width",
        "
            Gets the window width
        ",
    ),
    (
        "nvim_win_hide",
        "
            Closes the window and hide the buffer it contains (like |:hide| with a
            |window-ID|).

            Like |:hide| the buffer becomes hidden unless another window is editing
            it, or bufhidden is unload, delete or wipe as opposed to |:close|
            or |nvim_win_close()|, which will close the buffer.
        ",
    ),
    (
        "nvim_win_is_valid",
        "
            Checks if a window is valid
        ",
    ),
    (
        "nvim_win_set_buf",
        "
            Sets the current buffer in a window, without side effects
        ",
    ),
    (
        "nvim_win_set_cursor",
        "
            Sets the (1,0)-indexed cursor position in the window. This scrolls the
            window even if it is not the current one.
        ",
    ),
    (
        "nvim_win_set_height",
        "
            Sets the window height.
        ",
    ),
    (
        "nvim_win_set_hl_ns",
        "
            Set highlight namespace for a window. This will use highlights defined
            with |nvim_set_hl()| for this namespace, but fall back to global
            highlights (ns=0) when missing.

            This takes precedence over the winhighlight option.
        ",
    ),
    (
        "nvim_win_set_var",
        "
            Sets a window-scoped (w:) variable
        ",
    ),
    (
        "nvim_win_set_width",
        "
            Sets the window width. This will only succeed if the screen is split
            vertically.
        ",
    ),
    (
        "nvim_win_text_height",
        "
        Computes the number of screen lines occupied by a range of text in a given
        window. Works for off-screen text and takes folds into account.

        Diff filler or virtual lines above a line are counted as a part of that
        line, unless the line is on start_row and start_vcol is specified.

        Diff filler or virtual lines below the last buffer line are counted in the
        result when end_row is omitted.

        Line indexing is similar to `nvim_buf_get_text()`.
        ",
    ),
    (
        "nvim_get_autocmds",
        "
        Get all autocommands that match the corresponding {opts}.

        These examples will get autocommands matching ALL the given criteria:
        - Matches all criteria
        - All commands from one group
        
        NOTE: When multiple patterns or events are provided, it will find all the
        autocommands that match any combination of them.
        ",
    ),
    (
        "nvim_exec_autocmds",
        "
        Execute all autocommands for {event} that match the corresponding {opts}
        `autocmd-execute`.
        ",
    ),
    (
        "nvim_buf_create_user_command",
        "
        Creates a buffer-local command `user-commands`.
        ",
    ),
    (
        "nvim_buf_del_user_command",
        "
        Delete a buffer-local user-defined command.

        Only commands created with `:command-buffer` or
        `nvim_buf_create_user_command()` can be deleted with this function.
        ",
    ),
    (
        "nvim_buf_set_extmark",
        "
        Creates or updates an extmark.

        By default a new extmark is created when no id is passed in, but it is
        also possible to create a new mark by passing in a previously unused id or
        move an existing mark by passing in its id. The caller must then keep
        track of existing and unused ids itself. (Useful over RPC, to avoid
        waiting for the return value.)

        Using the optional arguments, it is possible to use this to highlight a
        range of text, and also to associate virtual text to the mark.

        If present, the position defined by end_col and end_row should be
        after the start position in order for the extmark to cover a range. An
        earlier end position is not an error, but then it behaves like an empty
        range (no highlighting).
        ",
    ),
    (
        "nvim_call_function",
        "
        Calls a Vimscript function with the given arguments.

        On execution error: fails with Vimscript error, updates v:errmsg.
        ",
    ),
    (
        "nvim_call_dict_function",
        "
        Calls a Vimscript `Dictionary-function` with the given arguments.

        On execution error: fails with Vimscript error, updates v:errmsg.
        ",
    ),
    (
        "nvim_win_set_config",
        "
        Configures window layout. Cannot be used to move the last window in a
        tabpage to a different one.

        When reconfiguring a window, absent option keys will not be changed.
        row/col and relative must be reconfigured together.
        ",
    ),
    (
        "nvim_win_get_config",
        "
        Gets window configuration.

        The returned value may be given to `nvim_open_win()`.

        relative is empty for normal windows.
        ",
    ),
];
