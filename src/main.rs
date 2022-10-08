use bevy::prelude::*;

const WIN_WIDTH: f32 = 800.0;
const WIN_HEIGHT: f32 = 500.0;

const GRAVITY: f32 = 10.0;
const JUMP_ACCEL: f32 = -500.0;

const BLOCK_WIDTH: f32 = 70.0;
const BLOCK_SPEED: f32 = 256.0;
const BLOCK_GAP: f32 = 140.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Crab Game".to_string(),
            width: WIN_WIDTH,
            height: WIN_HEIGHT,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_sprites)
        .add_system(start_gravity)
        .add_system(start_block_move)
        .run();
}

#[derive(Component)]
struct Crab;

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Velocity {
    speed: f32,
}

fn add_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    // crab entity (player)
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(-(WIN_WIDTH / 2.0) + 100.0, 0.0, 0.0),
            texture: asset_server.load("crab.png"),
            ..default()
        })
        .insert(Crab)
        .insert(Velocity { speed: 0.0 });

    // upper block entity (obstacle)
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: (Color::RED),
                custom_size: Some(Vec2::new(BLOCK_WIDTH, WIN_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(
                WIN_WIDTH / 2.0,
                // to get the top of the block lined up with the top of the window
                (WIN_HEIGHT / 2.0) + (BLOCK_GAP / 2.0),
                0.0,
            ),
            ..default()
        })
        .insert(Block)
        .insert(Velocity { speed: 0.0 });

    // lower block entity (obstacle)

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: (Color::RED),
                custom_size: Some(Vec2::new(BLOCK_WIDTH, WIN_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(
                WIN_WIDTH / 2.0,
                // to get the top of the block lined up with the top of the window
                -WIN_HEIGHT / 2.0 - BLOCK_GAP / 2.0,
                0.0,
            ),
            ..default()
        })
        .insert(Block)
        .insert(Velocity { speed: 0.0 });
}

/*
enum GameStatus {
    Menu,
    Play,
    Over,
}
*/

fn start_gravity(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Crab>>,
) {
    for (mut transform, mut vel) in query.iter_mut() {
        vel.speed += GRAVITY;
        if keys.just_pressed(KeyCode::Space) {
            vel.speed = JUMP_ACCEL;
        }
        transform.translation.y -= vel.speed * time.delta_seconds();
        // Put in limits to prevent going out of bounds.
        // Later, implement a loss when hitting the limit
        // Maybe only make the bottom bound a loss, upper bound can stay
        if transform.translation.y > WIN_HEIGHT / 2.0 {
            transform.translation.y = WIN_HEIGHT / 2.0;
        }
        if transform.translation.y < -WIN_HEIGHT / 2.0 {
            transform.translation.y = -WIN_HEIGHT / 2.0;
        }
    }
}

fn start_block_move(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Block>>,
) {
    for (mut transform, mut vel) in query.iter_mut() {
        vel.speed = BLOCK_SPEED;
        transform.translation.x -= vel.speed * time.delta_seconds();

        if transform.translation.x < (-WIN_WIDTH / 2.0) - (BLOCK_WIDTH / 2.0) {
            transform.translation.x = (WIN_WIDTH / 2.0) + (BLOCK_WIDTH / 2.0)
        }
    }
}
