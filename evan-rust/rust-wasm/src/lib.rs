use wasm_bindgen::prelude::*;
use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

// JS alert import (same as yours)
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// 1. String manipulation
#[wasm_bindgen]
pub fn greet_name(name: &str) -> String {
    format!("Hello, {} from Rust+WASM!", name)
}

// 2. Array sum (JS array -> Rust slice)
#[wasm_bindgen]
pub fn sum_array(arr: &[f64]) -> f64 {
    arr.iter().sum()
}

// 3. JS object input/output
#[wasm_bindgen]
pub extern "C" fn create_person(name: &str, age: u32) -> JsValue {
    let person = js_sys::Object::new();
    js_sys::Reflect::set(&person, &"name".into(), &name.into()).unwrap();
    js_sys::Reflect::set(&person, &"age".into(), &age.into()).unwrap();
    person.into()
}

// 4. Return JS array
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> Vec<u64> {
    let mut fib = vec![0, 1];
    for i in 2..=n as usize {
        fib.push(fib[i-1] + fib[i-2]);
    }
    fib
}

// 5. Panic handling (safe for browser)
#[wasm_bindgen]
pub fn safe_divide(a: f64, b: f64) -> Result<f64, JsValue> {
    if b == 0.0 {
        Err(JsValue::from_str("Division by zero!"))
    } else {
        Ok(a / b)
    }
}

// 6. Mutable buffer (for performance)
#[wasm_bindgen]
pub fn double_buffer(buffer: &mut [u8]) {
    for i in 0..buffer.len() {
        buffer[i] = buffer[i].wrapping_mul(2);
    }
}

// More tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_greet() {
        assert_eq!(greet_name("Evan"), "Hello, Evan from Rust+WASM!");
    }
    
    #[test]
    fn test_sum() {
        assert_eq!(sum_array(&[1.0, 2.0, 3.0]), 6.0);
    }
    
    #[test]
    fn test_fib() {
        assert_eq!(fibonacci(5), vec![0, 1, 1, 2, 3, 5]);
    }
}
