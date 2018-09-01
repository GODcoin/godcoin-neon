use sodiumoxide::crypto::sign::Signature;
use godcoin::crypto::SigPair;
use godcoin::asset::Asset;
use neon::prelude::*;
use godcoin::tx::*;

use crypto::JsPublicKey;
use asset::JsAsset;

pub fn tx_encode(mut cx: FunctionContext) -> JsResult<JsBuffer> {
	let object = cx.argument::<JsObject>(0)?;

	let tx: Tx = {
		let tx_type = object.get(&mut cx, "tx_type")?
                            .downcast_or_throw::<JsNumber, _>(&mut cx)?
                            .value();
		let timestamp = {
			let date = object.get(&mut cx, "timestamp")?
                                .downcast_or_throw::<JsObject, _>(&mut cx)?;
            let func = date.get(&mut cx, "getTime")?
                                .downcast_or_throw::<JsFunction, _>(&mut cx)?;
            (func.call::<_, _, JsValue, _>(&mut cx, date, vec![])?
                .downcast_or_throw::<JsNumber, _>(&mut cx)?
                .value() / 1000f64).floor()
		};
		let fee = object.get(&mut cx, "fee")?
                        .downcast_or_throw::<JsAsset, _>(&mut cx)?;

		let tx_type: TxType = match tx_type as u8 {
			t if t == TxType::REWARD as u8 => TxType::REWARD,
			t if t == TxType::BOND as u8 => TxType::BOND,
			t if t == TxType::TRANSFER as u8 => TxType::TRANSFER,
			_ => return cx.throw_error("invalid tx_type")
		};

		let fee = {
			let guard = cx.lock();
			let fee = fee.borrow(&guard);
			fee.clone()
		};

		Tx {
			tx_type,
			timestamp: timestamp as u32,
			fee,
			signature_pairs: vec![]
		}
	};

	let tx = match tx.tx_type {
		TxType::REWARD => {
			let to = object.get(&mut cx, "to")?.downcast_or_throw::<JsPublicKey, _>(&mut cx)?;
			let rewards = object.get(&mut cx, "rewards")?.downcast_or_throw::<JsArray, _>(&mut cx)?;

			let to = {
				let guard = cx.lock();
				let to = to.borrow(&guard);
				to.clone()
			};

			let rewards = {
				let mut vec: Vec<Asset> = Vec::with_capacity(rewards.len() as usize);
				for i in 0..rewards.len() {
					let asset = rewards.get(&mut cx, i)?.downcast_or_throw::<JsAsset, _>(&mut cx)?;
					let guard = cx.lock();
					let asset = asset.borrow(&guard).clone();
					vec.push(asset);
				}

				vec
			};

			TxVariant::RewardTx(RewardTx {
				base: tx,
				to,
				rewards
			})
		},
		TxType::BOND => {
			let minter = object.get(&mut cx, "minter")?
                                    .downcast_or_throw::<JsPublicKey, _>(&mut cx)?;
			let staker = object.get(&mut cx, "staker")?
                                    .downcast_or_throw::<JsPublicKey, _>(&mut cx)?;
			let stake_amt = object.get(&mut cx, "stake_amt")?
                                    .downcast_or_throw::<JsAsset, _>(&mut cx)?;
			let bond_fee = object.get(&mut cx, "bond_fee")?
                                    .downcast_or_throw::<JsAsset, _>(&mut cx)?;

			let guard = cx.lock();
			let minter = minter.borrow(&guard).clone();
			let staker = staker.borrow(&guard).clone();
			let stake_amt = stake_amt.borrow(&guard).clone();
			let bond_fee = bond_fee.borrow(&guard).clone();

			TxVariant::BondTx(BondTx {
				base: tx,
				minter,
				staker,
				stake_amt,
				bond_fee
			})
		},
		TxType::TRANSFER => {
			let from = object.get(&mut cx, "from")?
                                .downcast_or_throw::<JsPublicKey, _>(&mut cx)?;
			let to = object.get(&mut cx, "to")?
                            .downcast_or_throw::<JsPublicKey, _>(&mut cx)?;
			let amount = object.get(&mut cx, "amount")?
                                .downcast_or_throw::<JsAsset, _>(&mut cx)?;
			let memo = object.get(&mut cx, "memo")?
                                .downcast_or_throw::<JsBuffer, _>(&mut cx)?;

			let guard = cx.lock();
			let from = from.borrow(&guard).clone();
			let to = to.borrow(&guard).clone();
			let amount = amount.borrow(&guard).clone();
			let memo = {
				let slice = memo.borrow(&guard).as_slice();
				let mut vec = Vec::with_capacity(slice.len());
				vec.extend_from_slice(slice);
				vec
			};

			TxVariant::TransferTx(TransferTx {
				base: tx,
				from,
				to,
				amount,
				memo
			})
		}
	};

	let mut vec = Vec::with_capacity(4096);
	tx.encode(&mut vec);

	let mut buf = cx.buffer(vec.len() as u32)?;
	{
		let guard = cx.lock();
		let buf = buf.borrow_mut(&guard).as_mut_slice::<u8>();
		buf.copy_from_slice(&vec);
	}
	Ok(buf)
}


