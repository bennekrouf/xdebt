use serde_json::Value;

pub fn remove_null_values(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.retain(|_, v| !v.is_null());
            for v in map.values_mut() {
                remove_null_values(v);
            }
        }
        Value::Array(arr) => {
            arr.retain(|v| !v.is_null());
            for v in arr.iter_mut() {
                remove_null_values(v);
            }
        }
        _ => {}
    }
}


