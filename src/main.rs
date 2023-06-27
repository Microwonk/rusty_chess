use bevy::{prelude::*, window::{Cursor, PresentMode}, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
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
        .add_startup_system(create_board)
        .run();
}
