#[macro_use] extern crate neon;
extern crate godcoin;

mod asset;
use asset::*;

register_module!(mut cx, {
    cx.export_class::<JsAsset>("Asset")?;
    cx.export_function("Asset_from_string", asset_from_string)?;

    Ok(())
});
