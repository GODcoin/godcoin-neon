use godcoin::tx::TxVariant;
use godcoin::net::rpc::*;
use tokio_codec::Decoder;
use std::cell::RefCell;
use std::error::Error;
use neon::prelude::*;
use bytes::BytesMut;

use crate::crypto::JsPublicKey;
use crate::asset::JsAsset;

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
                                RpcMsg::Error(err) => {
                                    let msg_type = cx.number(RpcMsgType::Error as u8);
                                    let obj = cx.empty_object();
                                    let err = cx.string(err);
                                    obj.set(&mut cx, "error", err)?;
                                    Some((msg_type, RpcVariant::Res(obj)))
                                },
                                RpcMsg::Event(evt) => {
                                    let msg_type = cx.number(RpcMsgType::Event as u8);
                                    let obj = cx.empty_object();
                                    match *evt {
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
                                    Some((msg_type, RpcVariant::Req(obj)))
                                },
                                RpcMsg::Handshake(peer_type) => {
                                    let msg_type = cx.number(RpcMsgType::Handshake as u8);
                                    let peer_type = cx.number(peer_type as u8);
                                    let obj = cx.empty_object();
                                    obj.set(&mut cx, "peer_type", peer_type)?;
                                    Some((msg_type, RpcVariant::Req(obj)))
                                },
                                RpcMsg::Broadcast(tx) => {
                                    let msg_type = cx.number(RpcMsgType::Broadcast as u8);
                                    let tx = tx_variant_to_js!(cx, tx);
                                    Some((msg_type, RpcVariant::Req(tx)))
                                },
                                RpcMsg::Properties(rpc) => {
                                    let msg_type = cx.number(RpcMsgType::Properties as u8);
                                    let obj = cx.empty_object();
                                    if let Some(props) = rpc.res() {
                                        let height = cx.number(props.height as f64);
                                        obj.set(&mut cx, "height", height)?;
                                        {
                                            let gold = asset_to_js!(cx, props.token_supply.gold);
                                            let silver = asset_to_js!(cx, props.token_supply.silver);
                                            let arr = cx.empty_array();
                                            arr.set(&mut cx, 0, gold)?;
                                            arr.set(&mut cx, 1, silver)?;
                                            obj.set(&mut cx, "token_supply", arr)?;
                                        }
                                        {
                                            let gold = asset_to_js!(cx, props.network_fee.gold);
                                            let silver = asset_to_js!(cx, props.network_fee.silver);
                                            let arr = cx.empty_array();
                                            arr.set(&mut cx, 0, gold)?;
                                            arr.set(&mut cx, 1, silver)?;
                                            obj.set(&mut cx, "network_fee", arr)?;
                                        }

                                        Some((msg_type, RpcVariant::Res(obj)))
                                    } else {
                                        Some((msg_type, RpcVariant::Req(obj)))
                                    }
                                },
                                RpcMsg::Block(var) => {
                                    let msg_type = cx.number(RpcMsgType::Block as u8);
                                    match *var {
                                        RpcVariant::Req(height) => {
                                            let height = cx.number(height as f64);
                                            let obj = cx.empty_object();
                                            obj.set(&mut cx, "height", height)?;
                                            Some((msg_type, RpcVariant::Req(obj)))
                                        },
                                        RpcVariant::Res(block) => {
                                            if let Some(block) = block {
                                                let block = signed_block_to_js!(cx, block);
                                                Some((msg_type, RpcVariant::Res(block)))
                                            } else {
                                                let obj = cx.empty_object();
                                                Some((msg_type, RpcVariant::Res(obj)))
                                            }
                                        }
                                    }
                                },
                                RpcMsg::Balance(rpc) => {
                                    let msg_type = cx.number(RpcMsgType::Balance as u8);
                                    match rpc {
                                        RpcVariant::Req(addr) => {
                                            let addr = bytes_to_js!(JsPublicKey, cx, addr.as_bytes());
                                            Some((msg_type, RpcVariant::Req(addr.upcast())))
                                        },
                                        RpcVariant::Res(bal) => {
                                            let gold = asset_to_js!(cx, bal.gold);
                                            let silver = asset_to_js!(cx, bal.silver);
                                            let arr = cx.empty_array();
                                            arr.set(&mut cx, 0, gold)?;
                                            arr.set(&mut cx, 1, silver)?;
                                            Some((msg_type, RpcVariant::Res(arr.upcast())))
                                        }
                                    }
                                },
                                RpcMsg::TotalFee(rpc) => {
                                    let msg_type = cx.number(RpcMsgType::TotalFee as u8);
                                    match rpc {
                                        RpcVariant::Req(addr) => {
                                            let addr = bytes_to_js!(JsPublicKey, cx, addr.as_bytes());
                                            Some((msg_type, RpcVariant::Req(addr.upcast())))
                                        },
                                        RpcVariant::Res(bal) => {
                                            let gold = asset_to_js!(cx, bal.gold);
                                            let silver = asset_to_js!(cx, bal.silver);
                                            let arr = cx.empty_array();
                                            arr.set(&mut cx, 0, gold)?;
                                            arr.set(&mut cx, 1, silver)?;
                                            Some((msg_type, RpcVariant::Res(arr.upcast())))
                                        }
                                    }
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
                            match data {
                                RpcVariant::Req(data) => {
                                    obj.set(&mut cx, "req", data)?;
                                },
                                RpcVariant::Res(data) => {
                                    obj.set(&mut cx, "res", data)?;
                                }
                            }
                        } else {
                            let num = cx.number(-1);
                            obj.set(&mut cx, "msg_type", num)?;
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
