use godcoin::tx::TxVariant;
use neon::prelude::*;
use std::io::Cursor;

use crypto::JsPublicKey;
use asset::JsAsset;

macro_rules! tx_variant_to_js {
	($cx:expr, $tx:expr) => {
		{
			let obj = $cx.empty_object();
			{
				let tx_type = $cx.number($tx.tx_type as u8);
				(*obj).set(&mut $cx, "tx_type", tx_type)?;

				let timestamp = $cx.number($tx.timestamp);
				(*obj).set(&mut $cx, "timestamp", timestamp)?;

				let fee = asset_to_js!($cx, $tx.fee);
				(*obj).set(&mut $cx, "fee", fee)?;

				let arr = {
					let mut arr = $cx.empty_array();
					let len = $tx.signature_pairs.len();
					for i in 0..len {
						let pair = &$tx.signature_pairs[i];
						let key = bytes_to_js!(JsPublicKey, $cx, pair.pub_key.as_bytes());
						let sig = bytes_to_js!($cx, pair.signature.as_ref());
						let pair = $cx.empty_array();
						pair.set(&mut $cx, 0, key)?;
						pair.set(&mut $cx, 1, sig)?;
						arr.set(&mut $cx, i as u32, pair)?;
					}
					arr
				};
				(*obj).set(&mut $cx, "signature_pairs", arr)?;
			}

			match $tx {
				TxVariant::RewardTx(tx) => {
					let to = bytes_to_js!(JsPublicKey, $cx, tx.to.as_bytes());
					(*obj).set(&mut $cx, "to", to)?;

					let arr = {
						let mut arr = $cx.empty_array();
						let len = tx.rewards.len();
						for i in 0..len {
							let a = asset_to_js!($cx, tx.rewards[i]);
							arr.set(&mut $cx, i as u32, a)?;
						}
						arr
					};
					(*obj).set(&mut $cx, "rewards", arr)?;
				},
				TxVariant::BondTx(tx) => {
					let minter = bytes_to_js!(JsPublicKey, $cx, tx.minter.as_bytes());
					(*obj).set(&mut $cx, "minter", minter)?;

					let staker = bytes_to_js!(JsPublicKey, $cx, tx.staker.as_bytes());
					(*obj).set(&mut $cx, "staker", staker)?;

					let stake_amt = asset_to_js!($cx, tx.stake_amt);
					(*obj).set(&mut $cx, "stake_amt", stake_amt)?;

					let bond_fee = asset_to_js!($cx, tx.bond_fee);
					(*obj).set(&mut $cx, "bond_fee", bond_fee)?;
				},
				TxVariant::TransferTx(tx) => {
					let from = bytes_to_js!(JsPublicKey, $cx, tx.from.as_bytes());
					(*obj).set(&mut $cx, "from", from)?;

					let to = bytes_to_js!(JsPublicKey, $cx, tx.to.as_bytes());
					(*obj).set(&mut $cx, "to", to)?;

					let amt = asset_to_js!($cx, tx.amount);
					(*obj).set(&mut $cx, "amount", amt)?;

					let memo = bytes_to_js!($cx, &tx.memo);
					(*obj).set(&mut $cx, "memo", memo)?;
				}
			}
			obj
		}
	};
}

pub fn tx_decode_with_sigs(mut cx: FunctionContext) -> JsResult<JsValue> {
	let tx = {
		let buf = cx.argument::<JsBuffer>(0)?;
		let guard = cx.lock();
		let buf = buf.borrow(&guard).as_slice::<u8>();
		let mut cur = Cursor::<&[u8]>::new(buf);

		TxVariant::decode_with_sigs(&mut cur)
	};
	let tx = match tx {
		Some(tx) => tx,
		None => return Ok(cx.undefined().upcast())
	};

	let obj = tx_variant_to_js!(cx, tx);
	Ok(obj.upcast())
}
