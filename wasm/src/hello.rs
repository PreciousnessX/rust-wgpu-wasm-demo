use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &JsValue);
}
#[wasm_bindgen]
pub fn greet2(s: &str) {
    let message = format!("你好 {}", s);
    alert(&JsValue::from_str(&message));
}
