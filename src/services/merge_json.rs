use serde_json::Value;

pub fn merge(a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Object(mut a), Value::Object(b)) => {
            for (k, v) in b {
                let entry = a.entry(k).or_insert(Value::Null);
                *entry = merge(entry.clone(), v);
            }
            Value::Object(a)
        }
        (_, b) => b,
    }
}