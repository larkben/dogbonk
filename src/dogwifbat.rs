// dogwifbat.rs
use bevy::prelude::*;

// Constants
const PLAYER_SPEED: f32 = 6.0;

// Components
#[derive(Component)]
pub struct Animations {
    moveleft: MovementAnimation,
    moveright: MovementAnimation,
    moveback: MovementAnimation,
    moveforward: MovementAnimation,
}

#[derive(Component)]
pub struct MovementAnimation {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

// Systems
pub fn move_and_animate_dogwifbat(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Animations, &mut AnimationTimer, &mut TextureAtlas, &mut Transform)>,
    time: Res<Time>,
) {
    for (indices, mut timer, mut atlas, mut transform) in &mut query {
        // Check if the W key is pressed (you can adjust this for other keys)
        if input.pressed(KeyCode::KeyW) {
            // Update sprite position (similar to your move_dogwifbat logic)
            let mut direction = Vec3::ZERO;
            
            direction.y += 50.0;

            // Normalize the direction to prevent faster diagonal movement
            if direction.length_squared() > 1.0 {
                direction = direction.normalize();
            }

            transform.translation += direction * PLAYER_SPEED;

            // Update animation index (similar to your animate_sprite logic)
            timer.tick(time.delta());
            if timer.just_finished() {
                atlas.index = if atlas.index == indices.moveforward.last{
                    indices.moveforward.first
                } else {
                    if atlas.index < 8 || atlas.index > 11 {
                        atlas.index = 8;
                    }
                    atlas.index + 1
                };
            }
        }
        if input.pressed(KeyCode::KeyS) {
            // Update sprite position (similar to your move_dogwifbat logic)
            let mut direction = Vec3::ZERO;
        
            direction.y -= 50.0;

            // Normalize the direction to prevent faster diagonal movement
            if direction.length_squared() > 1.0 {
                direction = direction.normalize();
            }

            transform.translation += direction * PLAYER_SPEED;

            // Update animation index (similar to your animate_sprite logic)
            timer.tick(time.delta());
            if timer.just_finished() {
                atlas.index = if atlas.index == indices.moveback.last{
                    indices.moveback.first
                } else {
                    if atlas.index > 3 {
                        atlas.index = 0;
                    }
                    atlas.index + 1
                };
            }
        }
        if input.pressed(KeyCode::KeyA) {
            // Update sprite position (similar to your move_dogwifbat logic)
            let mut direction = Vec3::ZERO;
            // ... (your movement logic)
            direction.x -= 50.0;

            // Normalize the direction to prevent faster diagonal movement
            if direction.length_squared() > 1.0 {
                direction = direction.normalize();
            }

            transform.translation += direction * PLAYER_SPEED;

            // Update animation index (similar to your animate_sprite logic)
            timer.tick(time.delta());
            if timer.just_finished() {
                atlas.index = if atlas.index == indices.moveleft.last{
                    indices.moveleft.first
                } else {
                    if atlas.index < 12 || atlas.index > 15 {
                        atlas.index = 12;
                    }
                    atlas.index + 1
                };
            }
        }
        if input.pressed(KeyCode::KeyD) {
            // Update sprite position (similar to your move_dogwifbat logic)
            let mut direction = Vec3::ZERO;
            // ... (your movement logic)
            direction.x += 50.0;

            // Normalize the direction to prevent faster diagonal movement
            if direction.length_squared() > 1.0 {
                direction = direction.normalize();
            }

            transform.translation += direction * PLAYER_SPEED;

            // Update animation index (similar to your animate_sprite logic)
            timer.tick(time.delta());
            if timer.just_finished() {
                atlas.index = if atlas.index == indices.moveright.last{
                    indices.moveright.first
                } else {
                    if atlas.index < 4 || atlas.index > 6 {
                        atlas.index = 4;
                    }
                    atlas.index + 1
                };
            }
        } 
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // DOGWIFBAT
    let texture = asset_server.load("../assets/bonkdog.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(130.0, 130.0), 4, 9, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Use only the subset of sprites in the sheet that make up the run animation
    // DOGWIFBAT ANIMATION
    let movementleft = MovementAnimation { first: 12, last: 15 };
    let movementright = MovementAnimation { first: 4, last: 7 };
    let movementforward = MovementAnimation { first: 8, last: 11 };
    let movementback = MovementAnimation { first: 0, last: 3 };

    let animation_indices = Animations { moveleft: movementleft, moveright: movementright, moveback: movementback, moveforward: movementforward };

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.moveback.first,
            },
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}