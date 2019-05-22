mod js_object_utils;
mod options;
mod settings;

use crate::{
    js_object_utils::is_null_or_undefined, options::js_options_object_to_rust_options_struct,
    settings::js_settings_object_to_rust_settings_struct,
};
use eyeliner::inline;
use neon::{
    context::FunctionContext,
    prelude::*,
    register_module,
    types::{JsBoolean, JsObject, JsString},
};

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn default(mut cx: FunctionContext) -> JsResult<JsString> {
    let config = cx
        .argument::<JsObject>(0)?
        .downcast_or_throw::<JsObject, FunctionContext>(&mut cx)?;
    let html = config
        .get(&mut cx, "html")?
        .downcast_or_throw::<JsString, FunctionContext>(&mut cx)?
        .value();
    let css = {
        let css_property = config.get(&mut cx, "css")?;
        if is_null_or_undefined(css_property) {
            None
        } else {
            Some(
                css_property
                    .downcast_or_throw::<JsString, FunctionContext>(&mut cx)?
                    .value(),
            )
        }
    };
    let options = {
        let options_property = config.get(&mut cx, "options")?;
        if is_null_or_undefined(options_property) {
            None
        } else {
            let js_object =
                options_property.downcast_or_throw::<JsObject, FunctionContext>(&mut cx)?;
            Some(js_options_object_to_rust_options_struct(
                &mut cx, js_object,
            )?)
        }
    };
    let settings = {
        let settings_property = config.get(&mut cx, "settings")?;
        if is_null_or_undefined(settings_property) {
            None
        } else {
            let js_object =
                settings_property.downcast_or_throw::<JsObject, FunctionContext>(&mut cx)?;
            Some(js_settings_object_to_rust_settings_struct(
                &mut cx, js_object,
            )?)
        }
    };

    let result = inline(&html, css, options, settings);

    Ok(cx.string(&result))
}

register_module!(mut module, {
    // Register as an ES Module
    let es_module = JsObject::new(&mut module);

    let js_boolean = JsBoolean::new(&mut module, true);
    es_module
        .set(&mut module, "value", js_boolean)
        .expect("Can't create `__esModule` object.");

    module
        .exports_object()
        .expect("Can't get exports object.")
        .set(&mut module, "__esModule", es_module)
        .expect("Can't add `__esModule` object to exports.");

    // Export default function
    module
        .export_function("default", default)
        .expect("Can't export `default` function.");

    Ok(())
});
