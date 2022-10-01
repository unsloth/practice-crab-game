use bevy::prelude::*;

const WIN_WIDTH: f32 = 800.0;
const WIN_HEIGHT: f32 = 500.0;
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
        .add_system(start_game)
        .run();
}

#[derive(Component)]
struct Crab;

#[derive(Component)]
struct Velocity {
    speed: f32,
}

fn add_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(-(WIN_WIDTH / 2.0) + 100.0, 0.0, 0.0),
            texture: asset_server.load("crab.png"),
            ..default()
        })
        .insert(Crab)
        .insert(Velocity { speed: 0.0 });
}
/*
enum GameStatus {
    Menu,
    Play,
    Over,
}
*/

const GRAVITY: f32 = 10.0;
const JUMP_ACCEL: f32 = -500.0;

fn start_game(
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
