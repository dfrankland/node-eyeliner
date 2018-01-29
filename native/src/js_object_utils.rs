use std::collections::HashMap;

use neon::vm::Throw;
use neon::scope::Scope;
use neon::mem::Handle;
use neon::js::{JsObject, JsArray};
use neon::js::Object;
use neon::js::Variant as JsType;

pub fn get_bool<'a, S: Scope<'a>, E: From<Throw>>(scope: &mut S, obj: Handle<'a, JsObject>, property: &str) -> Result<Option<bool>, E> {
    Ok(
        match obj.get(scope, property)?.variant() {
            JsType::Boolean(boolean) => Some(boolean.value()),
            JsType::Null(_) | JsType::Undefined(_) => None,
            _ => panic!(format!("`{}` is not a boolean.", property)),
        }
    )
}

fn js_string_to_string(js_string: &JsType, err: String) -> String {
    match *js_string {
        JsType::String(string) => string.value(),
        _ => panic!(err),
    }
}

fn js_array_to_vec_of_str<'a, S: Scope<'a>, E: From<Throw>>(scope: &mut S, arr: Handle<'a, JsArray>, property: &str) -> Result<Vec<String>, E> {
    let js_vec = arr.to_vec(scope)?;

    let vec_of_strings: Vec<_> = js_vec.iter().enumerate().map(
        move |(index, js_value)| {
            let js_string = js_value.variant();
            let err = format!("`{}` contains a value that is not a string at index {}.", property, index);
            js_string_to_string(&js_string, err)
        }
    ).collect();

    Ok(vec_of_strings)
}

pub fn get_vec_of_strings<'a, S: Scope<'a>, E: From<Throw>>(scope: &mut S, obj: Handle<'a, JsObject>, property: &str) -> Result<Option<Vec<String>>, E> {
    let js_value = obj.get(scope, property)?;

    Ok(
        match js_value.variant() {
            JsType::Array(array) => Some(js_array_to_vec_of_str(scope, array, property)?),
            JsType::Null(_) | JsType::Undefined(_) => None,
            _ => panic!(format!("`{}` is not an array.", property)),
        }
    )
}

pub fn get_hashmap_of_strings<'a, S: Scope<'a>, E: From<Throw>>(scope: &mut S, obj: Handle<'a, JsObject>, property: &str) -> Result<Option<HashMap<String, String>>, E> {
    let js_value = obj.get(scope, property)?;

    let js_object = match js_value.variant() {
        JsType::Object(object) => object,
        JsType::Null(_) | JsType::Undefined(_) => return Ok(None),
        _ => panic!(format!("`{}` is not an object.", property)),
    };

    let js_array = js_object.get_own_property_names(scope)?;

    let object_properties = js_array_to_vec_of_str(
        scope,
        js_array,
        &format!("Object.keys({})", property),
    )?;

    let object_entries = object_properties.iter().fold(
        HashMap::new(),
        |mut hashmap, key| {
            let js_string = js_object.get(scope, key.as_ref()).unwrap();
            let err = format!("`{}` property in {} is not a string.", key, property);
            let value = js_string_to_string(&js_string.variant(), err);
            hashmap.insert(key.clone(), value);
            hashmap
        }
    );

    Ok(Some(object_entries))
}
