macro_rules! bytes_to_js {
	($ty:ident, $cx:expr, $bytes:expr) => {
		{
			let buf = bytes_to_js!($cx, $bytes);
			$ty::new(&mut $cx, vec![buf])?
		}
	};
	($cx:expr, $bytes:expr) => {
		{
			let mut buf = $cx.buffer($bytes.len() as u32)?;
			{
				let guard = $cx.lock();
				let data = buf.borrow_mut(&guard).as_mut_slice::<u8>();
				data.copy_from_slice($bytes);
			}
			buf
		}
	};
}

macro_rules! js_obj_to_sigpair {
	($cx:expr, $arr:expr) => {
		{
			let key = $arr.get(&mut $cx, 0)?
							.downcast_or_throw::<JsPublicKey, _>(&mut $cx)?;
			let sig = $arr.get(&mut $cx, 1)?
							.downcast_or_throw::<JsBuffer, _>(&mut $cx)?;
			let sig = match Signature::from_slice({
				let guard = $cx.lock();
				let sig = sig.borrow(&guard);
				sig.as_slice()
			}) {
				Some(s) => s,
				None => {
					return $cx.throw_error("invalid signature")
				}
			};
			let guard = $cx.lock();
			let key = key.borrow(&guard);
			SigPair {
				pub_key: key.clone(),
				signature: sig
			}
		}
	}
}
