#![allow(clippy::needless_question_mark)]
#![allow(clippy::needless_borrow)]
use crate::error::{Error, Result};
use crate::types::*;
use msgpack_rpc::Value;
use serde_rmpv::{from_value, to_value};
use tracing::trace;
#[derive(Clone)]
#[doc = r" Auto-generated API for Neovim's MessagePack-RPC protocol."]
pub struct NvimApi {
    pub(crate) m_client: msgpack_rpc::Client,
}
impl NvimApi {
    #[doc = r" Make a raw request over the MessagePack-RPC protocol."]
    pub async fn raw_request(
        &self,
        method: &str,
        params: &[msgpack_rpc::Value],
    ) -> Result<msgpack_rpc::Value, msgpack_rpc::Value> {
        trace!("send request: {:?} {:?}", method, params);
        self.m_client.request(method, params).await
    }
    #[doc = r" Send a raw notification over the MessagePack-RPC protocol."]
    pub async fn raw_notify(&self, method: &str, params: &[msgpack_rpc::Value]) -> Result<(), ()> {
        trace!("send notification: {:?} {:?}", method, params);
        self.m_client.notify(method, params).await
    }
    pub async fn nvim_get_autocmds(&self, opts: Value) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_autocmds", &[to_value(&opts)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_create_autocmd(
        &self,
        event: &[String],
        opts: Option<CreateAutocmdOpts>,
    ) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_create_autocmd",
                &[to_value(&event)?, to_value(&opts)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_del_autocmd(&self, id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_del_autocmd", &[to_value(&id)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_clear_autocmds(&self, opts: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_clear_autocmds", &[to_value(&opts)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_create_augroup(&self, name: &str, opts: Value) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_create_augroup", &[to_value(&name)?, to_value(&opts)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_del_augroup_by_id(&self, id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_del_augroup_by_id", &[to_value(&id)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_del_augroup_by_name(&self, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_del_augroup_by_name", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_exec_autocmds(&self, event: Value, opts: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_exec_autocmds", &[to_value(&event)?, to_value(&opts)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_line_count(&self, buffer: &Buffer) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_line_count", &[to_value(&buffer)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_attach(
        &self,
        buffer: &Buffer,
        send_buffer: bool,
        opts: Value,
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_detach(&self, buffer: &Buffer) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_detach", &[to_value(&buffer)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_get_lines(
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_set_lines(
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_set_text(
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_get_text(
        &self,
        buffer: &Buffer,
        start_row: i64,
        start_col: i64,
        end_row: i64,
        end_col: i64,
        opts: Value,
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_get_offset(&self, buffer: &Buffer, index: i64) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_get_offset",
                &[to_value(&buffer)?, to_value(&index)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_get_var(&self, buffer: &Buffer, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_get_var", &[to_value(&buffer)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_get_changedtick(&self, buffer: &Buffer) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_get_changedtick", &[to_value(&buffer)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_get_keymap(&self, buffer: &Buffer, mode: &str) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_get_keymap",
                &[to_value(&buffer)?, to_value(&mode)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_set_keymap(
        &self,
        buffer: &Buffer,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: Value,
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_del_keymap(&self, buffer: &Buffer, mode: &str, lhs: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_del_keymap",
                &[to_value(&buffer)?, to_value(&mode)?, to_value(&lhs)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_set_var(&self, buffer: &Buffer, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_set_var",
                &[to_value(&buffer)?, to_value(&name)?, to_value(&value)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_del_var(&self, buffer: &Buffer, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_del_var", &[to_value(&buffer)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_get_name(&self, buffer: &Buffer) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_get_name", &[to_value(&buffer)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_set_name(&self, buffer: &Buffer, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_set_name", &[to_value(&buffer)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_is_loaded(&self, buffer: &Buffer) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_is_loaded", &[to_value(&buffer)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_delete(&self, buffer: &Buffer, opts: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_delete", &[to_value(&buffer)?, to_value(&opts)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_is_valid(&self, buffer: &Buffer) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_is_valid", &[to_value(&buffer)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_del_mark(&self, buffer: &Buffer, name: &str) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_del_mark", &[to_value(&buffer)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_set_mark(
        &self,
        buffer: &Buffer,
        name: &str,
        line: i64,
        col: i64,
        opts: Value,
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_get_mark(&self, buffer: &Buffer, name: &str) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_get_mark", &[to_value(&buffer)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_parse_cmd(&self, str: &str, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_parse_cmd", &[to_value(&str)?, to_value(&opts)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_cmd(&self, cmd: Value, opts: Value) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_cmd", &[to_value(&cmd)?, to_value(&opts)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_create_user_command(
        &self,
        name: &str,
        command: Value,
        opts: Value,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_create_user_command",
                &[to_value(&name)?, to_value(&command)?, to_value(&opts)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_del_user_command(&self, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_del_user_command", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_create_user_command(
        &self,
        buffer: &Buffer,
        name: &str,
        command: Value,
        opts: Value,
    ) -> Result<()> {
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_del_user_command(&self, buffer: &Buffer, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_del_user_command",
                &[to_value(&buffer)?, to_value(&name)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_commands(&self, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_commands", &[to_value(&opts)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_get_commands(&self, buffer: &Buffer, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_get_commands",
                &[to_value(&buffer)?, to_value(&opts)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_exec(&self, src: &str, output: bool) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_exec", &[to_value(&src)?, to_value(&output)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_command_output(&self, command: &str) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_command_output", &[to_value(&command)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_execute_lua(&self, code: &str, args: Vec<Value>) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_execute_lua", &[to_value(&code)?, to_value(&args)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_get_number(&self, buffer: &Buffer) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_buf_get_number", &[to_value(&buffer)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_clear_highlight(
        &self,
        buffer: &Buffer,
        ns_id: i64,
        line_start: i64,
        line_end: i64,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_clear_highlight",
                &[
                    to_value(&buffer)?,
                    to_value(&ns_id)?,
                    to_value(&line_start)?,
                    to_value(&line_end)?,
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_set_virtual_text(
        &self,
        buffer: &Buffer,
        src_id: i64,
        line: i64,
        chunks: Vec<Value>,
        opts: Value,
    ) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_set_virtual_text",
                &[
                    to_value(&buffer)?,
                    to_value(&src_id)?,
                    to_value(&line)?,
                    to_value(&chunks)?,
                    to_value(&opts)?,
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_hl_by_id(&self, hl_id: i64, rgb: bool) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_hl_by_id", &[to_value(&hl_id)?, to_value(&rgb)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_hl_by_name(&self, name: &str, rgb: bool) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_hl_by_name", &[to_value(&name)?, to_value(&rgb)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_insert(
        &self,
        buffer: &Buffer,
        lnum: i64,
        lines: Vec<String>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "buffer_insert",
                &[to_value(&buffer)?, to_value(&lnum)?, to_value(&lines)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_get_line(&self, buffer: &Buffer, index: i64) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("buffer_get_line", &[to_value(&buffer)?, to_value(&index)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_set_line(&self, buffer: &Buffer, index: i64, line: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "buffer_set_line",
                &[to_value(&buffer)?, to_value(&index)?, to_value(&line)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_del_line(&self, buffer: &Buffer, index: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("buffer_del_line", &[to_value(&buffer)?, to_value(&index)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_get_line_slice(
        &self,
        buffer: &Buffer,
        start: i64,
        end: i64,
        include_start: bool,
        include_end: bool,
    ) -> Result<Vec<String>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "buffer_get_line_slice",
                &[
                    to_value(&buffer)?,
                    to_value(&start)?,
                    to_value(&end)?,
                    to_value(&include_start)?,
                    to_value(&include_end)?,
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_set_line_slice(
        &self,
        buffer: &Buffer,
        start: i64,
        end: i64,
        include_start: bool,
        include_end: bool,
        replacement: Vec<String>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "buffer_set_line_slice",
                &[
                    to_value(&buffer)?,
                    to_value(&start)?,
                    to_value(&end)?,
                    to_value(&include_start)?,
                    to_value(&include_end)?,
                    to_value(&replacement)?,
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_set_var(&self, buffer: &Buffer, name: &str, value: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "buffer_set_var",
                &[to_value(&buffer)?, to_value(&name)?, to_value(&value)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_del_var(&self, buffer: &Buffer, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("buffer_del_var", &[to_value(&buffer)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn window_set_var(&self, window: &Window, name: &str, value: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "window_set_var",
                &[to_value(&window)?, to_value(&name)?, to_value(&value)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn window_del_var(&self, window: &Window, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("window_del_var", &[to_value(&window)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn tabpage_set_var(
        &self,
        tabpage: &TabPage,
        name: &str,
        value: Value,
    ) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "tabpage_set_var",
                &[to_value(&tabpage)?, to_value(&name)?, to_value(&value)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn tabpage_del_var(&self, tabpage: &TabPage, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("tabpage_del_var", &[to_value(&tabpage)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_set_var(&self, name: &str, value: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_set_var", &[to_value(&name)?, to_value(&value)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_del_var(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_del_var", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_option_info(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_option_info", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_set_option(&self, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_option", &[to_value(&name)?, to_value(&value)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_option(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_option", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_get_option(&self, buffer: &Buffer, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_get_option",
                &[to_value(&buffer)?, to_value(&name)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_set_option(
        &self,
        buffer: &Buffer,
        name: &str,
        value: Value,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_set_option",
                &[to_value(&buffer)?, to_value(&name)?, to_value(&value)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_get_option(&self, window: &Window, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_get_option",
                &[to_value(&window)?, to_value(&name)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_set_option(
        &self,
        window: &Window,
        name: &str,
        value: Value,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_set_option",
                &[to_value(&window)?, to_value(&name)?, to_value(&value)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_call_atomic(&self, calls: Vec<Value>) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_call_atomic", &[to_value(&calls)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_create_namespace(&self, name: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_create_namespace", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_namespaces(&self) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_namespaces", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_get_extmark_by_id(
        &self,
        buffer: &Buffer,
        ns_id: i64,
        id: i64,
        opts: Value,
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_get_extmarks(
        &self,
        buffer: &Buffer,
        ns_id: i64,
        start: Value,
        end: Value,
        opts: Value,
    ) -> Result<Vec<Value>> {
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_set_extmark(
        &self,
        buffer: &Buffer,
        ns_id: i64,
        line: i64,
        col: i64,
        opts: Value,
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_del_extmark(&self, buffer: &Buffer, ns_id: i64, id: i64) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_buf_del_extmark",
                &[to_value(&buffer)?, to_value(&ns_id)?, to_value(&id)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_add_highlight(
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_buf_clear_namespace(
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_set_decoration_provider(&self, ns_id: i64, opts: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_set_decoration_provider",
                &[to_value(&ns_id)?, to_value(&opts)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_option_value(&self, name: &str, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_get_option_value",
                &[to_value(&name)?, to_value(&opts)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_set_option_value(&self, name: &str, value: Value, opts: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_set_option_value",
                &[to_value(&name)?, to_value(&value)?, to_value(&opts)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_all_options_info(&self) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_all_options_info", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_option_info2(&self, name: &str, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_get_option_info2",
                &[to_value(&name)?, to_value(&opts)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_tabpage_list_wins(&self, tabpage: &TabPage) -> Result<Vec<Window>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_tabpage_list_wins", &[to_value(&tabpage)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_tabpage_get_var(&self, tabpage: &TabPage, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_tabpage_get_var",
                &[to_value(&tabpage)?, to_value(&name)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_tabpage_set_var(
        &self,
        tabpage: &TabPage,
        name: &str,
        value: Value,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_tabpage_set_var",
                &[to_value(&tabpage)?, to_value(&name)?, to_value(&value)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_tabpage_del_var(&self, tabpage: &TabPage, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_tabpage_del_var",
                &[to_value(&tabpage)?, to_value(&name)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_tabpage_get_win(&self, tabpage: &TabPage) -> Result<Window> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_tabpage_get_win", &[to_value(&tabpage)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_tabpage_set_win(&self, tabpage: &TabPage, win: &Window) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_tabpage_set_win",
                &[to_value(&tabpage)?, to_value(&win)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_tabpage_get_number(&self, tabpage: &TabPage) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_tabpage_get_number", &[to_value(&tabpage)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_tabpage_is_valid(&self, tabpage: &TabPage) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_tabpage_is_valid", &[to_value(&tabpage)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_ui_attach(&self, width: i64, height: i64, options: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_ui_attach",
                &[to_value(&width)?, to_value(&height)?, to_value(&options)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn ui_attach(&self, width: i64, height: i64, enable_rgb: bool) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "ui_attach",
                &[
                    to_value(&width)?,
                    to_value(&height)?,
                    to_value(&enable_rgb)?,
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_ui_set_focus(&self, gained: bool) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_ui_set_focus", &[to_value(&gained)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_ui_detach(&self) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_ui_detach", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_ui_try_resize(&self, width: i64, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_ui_try_resize",
                &[to_value(&width)?, to_value(&height)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_ui_set_option(&self, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_ui_set_option", &[to_value(&name)?, to_value(&value)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_ui_try_resize_grid(&self, grid: i64, width: i64, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_ui_try_resize_grid",
                &[to_value(&grid)?, to_value(&width)?, to_value(&height)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_ui_pum_set_height(&self, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_ui_pum_set_height", &[to_value(&height)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_ui_pum_set_bounds(
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_ui_term_event(&self, event: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_ui_term_event",
                &[to_value(&event)?, to_value(&value)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_hl_id_by_name(&self, name: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_hl_id_by_name", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_hl(&self, ns_id: i64, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_hl", &[to_value(&ns_id)?, to_value(&opts)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_set_hl(&self, ns_id: i64, name: &str, val: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_set_hl",
                &[to_value(&ns_id)?, to_value(&name)?, to_value(&val)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_hl_ns(&self, opts: Value) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_hl_ns", &[to_value(&opts)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_set_hl_ns(&self, ns_id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_hl_ns", &[to_value(&ns_id)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_set_hl_ns_fast(&self, ns_id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_hl_ns_fast", &[to_value(&ns_id)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_feedkeys(&self, keys: &str, mode: &str, escape_ks: bool) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_feedkeys",
                &[to_value(&keys)?, to_value(&mode)?, to_value(&escape_ks)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_input(&self, keys: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_input", &[to_value(&keys)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_input_mouse(
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_replace_termcodes(
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_exec_lua(&self, code: &str, args: Vec<Value>) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_exec_lua", &[to_value(&code)?, to_value(&args)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_notify(&self, msg: &str, log_level: u64, opts: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_notify",
                &[to_value(&msg)?, to_value(&log_level)?, to_value(&opts)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_strwidth(&self, text: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_strwidth", &[to_value(&text)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_list_runtime_paths(&self) -> Result<Vec<String>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_list_runtime_paths", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_runtime_file(&self, name: &str, all: bool) -> Result<Vec<String>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_get_runtime_file",
                &[to_value(&name)?, to_value(&all)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_set_current_dir(&self, dir: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_current_dir", &[to_value(&dir)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_current_line(&self) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_current_line", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_set_current_line(&self, line: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_current_line", &[to_value(&line)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_del_current_line(&self) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_del_current_line", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_var(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_var", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_set_var(&self, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_var", &[to_value(&name)?, to_value(&value)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_del_var(&self, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_del_var", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_vvar(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_vvar", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_set_vvar(&self, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_vvar", &[to_value(&name)?, to_value(&value)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_echo(&self, chunks: Vec<Value>, history: bool, opts: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_echo",
                &[to_value(&chunks)?, to_value(&history)?, to_value(&opts)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_out_write(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_out_write", &[to_value(&str)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_err_write(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_err_write", &[to_value(&str)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_err_writeln(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_err_writeln", &[to_value(&str)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_list_bufs(&self) -> Result<Vec<Buffer>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_list_bufs", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_current_buf(&self) -> Result<Buffer> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_current_buf", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_set_current_buf(&self, buffer: &Buffer) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_current_buf", &[to_value(&buffer)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_list_wins(&self) -> Result<Vec<Window>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_list_wins", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_current_win(&self) -> Result<Window> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_current_win", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_set_current_win(&self, window: &Window) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_current_win", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_create_buf(&self, listed: bool, scratch: bool) -> Result<Buffer> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_create_buf",
                &[to_value(&listed)?, to_value(&scratch)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_open_term(&self, buffer: &Buffer, opts: Value) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_open_term", &[to_value(&buffer)?, to_value(&opts)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_chan_send(&self, chan: i64, data: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_chan_send", &[to_value(&chan)?, to_value(&data)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_list_tabpages(&self) -> Result<Vec<TabPage>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_list_tabpages", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_current_tabpage(&self) -> Result<TabPage> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_current_tabpage", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_set_current_tabpage(&self, tabpage: &TabPage) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_set_current_tabpage", &[to_value(&tabpage)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_paste(&self, data: &str, crlf: bool, phase: i64) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_paste",
                &[to_value(&data)?, to_value(&crlf)?, to_value(&phase)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_put(
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_subscribe(&self, event: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_subscribe", &[to_value(&event)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_unsubscribe(&self, event: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_unsubscribe", &[to_value(&event)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_color_by_name(&self, name: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_color_by_name", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_color_map(&self) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_color_map", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_context(&self, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_context", &[to_value(&opts)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_load_context(&self, dict: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_load_context", &[to_value(&dict)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_mode(&self) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_mode", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_keymap(&self, mode: &str) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_keymap", &[to_value(&mode)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_set_keymap(
        &self,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: Value,
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_del_keymap(&self, mode: &str, lhs: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_del_keymap", &[to_value(&mode)?, to_value(&lhs)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_api_info(&self) -> Result<(u64, ApiInfo)> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_api_info", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_set_client_info(
        &self,
        name: &str,
        version: Value,
        typ: &str,
        methods: Value,
        attributes: Value,
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_chan_info(&self, chan: i64) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_chan_info", &[to_value(&chan)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_list_chans(&self) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_list_chans", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_list_uis(&self) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_list_uis", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_proc_children(&self, pid: i64) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_proc_children", &[to_value(&pid)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_proc(&self, pid: i64) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_proc", &[to_value(&pid)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_select_popupmenu_item(
        &self,
        item: i64,
        insert: bool,
        finish: bool,
        opts: Value,
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
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_del_mark(&self, name: &str) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_del_mark", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_get_mark(&self, name: &str, opts: Value) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_get_mark", &[to_value(&name)?, to_value(&opts)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_eval_statusline(&self, str: &str, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_eval_statusline", &[to_value(&str)?, to_value(&opts)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_exec2(&self, src: &str, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_exec2", &[to_value(&src)?, to_value(&opts)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_command(&self, command: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_command", &[to_value(&command)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_eval(&self, expr: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_eval", &[to_value(&expr)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_call_function(&self, func: &str, args: Vec<Value>) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_call_function", &[to_value(&func)?, to_value(&args)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_call_dict_function(
        &self,
        dict: Value,
        func: &str,
        args: Vec<Value>,
    ) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_call_dict_function",
                &[to_value(&dict)?, to_value(&func)?, to_value(&args)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_parse_expression(
        &self,
        expr: &str,
        flags: &str,
        highlight: bool,
    ) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_parse_expression",
                &[to_value(&expr)?, to_value(&flags)?, to_value(&highlight)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_open_win(
        &self,
        buffer: &Buffer,
        enter: bool,
        config: Value,
    ) -> Result<Window> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_open_win",
                &[to_value(&buffer)?, to_value(&enter)?, to_value(&config)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_set_config(&self, window: &Window, config: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_set_config",
                &[to_value(&window)?, to_value(&config)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_get_config(&self, window: &Window) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_config", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_get_buf(&self, window: &Window) -> Result<Buffer> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_buf", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_set_buf(&self, window: &Window, buffer: &Buffer) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_set_buf",
                &[to_value(&window)?, to_value(&buffer)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_get_cursor(&self, window: &Window) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_cursor", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_set_cursor(&self, window: &Window, pos: Vec<i64>) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_set_cursor",
                &[to_value(&window)?, to_value(&pos)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_get_height(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_height", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_set_height(&self, window: &Window, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_set_height",
                &[to_value(&window)?, to_value(&height)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_get_width(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_width", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_set_width(&self, window: &Window, width: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_set_width",
                &[to_value(&window)?, to_value(&width)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_get_var(&self, window: &Window, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_var", &[to_value(&window)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_set_var(&self, window: &Window, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_set_var",
                &[to_value(&window)?, to_value(&name)?, to_value(&value)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_del_var(&self, window: &Window, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_del_var", &[to_value(&window)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_get_position(&self, window: &Window) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_position", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_get_tabpage(&self, window: &Window) -> Result<TabPage> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_tabpage", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_get_number(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_get_number", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_is_valid(&self, window: &Window) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_is_valid", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_hide(&self, window: &Window) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_hide", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_close(&self, window: &Window, force: bool) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("nvim_win_close", &[to_value(&window)?, to_value(&force)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_set_hl_ns(&self, window: &Window, ns_id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_set_hl_ns",
                &[to_value(&window)?, to_value(&ns_id)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn nvim_win_text_height(&self, window: &Window, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "nvim_win_text_height",
                &[to_value(&window)?, to_value(&opts)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_line_count(&self, buffer: &Buffer) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("buffer_line_count", &[to_value(&buffer)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_get_lines(
        &self,
        buffer: &Buffer,
        start: i64,
        end: i64,
        strict_indexing: bool,
    ) -> Result<Vec<String>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "buffer_get_lines",
                &[
                    to_value(&buffer)?,
                    to_value(&start)?,
                    to_value(&end)?,
                    to_value(&strict_indexing)?,
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_set_lines(
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
                "buffer_set_lines",
                &[
                    to_value(&buffer)?,
                    to_value(&start)?,
                    to_value(&end)?,
                    to_value(&strict_indexing)?,
                    to_value(&replacement)?,
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_get_var(&self, buffer: &Buffer, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("buffer_get_var", &[to_value(&buffer)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_get_name(&self, buffer: &Buffer) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("buffer_get_name", &[to_value(&buffer)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_set_name(&self, buffer: &Buffer, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("buffer_set_name", &[to_value(&buffer)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_is_valid(&self, buffer: &Buffer) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("buffer_is_valid", &[to_value(&buffer)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_get_mark(&self, buffer: &Buffer, name: &str) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("buffer_get_mark", &[to_value(&buffer)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_command_output(&self, command: &str) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_command_output", &[to_value(&command)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_get_number(&self, buffer: &Buffer) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("buffer_get_number", &[to_value(&buffer)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_clear_highlight(
        &self,
        buffer: &Buffer,
        ns_id: i64,
        line_start: i64,
        line_end: i64,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "buffer_clear_highlight",
                &[
                    to_value(&buffer)?,
                    to_value(&ns_id)?,
                    to_value(&line_start)?,
                    to_value(&line_end)?,
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_set_option(&self, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_set_option", &[to_value(&name)?, to_value(&value)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_get_option(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_get_option", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_get_option(&self, buffer: &Buffer, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("buffer_get_option", &[to_value(&buffer)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_set_option(&self, buffer: &Buffer, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "buffer_set_option",
                &[to_value(&buffer)?, to_value(&name)?, to_value(&value)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn window_get_option(&self, window: &Window, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("window_get_option", &[to_value(&window)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn window_set_option(&self, window: &Window, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "window_set_option",
                &[to_value(&window)?, to_value(&name)?, to_value(&value)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn buffer_add_highlight(
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
                "buffer_add_highlight",
                &[
                    to_value(&buffer)?,
                    to_value(&ns_id)?,
                    to_value(&hl_group)?,
                    to_value(&line)?,
                    to_value(&col_start)?,
                    to_value(&col_end)?,
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn tabpage_get_windows(&self, tabpage: &TabPage) -> Result<Vec<Window>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("tabpage_get_windows", &[to_value(&tabpage)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn tabpage_get_var(&self, tabpage: &TabPage, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("tabpage_get_var", &[to_value(&tabpage)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn tabpage_get_window(&self, tabpage: &TabPage) -> Result<Window> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("tabpage_get_window", &[to_value(&tabpage)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn tabpage_is_valid(&self, tabpage: &TabPage) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("tabpage_is_valid", &[to_value(&tabpage)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn ui_detach(&self) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("ui_detach", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn ui_try_resize(&self, width: i64, height: i64) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("ui_try_resize", &[to_value(&width)?, to_value(&height)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_feedkeys(&self, keys: &str, mode: &str, escape_ks: bool) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "vim_feedkeys",
                &[to_value(&keys)?, to_value(&mode)?, to_value(&escape_ks)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_input(&self, keys: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_input", &[to_value(&keys)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_replace_termcodes(
        &self,
        str: &str,
        from_part: bool,
        do_lt: bool,
        special: bool,
    ) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "vim_replace_termcodes",
                &[
                    to_value(&str)?,
                    to_value(&from_part)?,
                    to_value(&do_lt)?,
                    to_value(&special)?,
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_strwidth(&self, text: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_strwidth", &[to_value(&text)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_list_runtime_paths(&self) -> Result<Vec<String>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_list_runtime_paths", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_change_directory(&self, dir: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_change_directory", &[to_value(&dir)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_get_current_line(&self) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_get_current_line", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_set_current_line(&self, line: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_set_current_line", &[to_value(&line)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_del_current_line(&self) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_del_current_line", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_get_var(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_get_var", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_get_vvar(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_get_vvar", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_out_write(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_out_write", &[to_value(&str)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_err_write(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_err_write", &[to_value(&str)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_report_error(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_report_error", &[to_value(&str)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_get_buffers(&self) -> Result<Vec<Buffer>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_get_buffers", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_get_current_buffer(&self) -> Result<Buffer> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_get_current_buffer", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_set_current_buffer(&self, buffer: &Buffer) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_set_current_buffer", &[to_value(&buffer)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_get_windows(&self) -> Result<Vec<Window>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_get_windows", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_get_current_window(&self) -> Result<Window> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_get_current_window", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_set_current_window(&self, window: &Window) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_set_current_window", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_get_tabpages(&self) -> Result<Vec<TabPage>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_get_tabpages", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_get_current_tabpage(&self) -> Result<TabPage> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_get_current_tabpage", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_set_current_tabpage(&self, tabpage: &TabPage) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_set_current_tabpage", &[to_value(&tabpage)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_subscribe(&self, event: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_subscribe", &[to_value(&event)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_unsubscribe(&self, event: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_unsubscribe", &[to_value(&event)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_name_to_color(&self, name: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_name_to_color", &[to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_get_color_map(&self) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_get_color_map", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_get_api_info(&self) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_get_api_info", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_command(&self, command: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_command", &[to_value(&command)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_eval(&self, expr: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_eval", &[to_value(&expr)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn vim_call_function(&self, func: &str, args: Vec<Value>) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("vim_call_function", &[to_value(&func)?, to_value(&args)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn window_get_buffer(&self, window: &Window) -> Result<Buffer> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("window_get_buffer", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn window_get_cursor(&self, window: &Window) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("window_get_cursor", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn window_set_cursor(&self, window: &Window, pos: Vec<i64>) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("window_set_cursor", &[to_value(&window)?, to_value(&pos)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn window_get_height(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("window_get_height", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn window_set_height(&self, window: &Window, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request(
                "window_set_height",
                &[to_value(&window)?, to_value(&height)?],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn window_get_width(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("window_get_width", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn window_set_width(&self, window: &Window, width: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("window_set_width", &[to_value(&window)?, to_value(&width)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn window_get_var(&self, window: &Window, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("window_get_var", &[to_value(&window)?, to_value(&name)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn window_get_position(&self, window: &Window) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("window_get_position", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn window_get_tabpage(&self, window: &Window) -> Result<TabPage> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("window_get_tabpage", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
    pub async fn window_is_valid(&self, window: &Window) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .raw_request("window_is_valid", &[to_value(&window)?])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(from_value(&ret)?)
    }
}
