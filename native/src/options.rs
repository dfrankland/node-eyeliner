use crate::js_object_utils::{get_bool, get_vec_of_strings};
use eyeliner::AbstractOptions;
use neon::{context::Context, handle::Handle, result::Throw, types::JsObject};

pub fn js_options_object_to_rust_options_struct<'a, C: Context<'a>, E: From<Throw>>(
    cx: &mut C,
    options: Handle<'a, JsObject>,
) -> Result<AbstractOptions, E> {
    let eyeliner_options = AbstractOptions {
        apply_table_element_attributes: get_bool(cx, options, "applyTableElementAttributes")?,
        apply_height_attributes: get_bool(cx, options, "applyHeightAttributes")?,
        apply_style_tags: get_bool(cx, options, "applyStyleTags")?,
        apply_width_attributes: get_bool(cx, options, "applyWidthAttributes")?,
        insert_preserved_css: get_vec_of_strings(cx, options, "insertPreservedCss")?,
        preserve_font_faces: get_bool(cx, options, "preserveFontFaces")?,
        preserve_important: get_bool(cx, options, "preserveImportant")?,
        preserve_media_queries: get_bool(cx, options, "preserveMediaQueries")?,
        remove_style_tags: get_bool(cx, options, "removeStyleTags")?,
    };

    Ok(eyeliner_options)
}
