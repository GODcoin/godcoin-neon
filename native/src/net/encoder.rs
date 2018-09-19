use godcoin::blockchain::{Block, SignedBlock, Properties};
use sodiumoxide::crypto::hash::sha256::Digest;
use sodiumoxide::crypto::sign::Signature;
use godcoin::asset::{Asset, Balance};
use godcoin::net::{peer::*, rpc::*};
use godcoin::crypto::SigPair;
use tokio_codec::Encoder;
use neon::prelude::*;
use bytes::BytesMut;
use godcoin::tx::*;

use crypto::JsPublicKey;
use asset::JsAsset;

pub fn encode(mut cx: FunctionContext) -> JsResult<JsValue> {
    let obj = cx.argument::<JsObject>(0)?;
    let id = obj.get(&mut cx, "id")?
                .downcast_or_throw::<JsNumber, _>(&mut cx)?.value() as u32;
    let msg = {
        let msg_type = obj.get(&mut cx, "msg_type")?
                            .downcast_or_throw::<JsNumber, _>(&mut cx)?.value() as i8;
        match msg_type {
            t if t == -1 => None,
            t if t == RpcMsgType::Error as i8 => {
                let obj = obj.get(&mut cx, "res")?
                                .downcast_or_throw::<JsString, _>(&mut cx)?
                                .value();
                Some(RpcMsg::Error(obj))
            }
            t if t == RpcMsgType::Event as i8 => {
                let obj = obj.get(&mut cx, "req")?
                                .downcast_or_throw::<JsObject, _>(&mut cx)?;
                let event_type = obj.get(&mut cx, "type")?
                                    .downcast_or_throw::<JsString, _>(&mut cx)?
                                    .value();
                match event_type.as_ref() {
                    "tx" => {
                        let data = obj.get(&mut cx, "data")?
                                        .downcast_or_throw::<JsObject, _>(&mut cx)?;
                        let tx = read_js_obj_to_tx!(cx, data);
                        Some(RpcMsg::Event(RpcEvent::Tx(tx)))
                    },
                    "block" => {
                        let data = obj.get(&mut cx, "data")?
                                        .downcast_or_throw::<JsObject, _>(&mut cx)?;
                        let block = read_js_obj_to_signed_block!(cx, data);
                        Some(RpcMsg::Event(RpcEvent::Block(block)))
                    },
                    _ => return cx.throw_error("invalid event type")
                }
            },
            t if t == RpcMsgType::Handshake as i8 => {
                let obj = obj.get(&mut cx, "req")?
                                .downcast_or_throw::<JsObject, _>(&mut cx)?;
                let peer_type = obj.get(&mut cx, "peer_type")?
                                    .downcast_or_throw::<JsNumber, _>(&mut cx)?.value() as u8;
                let peer_type = match peer_type {
                    t if t == PeerType::NODE as u8 => PeerType::NODE,
                    t if t == PeerType::WALLET as u8 => PeerType::WALLET,
                    _ => return cx.throw_error("invalid peer_type")
                };
                Some(RpcMsg::Handshake(peer_type))
            },
            t if t == RpcMsgType::Properties as i8 => {
                let obj = obj.get(&mut cx, "res")?;
                let props = if obj.is_a::<JsObject>() {
                    let obj = obj.downcast_or_throw::<JsObject, _>(&mut cx)?;
                    let height = obj.get(&mut cx, "msg_type")?
                                    .downcast_or_throw::<JsNumber, _>(&mut cx)?.value() as u64;
                    RpcMsg::Properties(RpcVariant::Res(Properties { height }))
                } else {
                    RpcMsg::Properties(RpcVariant::Req(()))
                };
                Some(props)
            },
            t if t == RpcMsgType::Block as i8 => {
                let req = obj.get(&mut cx, "req")?;
                if req.is_a::<JsObject>() {
                    let req = req.downcast_or_throw::<JsObject, _>(&mut cx)?;
                    let height = req.get(&mut cx, "height")?
                                    .downcast_or_throw::<JsNumber, _>(&mut cx)?
                                    .value() as u64;
                    Some(RpcMsg::Block(RpcVariant::Req(height)))
                } else {
                    let res = obj.get(&mut cx, "res")?
                                    .downcast_or_throw::<JsObject, _>(&mut cx)?;
                    let block = read_js_obj_to_signed_block!(cx, res);
                    Some(RpcMsg::Block(RpcVariant::Res(block)))
                }
            },
            t if t == RpcMsgType::Balance as i8 => {
                let req = obj.get(&mut cx, "req")?;
                if req.is_a::<JsObject>() {
                    let req = req.downcast_or_throw::<JsPublicKey, _>(&mut cx)?;
                    let guard = cx.lock();
                    let addr = req.borrow(&guard).clone();
                    Some(RpcMsg::Balance(RpcVariant::Req(addr)))
                } else {
                    let res = obj.get(&mut cx, "res")?
                                    .downcast_or_throw::<JsArray, _>(&mut cx)?;
                    let gold = res.get(&mut cx, 0)?
                                    .downcast_or_throw::<JsAsset, _>(&mut cx)?;
                    let silver = res.get(&mut cx, 1)?
                                    .downcast_or_throw::<JsAsset, _>(&mut cx)?;
                    let guard = cx.lock();
                    let gold = gold.borrow(&guard).clone();
                    let silver = silver.borrow(&guard).clone();
                    Some(RpcMsg::Balance(RpcVariant::Res(Balance {
                        gold,
                        silver
                    })))
                }
            },
            t if t == RpcMsgType::TotalFee as i8 => {
                let req = obj.get(&mut cx, "req")?;
                if req.is_a::<JsObject>() {
                    let req = req.downcast_or_throw::<JsPublicKey, _>(&mut cx)?;
                    let guard = cx.lock();
                    let addr = req.borrow(&guard).clone();
                    Some(RpcMsg::TotalFee(RpcVariant::Req(addr)))
                } else {
                    let res = obj.get(&mut cx, "res")?
                                    .downcast_or_throw::<JsArray, _>(&mut cx)?;
                    let gold = res.get(&mut cx, 0)?
                                    .downcast_or_throw::<JsAsset, _>(&mut cx)?;
                    let silver = res.get(&mut cx, 1)?
                                    .downcast_or_throw::<JsAsset, _>(&mut cx)?;
                    let guard = cx.lock();
                    let gold = gold.borrow(&guard).clone();
                    let silver = silver.borrow(&guard).clone();
                    Some(RpcMsg::TotalFee(RpcVariant::Res(Balance {
                        gold,
                        silver
                    })))
                }
            },
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
