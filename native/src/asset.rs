use godcoin::{Asset, AssetSymbol};
use neon::prelude::*;

macro_rules! asset_to_js {
    ($cx:expr, $asset:ident) => {
        {
            let amount = $cx.number($asset.amount as f64);
            let decimals = $cx.number($asset.decimals);
            let symbol = match $asset.symbol {
                AssetSymbol::GOLD => $cx.number(0),
                AssetSymbol::SILVER => $cx.number(1)
            };
            JsAsset::new(&mut $cx, vec![amount, decimals, symbol])?
        }
    }
}

macro_rules! asset_arithmetic {
    ($cx:expr, $op:ident, $param:expr) => {
        match {
            let other: JsAsset = *$cx.argument::<JsValue>(0)?.downcast_or_throw(&mut $cx)?;
            let this = $cx.this();
            let guard = $cx.lock();
            let asset = this.borrow(&guard);
            let other = other.borrow(&guard);
            asset.$op(&other, $param)
        } {
            Some(asset) => {
                Ok(asset_to_js!($cx, asset).upcast())
            },
            None => Ok(JsUndefined::new().upcast())
        }
    };
    ($cx:expr, $op:ident) => {
        match {
            let other: JsAsset = *$cx.argument::<JsValue>(0)?.downcast_or_throw(&mut $cx)?;
            let this = $cx.this();
            let guard = $cx.lock();
            let asset = this.borrow(&guard);
            let other = other.borrow(&guard);
            asset.$op(&other)
        } {
            Some(asset) => {
                Ok(asset_to_js!($cx, asset).upcast())
            },
            None => Ok(JsUndefined::new().upcast())
        }
    }
}

macro_rules! asset_cmp {
    ($cx:expr, $op:ident) => {
        match {
            let other: JsAsset = *$cx.argument::<JsValue>(0)?.downcast_or_throw(&mut $cx)?;
            let this = $cx.this();
            let guard = $cx.lock();
            let asset = this.borrow(&guard);
            let other = other.borrow(&guard);
            asset.$op(&other)
        } {
            Some(cmp) => {
                Ok(JsBoolean::new(&mut $cx, cmp).upcast())
            },
            None => $cx.throw_error("asset symbol mismatch")
        }
    };
}

declare_types! {
    pub class JsAsset for Asset {
        init(mut cx) {
            let amt = cx.argument::<JsNumber>(0)?.value() as i64;
            let dec = cx.argument::<JsNumber>(1)?.value() as u8;
            if dec > 8 { return cx.throw_error("precision too high") }
            let sym = match cx.argument::<JsNumber>(2)?.value() as u8 {
                0 => AssetSymbol::GOLD,
                1 => AssetSymbol::SILVER,
                _ => return cx.throw_error("invalid symbol identifier")
            };
            Ok(Asset {
                amount: amt,
                decimals: dec,
                symbol: sym
            })
        }

        method add(mut cx) {
            asset_arithmetic!(cx, add)
        }

        method sub(mut cx) {
            asset_arithmetic!(cx, sub)
        }

        method mul(mut cx) {
            let prec = cx.argument::<JsNumber>(1)?.value() as u8;
            asset_arithmetic!(cx, mul, prec)
        }

        method div(mut cx) {
            let prec = cx.argument::<JsNumber>(1)?.value() as u8;
            asset_arithmetic!(cx, div, prec)
        }

        method pow(mut cx) {
            match {
                let pow = cx.argument::<JsNumber>(0)?.value() as u16;
                let prec = cx.argument::<JsNumber>(1)?.value() as u8;
                let this = cx.this();
                let guard = cx.lock();
                let asset = this.borrow(&guard);
                asset.pow(pow, prec)
            } {
                Some(asset) => {
                    Ok(asset_to_js!(cx, asset).upcast())
                },
                None => Ok(JsUndefined::new().upcast())
            }
        }

        method gt(mut cx) {
            asset_cmp!(cx, gt)
        }

        method geq(mut cx) {
            asset_cmp!(cx, geq)
        }

        method lt(mut cx) {
            asset_cmp!(cx, lt)
        }

        method leq(mut cx) {
            asset_cmp!(cx, leq)
        }

        method eq(mut cx) {
            asset_cmp!(cx, eq)
        }

        method to_string(mut cx) {
            let s = {
                let this = cx.this();
                let guard = cx.lock();
                let asset = this.borrow(&guard);
                asset.to_str()
            };
            Ok(JsString::new(&mut cx, s).upcast())
        }
    }
}

pub fn asset_from_string(mut cx: FunctionContext) -> JsResult<JsAsset> {
    use std::str::FromStr;
    use std::error::Error;

    let s = cx.argument::<JsString>(0)?.value();
    match Asset::from_str(&s) {
        Ok(asset) => { Ok(asset_to_js!(cx, asset)) },
        Err(e) => cx.throw_error(e.description())
    }
}
