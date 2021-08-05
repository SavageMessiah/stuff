use anyhow::Result;
use serde_json::{from_str, Map, Value};

fn has_red(m: &Map<String, Value>) -> bool {
    m.values().any(|s| s == "red")
}

fn sum_nums(json: &Value) -> f64 {
    use serde_json::Value::*;
    match json {
        Number(n) => n.as_f64().expect("weird number"),
        Array(vals) => vals.iter().map(sum_nums).sum(),
        Object(m) if !has_red(m) => m.values().map(sum_nums).sum(),
        _ => 0f64
    }
}

#[test]
fn test_sum_nums() {
    use serde_json::json;
    assert_eq!(sum_nums(&json!([1,{"c":"red","b":2},3])), 4f64);
}

fn main() -> Result<()> {
    let json = from_str(include_str!("input.txt"))?;
    println!("sum: {}", sum_nums(&json));

    Ok(())
}
