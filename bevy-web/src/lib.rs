//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlDivElement};

mod bevy_learn;
mod shaps;
mod utils;

pub use crate::{bevy_learn::*, shaps::run, utils::set_panic_hook};

#[wasm_bindgen]
pub fn init_warp(id: &str, class_name: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    // 获取要附加到的DOM元素
    let target_element = document.get_element_by_id(id).unwrap();

    // 创建一个新的div元素
    let div = document
        .create_element("div")?
        .dyn_into::<HtmlDivElement>()?;

    // 设置div的class属性
    div.set_class_name(class_name);

    // 将div元素附加到目标元素
    target_element
        .dyn_ref::<Element>()
        .unwrap()
        .append_child(&div)?;

    Ok(())
}
