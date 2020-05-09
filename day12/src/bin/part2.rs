fn main() {
    let json = aoc::parser::read_file_from_args(1);

    let object: serde_json::Value = serde_json::from_str(&json).unwrap();
    let mut total = 0;
    apply(&object, &mut |value: &serde_json::Value| {
        if let Some(i) = value.as_i64() {
            total += i;
        } else if let Some(n) = value.as_u64() {
            total += n as i64;
        } else if let Some(f) = value.as_f64() {
            total += f as i64;
        }
    });

    println!("{}", total);
}

fn apply(value: &serde_json::Value, fun: &mut impl FnMut(&serde_json::Value)) {
    match value {
        serde_json::Value::Array(arr) => {
            for v in arr.iter() {
                apply(v, fun);
            }
        }
        serde_json::Value::Object(obj) => {
            if let Some(_) = obj
                .values()
                .find(|el| **el == serde_json::Value::String("red".into()))
            {
                return;
            }
            for v in obj.values() {
                apply(v, fun);
            }
        }
        value => fun(value),
    }
}
