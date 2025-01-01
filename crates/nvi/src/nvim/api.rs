#![allow(clippy::needless_question_mark)]
#![allow(clippy::needless_borrow)]
use super::opts;
use super::types::*;
use crate::error::Result;
use mrpc::Value;
use serde::Serialize;
use serde_rmpv::{from_value, to_value};
use std::collections::HashMap;
use tracing::trace;
#[derive(Clone)]
#[doc = r" Generated bindings for Neovim's MessagePack-RPC API."]
pub struct NvimApi {
    pub(crate) rpc_sender: mrpc::RpcSender,
}
impl NvimApi {
    #[doc = r" Make a raw request over the MessagePack-RPC protocol."]
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
    #[doc = r" Send a raw notification over the MessagePack-RPC protocol."]
    pub async fn raw_notify(
        &self,
        method: &str,
        params: &[mrpc::Value],
    ) -> Result<(), mrpc::RpcError> {
        trace!("send notification: {:?} {:?}", method, params);
        self.rpc_sender.send_notification(method, params).await
    }
    #[doc = "\nGet all autocommands that match the corresponding {opts}.\n\nThese examples will get autocommands matching ALL the given criteria:\n- Matches all criteria\n- All commands from one group\n\nNOTE: When multiple patterns or events are provided, it will find all the\nautocommands that match any combination of them.\n"]
    pub async fn get_autocmds(&self, opts: HashMap<String, Value>) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_autocmds", &[to_value(&opts)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nCreates an autocommand event handler, defined by callback (Lua\nfunction or Vimscript function name string) or command (Ex command\nstring).\n\nNote: pattern is NOT automatically expanded (unlike with :autocmd),\nthus names like $HOME and ~ must be expanded explicitly.\n"]
    pub async fn create_autocmd(&self, event: &[Event], opts: opts::CreateAutocmd) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_create_autocmd",
                &[to_value(&event)?, to_value(&opts)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nDeletes an autocommand by id.\n"]
    pub async fn del_autocmd(&self, id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_del_autocmd", &[to_value(&id)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nClears all autocommands selected by {opts}. To delete autocmds see\n`nvim_del_autocmd()`.\n"]
    pub async fn clear_autocmds(&self, opts: opts::ClearAutocmds) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_clear_autocmds", &[to_value(&opts)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nCreate or get an autocommand group autocmd-groups.\n"]
    pub async fn create_augroup(&self, name: &str, opts: HashMap<String, Value>) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_create_augroup", &[to_value(&name)?, to_value(&opts)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nDelete an autocommand group by id.\n\nTo get a group id one can use nvim_get_autocmds().\n\nNOTE: behavior differs from :augroup-delete. When deleting a group,\nautocommands contained in this group will also be deleted and cleared.\nThis group will no longer exist.\n"]
    pub async fn del_augroup_by_id(&self, id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_del_augroup_by_id", &[to_value(&id)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nDelete an autocommand group by name.\n\nNOTE: behavior differs from :augroup-delete. When deleting a group,\nautocommands contained in this group will also be deleted and cleared.\nThis group will no longer exist.\n"]
    pub async fn del_augroup_by_name(&self, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_del_augroup_by_name", &[to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nExecute all autocommands for {event} that match the corresponding {opts}\n`autocmd-execute`.\n"]
    pub async fn exec_autocmds(&self, event: &[Event], opts: opts::ExecAutocmds) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_exec_autocmds", &[to_value(&event)?, to_value(&opts)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nReturns the number of lines in the given buffer.\n"]
    pub async fn buf_line_count(&self, buffer: &Buffer) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_line_count", &[to_value(&buffer)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nActivates buffer-update events on a channel, or as Lua callbacks.\n"]
    pub async fn buf_attach(
        &self,
        buffer: &Buffer,
        send_buffer: bool,
        opts: HashMap<String, Value>,
    ) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_attach",
                &[
                    to_value(&buffer)?,
                    to_value(&send_buffer)?,
                    to_value(&opts)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nDeactivates buffer-update events on the channel.\n"]
    pub async fn buf_detach(&self, buffer: &Buffer) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_detach", &[to_value(&buffer)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets a line-range from the buffer.\n\nIndexing is zero-based, end-exclusive. Negative indices are interpreted as\nlength+1+index: -1 refers to the index past the end. So to get the last\nelement use start=-2 and end=-1.\n\nOut-of-bounds indices are clamped to the nearest valid value, unless\nstrict_indexing is set.\n"]
    pub async fn buf_get_lines(
        &self,
        buffer: &Buffer,
        start: i64,
        end: i64,
        strict_indexing: bool,
    ) -> Result<Vec<String>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_get_lines",
                &[
                    to_value(&buffer)?,
                    to_value(&start)?,
                    to_value(&end)?,
                    to_value(&strict_indexing)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets (replaces) a line-range in the buffer.\n\nIndexing is zero-based, end-exclusive. Negative indices are interpreted as\nlength+1+index: -1 refers to the index past the end. So to change or\ndelete the last element use start=-2 and end=-1.\n\nTo insert lines at a given index, set start and end to the same index.\nTo delete a range of lines, set replacement to an empty array.\n\nOut-of-bounds indices are clamped to the nearest valid value, unless\nstrict_indexing is set.\n"]
    pub async fn buf_set_lines(
        &self,
        buffer: &Buffer,
        start: i64,
        end: i64,
        strict_indexing: bool,
        replacement: Vec<String>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_set_lines",
                &[
                    to_value(&buffer)?,
                    to_value(&start)?,
                    to_value(&end)?,
                    to_value(&strict_indexing)?,
                    to_value(&replacement)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets (replaces) a range in the buffer\n\nThis is recommended over nvim_buf_set_lines() when only modifying parts\nof a line, as extmarks will be preserved on non-modified parts of the\ntouched lines.\n\nIndexing is zero-based. Row indices are end-inclusive, and column indices\nare end-exclusive.\n\nTo insert text at a given (row, column) location, use\nstart_row = end_row = row and start_col = end_col = col. To delete the\ntext in a range, use replacement = {}.\n\nNote: Prefer nvim_buf_set_lines() (for performance) to add or delete\nentire lines.\nNote: Prefer nvim_paste() or nvim_put() to insert (instead of replace)\ntext at cursor.\n"]
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
        let ret = self
            .raw_request(
                "nvim_buf_set_text",
                &[
                    to_value(&buffer)?,
                    to_value(&start_row)?,
                    to_value(&start_col)?,
                    to_value(&end_row)?,
                    to_value(&end_col)?,
                    to_value(&replacement)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets a range from the buffer.\n\nThis differs from |nvim_buf_get_lines()| in that it allows retrieving only\nportions of a line.\n\nIndexing is zero-based. Row indices are end-inclusive, and column indices\nare end-exclusive.\n\nPrefer |nvim_buf_get_lines()| when retrieving entire lines.\n"]
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
        let ret = self
            .raw_request(
                "nvim_buf_get_text",
                &[
                    to_value(&buffer)?,
                    to_value(&start_row)?,
                    to_value(&start_col)?,
                    to_value(&end_row)?,
                    to_value(&end_col)?,
                    to_value(&opts)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nReturns the byte offset of a line (0-indexed). |api-indexing|\n\nLine 1 (index=0) has offset 0. UTF-8 bytes are counted. EOL is one byte.\nfileformat and fileencoding are ignored. The line index just after the\nlast line gives the total byte-count of the buffer. A final EOL byte is\ncounted if it would be written, see eol.\n\nUnlike |line2byte()|, throws error for out-of-bounds indexing. Returns -1\nfor unloaded buffer.\n"]
    pub async fn buf_get_offset(&self, buffer: &Buffer, index: i64) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_get_offset",
                &[to_value(&buffer)?, to_value(&index)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets a buffer-scoped (b:) variable.\n"]
    pub async fn buf_get_var<T>(&self, buffer: &Buffer, name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_get_var", &[to_value(&buffer)?, to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets a changed tick of a buffer\n"]
    pub async fn buf_get_changedtick(&self, buffer: &Buffer) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_get_changedtick", &[to_value(&buffer)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets a list of buffer-local |mapping| definitions.\n"]
    pub async fn buf_get_keymap(
        &self,
        buffer: &Buffer,
        mode: &str,
    ) -> Result<Vec<HashMap<String, Value>>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_get_keymap",
                &[to_value(&buffer)?, to_value(&mode)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets a buffer-local |mapping| for the given mode.\n"]
    pub async fn buf_set_keymap(
        &self,
        buffer: &Buffer,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: HashMap<String, Value>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_set_keymap",
                &[
                    to_value(&buffer)?,
                    to_value(&mode)?,
                    to_value(&lhs)?,
                    to_value(&rhs)?,
                    to_value(&opts)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nUnmaps a buffer-local |mapping| for the given mode.\n"]
    pub async fn buf_del_keymap(&self, buffer: &Buffer, mode: &str, lhs: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_del_keymap",
                &[to_value(&buffer)?, to_value(&mode)?, to_value(&lhs)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets a buffer-scoped (b:) variable\n"]
    pub async fn buf_set_var<T>(&self, buffer: &Buffer, name: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_set_var",
                &[to_value(&buffer)?, to_value(&name)?, to_value(&value)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nRemoves a buffer-scoped (b:) variable\n"]
    pub async fn buf_del_var(&self, buffer: &Buffer, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_del_var", &[to_value(&buffer)?, to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the full file name for the buffer\n"]
    pub async fn buf_get_name(&self, buffer: &Buffer) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_get_name", &[to_value(&buffer)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets the full file name for a buffer, like :file_f\n"]
    pub async fn buf_set_name(&self, buffer: &Buffer, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_set_name", &[to_value(&buffer)?, to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nChecks if a buffer is valid and loaded. See |api-buffer| for more info\nabout unloaded buffers.\n"]
    pub async fn buf_is_loaded(&self, buffer: &Buffer) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_is_loaded", &[to_value(&buffer)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nDeletes the buffer. See |:bwipeout|\n"]
    pub async fn buf_delete(
        &self,
        buffer: &Buffer,
        opts: HashMap<String, Value>,
    ) -> Result<opts::BufDelete> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_delete", &[to_value(&buffer)?, to_value(&opts)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nChecks if a buffer is valid.\n\nNote: Even if a buffer is valid it may have been unloaded. See |api-buffer|\nfor more info about unloaded buffers.\n"]
    pub async fn buf_is_valid(&self, buffer: &Buffer) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_is_valid", &[to_value(&buffer)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nDeletes a named mark in the buffer. See |mark-motions|.\n\nNote: only deletes marks set in the buffer, if the mark is not set in the\nbuffer it will return false.\n"]
    pub async fn buf_del_mark(&self, buffer: &Buffer, name: &str) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_del_mark", &[to_value(&buffer)?, to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets a named mark in the given buffer, all marks are allowed\nfile/uppercase, visual, last change, etc. See mark-motions.\n\nMarks are (1,0)-indexed. api-indexing\n\nNote: Passing 0 as line deletes the mark\n"]
    pub async fn buf_set_mark(
        &self,
        buffer: &Buffer,
        name: &str,
        line: i64,
        col: i64,
        opts: HashMap<String, Value>,
    ) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_set_mark",
                &[
                    to_value(&buffer)?,
                    to_value(&name)?,
                    to_value(&line)?,
                    to_value(&col)?,
                    to_value(&opts)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nReturns a (row,col) tuple representing the position of the named mark.\nEnd of line column position is returned as |v:maxcol| (big number).\nSee |mark-motions|.\n\nMarks are (1,0)-indexed. |api-indexing|\n"]
    pub async fn buf_get_mark(&self, buffer: &Buffer, name: &str) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_get_mark", &[to_value(&buffer)?, to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nParse command line.\n\nDoes not check the validity of command arguments.\n"]
    pub async fn parse_cmd(
        &self,
        str: &str,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_parse_cmd", &[to_value(&str)?, to_value(&opts)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nExecutes an Ex command.\n\nUnlike `nvim_command()` this command takes a structured Dict instead of a\nString. This allows for easier construction and manipulation of an Ex\ncommand. This also allows for things such as having spaces inside a\ncommand argument, expanding filenames in a command that otherwise does not\nexpand filenames, etc. Command arguments may also be Number, Boolean or\nString.\n\nThe first argument may also be used instead of count for commands that\nsupport it in order to make their usage simpler. For example, instead of\n`vim.cmd.bdelete{ count = 2 }`, you may do `vim.cmd.bdelete(2)`.\n\nOn execution error: fails with Vimscript error, updates v:errmsg.\n"]
    pub async fn cmd(
        &self,
        cmd: HashMap<String, Value>,
        opts: HashMap<String, Value>,
    ) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_cmd", &[to_value(&cmd)?, to_value(&opts)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nCreates a global user-commands command.\n"]
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
        let ret = self
            .raw_request(
                "nvim_create_user_command",
                &[to_value(&name)?, to_value(&command)?, to_value(&opts)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nDelete a user-defined command.\n"]
    pub async fn del_user_command(&self, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_del_user_command", &[to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nCreates a buffer-local command `user-commands`.\n"]
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
        let ret = self
            .raw_request(
                "nvim_buf_create_user_command",
                &[
                    to_value(&buffer)?,
                    to_value(&name)?,
                    to_value(&command)?,
                    to_value(&opts)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nDelete a buffer-local user-defined command.\n\nOnly commands created with `:command-buffer` or\n`nvim_buf_create_user_command()` can be deleted with this function.\n"]
    pub async fn buf_del_user_command(&self, buffer: &Buffer, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_del_user_command",
                &[to_value(&buffer)?, to_value(&name)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets a map of global (non-buffer-local) Ex commands.\n\nCurrently only |user-commands| are supported, not builtin Ex commands.\n"]
    pub async fn get_commands(
        &self,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_commands", &[to_value(&opts)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets a map of buffer-local |user-commands|.\n"]
    pub async fn buf_get_commands(
        &self,
        buffer: &Buffer,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_get_commands",
                &[to_value(&buffer)?, to_value(&opts)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nCreates a new namespace or gets an existing one.\n\nNamespaces are used for buffer highlights and virtual text, see\nnvim_buf_add_highlight() and nvim_buf_set_extmark().\n\nNamespaces can be named or anonymous. If name matches an existing\nnamespace, the associated id is returned. If name is an empty string a\nnew, anonymous namespace is created.\n"]
    pub async fn create_namespace(&self, name: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_create_namespace", &[to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets existing, non-anonymous |namespace|s.\n"]
    pub async fn get_namespaces(&self) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_get_namespaces", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the position (0-indexed) of an |extmark|.\n"]
    pub async fn buf_get_extmark_by_id(
        &self,
        buffer: &Buffer,
        ns_id: i64,
        id: i64,
        opts: HashMap<String, Value>,
    ) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_get_extmark_by_id",
                &[
                    to_value(&buffer)?,
                    to_value(&ns_id)?,
                    to_value(&id)?,
                    to_value(&opts)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets |extmarks| in traversal order from a |charwise| region defined by\nbuffer positions (inclusive, 0-indexed |api-indexing|).\n\nRegion can be given as (row,col) tuples, or valid extmark ids (whose\npositions define the bounds). 0 and -1 are understood as (0,0) and (-1,-1)\nrespectively.\n\nIf end is less than start, traversal works backwards. (Useful with\nlimit, to get the first marks prior to a given position.)\n\nNote: when using extmark ranges (marks with a end_row/end_col position)\nthe overlap option might be useful. Otherwise only the start position of\nan extmark will be considered.\n\nNote: legacy signs placed through the |:sign| commands are implemented as\nextmarks and will show up here. Their details array will contain a\nsign_name field.\n"]
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
        let ret = self
            .raw_request(
                "nvim_buf_get_extmarks",
                &[
                    to_value(&buffer)?,
                    to_value(&ns_id)?,
                    to_value(&start)?,
                    to_value(&end)?,
                    to_value(&opts)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nCreates or updates an extmark.\n\nBy default a new extmark is created when no id is passed in, but it is\nalso possible to create a new mark by passing in a previously unused id or\nmove an existing mark by passing in its id. The caller must then keep\ntrack of existing and unused ids itself. (Useful over RPC, to avoid\nwaiting for the return value.)\n\nUsing the optional arguments, it is possible to use this to highlight a\nrange of text, and also to associate virtual text to the mark.\n\nIf present, the position defined by end_col and end_row should be\nafter the start position in order for the extmark to cover a range. An\nearlier end position is not an error, but then it behaves like an empty\nrange (no highlighting).\n"]
    pub async fn buf_set_extmark(
        &self,
        buffer: &Buffer,
        ns_id: i64,
        line: i64,
        col: i64,
        opts: HashMap<String, Value>,
    ) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_set_extmark",
                &[
                    to_value(&buffer)?,
                    to_value(&ns_id)?,
                    to_value(&line)?,
                    to_value(&col)?,
                    to_value(&opts)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nRemoves an extmark.\n"]
    pub async fn buf_del_extmark(&self, buffer: &Buffer, ns_id: i64, id: i64) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_del_extmark",
                &[to_value(&buffer)?, to_value(&ns_id)?, to_value(&id)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nAdds a highlight to buffer.\n\nUseful for plugins that dynamically generate highlights to a buffer (like\na semantic highlighter or linter). The function adds a single highlight to\na buffer. Unlike `matchaddpos()` highlights follow changes to line\nnumbering (as lines are inserted/removed above the highlighted line), like\nsigns and marks do.\n\nNamespaces are used for batch deletion/updating of a set of highlights. To\ncreate a namespace, use `nvim_create_namespace()` which returns a\nnamespace id. Pass it in to this function as ns_id to add highlights to\nthe namespace. All highlights in the same namespace can then be cleared\nwith single call to `nvim_buf_clear_namespace()`. If the highlight never\nwill be deleted by an API call, pass ns_id = -1.\n\nAs a shorthand, ns_id = 0 can be used to create a new namespace for the\nhighlight, the allocated id is then returned. If hl_group is the empty\nstring no highlight is added, but a new ns_id is still returned. This is\nsupported for backwards compatibility, new code should use\n`nvim_create_namespace()` to create a new empty namespace.\n"]
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
        let ret = self
            .raw_request(
                "nvim_buf_add_highlight",
                &[
                    to_value(&buffer)?,
                    to_value(&ns_id)?,
                    to_value(&hl_group)?,
                    to_value(&line)?,
                    to_value(&col_start)?,
                    to_value(&col_end)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nClears namespaced objects (highlights, extmarks, virtual text) from a\nregion.\n\nLines are 0-indexed. `api-indexing` To clear the namespace in the entire\nbuffer, specify line_start=0 and line_end=-1.\n"]
    pub async fn buf_clear_namespace(
        &self,
        buffer: &Buffer,
        ns_id: i64,
        line_start: i64,
        line_end: i64,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_clear_namespace",
                &[
                    to_value(&buffer)?,
                    to_value(&ns_id)?,
                    to_value(&line_start)?,
                    to_value(&line_end)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSet or change decoration provider for a |namespace|\n\nThis is a very general purpose interface for having Lua callbacks being\ntriggered during the redraw code.\n\nThe expected usage is to set |extmarks| for the currently redrawn buffer.\n|nvim_buf_set_extmark()| can be called to add marks on a per-window or\nper-lines basis. Use the ephemeral key to only use the mark for the\ncurrent screen redraw (the callback will be called again for the next\nredraw).\n\nNote: this function should not be called often. Rather, the callbacks\nthemselves can be used to throttle unneeded callbacks. the on_start\ncallback can return false to disable the provider until the next redraw.\nSimilarly, return false in on_win will skip the on_line calls for\nthat window (but any extmarks set in on_win will still be used). A\nplugin managing multiple sources of decoration should ideally only set one\nprovider, and merge the sources internally. You can use multiple ns_id\nfor the extmarks set/modified inside the callback anyway.\n\nNote: doing anything other than setting extmarks is considered\nexperimental. Doing things like changing options are not explicitly\nforbidden, but is likely to have unexpected consequences (such as 100% CPU\nconsumption). Doing vim.rpcnotify should be OK, but vim.rpcrequest is\nquite dubious for the moment.\n\nNote: It is not allowed to remove or update extmarks in on_line\ncallbacks.\n"]
    pub async fn set_decoration_provider(
        &self,
        ns_id: i64,
        opts: HashMap<String, Value>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_set_decoration_provider",
                &[to_value(&ns_id)?, to_value(&opts)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the value of an option. The behavior of this function matches that of\n|:set|: the local value of an option is returned if it exists; otherwise,\nthe global value is returned. Local values always correspond to the\ncurrent buffer or window, unless buf or win is set in {opts}.\n"]
    pub async fn get_option_value<T>(&self, name: &str, opts: HashMap<String, Value>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_get_option_value",
                &[to_value(&name)?, to_value(&opts)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets the value of an option. The behavior of this function matches that of\n|:set|: for global-local options, both the global and local value are set\nunless otherwise specified with {scope}.\n\nNote the options {win} and {buf} cannot be used together.\n"]
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
        let ret = self
            .raw_request(
                "nvim_set_option_value",
                &[to_value(&name)?, to_value(&value)?, to_value(&opts)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the option information for all options.\n\nThe dict has the full option names as keys and option metadata dicts as\ndetailed at |nvim_get_option_info2()|.\n"]
    pub async fn get_all_options_info(&self) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_get_all_options_info", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the option information for one option from arbitrary buffer or window\n"]
    pub async fn get_option_info2(
        &self,
        name: &str,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_get_option_info2",
                &[to_value(&name)?, to_value(&opts)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the windows in a tabpage\n"]
    pub async fn tabpage_list_wins(&self, tabpage: &TabPage) -> Result<Vec<Window>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_tabpage_list_wins", &[to_value(&tabpage)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets a tab-scoped (t:) variable\n"]
    pub async fn tabpage_get_var<T>(&self, tabpage: &TabPage, name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_tabpage_get_var",
                &[to_value(&tabpage)?, to_value(&name)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets a tab-scoped (t:) variable\n"]
    pub async fn tabpage_set_var<T>(&self, tabpage: &TabPage, name: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_tabpage_set_var",
                &[to_value(&tabpage)?, to_value(&name)?, to_value(&value)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nRemoves a tab-scoped (t:) variable\n"]
    pub async fn tabpage_del_var(&self, tabpage: &TabPage, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_tabpage_del_var",
                &[to_value(&tabpage)?, to_value(&name)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the current window in a tabpage\n"]
    pub async fn tabpage_get_win(&self, tabpage: &TabPage) -> Result<Window> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_tabpage_get_win", &[to_value(&tabpage)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets the current window in a tabpage\n"]
    pub async fn tabpage_set_win(&self, tabpage: &TabPage, win: &Window) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_tabpage_set_win",
                &[to_value(&tabpage)?, to_value(&win)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the tabpage number\n"]
    pub async fn tabpage_get_number(&self, tabpage: &TabPage) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_tabpage_get_number", &[to_value(&tabpage)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nChecks if a tabpage is valid\n"]
    pub async fn tabpage_is_valid(&self, tabpage: &TabPage) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_tabpage_is_valid", &[to_value(&tabpage)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nActivates UI events on the channel.\n\nEntry point of all UI clients. Allows |--embed| to continue startup.\nImplies that the client is ready to show the UI. Adds the client to the\nlist of UIs. |nvim_list_uis()|\n\nNote: If multiple UI clients are attached, the global screen dimensions\ndegrade to the smallest client. E.g. if client A requests 80x40 but\nclient B requests 200x100, the global screen has size 80x40.\n"]
    pub async fn ui_attach(
        &self,
        width: i64,
        height: i64,
        options: HashMap<String, Value>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_ui_attach",
                &[to_value(&width)?, to_value(&height)?, to_value(&options)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nTells the nvim server if focus was gained or lost by the GUI\n"]
    pub async fn ui_set_focus(&self, gained: bool) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_ui_set_focus", &[to_value(&gained)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nDeactivates UI events on the channel.\n\nRemoves the client from the list of UIs. |nvim_list_uis()|\n"]
    pub async fn ui_detach(&self) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_ui_detach", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nTry to resize the UI.\n"]
    pub async fn ui_try_resize(&self, width: i64, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_ui_try_resize",
                &[to_value(&width)?, to_value(&height)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSet a UI option.\n"]
    pub async fn ui_set_option<T>(&self, name: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_ui_set_option", &[to_value(&name)?, to_value(&value)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nTell Nvim to resize a grid. Triggers a grid_resize event with the\nrequested grid size or the maximum size if it exceeds size limits.\n\nOn invalid grid handle, fails with error.\n"]
    pub async fn ui_try_resize_grid(&self, grid: i64, width: i64, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_ui_try_resize_grid",
                &[to_value(&grid)?, to_value(&width)?, to_value(&height)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nTells Nvim the number of elements displaying in the popupmenu, to decide\n*PageUp* and *PageDown* movement.\n"]
    pub async fn ui_pum_set_height(&self, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_ui_pum_set_height", &[to_value(&height)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nTells Nvim the geometry of the popupmenu, to align floating windows with\nan external popup menu.\n\nNote that this method is not to be confused with\n|nvim_ui_pum_set_height()|, which sets the number of visible items in the\npopup menu, while this function sets the bounding box of the popup menu,\nincluding visual elements such as borders and sliders. Floats need not use\nthe same font size, nor be anchored to exact grid corners, so one can set\nfloating-point numbers to the popup menu geometry.\n"]
    pub async fn ui_pum_set_bounds(
        &self,
        width: f64,
        height: f64,
        row: f64,
        col: f64,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_ui_pum_set_bounds",
                &[
                    to_value(&width)?,
                    to_value(&height)?,
                    to_value(&row)?,
                    to_value(&col)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nTells Nvim when a terminal event has occurred\n\nThe following terminal events are supported:\n* termresponse: The terminal sent an OSC or DCS response sequence to\nNvim. The payload is the received response. Sets |v:termresponse| and\nfires |TermResponse|.\n"]
    pub async fn ui_term_event<T>(&self, event: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_ui_term_event",
                &[to_value(&event)?, to_value(&value)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets a highlight group by name\n\nsimilar to |hlID()|, but allocates a new ID if not present.\n"]
    pub async fn get_hl_id_by_name(&self, name: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_hl_id_by_name", &[to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets all or specific highlight groups in a namespace.\n\nNote: When the link attribute is defined in the highlight definition map,\nother attributes will not be taking effect (see |:hi-link|).\n"]
    pub async fn get_hl(
        &self,
        ns_id: i64,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_hl", &[to_value(&ns_id)?, to_value(&opts)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets a highlight group.\n\nNote: Unlike the :highlight command which can update a highlight group,\nthis function completely replaces the definition. For example:\nnvim_set_hl(0, Visual, {}) will clear the highlight group 'Visual'.\n\nNote: The fg and bg keys also accept the string values fg or bg\nwhich act as aliases to the corresponding foreground and background\nvalues of the Normal group. If the Normal group has not been defined,\nusing these values results in an error.\n\nNote: If link is used in combination with other attributes; only the\nlink will take effect (see |:hi-link|).\n"]
    pub async fn set_hl(&self, ns_id: i64, name: &str, val: HashMap<String, Value>) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_set_hl",
                &[to_value(&ns_id)?, to_value(&name)?, to_value(&val)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the active highlight namespace.\n"]
    pub async fn get_hl_ns(&self, opts: HashMap<String, Value>) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_hl_ns", &[to_value(&opts)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSet active namespace for highlights defined with |nvim_set_hl()|. This can\nbe set for a single window, see |nvim_win_set_hl_ns()|.\n"]
    pub async fn set_hl_ns(&self, ns_id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_hl_ns", &[to_value(&ns_id)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSet active namespace for highlights defined with |nvim_set_hl()| while\nredrawing.\n\nThis function meant to be called while redrawing, primarily from\n|nvim_set_decoration_provider()| on_win and on_line callbacks, which are\nallowed to change the namespace during a redraw cycle.\n"]
    pub async fn set_hl_ns_fast(&self, ns_id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_hl_ns_fast", &[to_value(&ns_id)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSends input-keys to Nvim, subject to various quirks controlled by mode\nflags. This is a blocking call, unlike |nvim_input()|.\n\nOn execution error: does not fail, but updates v:errmsg.\n\nTo input sequences like *C-o* use |nvim_replace_termcodes()| (typically\nwith escape_ks=false) to replace |keycodes|, then pass the result to\nnvim_feedkeys().\n"]
    pub async fn feedkeys(&self, keys: &str, mode: &str, escape_ks: bool) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_feedkeys",
                &[to_value(&keys)?, to_value(&mode)?, to_value(&escape_ks)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nQueues raw user-input. Unlike |nvim_feedkeys()|, this uses a low-level\ninput buffer and the call is non-blocking (input is processed\nasynchronously by the eventloop).\n\nTo input blocks of text, |nvim_paste()| is much faster and should be\npreferred.\n\nOn execution error: does not fail, but updates v:errmsg.\n\nNote: |keycodes| like *CR* are translated, so < is special. To input a\nliteral *, send <LT*.\n\nNote: For mouse events use |nvim_input_mouse()|. The pseudokey form\n*LeftMouse**col,row* is deprecated since |api-level| 6.\n"]
    pub async fn input(&self, keys: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_input", &[to_value(&keys)?]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSend mouse event from GUI.\n\nNon-blocking: does not wait on any result, but queues the event to be\nprocessed soon by the event loop.\n\nNote: Currently this does not support scripting multiple mouse events by\ncalling it multiple times in a loop: the intermediate mouse positions\nwill be ignored. It should be used to implement real-time mouse input\nin a GUI. The deprecated pseudokey form (*LeftMouse**col,row*) of\n|nvim_input()| has the same limitation.\n"]
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
        let ret = self
            .raw_request(
                "nvim_input_mouse",
                &[
                    to_value(&button)?,
                    to_value(&action)?,
                    to_value(&modifier)?,
                    to_value(&grid)?,
                    to_value(&row)?,
                    to_value(&col)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nReplaces terminal codes and |keycodes| (*CR*, *Esc*, ...) in a string with\nthe internal representation.\n"]
    pub async fn replace_termcodes(
        &self,
        str: &str,
        from_part: bool,
        do_lt: bool,
        special: bool,
    ) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_replace_termcodes",
                &[
                    to_value(&str)?,
                    to_value(&from_part)?,
                    to_value(&do_lt)?,
                    to_value(&special)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nExecute Lua code. Parameters (if any) are available as ... inside the\nchunk. The chunk can return a value.\n\nOnly statements are executed. To evaluate an expression, prefix it with\nreturn: return my_function(...)\n"]
    pub async fn exec_lua<T>(&self, code: &str, args: Vec<Value>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_exec_lua", &[to_value(&code)?, to_value(&args)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nNotify the user with a message.\n\nRelays the call to vim.notify . By default forwards your message in the\necho area but can be overridden to trigger desktop notifications.\n"]
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
        let ret = self
            .raw_request(
                "nvim_notify",
                &[to_value(&msg)?, to_value(&log_level)?, to_value(&opts)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nCalculates the number of display cells occupied by text. Control\ncharacters including *Tab* count as one cell.\n"]
    pub async fn strwidth(&self, text: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_strwidth", &[to_value(&text)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the paths contained in |runtime-search-path|.\n"]
    pub async fn list_runtime_paths(&self) -> Result<Vec<String>> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_list_runtime_paths", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nFinds files in runtime directories, in runtimepath order.\n\nname can contain wildcards. For example\nnvim_get_runtime_file(colors/*.{vim,lua}, true) will return all color\nscheme files. Always use forward slashes (/) in the search pattern for\nsubdirectories regardless of platform.\n\nIt is not an error to not find any files. An empty array is returned then.\n"]
    pub async fn get_runtime_file(&self, name: &str, all: bool) -> Result<Vec<String>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_get_runtime_file",
                &[to_value(&name)?, to_value(&all)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nChanges the global working directory.\n"]
    pub async fn set_current_dir(&self, dir: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_current_dir", &[to_value(&dir)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the current line.\n"]
    pub async fn get_current_line(&self) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_get_current_line", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets the current line.\n"]
    pub async fn set_current_line(&self, line: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_current_line", &[to_value(&line)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nDeletes the current line.\n"]
    pub async fn del_current_line(&self) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_del_current_line", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets a global (g:) variable.\n"]
    pub async fn get_var<T>(&self, name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_var", &[to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets a global (g:) variable\n"]
    pub async fn set_var<T>(&self, name: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_var", &[to_value(&name)?, to_value(&value)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nRemoves a global (g:) variable.\n"]
    pub async fn del_var(&self, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_del_var", &[to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets a v: variable.\n"]
    pub async fn get_vvar<T>(&self, name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_vvar", &[to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets a v: variable, if it is not readonly.\n"]
    pub async fn set_vvar<T>(&self, name: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_vvar", &[to_value(&name)?, to_value(&value)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nEcho a message.\n"]
    pub async fn echo(
        &self,
        chunks: Vec<Value>,
        history: bool,
        opts: HashMap<String, Value>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_echo",
                &[to_value(&chunks)?, to_value(&history)?, to_value(&opts)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nWrites a message to the Vim output buffer. Does not append \\n, the\nmessage is buffered (will not display) until a linefeed is written.\n"]
    pub async fn out_write(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_out_write", &[to_value(&str)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nWrites a message to the Vim error buffer. Does not append \\n, the\nmessage is buffered (will not display) until a linefeed is written.\n"]
    pub async fn err_write(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_err_write", &[to_value(&str)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nWrites a message to the Vim error buffer. Appends \\n, so the buffer is\nflushed (and displayed).\n"]
    pub async fn err_writeln(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_err_writeln", &[to_value(&str)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the current list of buffer handles\n\nIncludes unlisted (unloaded/deleted) buffers, like :ls!. Use\n|nvim_buf_is_loaded()| to check if a buffer is loaded.\n"]
    pub async fn list_bufs(&self) -> Result<Vec<Buffer>> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_list_bufs", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the current buffer.\n"]
    pub async fn get_current_buf(&self) -> Result<Buffer> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_get_current_buf", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets the current buffer.\n"]
    pub async fn set_current_buf(&self, buffer: &Buffer) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_current_buf", &[to_value(&buffer)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the current list of window handles.\n"]
    pub async fn list_wins(&self) -> Result<Vec<Window>> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_list_wins", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the current window.\n"]
    pub async fn get_current_win(&self) -> Result<Window> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_get_current_win", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets the current window.\n"]
    pub async fn set_current_win(&self, window: &Window) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_current_win", &[to_value(&window)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nCreates a new, empty, unnamed buffer.\n"]
    pub async fn create_buf(&self, listed: bool, scratch: bool) -> Result<Buffer> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_create_buf",
                &[to_value(&listed)?, to_value(&scratch)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nOpen a terminal instance in a buffer\n\nBy default (and currently the only option) the terminal will not be\nconnected to an external process. Instead, input sent on the channel will\nbe echoed directly by the terminal. This is useful to display ANSI\nterminal sequences returned as part of a rpc message, or similar.\n\nNote: to directly initiate the terminal using the right size, display the\nbuffer in a configured window before calling this. For instance, for a\nfloating display, first create an empty buffer using |nvim_create_buf()|,\nthen display it using |nvim_open_win()|, and then call this function. Then\n|nvim_chan_send()| can be called immediately to process sequences in a\nvirtual terminal having the intended size.\n"]
    pub async fn open_term(&self, buffer: &Buffer, opts: HashMap<String, Value>) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_open_term", &[to_value(&buffer)?, to_value(&opts)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSend data to channel. For a job, it writes it to the stdin of the\nprocess. For the stdio channel `channel-stdio`, it writes to Nvim's\nstdout. For an internal terminal instance (`nvim_open_term()`) it writes\ndirectly to terminal output. See `channel-bytes` for more information.\n\nThis function writes raw data, not RPC messages. If the channel was\ncreated with rpc=true then the channel expects RPC messages, use\n`vim.rpcnotify()` and `vim.rpcrequest()` instead.\n"]
    pub async fn chan_send(&self, chan: i64, data: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_chan_send", &[to_value(&chan)?, to_value(&data)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the current list of tabpage handles.\n"]
    pub async fn list_tabpages(&self) -> Result<Vec<TabPage>> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_list_tabpages", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the current tabpage.\n"]
    pub async fn get_current_tabpage(&self) -> Result<TabPage> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_get_current_tabpage", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets the current tabpage.\n"]
    pub async fn set_current_tabpage(&self, tabpage: &TabPage) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_current_tabpage", &[to_value(&tabpage)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nPastes at cursor (in any mode), and sets redo so dot (|.|) will repeat\nthe input. UIs call this to implement paste, but it is also intended for\nuse by scripts to input large, dot-repeatable blocks of text (as opposed\nto |nvim_input()| which is subject to mappings/events and is thus much\nslower).\n\nInvokes the |vim.paste()| handler, which handles each mode appropriately.\n\nErrors (nomodifiable, vim.paste() failure, …) are reflected in err\nbut do not affect the return value (which is strictly decided by\nvim.paste()). On error or cancel, subsequent calls are ignored\n(drained) until the next paste is initiated (phase 1 or -1).\n"]
    pub async fn paste(&self, data: &str, crlf: bool, phase: i64) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_paste",
                &[to_value(&data)?, to_value(&crlf)?, to_value(&phase)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nPuts text at cursor, in any mode. For dot-repeatable input, use\n|nvim_paste()|.\n\nCompare |:put| and |p| which are always linewise.\n"]
    pub async fn put(
        &self,
        lines: Vec<String>,
        typ: &str,
        after: bool,
        follow: bool,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_put",
                &[
                    to_value(&lines)?,
                    to_value(&typ)?,
                    to_value(&after)?,
                    to_value(&follow)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nReturns the 24-bit RGB value of a |nvim_get_color_map()| color name or\n#rrggbb hexadecimal string.\n"]
    pub async fn get_color_by_name(&self, name: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_color_by_name", &[to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nReturns a map of color names and RGB values.\n\nKeys are color names (e.g. Aqua) and values are 24-bit RGB color values\n(e.g. 65535).\n"]
    pub async fn get_color_map(&self) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_get_color_map", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets a map of the current editor state.\n"]
    pub async fn get_context(
        &self,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_context", &[to_value(&opts)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets the current editor state from the given |context| map.\n"]
    pub async fn load_context<T>(&self, dict: HashMap<String, Value>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_load_context", &[to_value(&dict)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the current mode. |mode()| blocking is true if Nvim is waiting for\ninput.\n"]
    pub async fn get_mode(&self) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_get_mode", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets a list of global (non-buffer-local) |mapping| definitions.\n"]
    pub async fn get_keymap(&self, mode: &str) -> Result<Vec<HashMap<String, Value>>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_keymap", &[to_value(&mode)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets a global |mapping| for the given mode.\n\nTo set a buffer-local mapping, use |nvim_buf_set_keymap()|.\n\nUnlike |:map|, leading/trailing whitespace is accepted as part of the\n{lhs} or {rhs}. Empty {rhs} is *Nop*. |keycodes| are replaced as usual.\n"]
    pub async fn set_keymap(
        &self,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: HashMap<String, Value>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_set_keymap",
                &[
                    to_value(&mode)?,
                    to_value(&lhs)?,
                    to_value(&rhs)?,
                    to_value(&opts)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nUnmaps a global mapping for the given mode.\n\nTo unmap a buffer-local mapping, use nvim_buf_del_keymap().\n"]
    pub async fn del_keymap(&self, mode: &str, lhs: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_del_keymap", &[to_value(&mode)?, to_value(&lhs)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nReturns a 2-tuple (Array), where item 0 is the current channel id and item\n1 is the |api-metadata| map (Dict).\n"]
    pub async fn get_api_info(&self) -> Result<(u64, ApiInfo)> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_get_api_info", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSelf-identifies the client.\n\nThe client/plugin/application should call this after connecting, to\nprovide hints about its identity and purpose, for debugging and\norchestration.\n\nCan be called more than once; the caller should merge old info if\nappropriate. Example: library first identifies the channel, then a plugin\nusing that library later identifies itself.\n\nNote: Something is better than nothing. You do not need to include all the\nfields.\n"]
    pub async fn set_client_info(
        &self,
        name: &str,
        version: HashMap<String, Value>,
        typ: &str,
        methods: HashMap<String, Value>,
        attributes: HashMap<String, Value>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_set_client_info",
                &[
                    to_value(&name)?,
                    to_value(&version)?,
                    to_value(&typ)?,
                    to_value(&methods)?,
                    to_value(&attributes)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets information about a channel.\n"]
    pub async fn get_chan_info(&self, chan: i64) -> Result<ChanInfo> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_chan_info", &[to_value(&chan)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGet information about all open channels.\n"]
    pub async fn list_chans(&self) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_list_chans", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets a list of dictionaries representing attached UIs.\n"]
    pub async fn list_uis(&self) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_list_uis", &[]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the immediate children of process pid.\n"]
    pub async fn get_proc_children(&self, pid: i64) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_proc_children", &[to_value(&pid)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets info describing process pid.\n"]
    pub async fn get_proc<T>(&self, pid: i64) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_proc", &[to_value(&pid)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSelects an item in the completion popup menu.\n\nIf neither |ins-completion| nor |cmdline-completion| popup menu is active\nthis API call is silently ignored. Useful for an external UI using\n|ui-popupmenu| to control the popup menu with the mouse. Can also be used\nin a mapping; use *Cmd* |:map-cmd| or a Lua mapping to ensure the mapping\ndoes not end completion mode.\n"]
    pub async fn select_popupmenu_item(
        &self,
        item: i64,
        insert: bool,
        finish: bool,
        opts: HashMap<String, Value>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_select_popupmenu_item",
                &[
                    to_value(&item)?,
                    to_value(&insert)?,
                    to_value(&finish)?,
                    to_value(&opts)?,
                ],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nDeletes an uppercase/file named mark. See mark-motions.\n\nNote: Lowercase name (or other buffer-local mark) is an error.\n"]
    pub async fn del_mark(&self, name: &str) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_del_mark", &[to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nReturns a (row, col, buffer, buffername) tuple representing the position\nof the uppercase/file named mark. End of line column position is\nreturned as |v:maxcol| (big number). See |mark-motions|.\n\nMarks are (1,0)-indexed. |api-indexing|\n\nNote: Lowercase name (or other buffer-local mark) is an error.\n"]
    pub async fn get_mark(&self, name: &str, opts: HashMap<String, Value>) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_mark", &[to_value(&name)?, to_value(&opts)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nEvaluates statusline string.\n"]
    pub async fn eval_statusline(
        &self,
        str: &str,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_eval_statusline", &[to_value(&str)?, to_value(&opts)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nExecutes Vimscript (multiline block of Ex commands), like anonymous\n|:source|.\n\nUnlike |nvim_command()| this function supports heredocs, script-scope\n(s:), etc.\n\nOn execution error: fails with Vimscript error, updates v:errmsg.\n"]
    pub async fn exec2(
        &self,
        src: &str,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_exec2", &[to_value(&src)?, to_value(&opts)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nExecutes an Ex command.\n\nOn execution error: fails with Vimscript error, updates v:errmsg.\n\nPrefer `nvim_cmd()` or `nvim_exec2()` instead. To modify an Ex command in\na structured way before executing it, modify the result of\n`nvim_parse_cmd()` then pass it to `nvim_cmd()`.\n"]
    pub async fn command(&self, command: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_command", &[to_value(&command)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nEvaluates a Vimscript expression. Dicts and Lists are recursively\nexpanded.\n\nOn execution error: fails with Vimscript error, updates v:errmsg.\n"]
    pub async fn eval<T>(&self, expr: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let ret = self.raw_request("nvim_eval", &[to_value(&expr)?]).await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nCalls a Vimscript function with the given arguments.\n\nOn execution error: fails with Vimscript error, updates v:errmsg.\n"]
    pub async fn call_function<T>(&self, func: &str, args: Vec<Value>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_call_function", &[to_value(&func)?, to_value(&args)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nCalls a Vimscript `Dictionary-function` with the given arguments.\n\nOn execution error: fails with Vimscript error, updates v:errmsg.\n"]
    pub async fn call_dict_function<T, U>(&self, dict: T, func: &str, args: Vec<Value>) -> Result<U>
    where
        T: Serialize,
        U: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_call_dict_function",
                &[to_value(&dict)?, to_value(&func)?, to_value(&args)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nParse a Vimscript expression.\n"]
    pub async fn parse_expression(
        &self,
        expr: &str,
        flags: &str,
        highlight: bool,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_parse_expression",
                &[to_value(&expr)?, to_value(&flags)?, to_value(&highlight)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nOpens a new split window, or a floating window if relative is specified,\nor an external window (managed by the UI) if external is specified.\n\nFloats are windows that are drawn above the split layout, at some anchor\nposition in some other window. Floats can be drawn internally or by\nexternal GUI with the |ui-multigrid| extension. External windows are only\nsupported with multigrid GUIs, and are displayed as separate top-level\nwindows.\n\nFor a general overview of floats, see |api-floatwin|.\n\nThe width and height of the new window must be specified when opening\na floating window, but are optional for normal windows.\n\nIf relative and external are omitted, a normal split window is\ncreated. The win property determines which window will be split. If no\nwin is provided or win == 0, a window will be created adjacent to the\ncurrent window. If -1 is provided, a top-level split will be created.\nvertical and split are only valid for normal windows, and are used to\ncontrol split direction. For vertical, the exact direction is determined\nby splitright and splitbelow. Split windows cannot have\nbufpos/row/col/border/title/footer properties.\n\nWith relative=editor (row=0,col=0) refers to the top-left corner of the\nscreen-grid and (row=Lines-1,col=Columns-1) refers to the bottom-right\ncorner. Fractional values are allowed, but the builtin implementation\n(used by non-multigrid UIs) will always round down to nearest integer.\n\nOut-of-bounds values, and configurations that make the float not fit\ninside the main editor, are allowed. The builtin implementation truncates\nvalues so floats are fully within the main screen grid. External GUIs\ncould let floats hover outside of the main window like a tooltip, but this\nshould not be used to specify arbitrary WM screen positions.\n"]
    pub async fn open_win(
        &self,
        buffer: &Buffer,
        enter: bool,
        config: WindowConf,
    ) -> Result<Window> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_open_win",
                &[to_value(&buffer)?, to_value(&enter)?, to_value(&config)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nConfigures window layout. Cannot be used to move the last window in a\ntabpage to a different one.\n\nWhen reconfiguring a window, absent option keys will not be changed.\nrow/col and relative must be reconfigured together.\n"]
    pub async fn win_set_config(&self, window: &Window, config: WindowConf) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_set_config",
                &[to_value(&window)?, to_value(&config)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets window configuration.\n\nThe returned value may be given to `nvim_open_win()`.\n\nrelative is empty for normal windows.\n"]
    pub async fn win_get_config(&self, window: &Window) -> Result<WindowConf> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_config", &[to_value(&window)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the current buffer in a window\n"]
    pub async fn win_get_buf(&self, window: &Window) -> Result<Buffer> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_buf", &[to_value(&window)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets the current buffer in a window, without side effects\n"]
    pub async fn win_set_buf(&self, window: &Window, buffer: &Buffer) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_set_buf",
                &[to_value(&window)?, to_value(&buffer)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the (1,0)-indexed, buffer-relative cursor position for a given window\n(different windows showing the same buffer have independent cursor\npositions).\n"]
    pub async fn win_get_cursor(&self, window: &Window) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_cursor", &[to_value(&window)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets the (1,0)-indexed cursor position in the window. This scrolls the\nwindow even if it is not the current one.\n"]
    pub async fn win_set_cursor(&self, window: &Window, pos: Vec<i64>) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_set_cursor",
                &[to_value(&window)?, to_value(&pos)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the window height\n"]
    pub async fn win_get_height(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_height", &[to_value(&window)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets the window height.\n"]
    pub async fn win_set_height(&self, window: &Window, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_set_height",
                &[to_value(&window)?, to_value(&height)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the window width\n"]
    pub async fn win_get_width(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_width", &[to_value(&window)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets the window width. This will only succeed if the screen is split\nvertically.\n"]
    pub async fn win_set_width(&self, window: &Window, width: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_set_width",
                &[to_value(&window)?, to_value(&width)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets a window-scoped (w:) variable\n"]
    pub async fn win_get_var<T>(&self, window: &Window, name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_var", &[to_value(&window)?, to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSets a window-scoped (w:) variable\n"]
    pub async fn win_set_var<T>(&self, window: &Window, name: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_set_var",
                &[to_value(&window)?, to_value(&name)?, to_value(&value)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nRemoves a window-scoped (w:) variable\n"]
    pub async fn win_del_var(&self, window: &Window, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_del_var", &[to_value(&window)?, to_value(&name)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the window position in display cells. First position is zero.\n"]
    pub async fn win_get_position(&self, window: &Window) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_position", &[to_value(&window)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the window tabpage\n"]
    pub async fn win_get_tabpage(&self, window: &Window) -> Result<TabPage> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_tabpage", &[to_value(&window)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nGets the window number\n"]
    pub async fn win_get_number(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_number", &[to_value(&window)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nChecks if a window is valid\n"]
    pub async fn win_is_valid(&self, window: &Window) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_is_valid", &[to_value(&window)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nCloses the window and hide the buffer it contains (like |:hide| with a\n|window-ID|).\n\nLike |:hide| the buffer becomes hidden unless another window is editing\nit, or bufhidden is unload, delete or wipe as opposed to |:close|\nor |nvim_win_close()|, which will close the buffer.\n"]
    pub async fn win_hide(&self, window: &Window) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_hide", &[to_value(&window)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nCloses the window (like |:close| with a |window-ID|).\n"]
    pub async fn win_close(&self, window: &Window, force: bool) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_close", &[to_value(&window)?, to_value(&force)?])
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nSet highlight namespace for a window. This will use highlights defined\nwith |nvim_set_hl()| for this namespace, but fall back to global\nhighlights (ns=0) when missing.\n\nThis takes precedence over the winhighlight option.\n"]
    pub async fn win_set_hl_ns(&self, window: &Window, ns_id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_set_hl_ns",
                &[to_value(&window)?, to_value(&ns_id)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    #[doc = "\nComputes the number of screen lines occupied by a range of text in a given\nwindow. Works for off-screen text and takes folds into account.\n\nDiff filler or virtual lines above a line are counted as a part of that\nline, unless the line is on start_row and start_vcol is specified.\n\nDiff filler or virtual lines below the last buffer line are counted in the\nresult when end_row is omitted.\n\nLine indexing is similar to `nvim_buf_get_text()`.\n"]
    pub async fn win_text_height(
        &self,
        window: &Window,
        opts: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_text_height",
                &[to_value(&window)?, to_value(&opts)?],
            )
            .await?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
}
