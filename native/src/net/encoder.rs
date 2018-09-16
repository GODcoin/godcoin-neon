use godcoin::net::{peer::*, rpc::*};
use godcoin::blockchain::Properties;
use tokio_codec::Encoder;
use neon::prelude::*;
use bytes::BytesMut;

use super::constants::*;

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
                let data = obj.get(&mut cx, "data")?;
                let props = if data.is_a::<JsObject>() {
                    let obj = data.downcast_or_throw::<JsObject, _>(&mut cx)?;
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
