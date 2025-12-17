#![allow(clippy::needless_question_mark)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::doc_lazy_continuation)]
use std::collections::HashMap;

use mrpc::Value;
use serde::{de::DeserializeOwned, Serialize};
use tracing::trace;

use super::{opts, types::*};
use crate::error::Result;
const NO_PARAMS: [(); 0] = [];
#[derive(Clone, Debug)]
/// Generated bindings for Neovim's MessagePack-RPC API.
pub struct NvimApi {
    pub(crate) rpc_sender: mrpc::RpcSender,
}
impl NvimApi {
    /// Make a typed request over the MessagePack-RPC protocol.
    pub async fn rpc_call<Req, Resp>(&self, method: &str, req: Req) -> Result<Resp, mrpc::RpcError>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
    {
        let params = mrpc::serialize_params(&req)?;
        trace!("send request: {:?} {:?}", method, params);
        let ret = self.rpc_sender.send_request(method, &params).await?;
        trace!("got response for {:?}: {:?}", method, ret);
        mrpc::deserialize_response(&ret)
    }
    /// Send a typed notification over the MessagePack-RPC protocol.
    pub async fn rpc_notify<Req>(&self, method: &str, req: Req) -> Result<(), mrpc::RpcError>
    where
        Req: Serialize,
    {
        let params = mrpc::serialize_params(&req)?;
        trace!("send notification: {:?} {:?}", method, params);
        self.rpc_sender.send_notification(method, &params).await
    }
    /// Make a raw request over the MessagePack-RPC protocol.
    pub async fn raw_request(
        &self,
        method: &str,
        params: &[mrpc::Value],
    ) -> Result<mrpc::Value, mrpc::RpcError> {
        trace!("send request: {:?} {:?}", method, params);
        let ret = self.rpc_sender.send_request(method, params).await;
        trace!("got response for {:?}: {:?}", method, ret);
        ret
    }
    /// Send a raw notification over the MessagePack-RPC protocol.
    pub async fn raw_notify(
        &self,
        method: &str,
        params: &[mrpc::Value],
    ) -> Result<(), mrpc::RpcError> {
        trace!("send notification: {:?} {:?}", method, params);
        self.rpc_sender.send_notification(method, params).await
    }
    /// Get all autocommands that match the corresponding {opts}.
    ///
    /// These examples will get autocommands matching ALL the given criteria:
    /// - Matches all criteria
    /// - All commands from one group
    ///
    /// NOTE: When multiple patterns or events are provided, it will find all the
    /// autocommands that match any combination of them.
    pub async fn get_autocmds(&self, opts: HashMap<String, Value>) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let req = opts;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_autocmds", req).await?)
    }
    /// Creates an autocommand event handler, defined by callback (Lua
    /// function or Vimscript function name string) or command (Ex command
    /// string).
    ///
    /// Note: pattern is NOT automatically expanded (unlike with :autocmd),
    /// thus names like $HOME and ~ must be expanded explicitly.
    pub async fn create_autocmd(&self, event: &[Event], opts: opts::CreateAutocmd) -> Result<i64> {
        #[allow(unused_variables)]
        let req = (event, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_create_autocmd", req).await?)
    }
    /// Deletes an autocommand by id.
    pub async fn del_autocmd(&self, id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let req = id;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_del_autocmd", req).await?)
    }
    /// Clears all autocommands selected by {opts}. To delete autocmds see
    /// `nvim_del_autocmd()`.
    pub async fn clear_autocmds(&self, opts: opts::ClearAutocmds) -> Result<()> {
        #[allow(unused_variables)]
        let req = opts;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_clear_autocmds", req).await?)
    }
    /// Create or get an autocommand group autocmd-groups.
    pub async fn create_augroup(&self, name: &str, opts: HashMap<String, Value>) -> Result<i64> {
        #[allow(unused_variables)]
        let req = (name, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_create_augroup", req).await?)
    }
    /// Delete an autocommand group by id.
    ///
    /// To get a group id one can use nvim_get_autocmds().
    ///
    /// NOTE: behavior differs from :augroup-delete. When deleting a group,
    /// autocommands contained in this group will also be deleted and cleared.
    /// This group will no longer exist.
    pub async fn del_augroup_by_id(&self, id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let req = id;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_del_augroup_by_id", req).await?)
    }
    /// Delete an autocommand group by name.
    ///
    /// NOTE: behavior differs from :augroup-delete. When deleting a group,
    /// autocommands contained in this group will also be deleted and cleared.
    /// This group will no longer exist.
    pub async fn del_augroup_by_name(&self, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = name;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_del_augroup_by_name", req).await?)
    }
    /// Execute all autocommands for {event} that match the corresponding {opts}
    /// `autocmd-execute`.
    pub async fn exec_autocmds(&self, event: &[Event], opts: opts::ExecAutocmds) -> Result<()> {
        #[allow(unused_variables)]
        let req = (event, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_exec_autocmds", req).await?)
    }
    /// Returns the number of lines in the given buffer.
    pub async fn buf_line_count(&self, buffer: &Buffer) -> Result<i64> {
        #[allow(unused_variables)]
        let req = buffer;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_line_count", req).await?)
    }
    /// Activates buffer-update events on a channel, or as Lua callbacks.
    pub async fn buf_attach(
        &self,
        buffer: &Buffer,
        send_buffer: bool,
        opts: HashMap<String, Value>,
    ) -> Result<bool> {
        #[allow(unused_variables)]
        let req = (buffer, send_buffer, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_attach", req).await?)
    }
    /// Deactivates buffer-update events on the channel.
    pub async fn buf_detach(&self, buffer: &Buffer) -> Result<bool> {
        #[allow(unused_variables)]
        let req = buffer;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_detach", req).await?)
    }
    /// Gets a line-range from the buffer.
    ///
    /// Indexing is zero-based, end-exclusive. Negative indices are interpreted as
    /// length+1+index: -1 refers to the index past the end. So to get the last
    /// element use start=-2 and end=-1.
    ///
    /// Out-of-bounds indices are clamped to the nearest valid value, unless
    /// strict_indexing is set.
    pub async fn buf_get_lines(
        &self,
        buffer: &Buffer,
        start: i64,
        end: i64,
        strict_indexing: bool,
    ) -> Result<Vec<String>> {
        #[allow(unused_variables)]
        let req = (buffer, start, end, strict_indexing);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_get_lines", req).await?)
    }
    /// Sets (replaces) a line-range in the buffer.
    ///
    /// Indexing is zero-based, end-exclusive. Negative indices are interpreted as
    /// length+1+index: -1 refers to the index past the end. So to change or
    /// delete the last element use start=-2 and end=-1.
    ///
    /// To insert lines at a given index, set start and end to the same index.
    /// To delete a range of lines, set replacement to an empty array.
    ///
    /// Out-of-bounds indices are clamped to the nearest valid value, unless
    /// strict_indexing is set.
    pub async fn buf_set_lines(
        &self,
        buffer: &Buffer,
        start: i64,
        end: i64,
        strict_indexing: bool,
        replacement: Vec<String>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let req = (buffer, start, end, strict_indexing, replacement);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_set_lines", req).await?)
    }
    /// Sets (replaces) a range in the buffer
    ///
    /// This is recommended over nvim_buf_set_lines() when only modifying parts
    /// of a line, as extmarks will be preserved on non-modified parts of the
    /// touched lines.
    ///
    /// Indexing is zero-based. Row indices are end-inclusive, and column indices
    /// are end-exclusive.
    ///
    /// To insert text at a given (row, column) location, use
    /// start_row = end_row = row and start_col = end_col = col. To delete the
    /// text in a range, use replacement = {}.
    ///
    /// Note: Prefer nvim_buf_set_lines() (for performance) to add or delete
    /// entire lines.
    /// Note: Prefer nvim_paste() or nvim_put() to insert (instead of replace)
    /// text at cursor.
    pub async fn buf_set_text(
        &self,
        buffer: &Buffer,
        start_row: i64,
        start_col: i64,
        end_row: i64,
        end_col: i64,
        replacement: Vec<String>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let req = (buffer, start_row, start_col, end_row, end_col, replacement);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_set_text", req).await?)
    }
    /// Gets a range from the buffer.
    ///
    /// This differs from |nvim_buf_get_lines()| in that it allows retrieving only
    /// portions of a line.
    ///
    /// Indexing is zero-based. Row indices are end-inclusive, and column indices
    /// are end-exclusive.
    ///
    /// Prefer |nvim_buf_get_lines()| when retrieving entire lines.
    pub async fn buf_get_text(
        &self,
        buffer: &Buffer,
        start_row: i64,
        start_col: i64,
        end_row: i64,
        end_col: i64,
        opts: HashMap<String, Value>,
    ) -> Result<Vec<String>> {
        #[allow(unused_variables)]
        let req = (buffer, start_row, start_col, end_row, end_col, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_get_text", req).await?)
    }
    /// Returns the byte offset of a line (0-indexed). |api-indexing|
    ///
    /// Line 1 (index=0) has offset 0. UTF-8 bytes are counted. EOL is one byte.
    /// fileformat and fileencoding are ignored. The line index just after the
    /// last line gives the total byte-count of the buffer. A final EOL byte is
    /// counted if it would be written, see eol.
    ///
    /// Unlike |line2byte()|, throws error for out-of-bounds indexing. Returns -1
    /// for unloaded buffer.
    pub async fn buf_get_offset(&self, buffer: &Buffer, index: i64) -> Result<i64> {
        #[allow(unused_variables)]
        let req = (buffer, index);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_get_offset", req).await?)
    }
    /// Gets a buffer-scoped (b:) variable.
    pub async fn buf_get_var<T>(&self, buffer: &Buffer, name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let req = (buffer, name);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_get_var", req).await?)
    }
    /// Gets a changed tick of a buffer
    pub async fn buf_get_changedtick(&self, buffer: &Buffer) -> Result<i64> {
        #[allow(unused_variables)]
        let req = buffer;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_get_changedtick", req).await?)
    }
    /// Gets a list of buffer-local |mapping| definitions.
    pub async fn buf_get_keymap(
        &self,
        buffer: &Buffer,
        mode: &str,
    ) -> Result<Vec<HashMap<String, Value>>> {
        #[allow(unused_variables)]
        let req = (buffer, mode);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_get_keymap", req).await?)
    }
    /// Sets a buffer-local |mapping| for the given mode.
    pub async fn buf_set_keymap(
        &self,
        buffer: &Buffer,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: HashMap<String, Value>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let req = (buffer, mode, lhs, rhs, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_set_keymap", req).await?)
    }
    /// Unmaps a buffer-local |mapping| for the given mode.
    pub async fn buf_del_keymap(&self, buffer: &Buffer, mode: &str, lhs: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = (buffer, mode, lhs);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_del_keymap", req).await?)
    }
    /// Sets a buffer-scoped (b:) variable
    pub async fn buf_set_var<T>(&self, buffer: &Buffer, name: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let req = (buffer, name, value);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_set_var", req).await?)
    }
    /// Removes a buffer-scoped (b:) variable
    pub async fn buf_del_var(&self, buffer: &Buffer, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = (buffer, name);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_del_var", req).await?)
    }
    /// Gets the full file name for the buffer
    pub async fn buf_get_name(&self, buffer: &Buffer) -> Result<String> {
        #[allow(unused_variables)]
        let req = buffer;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_get_name", req).await?)
    }
    /// Sets the full file name for a buffer, like :file_f
    pub async fn buf_set_name(&self, buffer: &Buffer, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = (buffer, name);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_set_name", req).await?)
    }
    /// Checks if a buffer is valid and loaded. See |api-buffer| for more info
    /// about unloaded buffers.
    pub async fn buf_is_loaded(&self, buffer: &Buffer) -> Result<bool> {
        #[allow(unused_variables)]
        let req = buffer;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_is_loaded", req).await?)
    }
    /// Deletes the buffer. See |:bwipeout|
    pub async fn buf_delete(&self, buffer: &Buffer, opts: opts::BufDelete) -> Result<()> {
        #[allow(unused_variables)]
        let req = (buffer, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_delete", req).await?)
    }
    /// Checks if a buffer is valid.
    ///
    /// Note: Even if a buffer is valid it may have been unloaded. See |api-buffer|
    /// for more info about unloaded buffers.
    pub async fn buf_is_valid(&self, buffer: &Buffer) -> Result<bool> {
        #[allow(unused_variables)]
        let req = buffer;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_is_valid", req).await?)
    }
    /// Deletes a named mark in the buffer. See |mark-motions|.
    ///
    /// Note: only deletes marks set in the buffer, if the mark is not set in the
    /// buffer it will return false.
    pub async fn buf_del_mark(&self, buffer: &Buffer, name: &str) -> Result<bool> {
        #[allow(unused_variables)]
        let req = (buffer, name);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_del_mark", req).await?)
    }
    /// Sets a named mark in the given buffer, all marks are allowed
    /// file/uppercase, visual, last change, etc. See mark-motions.
    ///
    /// Marks are (1,0)-indexed. api-indexing
    ///
    /// Note: Passing 0 as line deletes the mark
    pub async fn buf_set_mark(
        &self,
        buffer: &Buffer,
        name: &str,
        line: i64,
        col: i64,
        opts: HashMap<String, Value>,
    ) -> Result<bool> {
        #[allow(unused_variables)]
        let req = (buffer, name, line, col, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_set_mark", req).await?)
    }
    /// Returns a (row,col) tuple representing the position of the named mark.
    /// End of line column position is returned as |v:maxcol| (big number).
    /// See |mark-motions|.
    ///
    /// Marks are (1,0)-indexed. |api-indexing|
    pub async fn buf_get_mark(&self, buffer: &Buffer, name: &str) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let req = (buffer, name);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_get_mark", req).await?)
    }
    /// Parse command line.
    ///
    /// Does not check the validity of command arguments.
    pub async fn parse_cmd(
        &self,
        str: &str,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let req = (str, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_parse_cmd", req).await?)
    }
    /// Executes an Ex command.
    ///
    /// Unlike `nvim_command()` this command takes a structured Dict instead of a
    /// String. This allows for easier construction and manipulation of an Ex
    /// command. This also allows for things such as having spaces inside a
    /// command argument, expanding filenames in a command that otherwise does not
    /// expand filenames, etc. Command arguments may also be Number, Boolean or
    /// String.
    ///
    /// The first argument may also be used instead of count for commands that
    /// support it in order to make their usage simpler. For example, instead of
    /// `vim.cmd.bdelete{ count = 2 }`, you may do `vim.cmd.bdelete(2)`.
    ///
    /// On execution error: fails with Vimscript error, updates v:errmsg.
    pub async fn cmd(
        &self,
        cmd: HashMap<String, Value>,
        opts: HashMap<String, Value>,
    ) -> Result<String> {
        #[allow(unused_variables)]
        let req = (cmd, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_cmd", req).await?)
    }
    /// Creates a global user-commands command.
    pub async fn create_user_command<T>(
        &self,
        name: &str,
        command: T,
        opts: HashMap<String, Value>,
    ) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let req = (name, command, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_create_user_command", req).await?)
    }
    /// Delete a user-defined command.
    pub async fn del_user_command(&self, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = name;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_del_user_command", req).await?)
    }
    /// Creates a buffer-local command `user-commands`.
    pub async fn buf_create_user_command<T>(
        &self,
        buffer: &Buffer,
        name: &str,
        command: T,
        opts: HashMap<String, Value>,
    ) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let req = (buffer, name, command, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_create_user_command", req).await?)
    }
    /// Delete a buffer-local user-defined command.
    ///
    /// Only commands created with `:command-buffer` or
    /// `nvim_buf_create_user_command()` can be deleted with this function.
    pub async fn buf_del_user_command(&self, buffer: &Buffer, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = (buffer, name);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_del_user_command", req).await?)
    }
    /// Gets a map of global (non-buffer-local) Ex commands.
    ///
    /// Currently only |user-commands| are supported, not builtin Ex commands.
    pub async fn get_commands(
        &self,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let req = opts;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_commands", req).await?)
    }
    /// Gets a map of buffer-local |user-commands|.
    pub async fn buf_get_commands(
        &self,
        buffer: &Buffer,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let req = (buffer, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_get_commands", req).await?)
    }
    /// Creates a new namespace or gets an existing one.
    ///
    /// Namespaces are used for buffer highlights and virtual text, see
    /// nvim_buf_add_highlight() and nvim_buf_set_extmark().
    ///
    /// Namespaces can be named or anonymous. If name matches an existing
    /// namespace, the associated id is returned. If name is an empty string a
    /// new, anonymous namespace is created.
    pub async fn create_namespace(&self, name: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let req = name;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_create_namespace", req).await?)
    }
    /// Gets existing, non-anonymous |namespace|s.
    pub async fn get_namespaces(&self) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_namespaces", req).await?)
    }
    /// Gets the position (0-indexed) of an |extmark|.
    pub async fn buf_get_extmark_by_id(
        &self,
        buffer: &Buffer,
        ns_id: i64,
        id: i64,
        opts: HashMap<String, Value>,
    ) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let req = (buffer, ns_id, id, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_get_extmark_by_id", req).await?)
    }
    /// Gets |extmarks| in traversal order from a |charwise| region defined by
    /// buffer positions (inclusive, 0-indexed |api-indexing|).
    ///
    /// Region can be given as (row,col) tuples, or valid extmark ids (whose
    /// positions define the bounds). 0 and -1 are understood as (0,0) and (-1,-1)
    /// respectively.
    ///
    /// If end is less than start, traversal works backwards. (Useful with
    /// limit, to get the first marks prior to a given position.)
    ///
    /// Note: when using extmark ranges (marks with a end_row/end_col position)
    /// the overlap option might be useful. Otherwise only the start position of
    /// an extmark will be considered.
    ///
    /// Note: legacy signs placed through the |:sign| commands are implemented as
    /// extmarks and will show up here. Their details array will contain a
    /// sign_name field.
    pub async fn buf_get_extmarks<T, U>(
        &self,
        buffer: &Buffer,
        ns_id: i64,
        start: T,
        end: U,
        opts: HashMap<String, Value>,
    ) -> Result<Vec<Value>>
    where
        T: Serialize,
        U: Serialize,
    {
        #[allow(unused_variables)]
        let req = (buffer, ns_id, start, end, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_get_extmarks", req).await?)
    }
    /// Creates or updates an extmark.
    ///
    /// By default a new extmark is created when no id is passed in, but it is
    /// also possible to create a new mark by passing in a previously unused id or
    /// move an existing mark by passing in its id. The caller must then keep
    /// track of existing and unused ids itself. (Useful over RPC, to avoid
    /// waiting for the return value.)
    ///
    /// Using the optional arguments, it is possible to use this to highlight a
    /// range of text, and also to associate virtual text to the mark.
    ///
    /// If present, the position defined by end_col and end_row should be
    /// after the start position in order for the extmark to cover a range. An
    /// earlier end position is not an error, but then it behaves like an empty
    /// range (no highlighting).
    pub async fn buf_set_extmark(
        &self,
        buffer: &Buffer,
        ns_id: i64,
        line: i64,
        col: i64,
        opts: HashMap<String, Value>,
    ) -> Result<i64> {
        #[allow(unused_variables)]
        let req = (buffer, ns_id, line, col, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_set_extmark", req).await?)
    }
    /// Removes an extmark.
    pub async fn buf_del_extmark(&self, buffer: &Buffer, ns_id: i64, id: i64) -> Result<bool> {
        #[allow(unused_variables)]
        let req = (buffer, ns_id, id);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_del_extmark", req).await?)
    }
    /// Adds a highlight to buffer.
    ///
    /// Useful for plugins that dynamically generate highlights to a buffer (like
    /// a semantic highlighter or linter). The function adds a single highlight to
    /// a buffer. Unlike `matchaddpos()` highlights follow changes to line
    /// numbering (as lines are inserted/removed above the highlighted line), like
    /// signs and marks do.
    ///
    /// Namespaces are used for batch deletion/updating of a set of highlights. To
    /// create a namespace, use `nvim_create_namespace()` which returns a
    /// namespace id. Pass it in to this function as ns_id to add highlights to
    /// the namespace. All highlights in the same namespace can then be cleared
    /// with single call to `nvim_buf_clear_namespace()`. If the highlight never
    /// will be deleted by an API call, pass ns_id = -1.
    ///
    /// As a shorthand, ns_id = 0 can be used to create a new namespace for the
    /// highlight, the allocated id is then returned. If hl_group is the empty
    /// string no highlight is added, but a new ns_id is still returned. This is
    /// supported for backwards compatibility, new code should use
    /// `nvim_create_namespace()` to create a new empty namespace.
    pub async fn buf_add_highlight(
        &self,
        buffer: &Buffer,
        ns_id: i64,
        hl_group: &str,
        line: i64,
        col_start: i64,
        col_end: i64,
    ) -> Result<i64> {
        #[allow(unused_variables)]
        let req = (buffer, ns_id, hl_group, line, col_start, col_end);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_add_highlight", req).await?)
    }
    /// Clears namespaced objects (highlights, extmarks, virtual text) from a
    /// region.
    ///
    /// Lines are 0-indexed. `api-indexing` To clear the namespace in the entire
    /// buffer, specify line_start=0 and line_end=-1.
    pub async fn buf_clear_namespace(
        &self,
        buffer: &Buffer,
        ns_id: i64,
        line_start: i64,
        line_end: i64,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let req = (buffer, ns_id, line_start, line_end);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_buf_clear_namespace", req).await?)
    }
    /// Set or change decoration provider for a |namespace|
    ///
    /// This is a very general purpose interface for having Lua callbacks being
    /// triggered during the redraw code.
    ///
    /// The expected usage is to set |extmarks| for the currently redrawn buffer.
    /// |nvim_buf_set_extmark()| can be called to add marks on a per-window or
    /// per-lines basis. Use the ephemeral key to only use the mark for the
    /// current screen redraw (the callback will be called again for the next
    /// redraw).
    ///
    /// Note: this function should not be called often. Rather, the callbacks
    /// themselves can be used to throttle unneeded callbacks. the on_start
    /// callback can return false to disable the provider until the next redraw.
    /// Similarly, return false in on_win will skip the on_line calls for
    /// that window (but any extmarks set in on_win will still be used). A
    /// plugin managing multiple sources of decoration should ideally only set one
    /// provider, and merge the sources internally. You can use multiple ns_id
    /// for the extmarks set/modified inside the callback anyway.
    ///
    /// Note: doing anything other than setting extmarks is considered
    /// experimental. Doing things like changing options are not explicitly
    /// forbidden, but is likely to have unexpected consequences (such as 100% CPU
    /// consumption). Doing vim.rpcnotify should be OK, but vim.rpcrequest is
    /// quite dubious for the moment.
    ///
    /// Note: It is not allowed to remove or update extmarks in on_line
    /// callbacks.
    pub async fn set_decoration_provider(
        &self,
        ns_id: i64,
        opts: HashMap<String, Value>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let req = (ns_id, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_set_decoration_provider", req).await?)
    }
    /// Gets the value of an option. The behavior of this function matches that of
    /// |:set|: the local value of an option is returned if it exists; otherwise,
    /// the global value is returned. Local values always correspond to the
    /// current buffer or window, unless buf or win is set in {opts}.
    pub async fn get_option_value<T>(&self, name: &str, opts: HashMap<String, Value>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let req = (name, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_option_value", req).await?)
    }
    /// Sets the value of an option. The behavior of this function matches that of
    /// |:set|: for global-local options, both the global and local value are set
    /// unless otherwise specified with {scope}.
    ///
    /// Note the options {win} and {buf} cannot be used together.
    pub async fn set_option_value<T>(
        &self,
        name: &str,
        value: T,
        opts: opts::SetOptionValue,
    ) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let req = (name, value, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_set_option_value", req).await?)
    }
    /// Gets the option information for all options.
    ///
    /// The dict has the full option names as keys and option metadata dicts as
    /// detailed at |nvim_get_option_info2()|.
    pub async fn get_all_options_info(&self) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_all_options_info", req).await?)
    }
    /// Gets the option information for one option from arbitrary buffer or window
    pub async fn get_option_info2(
        &self,
        name: &str,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let req = (name, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_option_info2", req).await?)
    }
    /// Gets the windows in a tabpage
    pub async fn tabpage_list_wins(&self, tabpage: &TabPage) -> Result<Vec<Window>> {
        #[allow(unused_variables)]
        let req = tabpage;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_tabpage_list_wins", req).await?)
    }
    /// Gets a tab-scoped (t:) variable
    pub async fn tabpage_get_var<T>(&self, tabpage: &TabPage, name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let req = (tabpage, name);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_tabpage_get_var", req).await?)
    }
    /// Sets a tab-scoped (t:) variable
    pub async fn tabpage_set_var<T>(&self, tabpage: &TabPage, name: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let req = (tabpage, name, value);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_tabpage_set_var", req).await?)
    }
    /// Removes a tab-scoped (t:) variable
    pub async fn tabpage_del_var(&self, tabpage: &TabPage, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = (tabpage, name);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_tabpage_del_var", req).await?)
    }
    /// Gets the current window in a tabpage
    pub async fn tabpage_get_win(&self, tabpage: &TabPage) -> Result<Window> {
        #[allow(unused_variables)]
        let req = tabpage;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_tabpage_get_win", req).await?)
    }
    /// Sets the current window in a tabpage
    pub async fn tabpage_set_win(&self, tabpage: &TabPage, win: &Window) -> Result<()> {
        #[allow(unused_variables)]
        let req = (tabpage, win);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_tabpage_set_win", req).await?)
    }
    /// Gets the tabpage number
    pub async fn tabpage_get_number(&self, tabpage: &TabPage) -> Result<i64> {
        #[allow(unused_variables)]
        let req = tabpage;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_tabpage_get_number", req).await?)
    }
    /// Checks if a tabpage is valid
    pub async fn tabpage_is_valid(&self, tabpage: &TabPage) -> Result<bool> {
        #[allow(unused_variables)]
        let req = tabpage;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_tabpage_is_valid", req).await?)
    }
    /// Activates UI events on the channel.
    ///
    /// Entry point of all UI clients. Allows |--embed| to continue startup.
    /// Implies that the client is ready to show the UI. Adds the client to the
    /// list of UIs. |nvim_list_uis()|
    ///
    /// Note: If multiple UI clients are attached, the global screen dimensions
    /// degrade to the smallest client. E.g. if client A requests 80x40 but
    /// client B requests 200x100, the global screen has size 80x40.
    pub async fn ui_attach(
        &self,
        width: i64,
        height: i64,
        options: HashMap<String, Value>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let req = (width, height, options);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_ui_attach", req).await?)
    }
    /// Tells the nvim server if focus was gained or lost by the GUI
    pub async fn ui_set_focus(&self, gained: bool) -> Result<()> {
        #[allow(unused_variables)]
        let req = gained;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_ui_set_focus", req).await?)
    }
    /// Deactivates UI events on the channel.
    ///
    /// Removes the client from the list of UIs. |nvim_list_uis()|
    pub async fn ui_detach(&self) -> Result<()> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_ui_detach", req).await?)
    }
    /// Try to resize the UI.
    pub async fn ui_try_resize(&self, width: i64, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let req = (width, height);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_ui_try_resize", req).await?)
    }
    /// Set a UI option.
    pub async fn ui_set_option<T>(&self, name: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let req = (name, value);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_ui_set_option", req).await?)
    }
    /// Tell Nvim to resize a grid. Triggers a grid_resize event with the
    /// requested grid size or the maximum size if it exceeds size limits.
    ///
    /// On invalid grid handle, fails with error.
    pub async fn ui_try_resize_grid(&self, grid: i64, width: i64, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let req = (grid, width, height);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_ui_try_resize_grid", req).await?)
    }
    /// Tells Nvim the number of elements displaying in the popupmenu, to decide
    /// <PageUp> and <PageDown> movement.
    pub async fn ui_pum_set_height(&self, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let req = height;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_ui_pum_set_height", req).await?)
    }
    /// Tells Nvim the geometry of the popupmenu, to align floating windows with
    /// an external popup menu.
    ///
    /// Note that this method is not to be confused with
    /// |nvim_ui_pum_set_height()|, which sets the number of visible items in the
    /// popup menu, while this function sets the bounding box of the popup menu,
    /// including visual elements such as borders and sliders. Floats need not use
    /// the same font size, nor be anchored to exact grid corners, so one can set
    /// floating-point numbers to the popup menu geometry.
    pub async fn ui_pum_set_bounds(
        &self,
        width: f64,
        height: f64,
        row: f64,
        col: f64,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let req = (width, height, row, col);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_ui_pum_set_bounds", req).await?)
    }
    /// Tells Nvim when a terminal event has occurred
    ///
    /// The following terminal events are supported:
    /// * termresponse: The terminal sent an OSC or DCS response sequence to
    /// Nvim. The payload is the received response. Sets |v:termresponse| and
    /// fires |TermResponse|.
    pub async fn ui_term_event<T>(&self, event: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let req = (event, value);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_ui_term_event", req).await?)
    }
    /// Gets a highlight group by name
    ///
    /// similar to |hlID()|, but allocates a new ID if not present.
    pub async fn get_hl_id_by_name(&self, name: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let req = name;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_hl_id_by_name", req).await?)
    }
    /// Gets all or specific highlight groups in a namespace.
    ///
    /// Note: When the link attribute is defined in the highlight definition map,
    /// other attributes will not be taking effect (see |:hi-link|).
    pub async fn get_hl(
        &self,
        ns_id: i64,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let req = (ns_id, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_hl", req).await?)
    }
    /// Sets a highlight group.
    ///
    /// Note: Unlike the :highlight command which can update a highlight group,
    /// this function completely replaces the definition. For example:
    /// nvim_set_hl(0, Visual, {}) will clear the highlight group 'Visual'.
    ///
    /// Note: The fg and bg keys also accept the string values fg or bg
    /// which act as aliases to the corresponding foreground and background
    /// values of the Normal group. If the Normal group has not been defined,
    /// using these values results in an error.
    ///
    /// Note: If link is used in combination with other attributes; only the
    /// link will take effect (see |:hi-link|).
    pub async fn set_hl(&self, ns_id: i64, name: &str, val: opts::SetHl) -> Result<()> {
        #[allow(unused_variables)]
        let req = (ns_id, name, val);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_set_hl", req).await?)
    }
    /// Gets the active highlight namespace.
    pub async fn get_hl_ns(&self, opts: HashMap<String, Value>) -> Result<i64> {
        #[allow(unused_variables)]
        let req = opts;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_hl_ns", req).await?)
    }
    /// Set active namespace for highlights defined with |nvim_set_hl()|. This can
    /// be set for a single window, see |nvim_win_set_hl_ns()|.
    pub async fn set_hl_ns(&self, ns_id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let req = ns_id;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_set_hl_ns", req).await?)
    }
    /// Set active namespace for highlights defined with |nvim_set_hl()| while
    /// redrawing.
    ///
    /// This function meant to be called while redrawing, primarily from
    /// |nvim_set_decoration_provider()| on_win and on_line callbacks, which are
    /// allowed to change the namespace during a redraw cycle.
    pub async fn set_hl_ns_fast(&self, ns_id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let req = ns_id;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_set_hl_ns_fast", req).await?)
    }
    /// Sends input-keys to Nvim, subject to various quirks controlled by mode
    /// flags. This is a blocking call, unlike |nvim_input()|.
    ///
    /// On execution error: does not fail, but updates v:errmsg.
    ///
    /// To input sequences like <C-o> use |nvim_replace_termcodes()| (typically
    /// with escape_ks=false) to replace |keycodes|, then pass the result to
    /// nvim_feedkeys().
    pub async fn feedkeys(&self, keys: &str, mode: &str, escape_ks: bool) -> Result<()> {
        #[allow(unused_variables)]
        let req = (keys, mode, escape_ks);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_feedkeys", req).await?)
    }
    /// Queues raw user-input. Unlike |nvim_feedkeys()|, this uses a low-level
    /// input buffer and the call is non-blocking (input is processed
    /// asynchronously by the eventloop).
    ///
    /// To input blocks of text, |nvim_paste()| is much faster and should be
    /// preferred.
    ///
    /// On execution error: does not fail, but updates v:errmsg.
    ///
    /// Note: |keycodes| like <CR> are translated, so < is special. To input a
    /// literal <, send <LT>.
    ///
    /// Note: For mouse events use |nvim_input_mouse()|. The pseudokey form
    /// <LeftMouse><col,row> is deprecated since |api-level| 6.
    pub async fn input(&self, keys: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let req = keys;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_input", req).await?)
    }
    /// Send mouse event from GUI.
    ///
    /// Non-blocking: does not wait on any result, but queues the event to be
    /// processed soon by the event loop.
    ///
    /// Note: Currently this does not support scripting multiple mouse events by
    /// calling it multiple times in a loop: the intermediate mouse positions
    /// will be ignored. It should be used to implement real-time mouse input
    /// in a GUI. The deprecated pseudokey form (<LeftMouse><col,row>) of
    /// |nvim_input()| has the same limitation.
    pub async fn input_mouse(
        &self,
        button: &str,
        action: &str,
        modifier: &str,
        grid: i64,
        row: i64,
        col: i64,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let req = (button, action, modifier, grid, row, col);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_input_mouse", req).await?)
    }
    /// Replaces terminal codes and |keycodes| (<CR>, <Esc>, ...) in a string with
    /// the internal representation.
    pub async fn replace_termcodes(
        &self,
        str: &str,
        from_part: bool,
        do_lt: bool,
        special: bool,
    ) -> Result<String> {
        #[allow(unused_variables)]
        let req = (str, from_part, do_lt, special);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_replace_termcodes", req).await?)
    }
    /// Execute Lua code. Parameters (if any) are available as ... inside the
    /// chunk. The chunk can return a value.
    ///
    /// Only statements are executed. To evaluate an expression, prefix it with
    /// return: return my_function(...)
    pub async fn exec_lua<T>(&self, code: &str, args: Vec<Value>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let req = (code, args);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_exec_lua", req).await?)
    }
    /// Notify the user with a message.
    ///
    /// Relays the call to vim.notify . By default forwards your message in the
    /// echo area but can be overridden to trigger desktop notifications.
    pub async fn notify<T>(
        &self,
        msg: &str,
        log_level: u64,
        opts: HashMap<String, Value>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let req = (msg, log_level, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_notify", req).await?)
    }
    /// Calculates the number of display cells occupied by text. Control
    /// characters including <Tab> count as one cell.
    pub async fn strwidth(&self, text: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let req = text;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_strwidth", req).await?)
    }
    /// Gets the paths contained in |runtime-search-path|.
    pub async fn list_runtime_paths(&self) -> Result<Vec<String>> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_list_runtime_paths", req).await?)
    }
    /// Finds files in runtime directories, in runtimepath order.
    ///
    /// name can contain wildcards. For example
    /// nvim_get_runtime_file(colors/*.{vim,lua}, true) will return all color
    /// scheme files. Always use forward slashes (/) in the search pattern for
    /// subdirectories regardless of platform.
    ///
    /// It is not an error to not find any files. An empty array is returned then.
    pub async fn get_runtime_file(&self, name: &str, all: bool) -> Result<Vec<String>> {
        #[allow(unused_variables)]
        let req = (name, all);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_runtime_file", req).await?)
    }
    /// Changes the global working directory.
    pub async fn set_current_dir(&self, dir: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = dir;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_set_current_dir", req).await?)
    }
    /// Gets the current line.
    pub async fn get_current_line(&self) -> Result<String> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_current_line", req).await?)
    }
    /// Sets the current line.
    pub async fn set_current_line(&self, line: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = line;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_set_current_line", req).await?)
    }
    /// Deletes the current line.
    pub async fn del_current_line(&self) -> Result<()> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_del_current_line", req).await?)
    }
    /// Gets a global (g:) variable.
    pub async fn get_var<T>(&self, name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let req = name;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_var", req).await?)
    }
    /// Sets a global (g:) variable
    pub async fn set_var<T>(&self, name: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let req = (name, value);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_set_var", req).await?)
    }
    /// Removes a global (g:) variable.
    pub async fn del_var(&self, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = name;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_del_var", req).await?)
    }
    /// Gets a v: variable.
    pub async fn get_vvar<T>(&self, name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let req = name;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_vvar", req).await?)
    }
    /// Sets a v: variable, if it is not readonly.
    pub async fn set_vvar<T>(&self, name: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let req = (name, value);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_set_vvar", req).await?)
    }
    /// Echo a message.
    pub async fn echo(
        &self,
        chunks: Vec<Value>,
        history: bool,
        opts: HashMap<String, Value>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let req = (chunks, history, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_echo", req).await?)
    }
    /// Writes a message to the Vim output buffer. Does not append \n, the
    /// message is buffered (will not display) until a linefeed is written.
    pub async fn out_write(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = str;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_out_write", req).await?)
    }
    /// Writes a message to the Vim error buffer. Does not append \n, the
    /// message is buffered (will not display) until a linefeed is written.
    pub async fn err_write(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = str;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_err_write", req).await?)
    }
    /// Writes a message to the Vim error buffer. Appends \n, so the buffer is
    /// flushed (and displayed).
    pub async fn err_writeln(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = str;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_err_writeln", req).await?)
    }
    /// Gets the current list of buffer handles
    ///
    /// Includes unlisted (unloaded/deleted) buffers, like :ls!. Use
    /// |nvim_buf_is_loaded()| to check if a buffer is loaded.
    pub async fn list_bufs(&self) -> Result<Vec<Buffer>> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_list_bufs", req).await?)
    }
    /// Gets the current buffer.
    pub async fn get_current_buf(&self) -> Result<Buffer> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_current_buf", req).await?)
    }
    /// Sets the current buffer.
    pub async fn set_current_buf(&self, buffer: &Buffer) -> Result<()> {
        #[allow(unused_variables)]
        let req = buffer;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_set_current_buf", req).await?)
    }
    /// Gets the current list of window handles.
    pub async fn list_wins(&self) -> Result<Vec<Window>> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_list_wins", req).await?)
    }
    /// Gets the current window.
    pub async fn get_current_win(&self) -> Result<Window> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_current_win", req).await?)
    }
    /// Sets the current window.
    pub async fn set_current_win(&self, window: &Window) -> Result<()> {
        #[allow(unused_variables)]
        let req = window;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_set_current_win", req).await?)
    }
    /// Creates a new, empty, unnamed buffer.
    pub async fn create_buf(&self, listed: bool, scratch: bool) -> Result<Buffer> {
        #[allow(unused_variables)]
        let req = (listed, scratch);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_create_buf", req).await?)
    }
    /// Open a terminal instance in a buffer
    ///
    /// By default (and currently the only option) the terminal will not be
    /// connected to an external process. Instead, input sent on the channel will
    /// be echoed directly by the terminal. This is useful to display ANSI
    /// terminal sequences returned as part of a rpc message, or similar.
    ///
    /// Note: to directly initiate the terminal using the right size, display the
    /// buffer in a configured window before calling this. For instance, for a
    /// floating display, first create an empty buffer using |nvim_create_buf()|,
    /// then display it using |nvim_open_win()|, and then call this function. Then
    /// |nvim_chan_send()| can be called immediately to process sequences in a
    /// virtual terminal having the intended size.
    pub async fn open_term(&self, buffer: &Buffer, opts: HashMap<String, Value>) -> Result<i64> {
        #[allow(unused_variables)]
        let req = (buffer, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_open_term", req).await?)
    }
    /// Send data to channel. For a job, it writes it to the stdin of the
    /// process. For the stdio channel `channel-stdio`, it writes to Nvim's
    /// stdout. For an internal terminal instance (`nvim_open_term()`) it writes
    /// directly to terminal output. See `channel-bytes` for more information.
    ///
    /// This function writes raw data, not RPC messages. If the channel was
    /// created with rpc=true then the channel expects RPC messages, use
    /// `vim.rpcnotify()` and `vim.rpcrequest()` instead.
    pub async fn chan_send(&self, chan: i64, data: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = (chan, data);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_chan_send", req).await?)
    }
    /// Gets the current list of tabpage handles.
    pub async fn list_tabpages(&self) -> Result<Vec<TabPage>> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_list_tabpages", req).await?)
    }
    /// Gets the current tabpage.
    pub async fn get_current_tabpage(&self) -> Result<TabPage> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_current_tabpage", req).await?)
    }
    /// Sets the current tabpage.
    pub async fn set_current_tabpage(&self, tabpage: &TabPage) -> Result<()> {
        #[allow(unused_variables)]
        let req = tabpage;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_set_current_tabpage", req).await?)
    }
    /// Pastes at cursor (in any mode), and sets redo so dot (|.|) will repeat
    /// the input. UIs call this to implement paste, but it is also intended for
    /// use by scripts to input large, dot-repeatable blocks of text (as opposed
    /// to |nvim_input()| which is subject to mappings/events and is thus much
    /// slower).
    ///
    /// Invokes the |vim.paste()| handler, which handles each mode appropriately.
    ///
    /// Errors (nomodifiable, vim.paste() failure, …) are reflected in err
    /// but do not affect the return value (which is strictly decided by
    /// vim.paste()). On error or cancel, subsequent calls are ignored
    /// (drained) until the next paste is initiated (phase 1 or -1).
    pub async fn paste(&self, data: &str, crlf: bool, phase: i64) -> Result<bool> {
        #[allow(unused_variables)]
        let req = (data, crlf, phase);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_paste", req).await?)
    }
    /// Puts text at cursor, in any mode. For dot-repeatable input, use
    /// |nvim_paste()|.
    ///
    /// Compare |:put| and |p| which are always linewise.
    pub async fn put(
        &self,
        lines: Vec<String>,
        typ: &str,
        after: bool,
        follow: bool,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let req = (lines, typ, after, follow);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_put", req).await?)
    }
    /// Returns the 24-bit RGB value of a |nvim_get_color_map()| color name or
    /// #rrggbb hexadecimal string.
    pub async fn get_color_by_name(&self, name: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let req = name;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_color_by_name", req).await?)
    }
    /// Returns a map of color names and RGB values.
    ///
    /// Keys are color names (e.g. Aqua) and values are 24-bit RGB color values
    /// (e.g. 65535).
    pub async fn get_color_map(&self) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_color_map", req).await?)
    }
    /// Gets a map of the current editor state.
    pub async fn get_context(
        &self,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let req = opts;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_context", req).await?)
    }
    /// Sets the current editor state from the given |context| map.
    pub async fn load_context<T>(&self, dict: HashMap<String, Value>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let req = dict;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_load_context", req).await?)
    }
    /// Gets the current mode. |mode()| blocking is true if Nvim is waiting for
    /// input.
    pub async fn get_mode(&self) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_mode", req).await?)
    }
    /// Gets a list of global (non-buffer-local) |mapping| definitions.
    pub async fn get_keymap(&self, mode: &str) -> Result<Vec<HashMap<String, Value>>> {
        #[allow(unused_variables)]
        let req = mode;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_keymap", req).await?)
    }
    /// Sets a global |mapping| for the given mode.
    ///
    /// To set a buffer-local mapping, use |nvim_buf_set_keymap()|.
    ///
    /// Unlike |:map|, leading/trailing whitespace is accepted as part of the
    /// {lhs} or {rhs}. Empty {rhs} is <Nop>. |keycodes| are replaced as usual.
    pub async fn set_keymap(
        &self,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: HashMap<String, Value>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let req = (mode, lhs, rhs, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_set_keymap", req).await?)
    }
    /// Unmaps a global mapping for the given mode.
    ///
    /// To unmap a buffer-local mapping, use nvim_buf_del_keymap().
    pub async fn del_keymap(&self, mode: &str, lhs: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = (mode, lhs);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_del_keymap", req).await?)
    }
    /// Returns a 2-tuple (Array), where item 0 is the current channel id and item
    /// 1 is the |api-metadata| map (Dict).
    pub async fn get_api_info(&self) -> Result<(u64, ApiInfo)> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_api_info", req).await?)
    }
    /// Self-identifies the client.
    ///
    /// The client/plugin/application should call this after connecting, to
    /// provide hints about its identity and purpose, for debugging and
    /// orchestration.
    ///
    /// Can be called more than once; the caller should merge old info if
    /// appropriate. Example: library first identifies the channel, then a plugin
    /// using that library later identifies itself.
    ///
    /// Note: Something is better than nothing. You do not need to include all the
    /// fields.
    pub async fn set_client_info(
        &self,
        name: &str,
        version: HashMap<String, Value>,
        typ: &str,
        methods: HashMap<String, Value>,
        attributes: HashMap<String, Value>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let req = (name, version, typ, methods, attributes);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_set_client_info", req).await?)
    }
    /// Gets information about a channel.
    pub async fn get_chan_info(&self, chan: i64) -> Result<ChanInfo> {
        #[allow(unused_variables)]
        let req = chan;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_chan_info", req).await?)
    }
    /// Get information about all open channels.
    pub async fn list_chans(&self) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_list_chans", req).await?)
    }
    /// Gets a list of dictionaries representing attached UIs.
    pub async fn list_uis(&self) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let req = NO_PARAMS;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_list_uis", req).await?)
    }
    /// Gets the immediate children of process pid.
    pub async fn get_proc_children(&self, pid: i64) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let req = pid;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_proc_children", req).await?)
    }
    /// Gets info describing process pid.
    pub async fn get_proc<T>(&self, pid: i64) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let req = pid;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_proc", req).await?)
    }
    /// Selects an item in the completion popup menu.
    ///
    /// If neither |ins-completion| nor |cmdline-completion| popup menu is active
    /// this API call is silently ignored. Useful for an external UI using
    /// |ui-popupmenu| to control the popup menu with the mouse. Can also be used
    /// in a mapping; use <Cmd> |:map-cmd| or a Lua mapping to ensure the mapping
    /// does not end completion mode.
    pub async fn select_popupmenu_item(
        &self,
        item: i64,
        insert: bool,
        finish: bool,
        opts: HashMap<String, Value>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let req = (item, insert, finish, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_select_popupmenu_item", req).await?)
    }
    /// Deletes an uppercase/file named mark. See mark-motions.
    ///
    /// Note: Lowercase name (or other buffer-local mark) is an error.
    pub async fn del_mark(&self, name: &str) -> Result<bool> {
        #[allow(unused_variables)]
        let req = name;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_del_mark", req).await?)
    }
    /// Returns a (row, col, buffer, buffername) tuple representing the position
    /// of the uppercase/file named mark. End of line column position is
    /// returned as |v:maxcol| (big number). See |mark-motions|.
    ///
    /// Marks are (1,0)-indexed. |api-indexing|
    ///
    /// Note: Lowercase name (or other buffer-local mark) is an error.
    pub async fn get_mark(&self, name: &str, opts: HashMap<String, Value>) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let req = (name, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_get_mark", req).await?)
    }
    /// Evaluates statusline string.
    pub async fn eval_statusline(
        &self,
        str: &str,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let req = (str, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_eval_statusline", req).await?)
    }
    /// Executes Vimscript (multiline block of Ex commands), like anonymous
    /// |:source|.
    ///
    /// Unlike |nvim_command()| this function supports heredocs, script-scope
    /// (s:), etc.
    ///
    /// On execution error: fails with Vimscript error, updates v:errmsg.
    pub async fn exec2(
        &self,
        src: &str,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let req = (src, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_exec2", req).await?)
    }
    /// Executes an Ex command.
    ///
    /// On execution error: fails with Vimscript error, updates v:errmsg.
    ///
    /// Prefer `nvim_cmd()` or `nvim_exec2()` instead. To modify an Ex command in
    /// a structured way before executing it, modify the result of
    /// `nvim_parse_cmd()` then pass it to `nvim_cmd()`.
    pub async fn command(&self, command: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = command;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_command", req).await?)
    }
    /// Evaluates a Vimscript expression. Dicts and Lists are recursively
    /// expanded.
    ///
    /// On execution error: fails with Vimscript error, updates v:errmsg.
    pub async fn eval<T>(&self, expr: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let req = expr;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_eval", req).await?)
    }
    /// Calls a Vimscript function with the given arguments.
    ///
    /// On execution error: fails with Vimscript error, updates v:errmsg.
    pub async fn call_function<T>(&self, func: &str, args: Vec<Value>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let req = (func, args);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_call_function", req).await?)
    }
    /// Calls a Vimscript `Dictionary-function` with the given arguments.
    ///
    /// On execution error: fails with Vimscript error, updates v:errmsg.
    pub async fn call_dict_function<T, U>(&self, dict: T, func: &str, args: Vec<Value>) -> Result<U>
    where
        T: Serialize,
        U: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let req = (dict, func, args);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_call_dict_function", req).await?)
    }
    /// Parse a Vimscript expression.
    pub async fn parse_expression(
        &self,
        expr: &str,
        flags: &str,
        highlight: bool,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let req = (expr, flags, highlight);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_parse_expression", req).await?)
    }
    /// Opens a new split window, or a floating window if relative is specified,
    /// or an external window (managed by the UI) if external is specified.
    ///
    /// Floats are windows that are drawn above the split layout, at some anchor
    /// position in some other window. Floats can be drawn internally or by
    /// external GUI with the |ui-multigrid| extension. External windows are only
    /// supported with multigrid GUIs, and are displayed as separate top-level
    /// windows.
    ///
    /// For a general overview of floats, see |api-floatwin|.
    ///
    /// The width and height of the new window must be specified when opening
    /// a floating window, but are optional for normal windows.
    ///
    /// If relative and external are omitted, a normal split window is
    /// created. The win property determines which window will be split. If no
    /// win is provided or win == 0, a window will be created adjacent to the
    /// current window. If -1 is provided, a top-level split will be created.
    /// vertical and split are only valid for normal windows, and are used to
    /// control split direction. For vertical, the exact direction is determined
    /// by splitright and splitbelow. Split windows cannot have
    /// bufpos/row/col/border/title/footer properties.
    ///
    /// With relative=editor (row=0,col=0) refers to the top-left corner of the
    /// screen-grid and (row=Lines-1,col=Columns-1) refers to the bottom-right
    /// corner. Fractional values are allowed, but the builtin implementation
    /// (used by non-multigrid UIs) will always round down to nearest integer.
    ///
    /// Out-of-bounds values, and configurations that make the float not fit
    /// inside the main editor, are allowed. The builtin implementation truncates
    /// values so floats are fully within the main screen grid. External GUIs
    /// could let floats hover outside of the main window like a tooltip, but this
    /// should not be used to specify arbitrary WM screen positions.
    pub async fn open_win(
        &self,
        buffer: &Buffer,
        enter: bool,
        config: WindowConf,
    ) -> Result<Window> {
        #[allow(unused_variables)]
        let req = (buffer, enter, config);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_open_win", req).await?)
    }
    /// Configures window layout. Cannot be used to move the last window in a
    /// tabpage to a different one.
    ///
    /// When reconfiguring a window, absent option keys will not be changed.
    /// row/col and relative must be reconfigured together.
    pub async fn win_set_config(&self, window: &Window, config: WindowConf) -> Result<()> {
        #[allow(unused_variables)]
        let req = (window, config);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_set_config", req).await?)
    }
    /// Gets window configuration.
    ///
    /// The returned value may be given to `nvim_open_win()`.
    ///
    /// relative is empty for normal windows.
    pub async fn win_get_config(&self, window: &Window) -> Result<WindowConf> {
        #[allow(unused_variables)]
        let req = window;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_get_config", req).await?)
    }
    /// Gets the current buffer in a window
    pub async fn win_get_buf(&self, window: &Window) -> Result<Buffer> {
        #[allow(unused_variables)]
        let req = window;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_get_buf", req).await?)
    }
    /// Sets the current buffer in a window, without side effects
    pub async fn win_set_buf(&self, window: &Window, buffer: &Buffer) -> Result<()> {
        #[allow(unused_variables)]
        let req = (window, buffer);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_set_buf", req).await?)
    }
    /// Gets the (1,0)-indexed, buffer-relative cursor position for a given window
    /// (different windows showing the same buffer have independent cursor
    /// positions).
    pub async fn win_get_cursor(&self, window: &Window) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let req = window;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_get_cursor", req).await?)
    }
    /// Sets the (1,0)-indexed cursor position in the window. This scrolls the
    /// window even if it is not the current one.
    pub async fn win_set_cursor(&self, window: &Window, pos: Vec<i64>) -> Result<()> {
        #[allow(unused_variables)]
        let req = (window, pos);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_set_cursor", req).await?)
    }
    /// Gets the window height
    pub async fn win_get_height(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let req = window;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_get_height", req).await?)
    }
    /// Sets the window height.
    pub async fn win_set_height(&self, window: &Window, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let req = (window, height);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_set_height", req).await?)
    }
    /// Gets the window width
    pub async fn win_get_width(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let req = window;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_get_width", req).await?)
    }
    /// Sets the window width. This will only succeed if the screen is split
    /// vertically.
    pub async fn win_set_width(&self, window: &Window, width: i64) -> Result<()> {
        #[allow(unused_variables)]
        let req = (window, width);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_set_width", req).await?)
    }
    /// Gets a window-scoped (w:) variable
    pub async fn win_get_var<T>(&self, window: &Window, name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let req = (window, name);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_get_var", req).await?)
    }
    /// Sets a window-scoped (w:) variable
    pub async fn win_set_var<T>(&self, window: &Window, name: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let req = (window, name, value);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_set_var", req).await?)
    }
    /// Removes a window-scoped (w:) variable
    pub async fn win_del_var(&self, window: &Window, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let req = (window, name);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_del_var", req).await?)
    }
    /// Gets the window position in display cells. First position is zero.
    pub async fn win_get_position(&self, window: &Window) -> Result<(i64, i64)> {
        #[allow(unused_variables)]
        let req = window;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_get_position", req).await?)
    }
    /// Gets the window tabpage
    pub async fn win_get_tabpage(&self, window: &Window) -> Result<TabPage> {
        #[allow(unused_variables)]
        let req = window;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_get_tabpage", req).await?)
    }
    /// Gets the window number
    pub async fn win_get_number(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let req = window;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_get_number", req).await?)
    }
    /// Checks if a window is valid
    pub async fn win_is_valid(&self, window: &Window) -> Result<bool> {
        #[allow(unused_variables)]
        let req = window;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_is_valid", req).await?)
    }
    /// Closes the window and hide the buffer it contains (like |:hide| with a
    /// |window-ID|).
    ///
    /// Like |:hide| the buffer becomes hidden unless another window is editing
    /// it, or bufhidden is unload, delete or wipe as opposed to |:close|
    /// or |nvim_win_close()|, which will close the buffer.
    pub async fn win_hide(&self, window: &Window) -> Result<()> {
        #[allow(unused_variables)]
        let req = window;
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_hide", req).await?)
    }
    /// Closes the window (like |:close| with a |window-ID|).
    pub async fn win_close(&self, window: &Window, force: bool) -> Result<()> {
        #[allow(unused_variables)]
        let req = (window, force);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_close", req).await?)
    }
    /// Set highlight namespace for a window. This will use highlights defined
    /// with |nvim_set_hl()| for this namespace, but fall back to global
    /// highlights (ns=0) when missing.
    ///
    /// This takes precedence over the winhighlight option.
    pub async fn win_set_hl_ns(&self, window: &Window, ns_id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let req = (window, ns_id);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_set_hl_ns", req).await?)
    }
    /// Computes the number of screen lines occupied by a range of text in a given
    /// window. Works for off-screen text and takes folds into account.
    ///
    /// Diff filler or virtual lines above a line are counted as a part of that
    /// line, unless the line is on start_row and start_vcol is specified.
    ///
    /// Diff filler or virtual lines below the last buffer line are counted in the
    /// result when end_row is omitted.
    ///
    /// Line indexing is similar to `nvim_buf_get_text()`.
    pub async fn win_text_height(
        &self,
        window: &Window,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let req = (window, opts);
        #[allow(clippy::needless_question_mark)]
        Ok(self.rpc_call("nvim_win_text_height", req).await?)
    }
}
