use eyeliner::Options;

use neon::vm::Throw;
use neon::scope::Scope;
use neon::mem::Handle;
use neon::js::JsObject;

use js_object_utils::{get_bool, get_vec_of_strings};

pub fn js_options_object_to_rust_options_struct<'a, S: Scope<'a>, E: From<Throw>>(scope: &mut S, options: Handle<'a, JsObject>) -> Result<Options, E> {
    let eyeliner_options = Options {
        apply_table_element_attributes: get_bool(scope, options, "applyTableElementAttributes")?,
        apply_height_attributes: get_bool(scope, options, "applyHeightAttributes")?,
        apply_style_tags: get_bool(scope, options, "applyStyleTags")?,
        apply_width_attributes: get_bool(scope, options, "applyWidthAttributes")?,
        insert_preserved_css: get_vec_of_strings(scope, options, "insertPreservedCss")?,
        preserve_font_faces: get_bool(scope, options, "preserveFontFaces")?,
        preserve_important: get_bool(scope, options, "preserveImportant")?,
        preserve_media_queries: get_bool(scope, options, "preserveMediaQueries")?,
        remove_style_tags: get_bool(scope, options, "removeStyleTags")?,
    };

    Ok(eyeliner_options)
}
