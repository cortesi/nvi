#![allow(clippy::needless_question_mark)]
#![allow(clippy::needless_borrow)]
use crate::error::{Error, Result};
use crate::types::*;
use msgpack_rpc::Value;
use tracing::trace;
pub struct NviClient {
    pub(crate) m_client: msgpack_rpc::Client,
}
impl NviClient {
    pub async fn raw_request(
        &mut self,
        method: &str,
        params: &[msgpack_rpc::Value],
    ) -> Result<msgpack_rpc::Value, msgpack_rpc::Value> {
        trace!("send request: {:?} {:?}", method, params);
        self.m_client.request(method, params).await
    }
    pub async fn raw_notify(
        &mut self,
        method: &str,
        params: &[msgpack_rpc::Value],
    ) -> Result<(), ()> {
        trace!("send notification: {:?} {:?}", method, params);
        self.m_client.notify(method, params).await
    }
    pub async fn nvim_get_autocmds(&self, opts: Value) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_autocmds", &[opts.clone()])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .to_vec())
    }
    pub async fn nvim_create_autocmd(&self, event: Value, opts: Value) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_create_autocmd", &[event.clone(), opts.clone()])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_del_autocmd(&self, id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_del_autocmd", &[Value::Integer(id.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_clear_autocmds(&self, opts: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_clear_autocmds", &[opts.clone()])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_create_augroup(&self, name: &str, opts: Value) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_create_augroup",
                &[Value::String(name.into()), opts.clone()],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_del_augroup_by_id(&self, id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_del_augroup_by_id", &[Value::Integer(id.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_del_augroup_by_name(&self, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_del_augroup_by_name", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_exec_autocmds(&self, event: Value, opts: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_exec_autocmds", &[event.clone(), opts.clone()])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_buf_line_count(&self, buffer: &Buffer) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_line_count",
                &[Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_buf_attach(
        &self,
        buffer: &Buffer,
        send_buffer: bool,
        opts: Value,
    ) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_attach",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Boolean(send_buffer),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_bool().ok_or(Error::Decode {
            msg: "expected boolean".into(),
        })?)
    }
    pub async fn nvim_buf_detach(&self, buffer: &Buffer) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_detach",
                &[Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_bool().ok_or(Error::Decode {
            msg: "expected boolean".into(),
        })?)
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
            .m_client
            .request(
                "nvim_buf_get_lines",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(start.into()),
                    Value::Integer(end.into()),
                    Value::Boolean(strict_indexing),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> {
                Ok(ret
                    .as_str()
                    .ok_or(Error::Decode {
                        msg: "expected string".into(),
                    })?
                    .to_string())
            })
            .collect::<Result<Vec<_>, _>>()?)
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
            .m_client
            .request(
                "nvim_buf_set_lines",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(start.into()),
                    Value::Integer(end.into()),
                    Value::Boolean(strict_indexing),
                    #[allow(clippy::clone_on_copy)]
                    Value::Array(replacement.iter().map(|x| Value::from(x.clone())).collect()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
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
            .m_client
            .request(
                "nvim_buf_set_text",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(start_row.into()),
                    Value::Integer(start_col.into()),
                    Value::Integer(end_row.into()),
                    Value::Integer(end_col.into()),
                    #[allow(clippy::clone_on_copy)]
                    Value::Array(replacement.iter().map(|x| Value::from(x.clone())).collect()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
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
            .m_client
            .request(
                "nvim_buf_get_text",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(start_row.into()),
                    Value::Integer(start_col.into()),
                    Value::Integer(end_row.into()),
                    Value::Integer(end_col.into()),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> {
                Ok(ret
                    .as_str()
                    .ok_or(Error::Decode {
                        msg: "expected string".into(),
                    })?
                    .to_string())
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn nvim_buf_get_offset(&self, buffer: &Buffer, index: i64) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_get_offset",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(index.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_buf_get_var(&self, buffer: &Buffer, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_get_var",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_buf_get_changedtick(&self, buffer: &Buffer) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_get_changedtick",
                &[Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_buf_get_keymap(&self, buffer: &Buffer, mode: &str) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_get_keymap",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(mode.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> { Ok(ret.clone()) })
            .collect::<Result<Vec<_>, _>>()?)
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
            .m_client
            .request(
                "nvim_buf_set_keymap",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(mode.into()),
                    Value::String(lhs.into()),
                    Value::String(rhs.into()),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_buf_del_keymap(&self, buffer: &Buffer, mode: &str, lhs: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_del_keymap",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(mode.into()),
                    Value::String(lhs.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_buf_set_var(&self, buffer: &Buffer, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_set_var",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                    value.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_buf_del_var(&self, buffer: &Buffer, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_del_var",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_buf_get_name(&self, buffer: &Buffer) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_get_name",
                &[Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_str()
            .ok_or(Error::Decode {
                msg: "expected string".into(),
            })?
            .to_string())
    }
    pub async fn nvim_buf_set_name(&self, buffer: &Buffer, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_set_name",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_buf_is_loaded(&self, buffer: &Buffer) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_is_loaded",
                &[Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_bool().ok_or(Error::Decode {
            msg: "expected boolean".into(),
        })?)
    }
    pub async fn nvim_buf_delete(&self, buffer: &Buffer, opts: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_delete",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_buf_is_valid(&self, buffer: &Buffer) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_is_valid",
                &[Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_bool().ok_or(Error::Decode {
            msg: "expected boolean".into(),
        })?)
    }
    pub async fn nvim_buf_del_mark(&self, buffer: &Buffer, name: &str) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_del_mark",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_bool().ok_or(Error::Decode {
            msg: "expected boolean".into(),
        })?)
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
            .m_client
            .request(
                "nvim_buf_set_mark",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                    Value::Integer(line.into()),
                    Value::Integer(col.into()),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_bool().ok_or(Error::Decode {
            msg: "expected boolean".into(),
        })?)
    }
    pub async fn nvim_buf_get_mark(&self, buffer: &Buffer, name: &str) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_get_mark",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> {
                Ok(ret.as_i64().ok_or(Error::Decode {
                    msg: "expected integer".into(),
                })?)
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn nvim_parse_cmd(&self, str: &str, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_parse_cmd", &[Value::String(str.into()), opts.clone()])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_cmd(&self, cmd: Value, opts: Value) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_cmd", &[cmd.clone(), opts.clone()])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_str()
            .ok_or(Error::Decode {
                msg: "expected string".into(),
            })?
            .to_string())
    }
    pub async fn nvim_create_user_command(
        &self,
        name: &str,
        command: Value,
        opts: Value,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_create_user_command",
                &[Value::String(name.into()), command.clone(), opts.clone()],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_del_user_command(&self, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_del_user_command", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
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
            .m_client
            .request(
                "nvim_buf_create_user_command",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                    command.clone(),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_buf_del_user_command(&self, buffer: &Buffer, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_del_user_command",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_get_commands(&self, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_commands", &[opts.clone()])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_buf_get_commands(&self, buffer: &Buffer, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_get_commands",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_exec(&self, src: &str, output: bool) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_exec",
                &[Value::String(src.into()), Value::Boolean(output)],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_str()
            .ok_or(Error::Decode {
                msg: "expected string".into(),
            })?
            .to_string())
    }
    pub async fn nvim_command_output(&self, command: &str) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_command_output", &[Value::String(command.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_str()
            .ok_or(Error::Decode {
                msg: "expected string".into(),
            })?
            .to_string())
    }
    pub async fn nvim_execute_lua(&self, code: &str, args: Vec<Value>) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_execute_lua",
                &[Value::String(code.into()), Value::Array(args.to_vec())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_buf_get_number(&self, buffer: &Buffer) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_get_number",
                &[Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
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
            .m_client
            .request(
                "nvim_buf_clear_highlight",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(ns_id.into()),
                    Value::Integer(line_start.into()),
                    Value::Integer(line_end.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
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
            .m_client
            .request(
                "nvim_buf_set_virtual_text",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(src_id.into()),
                    Value::Integer(line.into()),
                    Value::Array(chunks.to_vec()),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_get_hl_by_id(&self, hl_id: i64, rgb: bool) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_get_hl_by_id",
                &[Value::Integer(hl_id.into()), Value::Boolean(rgb)],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_get_hl_by_name(&self, name: &str, rgb: bool) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_get_hl_by_name",
                &[Value::String(name.into()), Value::Boolean(rgb)],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn buffer_insert(
        &self,
        buffer: &Buffer,
        lnum: i64,
        lines: Vec<String>,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "buffer_insert",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(lnum.into()),
                    #[allow(clippy::clone_on_copy)]
                    Value::Array(lines.iter().map(|x| Value::from(x.clone())).collect()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn buffer_get_line(&self, buffer: &Buffer, index: i64) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "buffer_get_line",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(index.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_str()
            .ok_or(Error::Decode {
                msg: "expected string".into(),
            })?
            .to_string())
    }
    pub async fn buffer_set_line(&self, buffer: &Buffer, index: i64, line: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "buffer_set_line",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(index.into()),
                    Value::String(line.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn buffer_del_line(&self, buffer: &Buffer, index: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "buffer_del_line",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(index.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
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
            .m_client
            .request(
                "buffer_get_line_slice",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(start.into()),
                    Value::Integer(end.into()),
                    Value::Boolean(include_start),
                    Value::Boolean(include_end),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> {
                Ok(ret
                    .as_str()
                    .ok_or(Error::Decode {
                        msg: "expected string".into(),
                    })?
                    .to_string())
            })
            .collect::<Result<Vec<_>, _>>()?)
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
            .m_client
            .request(
                "buffer_set_line_slice",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(start.into()),
                    Value::Integer(end.into()),
                    Value::Boolean(include_start),
                    Value::Boolean(include_end),
                    #[allow(clippy::clone_on_copy)]
                    Value::Array(replacement.iter().map(|x| Value::from(x.clone())).collect()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn buffer_set_var(&self, buffer: &Buffer, name: &str, value: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "buffer_set_var",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                    value.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn buffer_del_var(&self, buffer: &Buffer, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "buffer_del_var",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn window_set_var(&self, window: &Window, name: &str, value: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "window_set_var",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::String(name.into()),
                    value.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn window_del_var(&self, window: &Window, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "window_del_var",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn tabpage_set_var(
        &self,
        tabpage: &TabPage,
        name: &str,
        value: Value,
    ) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "tabpage_set_var",
                &[
                    Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone()),
                    Value::String(name.into()),
                    value.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn tabpage_del_var(&self, tabpage: &TabPage, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "tabpage_del_var",
                &[
                    Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn vim_set_var(&self, name: &str, value: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_set_var", &[Value::String(name.into()), value.clone()])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn vim_del_var(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_del_var", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_get_option_info(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_option_info", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_set_option(&self, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_set_option",
                &[Value::String(name.into()), value.clone()],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_get_option(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_option", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_buf_get_option(&self, buffer: &Buffer, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_get_option",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_buf_set_option(
        &self,
        buffer: &Buffer,
        name: &str,
        value: Value,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_set_option",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                    value.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_win_get_option(&self, window: &Window, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_get_option",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_win_set_option(
        &self,
        window: &Window,
        name: &str,
        value: Value,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_set_option",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::String(name.into()),
                    value.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_call_atomic(&self, calls: Vec<Value>) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_call_atomic", &[Value::Array(calls.to_vec())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .to_vec())
    }
    pub async fn nvim_create_namespace(&self, name: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_create_namespace", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_get_namespaces(&self) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_namespaces", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
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
            .m_client
            .request(
                "nvim_buf_get_extmark_by_id",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(ns_id.into()),
                    Value::Integer(id.into()),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> {
                Ok(ret.as_i64().ok_or(Error::Decode {
                    msg: "expected integer".into(),
                })?)
            })
            .collect::<Result<Vec<_>, _>>()?)
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
            .m_client
            .request(
                "nvim_buf_get_extmarks",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(ns_id.into()),
                    start.clone(),
                    end.clone(),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .to_vec())
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
            .m_client
            .request(
                "nvim_buf_set_extmark",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(ns_id.into()),
                    Value::Integer(line.into()),
                    Value::Integer(col.into()),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_buf_del_extmark(&self, buffer: &Buffer, ns_id: i64, id: i64) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_buf_del_extmark",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(ns_id.into()),
                    Value::Integer(id.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_bool().ok_or(Error::Decode {
            msg: "expected boolean".into(),
        })?)
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
            .m_client
            .request(
                "nvim_buf_add_highlight",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(ns_id.into()),
                    Value::String(hl_group.into()),
                    Value::Integer(line.into()),
                    Value::Integer(col_start.into()),
                    Value::Integer(col_end.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
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
            .m_client
            .request(
                "nvim_buf_clear_namespace",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(ns_id.into()),
                    Value::Integer(line_start.into()),
                    Value::Integer(line_end.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_set_decoration_provider(&self, ns_id: i64, opts: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_set_decoration_provider",
                &[Value::Integer(ns_id.into()), opts.clone()],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_get_option_value(&self, name: &str, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_get_option_value",
                &[Value::String(name.into()), opts.clone()],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_set_option_value(&self, name: &str, value: Value, opts: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_set_option_value",
                &[Value::String(name.into()), value.clone(), opts.clone()],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_get_all_options_info(&self) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_all_options_info", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_get_option_info2(&self, name: &str, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_get_option_info2",
                &[Value::String(name.into()), opts.clone()],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_tabpage_list_wins(&self, tabpage: &TabPage) -> Result<Vec<Window>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_tabpage_list_wins",
                &[Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> { Ok(Window::from_value(&ret)?) })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn nvim_tabpage_get_var(&self, tabpage: &TabPage, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_tabpage_get_var",
                &[
                    Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_tabpage_set_var(
        &self,
        tabpage: &TabPage,
        name: &str,
        value: Value,
    ) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_tabpage_set_var",
                &[
                    Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone()),
                    Value::String(name.into()),
                    value.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_tabpage_del_var(&self, tabpage: &TabPage, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_tabpage_del_var",
                &[
                    Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_tabpage_get_win(&self, tabpage: &TabPage) -> Result<Window> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_tabpage_get_win",
                &[Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(Window::from_value(&ret)?)
    }
    pub async fn nvim_tabpage_set_win(&self, tabpage: &TabPage, win: &Window) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_tabpage_set_win",
                &[
                    Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone()),
                    Value::Ext(WINDOW_EXT_TYPE, win.data.clone()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_tabpage_get_number(&self, tabpage: &TabPage) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_tabpage_get_number",
                &[Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_tabpage_is_valid(&self, tabpage: &TabPage) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_tabpage_is_valid",
                &[Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_bool().ok_or(Error::Decode {
            msg: "expected boolean".into(),
        })?)
    }
    pub async fn nvim_ui_attach(&self, width: i64, height: i64, options: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_ui_attach",
                &[
                    Value::Integer(width.into()),
                    Value::Integer(height.into()),
                    options.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn ui_attach(&self, width: i64, height: i64, enable_rgb: bool) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "ui_attach",
                &[
                    Value::Integer(width.into()),
                    Value::Integer(height.into()),
                    Value::Boolean(enable_rgb),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_ui_set_focus(&self, gained: bool) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_ui_set_focus", &[Value::Boolean(gained)])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_ui_detach(&self) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_ui_detach", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_ui_try_resize(&self, width: i64, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_ui_try_resize",
                &[Value::Integer(width.into()), Value::Integer(height.into())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_ui_set_option(&self, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_ui_set_option",
                &[Value::String(name.into()), value.clone()],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_ui_try_resize_grid(&self, grid: i64, width: i64, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_ui_try_resize_grid",
                &[
                    Value::Integer(grid.into()),
                    Value::Integer(width.into()),
                    Value::Integer(height.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_ui_pum_set_height(&self, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_ui_pum_set_height", &[Value::Integer(height.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
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
            .m_client
            .request(
                "nvim_ui_pum_set_bounds",
                &[
                    Value::F64(width),
                    Value::F64(height),
                    Value::F64(row),
                    Value::F64(col),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_ui_term_event(&self, event: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_ui_term_event",
                &[Value::String(event.into()), value.clone()],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_get_hl_id_by_name(&self, name: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_hl_id_by_name", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_get_hl(&self, ns_id: i64, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_hl", &[Value::Integer(ns_id.into()), opts.clone()])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_set_hl(&self, ns_id: i64, name: &str, val: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_set_hl",
                &[
                    Value::Integer(ns_id.into()),
                    Value::String(name.into()),
                    val.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_get_hl_ns(&self, opts: Value) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_hl_ns", &[opts.clone()])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_set_hl_ns(&self, ns_id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_set_hl_ns", &[Value::Integer(ns_id.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_set_hl_ns_fast(&self, ns_id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_set_hl_ns_fast", &[Value::Integer(ns_id.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_feedkeys(&self, keys: &str, mode: &str, escape_ks: bool) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_feedkeys",
                &[
                    Value::String(keys.into()),
                    Value::String(mode.into()),
                    Value::Boolean(escape_ks),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_input(&self, keys: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_input", &[Value::String(keys.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
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
            .m_client
            .request(
                "nvim_input_mouse",
                &[
                    Value::String(button.into()),
                    Value::String(action.into()),
                    Value::String(modifier.into()),
                    Value::Integer(grid.into()),
                    Value::Integer(row.into()),
                    Value::Integer(col.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
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
            .m_client
            .request(
                "nvim_replace_termcodes",
                &[
                    Value::String(str.into()),
                    Value::Boolean(from_part),
                    Value::Boolean(do_lt),
                    Value::Boolean(special),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_str()
            .ok_or(Error::Decode {
                msg: "expected string".into(),
            })?
            .to_string())
    }
    pub async fn nvim_exec_lua(&self, code: &str, args: Vec<Value>) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_exec_lua",
                &[Value::String(code.into()), Value::Array(args.to_vec())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_notify(&self, msg: &str, log_level: i64, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_notify",
                &[
                    Value::String(msg.into()),
                    Value::Integer(log_level.into()),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_strwidth(&self, text: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_strwidth", &[Value::String(text.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_list_runtime_paths(&self) -> Result<Vec<String>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_list_runtime_paths", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> {
                Ok(ret
                    .as_str()
                    .ok_or(Error::Decode {
                        msg: "expected string".into(),
                    })?
                    .to_string())
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn nvim_get_runtime_file(&self, name: &str, all: bool) -> Result<Vec<String>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_get_runtime_file",
                &[Value::String(name.into()), Value::Boolean(all)],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> {
                Ok(ret
                    .as_str()
                    .ok_or(Error::Decode {
                        msg: "expected string".into(),
                    })?
                    .to_string())
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn nvim_set_current_dir(&self, dir: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_set_current_dir", &[Value::String(dir.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_get_current_line(&self) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_current_line", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_str()
            .ok_or(Error::Decode {
                msg: "expected string".into(),
            })?
            .to_string())
    }
    pub async fn nvim_set_current_line(&self, line: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_set_current_line", &[Value::String(line.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_del_current_line(&self) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_del_current_line", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_get_var(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_var", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_set_var(&self, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_set_var", &[Value::String(name.into()), value.clone()])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_del_var(&self, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_del_var", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_get_vvar(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_vvar", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_set_vvar(&self, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_set_vvar",
                &[Value::String(name.into()), value.clone()],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_echo(&self, chunks: Vec<Value>, history: bool, opts: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_echo",
                &[
                    Value::Array(chunks.to_vec()),
                    Value::Boolean(history),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_out_write(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_out_write", &[Value::String(str.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_err_write(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_err_write", &[Value::String(str.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_err_writeln(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_err_writeln", &[Value::String(str.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_list_bufs(&self) -> Result<Vec<Buffer>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_list_bufs", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> { Ok(Buffer::from_value(&ret)?) })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn nvim_get_current_buf(&self) -> Result<Buffer> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_current_buf", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(Buffer::from_value(&ret)?)
    }
    pub async fn nvim_set_current_buf(&self, buffer: &Buffer) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_set_current_buf",
                &[Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_list_wins(&self) -> Result<Vec<Window>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_list_wins", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> { Ok(Window::from_value(&ret)?) })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn nvim_get_current_win(&self) -> Result<Window> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_current_win", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(Window::from_value(&ret)?)
    }
    pub async fn nvim_set_current_win(&self, window: &Window) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_set_current_win",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_create_buf(&self, listed: bool, scratch: bool) -> Result<Buffer> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_create_buf",
                &[Value::Boolean(listed), Value::Boolean(scratch)],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(Buffer::from_value(&ret)?)
    }
    pub async fn nvim_open_term(&self, buffer: &Buffer, opts: Value) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_open_term",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_chan_send(&self, chan: i64, data: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_chan_send",
                &[Value::Integer(chan.into()), Value::String(data.into())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_list_tabpages(&self) -> Result<Vec<TabPage>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_list_tabpages", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> { Ok(TabPage::from_value(&ret)?) })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn nvim_get_current_tabpage(&self) -> Result<TabPage> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_current_tabpage", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(TabPage::from_value(&ret)?)
    }
    pub async fn nvim_set_current_tabpage(&self, tabpage: &TabPage) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_set_current_tabpage",
                &[Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_paste(&self, data: &str, crlf: bool, phase: i64) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_paste",
                &[
                    Value::String(data.into()),
                    Value::Boolean(crlf),
                    Value::Integer(phase.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_bool().ok_or(Error::Decode {
            msg: "expected boolean".into(),
        })?)
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
            .m_client
            .request(
                "nvim_put",
                &[
                    #[allow(clippy::clone_on_copy)]
                    Value::Array(lines.iter().map(|x| Value::from(x.clone())).collect()),
                    Value::String(typ.into()),
                    Value::Boolean(after),
                    Value::Boolean(follow),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_subscribe(&self, event: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_subscribe", &[Value::String(event.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_unsubscribe(&self, event: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_unsubscribe", &[Value::String(event.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_get_color_by_name(&self, name: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_color_by_name", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_get_color_map(&self) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_color_map", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_get_context(&self, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_context", &[opts.clone()])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_load_context(&self, dict: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_load_context", &[dict.clone()])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_get_mode(&self) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_mode", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_get_keymap(&self, mode: &str) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_keymap", &[Value::String(mode.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> { Ok(ret.clone()) })
            .collect::<Result<Vec<_>, _>>()?)
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
            .m_client
            .request(
                "nvim_set_keymap",
                &[
                    Value::String(mode.into()),
                    Value::String(lhs.into()),
                    Value::String(rhs.into()),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_del_keymap(&self, mode: &str, lhs: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_del_keymap",
                &[Value::String(mode.into()), Value::String(lhs.into())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_get_api_info(&self) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_api_info", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .to_vec())
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
            .m_client
            .request(
                "nvim_set_client_info",
                &[
                    Value::String(name.into()),
                    version.clone(),
                    Value::String(typ.into()),
                    methods.clone(),
                    attributes.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_get_chan_info(&self, chan: i64) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_chan_info", &[Value::Integer(chan.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_list_chans(&self) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_list_chans", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .to_vec())
    }
    pub async fn nvim_list_uis(&self) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_list_uis", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .to_vec())
    }
    pub async fn nvim_get_proc_children(&self, pid: i64) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_proc_children", &[Value::Integer(pid.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .to_vec())
    }
    pub async fn nvim_get_proc(&self, pid: i64) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_proc", &[Value::Integer(pid.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
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
            .m_client
            .request(
                "nvim_select_popupmenu_item",
                &[
                    Value::Integer(item.into()),
                    Value::Boolean(insert),
                    Value::Boolean(finish),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_del_mark(&self, name: &str) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_del_mark", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_bool().ok_or(Error::Decode {
            msg: "expected boolean".into(),
        })?)
    }
    pub async fn nvim_get_mark(&self, name: &str, opts: Value) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_get_mark", &[Value::String(name.into()), opts.clone()])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .to_vec())
    }
    pub async fn nvim_eval_statusline(&self, str: &str, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_eval_statusline",
                &[Value::String(str.into()), opts.clone()],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_exec2(&self, src: &str, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_exec2", &[Value::String(src.into()), opts.clone()])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_command(&self, command: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_command", &[Value::String(command.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_eval(&self, expr: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("nvim_eval", &[Value::String(expr.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_call_function(&self, func: &str, args: Vec<Value>) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_call_function",
                &[Value::String(func.into()), Value::Array(args.to_vec())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_call_dict_function(
        &self,
        dict: Value,
        func: &str,
        args: Vec<Value>,
    ) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_call_dict_function",
                &[
                    dict.clone(),
                    Value::String(func.into()),
                    Value::Array(args.to_vec()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_parse_expression(
        &self,
        expr: &str,
        flags: &str,
        highlight: bool,
    ) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_parse_expression",
                &[
                    Value::String(expr.into()),
                    Value::String(flags.into()),
                    Value::Boolean(highlight),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_open_win(
        &self,
        buffer: &Buffer,
        enter: bool,
        config: Value,
    ) -> Result<Window> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_open_win",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Boolean(enter),
                    config.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(Window::from_value(&ret)?)
    }
    pub async fn nvim_win_set_config(&self, window: &Window, config: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_set_config",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    config.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_win_get_config(&self, window: &Window) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_get_config",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_win_get_buf(&self, window: &Window) -> Result<Buffer> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_get_buf",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(Buffer::from_value(&ret)?)
    }
    pub async fn nvim_win_set_buf(&self, window: &Window, buffer: &Buffer) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_set_buf",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_win_get_cursor(&self, window: &Window) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_get_cursor",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> {
                Ok(ret.as_i64().ok_or(Error::Decode {
                    msg: "expected integer".into(),
                })?)
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn nvim_win_set_cursor(&self, window: &Window, pos: Vec<i64>) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_set_cursor",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    #[allow(clippy::clone_on_copy)]
                    Value::Array(pos.iter().map(|x| Value::from(x.clone())).collect()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_win_get_height(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_get_height",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_win_set_height(&self, window: &Window, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_set_height",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::Integer(height.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_win_get_width(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_get_width",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_win_set_width(&self, window: &Window, width: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_set_width",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::Integer(width.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_win_get_var(&self, window: &Window, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_get_var",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn nvim_win_set_var(&self, window: &Window, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_set_var",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::String(name.into()),
                    value.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_win_del_var(&self, window: &Window, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_del_var",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_win_get_position(&self, window: &Window) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_get_position",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> {
                Ok(ret.as_i64().ok_or(Error::Decode {
                    msg: "expected integer".into(),
                })?)
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn nvim_win_get_tabpage(&self, window: &Window) -> Result<TabPage> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_get_tabpage",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(TabPage::from_value(&ret)?)
    }
    pub async fn nvim_win_get_number(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_get_number",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn nvim_win_is_valid(&self, window: &Window) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_is_valid",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_bool().ok_or(Error::Decode {
            msg: "expected boolean".into(),
        })?)
    }
    pub async fn nvim_win_hide(&self, window: &Window) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_hide",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_win_close(&self, window: &Window, force: bool) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_close",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::Boolean(force),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_win_set_hl_ns(&self, window: &Window, ns_id: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_set_hl_ns",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::Integer(ns_id.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn nvim_win_text_height(&self, window: &Window, opts: Value) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "nvim_win_text_height",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    opts.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn buffer_line_count(&self, buffer: &Buffer) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "buffer_line_count",
                &[Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
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
            .m_client
            .request(
                "buffer_get_lines",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(start.into()),
                    Value::Integer(end.into()),
                    Value::Boolean(strict_indexing),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> {
                Ok(ret
                    .as_str()
                    .ok_or(Error::Decode {
                        msg: "expected string".into(),
                    })?
                    .to_string())
            })
            .collect::<Result<Vec<_>, _>>()?)
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
            .m_client
            .request(
                "buffer_set_lines",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(start.into()),
                    Value::Integer(end.into()),
                    Value::Boolean(strict_indexing),
                    #[allow(clippy::clone_on_copy)]
                    Value::Array(replacement.iter().map(|x| Value::from(x.clone())).collect()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn buffer_get_var(&self, buffer: &Buffer, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "buffer_get_var",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn buffer_get_name(&self, buffer: &Buffer) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "buffer_get_name",
                &[Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_str()
            .ok_or(Error::Decode {
                msg: "expected string".into(),
            })?
            .to_string())
    }
    pub async fn buffer_set_name(&self, buffer: &Buffer, name: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "buffer_set_name",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn buffer_is_valid(&self, buffer: &Buffer) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "buffer_is_valid",
                &[Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_bool().ok_or(Error::Decode {
            msg: "expected boolean".into(),
        })?)
    }
    pub async fn buffer_get_mark(&self, buffer: &Buffer, name: &str) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "buffer_get_mark",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> {
                Ok(ret.as_i64().ok_or(Error::Decode {
                    msg: "expected integer".into(),
                })?)
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn vim_command_output(&self, command: &str) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_command_output", &[Value::String(command.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_str()
            .ok_or(Error::Decode {
                msg: "expected string".into(),
            })?
            .to_string())
    }
    pub async fn buffer_get_number(&self, buffer: &Buffer) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "buffer_get_number",
                &[Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
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
            .m_client
            .request(
                "buffer_clear_highlight",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(ns_id.into()),
                    Value::Integer(line_start.into()),
                    Value::Integer(line_end.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn vim_set_option(&self, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "vim_set_option",
                &[Value::String(name.into()), value.clone()],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn vim_get_option(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_get_option", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn buffer_get_option(&self, buffer: &Buffer, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "buffer_get_option",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn buffer_set_option(&self, buffer: &Buffer, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "buffer_set_option",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::String(name.into()),
                    value.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn window_get_option(&self, window: &Window, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "window_get_option",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn window_set_option(&self, window: &Window, name: &str, value: Value) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "window_set_option",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::String(name.into()),
                    value.clone(),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
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
            .m_client
            .request(
                "buffer_add_highlight",
                &[
                    Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone()),
                    Value::Integer(ns_id.into()),
                    Value::String(hl_group.into()),
                    Value::Integer(line.into()),
                    Value::Integer(col_start.into()),
                    Value::Integer(col_end.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn tabpage_get_windows(&self, tabpage: &TabPage) -> Result<Vec<Window>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "tabpage_get_windows",
                &[Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> { Ok(Window::from_value(&ret)?) })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn tabpage_get_var(&self, tabpage: &TabPage, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "tabpage_get_var",
                &[
                    Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn tabpage_get_window(&self, tabpage: &TabPage) -> Result<Window> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "tabpage_get_window",
                &[Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(Window::from_value(&ret)?)
    }
    pub async fn tabpage_is_valid(&self, tabpage: &TabPage) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "tabpage_is_valid",
                &[Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_bool().ok_or(Error::Decode {
            msg: "expected boolean".into(),
        })?)
    }
    pub async fn ui_detach(&self) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("ui_detach", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn ui_try_resize(&self, width: i64, height: i64) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "ui_try_resize",
                &[Value::Integer(width.into()), Value::Integer(height.into())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn vim_feedkeys(&self, keys: &str, mode: &str, escape_ks: bool) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "vim_feedkeys",
                &[
                    Value::String(keys.into()),
                    Value::String(mode.into()),
                    Value::Boolean(escape_ks),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn vim_input(&self, keys: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_input", &[Value::String(keys.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
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
            .m_client
            .request(
                "vim_replace_termcodes",
                &[
                    Value::String(str.into()),
                    Value::Boolean(from_part),
                    Value::Boolean(do_lt),
                    Value::Boolean(special),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_str()
            .ok_or(Error::Decode {
                msg: "expected string".into(),
            })?
            .to_string())
    }
    pub async fn vim_strwidth(&self, text: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_strwidth", &[Value::String(text.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn vim_list_runtime_paths(&self) -> Result<Vec<String>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_list_runtime_paths", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> {
                Ok(ret
                    .as_str()
                    .ok_or(Error::Decode {
                        msg: "expected string".into(),
                    })?
                    .to_string())
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn vim_change_directory(&self, dir: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_change_directory", &[Value::String(dir.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn vim_get_current_line(&self) -> Result<String> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_get_current_line", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_str()
            .ok_or(Error::Decode {
                msg: "expected string".into(),
            })?
            .to_string())
    }
    pub async fn vim_set_current_line(&self, line: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_set_current_line", &[Value::String(line.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn vim_del_current_line(&self) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_del_current_line", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn vim_get_var(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_get_var", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn vim_get_vvar(&self, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_get_vvar", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn vim_out_write(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_out_write", &[Value::String(str.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn vim_err_write(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_err_write", &[Value::String(str.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn vim_report_error(&self, str: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_report_error", &[Value::String(str.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn vim_get_buffers(&self) -> Result<Vec<Buffer>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_get_buffers", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> { Ok(Buffer::from_value(&ret)?) })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn vim_get_current_buffer(&self) -> Result<Buffer> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_get_current_buffer", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(Buffer::from_value(&ret)?)
    }
    pub async fn vim_set_current_buffer(&self, buffer: &Buffer) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "vim_set_current_buffer",
                &[Value::Ext(BUFFER_EXT_TYPE, buffer.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn vim_get_windows(&self) -> Result<Vec<Window>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_get_windows", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> { Ok(Window::from_value(&ret)?) })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn vim_get_current_window(&self) -> Result<Window> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_get_current_window", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(Window::from_value(&ret)?)
    }
    pub async fn vim_set_current_window(&self, window: &Window) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "vim_set_current_window",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn vim_get_tabpages(&self) -> Result<Vec<TabPage>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_get_tabpages", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> { Ok(TabPage::from_value(&ret)?) })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn vim_get_current_tabpage(&self) -> Result<TabPage> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_get_current_tabpage", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(TabPage::from_value(&ret)?)
    }
    pub async fn vim_set_current_tabpage(&self, tabpage: &TabPage) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "vim_set_current_tabpage",
                &[Value::Ext(TABPAGE_EXT_TYPE, tabpage.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn vim_subscribe(&self, event: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_subscribe", &[Value::String(event.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn vim_unsubscribe(&self, event: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_unsubscribe", &[Value::String(event.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn vim_name_to_color(&self, name: &str) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_name_to_color", &[Value::String(name.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn vim_get_color_map(&self) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_get_color_map", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn vim_get_api_info(&self) -> Result<Vec<Value>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_get_api_info", &[])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .to_vec())
    }
    pub async fn vim_command(&self, command: &str) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_command", &[Value::String(command.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn vim_eval(&self, expr: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request("vim_eval", &[Value::String(expr.into())])
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn vim_call_function(&self, func: &str, args: Vec<Value>) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "vim_call_function",
                &[Value::String(func.into()), Value::Array(args.to_vec())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn window_get_buffer(&self, window: &Window) -> Result<Buffer> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "window_get_buffer",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(Buffer::from_value(&ret)?)
    }
    pub async fn window_get_cursor(&self, window: &Window) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "window_get_cursor",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> {
                Ok(ret.as_i64().ok_or(Error::Decode {
                    msg: "expected integer".into(),
                })?)
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn window_set_cursor(&self, window: &Window, pos: Vec<i64>) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "window_set_cursor",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    #[allow(clippy::clone_on_copy)]
                    Value::Array(pos.iter().map(|x| Value::from(x.clone())).collect()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn window_get_height(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "window_get_height",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn window_set_height(&self, window: &Window, height: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "window_set_height",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::Integer(height.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn window_get_width(&self, window: &Window) -> Result<i64> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "window_get_width",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_i64().ok_or(Error::Decode {
            msg: "expected integer".into(),
        })?)
    }
    pub async fn window_set_width(&self, window: &Window, width: i64) -> Result<()> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "window_set_width",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::Integer(width.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(())
    }
    pub async fn window_get_var(&self, window: &Window, name: &str) -> Result<Value> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "window_get_var",
                &[
                    Value::Ext(WINDOW_EXT_TYPE, window.data.clone()),
                    Value::String(name.into()),
                ],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.clone())
    }
    pub async fn window_get_position(&self, window: &Window) -> Result<Vec<i64>> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "window_get_position",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret
            .as_array()
            .ok_or(Error::Decode {
                msg: "expected array".into(),
            })?
            .iter()
            .map(|ret| -> Result<_> {
                Ok(ret.as_i64().ok_or(Error::Decode {
                    msg: "expected integer".into(),
                })?)
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn window_get_tabpage(&self, window: &Window) -> Result<TabPage> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "window_get_tabpage",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(TabPage::from_value(&ret)?)
    }
    pub async fn window_is_valid(&self, window: &Window) -> Result<bool> {
        #[allow(unused_variables)]
        let ret = self
            .m_client
            .request(
                "window_is_valid",
                &[Value::Ext(WINDOW_EXT_TYPE, window.data.clone())],
            )
            .await
            .map_err(Error::RemoteError)?;
        #[allow(clippy::needless_question_mark)]
        Ok(ret.as_bool().ok_or(Error::Decode {
            msg: "expected boolean".into(),
        })?)
    }
}
