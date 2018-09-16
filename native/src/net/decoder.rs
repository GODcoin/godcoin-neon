use godcoin::net::rpc::*;
use tokio_codec::Decoder;
use std::cell::RefCell;
use std::error::Error;
use neon::prelude::*;
use bytes::BytesMut;

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

        method decode(mut cx) {
            let payload = {
                let node_buf = cx.argument::<JsBuffer>(0)?;
                let mut this = cx.this();
                let guard = cx.lock();
                let mut this = this.borrow_mut(&guard);

                let node_buf = node_buf.borrow(&guard);
                let buf = this.buf.take().unwrap();
                buf.borrow_mut().extend_from_slice(node_buf.as_slice());
                let payload = this.inner.decode(&mut *buf.borrow_mut());
                this.buf = Some(buf);
                payload
            };
            match payload {
                Ok(payload) => {
                    if let Some(payload) = payload {
                        let id = cx.number(payload.id);
                        // TODO add the rpc payload to the object
                        let obj = cx.empty_object();
                        obj.set(&mut cx, "id", id)?;

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
