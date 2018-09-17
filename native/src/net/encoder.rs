use godcoin::blockchain::{Block, SignedBlock, Properties};
use sodiumoxide::crypto::hash::sha256::Digest;
use sodiumoxide::crypto::sign::Signature;
use godcoin::net::{peer::*, rpc::*};
use godcoin::crypto::SigPair;
use godcoin::asset::Asset;
use tokio_codec::Encoder;
use neon::prelude::*;
use bytes::BytesMut;
use godcoin::tx::*;

use super::constants::*;
use crypto::JsPublicKey;
use asset::JsAsset;

pub fn encode(mut cx: FunctionContext) -> JsResult<JsValue> {
    let obj = cx.argument::<JsObject>(0)?;
    let id = obj.get(&mut cx, "id")?
                .downcast_or_throw::<JsNumber, _>(&mut cx)?.value() as u32;
    let msg = {
        let msg_type = obj.get(&mut cx, "msg_type")?
                            .downcast_or_throw::<JsNumber, _>(&mut cx)?.value() as u8;
        match msg_type {
            t if t == MsgType::NONE as u8 => None,
            t if t == MsgType::HANDSHAKE as u8 => {
                let obj = obj.get(&mut cx, "data")?
                                .downcast_or_throw::<JsObject, _>(&mut cx)?;
                let peer_type = obj.get(&mut cx, "peer_type")?
                                    .downcast_or_throw::<JsNumber, _>(&mut cx)?.value() as u8;
                let peer_type = match peer_type {
                    t if t == PeerType::NODE as u8 => PeerType::NODE,
                    t if t == PeerType::WALLET as u8 => PeerType::WALLET,
                    _ => return cx.throw_error("invalid peer_type")
                };
                Some(RpcMsg::Handshake(RpcMsgHandshake {
                    peer_type
                }))
            },
            t if t == MsgType::PROPERTIES as u8 => {
                let obj = obj.get(&mut cx, "data")?;
                let props = if obj.is_a::<JsObject>() {
                    let obj = obj.downcast_or_throw::<JsObject, _>(&mut cx)?;
                    let height = obj.get(&mut cx, "msg_type")?
                                    .downcast_or_throw::<JsNumber, _>(&mut cx)?.value() as u64;
                    RpcMsg::Properties(Some(Properties {
                        height
                    }))
                } else {
                    RpcMsg::Properties(None)
                };
                Some(props)
            },
            t if t == MsgType::EVENT as u8 => {
                let obj = obj.get(&mut cx, "data")?
                                .downcast_or_throw::<JsObject, _>(&mut cx)?;
                let event_type = obj.get(&mut cx, "type")?
                                    .downcast_or_throw::<JsString, _>(&mut cx)?
                                    .value();
                let sub: bool = {
                    let sub = obj.get(&mut cx, "subscribe")?;
                    if sub.is_a::<JsBoolean>() {
                        sub.downcast_or_throw::<JsBoolean, _>(&mut cx)?.value()
                    } else {
                        false
                    }
                };
                match event_type.as_ref() {
                    "tx" => {
                        if sub {
                            Some(RpcMsg::Event(RpcEvent::Tx(None)))
                        } else {
                            let data = obj.get(&mut cx, "data")?
                                            .downcast_or_throw::<JsObject, _>(&mut cx)?;
                            let tx = read_js_obj_to_tx!(cx, data);
                            Some(RpcMsg::Event(RpcEvent::Tx(Some(tx))))
                        }
                    },
                    "block" => {
                        if sub {
                            Some(RpcMsg::Event(RpcEvent::Block(None)))
                        } else {
                            let data = obj.get(&mut cx, "data")?
                                            .downcast_or_throw::<JsObject, _>(&mut cx)?;
                            let block = read_js_obj_to_signed_block!(cx, data);
                            Some(RpcMsg::Event(RpcEvent::Block(Some(block))))
                        }
                    },
                    _ => return cx.throw_error("invalid event type")
                }
            }
            _ => return cx.throw_error("invalid msg_type")
        }
    };

    let mut buf = BytesMut::with_capacity(4096);
    let mut rpc = codec::RpcCodec::new();
    rpc.encode(RpcPayload {
        id,
        msg
    }, &mut buf).unwrap();

    let buf = bytes_to_js!(cx, &buf);
    Ok(buf.upcast())
}
