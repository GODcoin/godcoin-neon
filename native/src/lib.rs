#[macro_use] extern crate neon;
extern crate godcoin;

use neon::prelude::*;

mod asset;
use asset::*;

fn init(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    if godcoin::init().is_ok() { Ok(JsUndefined::new()) }
    else { cx.throw_error("failed to initialize library") }
}

register_module!(mut cx, {
    cx.export_function("init", init)?;

    cx.export_class::<JsAsset>("Asset")?;
    cx.export_function("Asset_from_string", asset_from_string)?;

    Ok(())
});
