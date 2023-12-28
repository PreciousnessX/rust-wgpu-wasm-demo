use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;

pub fn bevy_learn_run() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}

fn hello_world() {
    println!("hello word!");
}

// 定义 Component
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("张三".to_string())));
    commands.spawn((Person, Name("李四".to_string())));
    commands.spawn((Person, Name("王五".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.0)
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, greet_people));
    }
}
