use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::{ExitCondition, Window},
};
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;

pub fn bevy_learn_run(s: String) {
    App::new()
        .add_plugins(DefaultPlugins.set(get_diy_window_plugin(s)))
        .run();
}

fn get_diy_window(id_selector: String) -> Window {
    Window {
        title: "App".to_owned(),
        cursor: Default::default(),
        present_mode: Default::default(),
        mode: Default::default(),
        position: Default::default(),
        resolution: Default::default(),
        internal: Default::default(),
        composite_alpha_mode: Default::default(),
        resize_constraints: Default::default(),
        ime_enabled: Default::default(),
        ime_position: Default::default(),
        resizable: true,
        enabled_buttons: Default::default(),
        decorations: true,
        transparent: false,
        focused: true,
        window_level: Default::default(),
        fit_canvas_to_parent: false,
        prevent_default_event_handling: true,
        canvas: Some(id_selector),
        window_theme: None,
        visible: true,
    }
}

fn get_diy_window_plugin(s: String) -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(get_diy_window(s)),
        exit_condition: ExitCondition::OnAllClosed,
        close_when_requested: true,
    }
}
