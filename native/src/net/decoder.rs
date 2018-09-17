use godcoin::tx::TxVariant;
use godcoin::net::rpc::*;
use tokio_codec::Decoder;
use crypto::JsPublicKey;
use std::cell::RefCell;
use std::error::Error;
use neon::prelude::*;
use bytes::BytesMut;

use super::constants::*;
use asset::JsAsset;

pub struct BufRpcCodec {
    inner: codec::RpcCodec,
    buf: Option<RefCell<BytesMut>>
}

declare_types! {
    pub class JsRpcCodec for BufRpcCodec {
        init(_) {
            Ok(BufRpcCodec {
                inner: codec::RpcCodec::new(),
                buf: Some(RefCell::new(BytesMut::with_capacity(4096)))
            })
        }

        method update(mut cx) {
            {
                let node_buf = cx.argument::<JsBuffer>(0)?;
                let mut this = cx.this();
                let guard = cx.lock();
                let mut this = this.borrow_mut(&guard);

                let node_buf = node_buf.borrow(&guard);
                let buf = this.buf.take().unwrap();
                buf.borrow_mut().extend_from_slice(node_buf.as_slice());
                this.buf = Some(buf);
            }
            Ok(cx.undefined().upcast())
        }

        method decode(mut cx) {
            let payload = {
                let mut this = cx.this();
                let guard = cx.lock();
                let mut this = this.borrow_mut(&guard);

                let buf = this.buf.take().unwrap();
                let payload = this.inner.decode(&mut *buf.borrow_mut());
                this.buf = Some(buf);
                payload
            };
            match payload {
                Ok(payload) => {
                    if let Some(payload) = payload {
                        let id = cx.number(payload.id);
                        let msg = if let Some(msg) = payload.msg {
                            match msg {
                                RpcMsg::Handshake(hs) => {
                                    let msg_type = cx.number(MsgType::HANDSHAKE as u8);
                                    let peer_type = cx.number(hs.peer_type as u8);
                                    let obj = cx.empty_object();
                                    obj.set(&mut cx, "peer_type", peer_type)?;
                                    Some((msg_type, Some(obj)))
                                },
                                RpcMsg::Properties(props) => {
                                    let msg_type = cx.number(MsgType::PROPERTIES as u8);
                                    if let Some(props) = props {
                                       let height = cx.number(props.height as f64);
                                        let obj = cx.empty_object();
                                        obj.set(&mut cx, "height", height)?;
                                        Some((msg_type, Some(obj)))
                                    } else {
                                        Some((msg_type, None))
                                    }
                                },
                                RpcMsg::Event(event) => {
                                    let msg_type = cx.number(MsgType::EVENT as u8);
                                    let obj = cx.empty_object();
                                    match event {
                                        RpcEvent::Tx(tx) => {
                                            let s = cx.string("tx");
                                            obj.set(&mut cx, "type", s)?;
                                            let tx = tx_variant_to_js!(cx, tx);
                                            obj.set(&mut cx, "data", tx)?;
                                        },
                                        RpcEvent::Block(block) => {
                                            let s = cx.string("block");
                                            obj.set(&mut cx, "type", s)?;
                                            let block = signed_block_to_js!(cx, block);
                                            obj.set(&mut cx, "data", block)?;
                                        }
                                    }
                                    Some((msg_type, Some(obj)))
                                }
                            }
                        } else {
                            None
                        };

                        let obj = cx.empty_object();
                        obj.set(&mut cx, "id", id)?;
                        if let Some(msg) = msg {
                            let (msg_type, data) = msg;
                            obj.set(&mut cx, "msg_type", msg_type)?;
                            if let Some(data) = data {
                                obj.set(&mut cx, "data", data)?;
                            }
                        }
                        Ok(obj.upcast())
                    } else {
                        Ok(JsUndefined::new().upcast())
                    }
                },
                Err(e) => cx.throw_error(e.description())
            }
        }
    }
}
