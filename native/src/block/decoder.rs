use godcoin::blockchain::block::*;
use godcoin::tx::TxVariant;
use neon::prelude::*;
use std::io::Cursor;

use crypto::JsPublicKey;
use asset::JsAsset;

macro_rules! signed_block_to_js {
    ($cx:expr, $block:expr) => {
        {
            let obj = $cx.empty_object();
            {
                let buf = bytes_to_js!($cx, $block.previous_hash.as_ref());
                obj.set(&mut $cx, "previous_hash", buf)?;

                let num = $cx.number($block.height as f64);
                obj.set(&mut $cx, "height", num)?;

                let num = $cx.number($block.timestamp);
                obj.set(&mut $cx, "timestamp", num)?;

                let buf = bytes_to_js!($cx, $block.tx_merkle_root.as_ref());
                obj.set(&mut $cx, "tx_merkle_root", buf)?;

                let pair = &$block.sig_pair;
                let key = bytes_to_js!(JsPublicKey, $cx, pair.pub_key.as_bytes());
                let sig = bytes_to_js!($cx, pair.signature.as_ref());
                let pair = $cx.empty_array();
                pair.set(&mut $cx, 0, key)?;
                pair.set(&mut $cx, 1, sig)?;
                obj.set(&mut $cx, "sig_pair", pair)?;
            }

            {
                let txs_arr = $cx.empty_array();
                let len = $block.transactions.len();
                for i in 0..len {
                    let tx = &$block.transactions[i];
                    let obj = tx_variant_to_js!($cx, tx);
                    txs_arr.set(&mut $cx, i as u32, obj)?;
                }

                obj.set(&mut $cx, "transactions", txs_arr)?;
            }

            obj
        }
    };
}

pub fn signed_block_decode_with_tx(mut cx: FunctionContext) -> JsResult<JsValue> {
    let block = {
		let buf = cx.argument::<JsBuffer>(0)?;
		let guard = cx.lock();
		let buf = buf.borrow(&guard).as_slice::<u8>();
		let mut cur = Cursor::<&[u8]>::new(buf);

		SignedBlock::decode_with_tx(&mut cur)
	};
	match block {
		Some(block) => {
            let block = signed_block_to_js!(cx, block);
            Ok(block.upcast())
        },
		None => Ok(cx.undefined().upcast())
	}
}
