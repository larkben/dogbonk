use bevy::prelude::*;
use rand::*;

mod dogwifbat;
mod tilegeneration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // Modfiy Bakground Color
        .insert_resource(ClearColor(Color::rgb(0.3, 0.7, 0.1)))                 // green (grassy) background color
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, tilegeneration::spawn_grass_grid)
        .add_systems(Startup, dogwifbat::setup)
        .add_systems(FixedUpdate, dogwifbat::move_and_animate_dogwifbat)
        .run()
}