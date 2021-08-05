use anyhow::Result;
use serde_json::{from_str, Value};

fn sum_nums(json: &Value) -> f64 {
    use serde_json::Value::*;
    match json {
        Number(n) => n.as_f64().expect("weird number"),
        Array(vals) => vals.iter().map(sum_nums).sum(),
        Value::Object(m) => m.values().map(sum_nums).sum(),
        _ => 0f64
    }
}

fn main() -> Result<()> {
    let json = from_str(include_str!("input.txt"))?;
    println!("sum: {}", sum_nums(&json));

    Ok(())
}
