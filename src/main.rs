use bevy::{prelude::*, window::{PresentMode}};
use consts::WINDOW_SIZE;
use gui::*;

mod gui;
mod consts;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Rusty Chess".into(),
            resolution: (WINDOW_SIZE, WINDOW_SIZE).into(),
            resizable: false,
            present_mode: PresentMode::AutoVsync,
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            transparent: true,
            ..default()
        }),
        ..default()
    }))
        .add_startup_system(setup)
        .add_startup_system(create)
        .run();
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
}