pub fn tx_encode_with_sigs(mut cx: FunctionContext) -> JsResult<JsBuffer> {
	let object = cx.argument::<JsObject>(0)?;

	let tx: Tx = {
		let tx_type = object.get(&mut cx, "tx_type")?
                            .downcast_or_throw::<JsNumber, _>(&mut cx)?
                            .value();
		let timestamp = {
			let date = object.get(&mut cx, "timestamp")?
                                .downcast_or_throw::<JsObject, _>(&mut cx)?;
            let func = date.get(&mut cx, "getTime")?
                            .downcast_or_throw::<JsFunction, _>(&mut cx)?;
            (func.call::<_, _, JsValue, _>(&mut cx, date, vec![])?
                .downcast_or_throw::<JsNumber, _>(&mut cx)?
                .value() / 1000f64).floor()
		};

		let fee = object.get(&mut cx, "fee")?
                            .downcast_or_throw::<JsAsset, _>(&mut cx)?;
		let sigs = object.get(&mut cx, "signature_pairs")?
                            .downcast_or_throw::<JsArray, _>(&mut cx)?;

		let tx_type: TxType = match tx_type as u8 {
			t if t == TxType::REWARD as u8 => TxType::REWARD,
			t if t == TxType::BOND as u8 => TxType::BOND,
			t if t == TxType::TRANSFER as u8 => TxType::TRANSFER,
			_ => return cx.throw_error("invalid tx_type")
		};

		let fee = {
			let guard = cx.lock();
			let fee = fee.borrow(&guard);
			fee.clone()
		};

		let sigs: Vec<SigPair> = {
			let mut vec = Vec::with_capacity(sigs.len() as usize);
			for i in 0..sigs.len() {
				let arr = sigs.get(&mut cx, i)?
                                .downcast_or_throw::<JsArray, _>(&mut cx)?;
				let key = arr.get(&mut cx, 0)?
                                .downcast_or_throw::<JsPublicKey, _>(&mut cx)?;
				let sig = arr.get(&mut cx, 1)?
                                .downcast_or_throw::<JsBuffer, _>(&mut cx)?;

				let sig = match Signature::from_slice({
					let guard = cx.lock();
					let sig = sig.borrow(&guard);
					sig.as_slice()
				}) {
					Some(s) => s,
					None => {
						return cx.throw_error("invalid signature")
					}
				};
				let guard = cx.lock();
				let key = key.borrow(&guard);

				vec.push(SigPair {
					pub_key: key.clone(),
					signature: sig
				});
			}
			vec
		};

		Tx {
			tx_type,
			timestamp: timestamp as u32,
			fee,
			signature_pairs: sigs
		}
	};

	let tx = match tx.tx_type {
		TxType::REWARD => {
			let to = object.get(&mut cx, "to")?.downcast_or_throw::<JsPublicKey, _>(&mut cx)?;
			let rewards = object.get(&mut cx, "rewards")?.downcast_or_throw::<JsArray, _>(&mut cx)?;

			let to = {
				let guard = cx.lock();
				let to = to.borrow(&guard);
				to.clone()
			};

			let rewards = {
				let mut vec: Vec<Asset> = Vec::with_capacity(rewards.len() as usize);
				for i in 0..rewards.len() {
					let asset = rewards.get(&mut cx, i)?.downcast_or_throw::<JsAsset, _>(&mut cx)?;
					let guard = cx.lock();
					let asset = asset.borrow(&guard).clone();
					vec.push(asset);
				}

				vec
			};

			TxVariant::RewardTx(RewardTx {
				base: tx,
				to,
				rewards
			})
		},
		TxType::BOND => {
			let minter = object.get(&mut cx, "minter")?
                                .downcast_or_throw::<JsPublicKey, _>(&mut cx)?;
			let staker = object.get(&mut cx, "staker")?
                                .downcast_or_throw::<JsPublicKey, _>(&mut cx)?;
			let stake_amt = object.get(&mut cx, "stake_amt")?
                                .downcast_or_throw::<JsAsset, _>(&mut cx)?;
			let bond_fee = object.get(&mut cx, "bond_fee")?
                                .downcast_or_throw::<JsAsset, _>(&mut cx)?;

			let guard = cx.lock();
			let minter = minter.borrow(&guard).clone();
			let staker = staker.borrow(&guard).clone();
			let stake_amt = stake_amt.borrow(&guard).clone();
			let bond_fee = bond_fee.borrow(&guard).clone();

			TxVariant::BondTx(BondTx {
				base: tx,
				minter,
				staker,
				stake_amt,
				bond_fee
			})
		},
		TxType::TRANSFER => {
			let from = object.get(&mut cx, "from")?
                                .downcast_or_throw::<JsPublicKey, _>(&mut cx)?;
			let to = object.get(&mut cx, "to")?
                                .downcast_or_throw::<JsPublicKey, _>(&mut cx)?;
			let amount = object.get(&mut cx, "amount")?
                                .downcast_or_throw::<JsAsset, _>(&mut cx)?;
			let memo = object.get(&mut cx, "memo")?
                                .downcast_or_throw::<JsBuffer, _>(&mut cx)?;

			let guard = cx.lock();
			let from = from.borrow(&guard).clone();
			let to = to.borrow(&guard).clone();
			let amount = amount.borrow(&guard).clone();
			let memo = {
				let slice = memo.borrow(&guard).as_slice();
				let mut vec = Vec::with_capacity(slice.len());
				vec.extend_from_slice(slice);
				vec
			};

			TxVariant::TransferTx(TransferTx {
				base: tx,
				from,
				to,
				amount,
				memo
			})
		}
	};

	let mut vec = Vec::with_capacity(4096);
	tx.encode_with_sigs(&mut vec);

	let mut buf = cx.buffer(vec.len() as u32)?;
	{
		let guard = cx.lock();
		let buf = buf.borrow_mut(&guard).as_mut_slice::<u8>();
		buf.copy_from_slice(&vec);
	}
	Ok(buf)
}
