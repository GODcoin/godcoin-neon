#[macro_use] extern crate neon;
extern crate tokio_codec;
extern crate sodiumoxide;
extern crate godcoin;
extern crate futures;
extern crate bytes;

use neon::prelude::*;

#[macro_use] mod util;
#[macro_use] mod asset;
use asset::*;

mod crypto;
use crypto::*;

#[macro_use] mod tx;
mod block;

mod net;
use net::*;

fn init(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    if godcoin::init().is_ok() { Ok(JsUndefined::new()) }
    else { cx.throw_error("failed to initialize library") }
}

register_module!(mut cx, {
    cx.export_function("init", init)?;

    cx.export_class::<JsAsset>("Asset")?;
    cx.export_function("Asset_from_string", asset_from_string)?;

    cx.export_class::<JsPublicKey>("PublicKey")?;
    cx.export_function("PublicKey_from_wif", public_key_from_wif)?;

    cx.export_class::<JsPrivateKey>("PrivateKey")?;
    cx.export_function("PrivateKey_from_wif", private_key_from_wif)?;
    cx.export_function("PrivateKey_gen_key_pair", private_key_gen_key_pair)?;

    cx.export_function("Tx_encode", tx::encoder::tx_encode)?;
    cx.export_function("Tx_encode_with_sigs", tx::encoder::tx_encode_with_sigs)?;
    cx.export_function("Tx_decode_with_sigs", tx::decoder::tx_decode_with_sigs)?;

    cx.export_function("Block_calc_tx_merkle_root", block::encoder::block_calc_tx_merkle_root)?;
    cx.export_function("Block_encode_header", block::encoder::block_encode_header)?;
    cx.export_function("SignedBlock_encode_with_tx", block::encoder::signed_block_encode_with_tx)?;
    cx.export_function("SignedBlock_decode_with_tx", block::decoder::signed_block_decode_with_tx)?;

    cx.export_class::<decoder::JsRpcCodec>("Net_RpcDecoder")?;
    cx.export_function("Net_rpc_encoder", encoder::encode)?;

    Ok(())
});
