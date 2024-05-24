use std::io::{Read, Write};

use crate::error::Error;

const REQUEST_MESSAGE: u8 = 0;
const RESPONSE_MESSAGE: u8 = 1;
const NOTIFICATION_MESSAGE: u8 = 2;

fn read_string<R>(r: &mut R) -> Result<String, Error>
where
    R: Read,
{
    let len = rmp::decode::read_str_len(r)?;
    let mut buf = vec![0; len as usize];
    r.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|_| Error::Decode {
        msg: "invalid utf8 string".to_string(),
    })
}

/// A `msgpack-rpc` request
#[derive(PartialEq, Clone, Debug)]
pub struct Request {
    pub id: u64,
    pub method: String,
    pub params: Vec<rmpv::Value>,
}

/// A `msgpack-rpc` response
#[derive(PartialEq, Clone, Debug)]
pub struct Response {
    pub id: u64,
    pub result: Result<rmpv::Value, rmpv::Value>,
}

/// A `msgpack-rpc` notification
#[derive(PartialEq, Clone, Debug)]
pub struct Notification {
    pub method: String,
    pub params: Vec<rmpv::Value>,
}

/// A `msgpack-rpc` message
#[derive(PartialEq, Clone, Debug)]
pub enum Message {
    Request(Request),
    Response(Response),
    Notification(Notification),
}

impl Message {
    pub fn decode<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let len = rmp::decode::read_array_len(r)?;
        match len {
            4 => {
                let msg_type = rmp::decode::read_u8(r)?;
                match msg_type {
                    REQUEST_MESSAGE => {
                        let id = rmp::decode::read_u64(r)?;
                        let method = read_string(r)?;
                        let params_len = rmp::decode::read_array_len(r)?;
                        let mut params = Vec::with_capacity(params_len as usize);
                        for _ in 0..params_len {
                            params.push(rmpv::decode::read_value(r)?);
                        }
                        Ok(Message::Request(Request { id, method, params }))
                    }
                    RESPONSE_MESSAGE => {
                        let id = rmp::decode::read_u64(r)?;
                        let err = rmpv::decode::read_value(r)?;
                        let result = rmpv::decode::read_value(r)?;
                        Ok(Message::Response(if err.is_nil() {
                            Response {
                                id,
                                result: Ok(result),
                            }
                        } else {
                            Response {
                                id,
                                result: Err(err),
                            }
                        }))
                    }

                    _ => Err(Error::Decode {
                        msg: "invalid message type".to_string(),
                    }),
                }
            }
            3 => {
                let msg_type = rmp::decode::read_u8(r)?;
                if msg_type != NOTIFICATION_MESSAGE {
                    return Err(Error::Decode {
                        msg: "invalid message type".to_string(),
                    });
                }
                let method = read_string(r)?;
                let params_len = rmp::decode::read_array_len(r)?;
                let mut params = Vec::with_capacity(params_len as usize);
                for _ in 0..params_len {
                    params.push(rmpv::decode::read_value(r)?);
                }
                Ok(Message::Notification(Notification { method, params }))
            }
            _ => Err(Error::Decode {
                msg: "invalid message length".to_string(),
            }),
        }
    }

    pub fn encode<W>(&self, w: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        match self {
            Message::Request(req) => {
                rmp::encode::write_array_len(w, 4)?;
                rmp::encode::write_u8(w, REQUEST_MESSAGE)?;
                rmp::encode::write_u64(w, req.id)?;
                rmp::encode::write_str(w, &req.method)?;
                rmp::encode::write_array_len(w, req.params.len() as u32)?;
                for param in &req.params {
                    rmpv::encode::write_value(w, param)?;
                }
            }
            Message::Response(res) => {
                rmp::encode::write_array_len(w, 4)?;
                rmp::encode::write_u8(w, RESPONSE_MESSAGE)?;
                rmp::encode::write_u64(w, res.id)?;
                match &res.result {
                    Ok(val) => {
                        rmp::encode::write_nil(w)?;
                        rmpv::encode::write_value(w, val)?;
                    }
                    Err(err) => {
                        rmpv::encode::write_value(w, err)?;
                        rmp::encode::write_nil(w)?;
                    }
                }
            }
            Message::Notification(not) => {
                rmp::encode::write_array_len(w, 3)?;
                rmp::encode::write_u8(w, NOTIFICATION_MESSAGE)?;
                rmp::encode::write_str(w, &not.method)?;
                rmp::encode::write_array_len(w, not.params.len() as u32)?;
                for param in &not.params {
                    rmpv::encode::write_value(w, param)?;
                }
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_request() {
        fn t_idemp(msg: Message) {
            let mut buf = Vec::new();
            msg.encode(&mut buf).unwrap();
            let decoded = Message::decode(&mut buf.as_slice()).unwrap();
            assert_eq!(msg, decoded);
        }

        t_idemp(Message::Request(Request {
            id: 1,
            method: "test".to_string(),
            params: vec![rmpv::Value::from(1), rmpv::Value::from("test")],
        }));
        t_idemp(Message::Response(Response {
            id: 1,
            result: Ok(rmpv::Value::from(vec![
                rmpv::Value::from(1),
                rmpv::Value::from("test"),
            ])),
        }));
        t_idemp(Message::Notification(Notification {
            method: "test".to_string(),
            params: vec![rmpv::Value::from(1), rmpv::Value::from("test")],
        }));
    }
}
