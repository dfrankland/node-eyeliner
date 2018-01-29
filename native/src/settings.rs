use eyeliner::Settings;

use neon::vm::Throw;
use neon::scope::Scope;
use neon::mem::Handle;
use neon::js::JsObject;

use js_object_utils::{get_vec_of_strings, get_hashmap_of_strings};

pub fn js_settings_object_to_rust_settings_struct<'a, S: Scope<'a>, E: From<Throw>>(scope: &mut S, settings: Handle<'a, JsObject>) -> Result<Settings, E> {
    let eyeliner_settings = Settings {
        width_elements: get_vec_of_strings(scope, settings, "widthElements")?,
        height_elements: get_vec_of_strings(scope, settings, "heightElements")?,
        style_to_attribute: get_hashmap_of_strings(scope, settings, "styleToAttribute")?,
        table_elements: get_vec_of_strings(scope, settings, "tableElements")?,
        non_visual_elements: get_vec_of_strings(scope, settings, "nonVisualElements")?,
        excluded_properties: get_vec_of_strings(scope, settings, "excludedProperties")?,
    };

    Ok(eyeliner_settings)
}
