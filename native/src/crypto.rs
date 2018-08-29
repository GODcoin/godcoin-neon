use godcoin::{PublicKey, PrivateKey, Wif, KeyPair};
use sodiumoxide::crypto::sign::*;
use std::error::Error;
use neon::prelude::*;

declare_types! {
    pub class JsPublicKey for PublicKey {
        init(mut cx) {
            let key = cx.argument::<JsArrayBuffer>(0)?;
            let len = {
                let guard = cx.lock();
                let bytes = key.borrow(&guard);
                bytes.len()
            };
            if len != PUBLICKEYBYTES { return cx.throw_error("invalid buffer len") }

            let res = {
                let guard = cx.lock();
                let bytes = key.borrow(&guard);
                PublicKey::from_bytes(bytes.as_slice())
            };
            match res {
                Some(key) => Ok(key),
                None => cx.throw_error("invalid buffer size")
            }
        }

        method verify(mut cx) {
            let sig = cx.argument::<JsBuffer>(0)?;
            let msg = cx.argument::<JsBuffer>(1)?;

            let verified = {
                let data = {
                    let guard = cx.lock();
                    let sig = sig.borrow(&guard);
                    let msg = msg.borrow(&guard);
                    (sig.as_slice(), msg.as_slice())
                };

                let sig = match Signature::from_slice(data.0) {
                    Some(sig) => sig,
                    None => return cx.throw_error("invalid signature")
                };

                let this = cx.this();
                let guard = cx.lock();
                let key = this.borrow(&guard);
                key.verify(data.1, &sig)
            };
            Ok(JsBoolean::new(&mut cx, verified).upcast())
        }

        method to_wif(mut cx) {
            let s = {
                let this = cx.this();
                let guard = cx.lock();
                let key = this.borrow(&guard);
                key.to_wif()
            };
            Ok(JsString::new(&mut cx, s).upcast())
        }
    }
}

pub fn public_key_from_wif(mut cx: FunctionContext) -> JsResult<JsPublicKey> {
    let s = cx.argument::<JsString>(0)?.value();
    match PublicKey::from_wif(&s) {
        Ok(key) => {
            let bytes = key.as_bytes();
            let buf = {
                let mut buf = cx.array_buffer(bytes.len() as u32)?;
                {
                    let guard = cx.lock();
                    let data = buf.borrow_mut(&guard).as_mut_slice::<u8>();
                    data.copy_from_slice(bytes);
                }
                buf
            };
            Ok(JsPublicKey::new(&mut cx, vec![buf]).unwrap())
        },
        Err(e) => cx.throw_error(e.description())
    }
}

declare_types! {
    pub class JsPrivateKey for PrivateKey {
        init(mut cx) {
            let seed = cx.argument::<JsArrayBuffer>(0)?;
            let secret = cx.argument::<JsArrayBuffer>(1)?;

            let seed_len = {
                let guard = cx.lock();
                let bytes = seed.borrow(&guard);
                bytes.len()
            };

            let secret_len = {
                let guard = cx.lock();
                let bytes = secret.borrow(&guard);
                bytes.len()
            };

            if seed_len != SEEDBYTES { return cx.throw_error("invalid seed len") }
            if secret_len != SECRETKEYBYTES { return cx.throw_error("invalid secret len") }

            let seed_bytes = {
                let guard = cx.lock();
                let bytes = seed.borrow(&guard);
                bytes.as_slice()
            };

            let secret_bytes = {
                let guard = cx.lock();
                let bytes = secret.borrow(&guard);
                bytes.as_slice()
            };

            match PrivateKey::from_bytes(&seed_bytes, &secret_bytes) {
                Some(key) => Ok(key),
                None => unreachable!()
            }
        }

        method sign(mut cx) {
            let msg = cx.argument::<JsBuffer>(0)?;
            let sig = {
                let this = cx.this();
                let guard = cx.lock();
                let key = this.borrow(&guard);
                let bytes = msg.borrow(&guard).as_slice();
                key.sign(bytes)
            };

            let mut buf = cx.buffer(SIGNATUREBYTES as u32)?;
            {
                let guard = cx.lock();
                let data = buf.borrow_mut(&guard).as_mut_slice::<u8>();
                data.copy_from_slice(sig.as_ref())
            }

            Ok(buf.upcast())
        }

        method to_wif(mut cx) {
            let s = {
                let this = cx.this();
                let guard = cx.lock();
                let key = this.borrow(&guard);
                key.to_wif()
            };
            Ok(JsString::new(&mut cx, s).upcast())
        }
    }
}

pub fn private_key_from_wif(mut cx: FunctionContext) -> JsResult<JsArray> {
    let s = cx.argument::<JsString>(0)?.value();
    match PrivateKey::from_wif(&s) {
        Ok(key) => {
            let pub_key = {
                let bytes = key.0.as_bytes();
                let mut buf = cx.array_buffer(bytes.len() as u32)?;
                {
                    let guard = cx.lock();
                    let data = buf.borrow_mut(&guard).as_mut_slice::<u8>();
                    data.copy_from_slice(bytes);
                }
                JsPublicKey::new(&mut cx, vec![buf]).unwrap()
            };

            let priv_key = {
                let bytes = key.1.as_bytes();
                let mut seed_buf = cx.array_buffer(bytes.0.len() as u32)?;
                let mut priv_buf = cx.array_buffer(bytes.1.len() as u32)?;
                {
                    let guard = cx.lock();
                    let seed_bytes = seed_buf.borrow_mut(&guard).as_mut_slice();
                    seed_bytes.copy_from_slice(&bytes.0);

                    let priv_bytes = priv_buf.borrow_mut(&guard).as_mut_slice();
                    priv_bytes.copy_from_slice(&bytes.1);
                }
                JsPrivateKey::new(&mut cx, vec![seed_buf, priv_buf]).unwrap()
            };

            let mut arr = JsArray::new(&mut cx, 2);
            arr.set(&mut cx, 0, pub_key)?;
            arr.set(&mut cx, 1, priv_key)?;
            Ok(arr)
        },
        Err(e) => cx.throw_error(e.description())
    }
}

pub fn private_key_gen_key_pair(mut cx: FunctionContext) -> JsResult<JsArray> {
    let key = KeyPair::gen_keypair();

    let pub_key = {
        let bytes = key.0.as_bytes();
        let mut buf = cx.array_buffer(bytes.len() as u32)?;
        {
            let guard = cx.lock();
            let data = buf.borrow_mut(&guard).as_mut_slice::<u8>();
            data.copy_from_slice(bytes);
        }
        JsPublicKey::new(&mut cx, vec![buf]).expect("public key to be created")
    };

    let priv_key = {
        let bytes = key.1.as_bytes();
        let mut seed_buf = cx.array_buffer(bytes.0.len() as u32)?;
        let mut priv_buf = cx.array_buffer(bytes.1.len() as u32)?;
        {
            let guard = cx.lock();
            let seed_bytes = seed_buf.borrow_mut(&guard).as_mut_slice();
            seed_bytes.copy_from_slice(&bytes.0);

            let priv_bytes = priv_buf.borrow_mut(&guard).as_mut_slice();
            priv_bytes.copy_from_slice(&bytes.1);
        }
        JsPrivateKey::new(&mut cx, vec![seed_buf, priv_buf]).expect("private key to be created")
    };

    let arr = JsArray::new(&mut cx, 2);
    arr.set(&mut cx, 0, pub_key)?;
    arr.set(&mut cx, 1, priv_key)?;
    Ok(arr)
}
