use bevy::prelude::*;
use rand::*;

mod dogwifbat;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // Modfiy Bakground Color
        .insert_resource(ClearColor(Color::rgb(0.3, 0.7, 0.1)))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, spawn_grass_grid)
        .add_systems(Startup, dogwifbat::setup)
        .add_systems(FixedUpdate, dogwifbat::move_and_animate_dogwifbat)
        .run()
}

// Define the grass tileset layout (adjust as needed)
struct GrassTileSet {
    first: usize,
    last: usize,
}

// Function to spawn grass entities
fn spawn_grass_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load your grass texture and create a layout
    let grasstexture: Handle<Image> = asset_server.load("../assets/grasstileset.png");
    let grasslayout: TextureAtlasLayout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 16, 16, None, None);
    let texture_atlas_layout_grass = texture_atlas_layouts.add(grasslayout);

    // Define the range of grass tiles in your tileset
    let animation_indices_grass = GrassTileSet { first: 0, last: 6 };

    // Set grid dimensions (adjust as needed)
    let grid_width = 100;
    let grid_height = 100;

    // Calculate the center of the grid
    let center_x = 30.0;
    let center_y = 30.0;

    // Initialize the random number generator
    let mut rng = rand::thread_rng();

    // Spawn grass entities in a grid
    for x in 0..grid_width {
        for y in 0..grid_height {
            // Calculate the position based on grid coordinates and center
            let position = Vec3::new((x as f32 - center_x) * 25.0, (y as f32 - center_y) * 25.0, -1.0); // Set Z-coordinate to -1.0

            // Get a random index within the specified range
            let random_index = rng.gen_range(animation_indices_grass.first..=animation_indices_grass.last);

            // Spawn the grass entity with the random index
            commands.spawn(SpriteSheetBundle {
                texture: grasstexture.clone(),
                atlas: TextureAtlas {
                    layout: texture_atlas_layout_grass.clone(),
                    index: random_index,
                },
                transform: Transform::from_translation(position),
                ..Default::default()
            });
        }
    }
}