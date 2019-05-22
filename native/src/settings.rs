use crate::js_object_utils::{get_hashmap_of_strings, get_vec_of_strings};
use eyeliner::AbstractSettings;
use neon::{context::Context, handle::Handle, result::Throw, types::JsObject};

pub fn js_settings_object_to_rust_settings_struct<'a, C: Context<'a>, E: From<Throw>>(
    cx: &mut C,
    settings: Handle<'a, JsObject>,
) -> Result<AbstractSettings, E> {
    let eyeliner_settings = AbstractSettings {
        width_elements: get_vec_of_strings(cx, settings, "widthElements")?,
        height_elements: get_vec_of_strings(cx, settings, "heightElements")?,
        style_to_attribute: get_hashmap_of_strings(cx, settings, "styleToAttribute")?,
        table_elements: get_vec_of_strings(cx, settings, "tableElements")?,
        non_visual_elements: get_vec_of_strings(cx, settings, "nonVisualElements")?,
        excluded_properties: get_vec_of_strings(cx, settings, "excludedProperties")?,
    };

    Ok(eyeliner_settings)
}
