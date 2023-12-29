//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlCanvasElement};

mod bevy_learn;
mod utils;

pub use crate::utils::set_panic_hook;

#[wasm_bindgen]
pub fn init_warp(id: &str, class_name: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    // 获取要附加到的DOM元素
    let target_element = document
        .get_element_by_id(id)
        .expect("Failed to get element")
        .dyn_into::<Element>()?;

    // 创建一个新的div元素
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?; // 转换为HtmlDivElement类型

    // 设置id
    let uuid = Uuid::new_v4().to_string();
    canvas.set_id(format!("{}-{}", "bevy-canvas", uuid.as_str()).as_str());

    // 设置div的class属性
    canvas.set_class_name(class_name);

    // 设置canvas·
    // let rect = target_element.get_bounding_client_rect();
    // canvas.set_width(rect.width() as u32);
    // canvas.set_height(rect.height() as u32);

    // 将div元素附加到目标元素
    target_element
        .dyn_ref::<Element>()
        .unwrap()
        .append_child(&canvas)?;

    let id_selector = format!("#{}", canvas.id());
    println!("id_selector:{}", id_selector);
    bevy_learn::bevy_learn_run(id_selector);

    Ok(())
}
