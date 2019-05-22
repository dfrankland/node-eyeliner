use neon::{
    context::Context,
    handle::Handle,
    prelude::*,
    result::Throw,
    types::{JsArray, JsNull, JsObject, JsUndefined, JsValue},
};
use std::collections::HashMap;

pub fn is_null_or_undefined<'a, U: Value + 'a>(handle: Handle<'a, U>) -> bool {
    handle.is_a::<JsNull>() || handle.is_a::<JsUndefined>()
}

pub fn get_bool<'a, C: Context<'a>, E: From<Throw>>(
    cx: &mut C,
    obj: Handle<'a, JsObject>,
    property: &str,
) -> Result<Option<bool>, E> {
    let property_value = obj.get(cx, property)?;

    let value = if is_null_or_undefined(property_value) {
        None
    } else {
        Some(
            property_value
                .downcast_or_throw::<JsBoolean, C>(cx)?
                .value(),
        )
    };

    Ok(value)
}

fn js_string_to_string<'a, C: Context<'a>, E: From<Throw>>(
    cx: &mut C,
    js_string: &Handle<JsValue>,
) -> Result<String, E> {
    Ok(js_string.downcast_or_throw::<JsString, C>(cx)?.value())
}

fn js_array_to_vec_of_str<'a, C: Context<'a>, E: From<Throw>>(
    cx: &mut C,
    arr: Handle<'a, JsArray>,
) -> Result<Vec<String>, E> {
    let mut vec_of_strings = vec![];
    for js_string in arr.to_vec(cx)?.iter() {
        vec_of_strings.push(js_string_to_string(cx, js_string)?);
    }
    Ok(vec_of_strings)
}

pub fn get_vec_of_strings<'a, C: Context<'a>, E: From<Throw>>(
    cx: &mut C,
    obj: Handle<'a, JsObject>,
    property: &str,
) -> Result<Option<Vec<String>>, E> {
    let property_value = obj.get(cx, property)?;

    if is_null_or_undefined(property_value) {
        return Ok(None);
    }

    let js_array = property_value.downcast_or_throw::<JsArray, C>(cx)?;

    let value = js_array_to_vec_of_str(cx, js_array)?;

    Ok(Some(value))
}

pub fn get_hashmap_of_strings<'a, C: Context<'a>, E: From<Throw>>(
    cx: &mut C,
    obj: Handle<'a, JsObject>,
    property: &str,
) -> Result<Option<HashMap<String, String>>, E> {
    let property_value = obj.get(cx, property)?;

    if is_null_or_undefined(property_value) {
        return Ok(None);
    }

    let js_object = property_value.downcast_or_throw::<JsObject, C>(cx)?;

    let js_array = js_object.get_own_property_names(cx)?;
    let value_keys = js_array_to_vec_of_str(cx, js_array)?;

    let mut value = HashMap::new();

    for value_property in value_keys.iter() {
        let js_string = js_object.get(cx, value_property.as_ref())?;
        let string = js_string_to_string(cx, &js_string)?;
        value.insert(value_property.clone(), string);
    }

    Ok(Some(value))
}
