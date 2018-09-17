use sodiumoxide::crypto::hash::sha256::Digest;
use godcoin::blockchain::{Block, SignedBlock};
use godcoin::crypto::{double_sha256, SigPair};
use sodiumoxide::crypto::sign::Signature;
use godcoin::asset::Asset;
use neon::prelude::*;
use godcoin::tx::*;

use crypto::JsPublicKey;
use asset::JsAsset;

macro_rules! read_js_obj_to_block {
    ($cx:expr, $obj:expr) => {
        {
            let previous_hash = $obj.get(&mut $cx, "previous_hash")?
                                        .downcast_or_throw::<JsBuffer, _>(&mut $cx)?;
            let height = $obj.get(&mut $cx, "height")?
                                        .downcast_or_throw::<JsNumber, _>(&mut $cx)?;
            let timestamp = {
                let date = $obj.get(&mut $cx, "timestamp")?
                                    .downcast_or_throw::<JsObject, _>(&mut $cx)?;
                let func = date.get(&mut $cx, "getTime")?
                                    .downcast_or_throw::<JsFunction, _>(&mut $cx)?;
                (func.call::<_, _, JsValue, _>(&mut $cx, date, vec![])?
                    .downcast_or_throw::<JsNumber, _>(&mut $cx)?
                    .value() / 1000f64).floor()
            };
            let tx_merkle_root = $obj.get(&mut $cx, "tx_merkle_root")?
                                        .downcast_or_throw::<JsBuffer, _>(&mut $cx)?;
            let transactions = $obj.get(&mut $cx, "transactions")?
                                        .downcast_or_throw::<JsArray, _>(&mut $cx)?;

            let previous_hash = match Digest::from_slice({
                let guard = $cx.lock();
                let buf = previous_hash.borrow(&guard);
                buf.as_slice()
            }) {
                Some(d) => d,
                None => {
                    return $cx.throw_error("invalid previous_hash digest")
                }
            };

            let height = height.value() as u64;
            let timestamp = timestamp as u32;

            let tx_merkle_root = match Digest::from_slice({
                let guard = $cx.lock();
                let buf = tx_merkle_root.borrow(&guard);
                buf.as_slice()
            }) {
                Some(d) => d,
                None => {
                    return $cx.throw_error("invalid tx_merkle_root digest")
                }
            };

            let transactions = {
                let len = transactions.len();
                let mut vec: Vec<TxVariant> = Vec::with_capacity(len as usize);
                for i in 0..len {
                    let obj = transactions.get(&mut $cx, i)?
                                            .downcast_or_throw::<JsObject, _>(&mut $cx)?;
                    let v = read_js_obj_to_tx!($cx, obj);
                    vec.push(v);
                }
                vec
            };
            Block {
                previous_hash,
                height,
                timestamp,
                tx_merkle_root,
                transactions
            }
        }
    };
}

macro_rules! read_js_obj_to_signed_block {
    ($cx:expr, $obj:expr) => {
        {
            let block = read_js_obj_to_block!($cx, $obj);
            let sig_pair = $obj.get(&mut $cx, "sig_pair")?
                                    .downcast_or_throw::<JsArray, _>(&mut $cx)?;
            let sig_pair = js_sigpair_to_rs!($cx, sig_pair);
            SignedBlock {
                base: block,
                sig_pair
            }
        }
    };
}

pub fn block_encode_header(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let block = {
        let object = cx.argument::<JsObject>(0)?;
        let previous_hash = object.get(&mut cx, "previous_hash")?
                                    .downcast_or_throw::<JsBuffer, _>(&mut cx)?;
        let height = object.get(&mut cx, "height")?
                                    .downcast_or_throw::<JsNumber, _>(&mut cx)?;
        let timestamp = {
            let date = object.get(&mut cx, "timestamp")?
                                .downcast_or_throw::<JsObject, _>(&mut cx)?;
            let func = date.get(&mut cx, "getTime")?
                                .downcast_or_throw::<JsFunction, _>(&mut cx)?;
            (func.call::<_, _, JsValue, _>(&mut cx, date, vec![])?
                .downcast_or_throw::<JsNumber, _>(&mut cx)?
                .value() / 1000f64).floor()
        };
        let tx_merkle_root = object.get(&mut cx, "tx_merkle_root")?
                                    .downcast_or_throw::<JsBuffer, _>(&mut cx)?;

        let previous_hash = match Digest::from_slice({
            let guard = cx.lock();
            let buf = previous_hash.borrow(&guard);
            buf.as_slice()
        }) {
            Some(d) => d,
            None => {
                return cx.throw_error("invalid previous_hash digest")
            }
        };

        let height = height.value() as u64;
        let timestamp = timestamp as u32;

        let tx_merkle_root = match Digest::from_slice({
            let guard = cx.lock();
            let buf = tx_merkle_root.borrow(&guard);
            buf.as_slice()
        }) {
            Some(d) => d,
            None => {
                return cx.throw_error("invalid tx_merkle_root digest")
            }
        };

        Block {
            previous_hash,
            height,
            timestamp,
            tx_merkle_root,
            transactions: vec![]
        }
    };

    let mut vec = Vec::with_capacity(32768);
    block.encode_with_tx(&mut vec);
    let buf = bytes_to_js!(cx, &vec);
	Ok(buf)
}

pub fn block_calc_tx_merkle_root(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let transactions = {
        let arr = cx.argument::<JsArray>(0)?;
        let len = arr.len();
        let mut vec: Vec<TxVariant> = Vec::with_capacity(len as usize);
        for i in 0..len {
            let obj = arr.get(&mut cx, i)?
                            .downcast_or_throw::<JsObject, _>(&mut cx)?;
            let v = read_js_obj_to_tx!(cx, obj);
            vec.push(v);
        }
        vec
    };

    Ok({
        let mut buf = Vec::with_capacity(4096 * transactions.len());
        for tx in &transactions { tx.encode_with_sigs(&mut buf) };
        let buf = double_sha256(&buf);
        bytes_to_js!(cx, buf.as_ref())
    })
}

pub fn signed_block_encode_with_tx(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let obj = cx.argument::<JsObject>(0)?;
    let block = read_js_obj_to_signed_block!(cx, obj);

    let mut vec = Vec::with_capacity(32768);
    block.encode_with_tx(&mut vec);
    let buf = bytes_to_js!(cx, &vec);
	Ok(buf)
}
