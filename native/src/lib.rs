#[macro_use] extern crate neon;
extern crate sodiumoxide;
extern crate godcoin;

use neon::prelude::*;

mod asset;
use asset::*;

mod crypto;
use crypto::*;

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

    Ok(())
});
