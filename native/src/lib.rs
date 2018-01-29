#[macro_use] extern crate neon;
extern crate eyeliner;

use neon::vm::{Call, JsResult};
use neon::mem::Managed;
use neon::js::{JsObject, JsBoolean, JsString};
use neon::js::Object;
use neon::js::Variant as JsType;

use eyeliner::inline;

mod js_object_utils;
mod options;
mod settings;

use options::js_options_object_to_rust_options_struct;
use settings::js_settings_object_to_rust_settings_struct;

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn default(call: Call) -> JsResult<JsString> {
    let config = JsObject::from_raw(
        call.arguments.get(call.scope, 0).expect("Couldn't get options.").to_raw()
    );

    let html = match config.get(call.scope, "html")?.variant() {
        JsType::String(html_string) => html_string.value(),
        _ => panic!("`html` is not a string."),
    };

    let css = match config.get(call.scope, "css")?.variant() {
        JsType::String(css_string) => Some(css_string.value()),
        JsType::Null(_) | JsType::Undefined(_) => None,
        _ => panic!("`css` is not a string."),
    };

    let options = match config.get(call.scope, "options")?.variant() {
        JsType::Object(options_object) => Some(
            js_options_object_to_rust_options_struct(call.scope, options_object)?
        ),
        JsType::Null(_) | JsType::Undefined(_) => None,
        _ => panic!("`html` is not a string."),
    };

    let settings = match config.get(call.scope, "settings")?.variant() {
        JsType::Object(settings_object) => Some(
            js_settings_object_to_rust_settings_struct(call.scope, settings_object)?
        ),
        JsType::Null(_) | JsType::Undefined(_) => None,
        _ => panic!("`html` is not a string."),
    };

    let result = inline(&html, css, options, settings);

    JsString::new_or_throw(call.scope, &result)
}

register_module!(
    module,
    {
        // Register as an ES Module
        let es_module = JsObject::new(module.scope);

        es_module.set(
            "value",
            JsBoolean::new(module.scope, true),
        ).expect(
            "Can't create `__esModule` object."
        );

        module.exports.set(
            "__esModule",
            es_module,
        ).expect(
            "Can't add `__esModule` object to exports."
        );

        // Export default function
        module.export(
            "default",
            default
        ).expect(
            "Can't export `default` function."
        );

        Ok(())
    }
);
