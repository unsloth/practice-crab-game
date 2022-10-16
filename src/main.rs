use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

const WIN_WIDTH: f32 = 800.0;
const WIN_HEIGHT: f32 = 500.0;

const GRAVITY: f32 = 10.0;
const JUMP_ACCEL: f32 = -500.0;

const BLOCK_WIDTH: f32 = 70.0;
const BLOCK_SIZE: Vec2 = Vec2::new(BLOCK_WIDTH, WIN_HEIGHT);
const BLOCK_SPEED: f32 = 256.0;
const BLOCK_GAP: f32 = 140.0;

const CRAB_SIZE: Vec2 = Vec2::new(32.0, 32.0);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Crab Game".to_string(),
            width: WIN_WIDTH,
            height: WIN_HEIGHT,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Play)
        .add_startup_system(setup_camera)
        .add_system_set(SystemSet::on_enter(GameState::Play).with_system(add_sprites))
        .add_system_set(
            SystemSet::on_update(GameState::Play)
                .with_system(start_gravity)
                .with_system(start_block_move)
                .with_system(check_collision),
        )
        .run();
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Menu,
    Play,
    Over,
}

#[derive(Component)]
struct Crab;

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Velocity {
    speed: f32,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn add_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    // crab entity (player)
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(CRAB_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(-(WIN_WIDTH / 2.0) + 100.0, 0.0, 0.0),
            // currently the initial transform is baked into the entity. May want
            // to change it to being set when the game starts. Perhaps in
            // start_gravity. Since the different game states won't have the crab
            // in the same place. Main menu could have it center, for example.
            // same with the blocks.
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
                custom_size: Some(BLOCK_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(
                WIN_WIDTH / 2.0,
                // to account for gap between blocks
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
                custom_size: Some(BLOCK_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(
                WIN_WIDTH / 2.0,
                // to account for gap between blocks
                -WIN_HEIGHT / 2.0 - BLOCK_GAP / 2.0,
                0.0,
            ),
            ..default()
        })
        .insert(Block)
        .insert(Velocity { speed: 0.0 });
}

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
        // Or maybe both bounds can stay
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
            transform.translation.x = (WIN_WIDTH / 2.0) + (BLOCK_WIDTH / 2.0);
            println!("1 more!");

            // the plan was to use a random number generator to randomly move the
            // y transform of the blocks. The issue is that I would then need to
            // reset the y value after every go around or the blocks may eventually
            // go off screen. The issue is there are 2 blocks with different
            // y transforms, so I can't just reset the y value.
            // I could initially set the rng variable to 0 and subtract it from
            // the y value. Then I set the variable equal to a new rng and add it
            // to the y value. Therefore with each loop, it first resets the
            // transform by whatever value it was and then gets a new value to
            // change it after. That idea seems a little messy though.
        }
    }
}

fn check_collision(
    crab_query: Query<(&Transform, &Sprite), With<Crab>>,
    block_query: Query<(&Transform, &Sprite), With<Block>>,
) {
    let (crab_transform, crab_size) = crab_query.single();
    // Consider looking into doing something about this unwrap.
    // Maybe not since it should be guaranteed but idk.
    let crab_size = crab_size.custom_size.unwrap();

    for (block_transform, block_size) in block_query.iter() {
        let block_size = block_size.custom_size.unwrap();
        let collision = collide(
            crab_transform.translation,
            crab_size,
            block_transform.translation,
            block_size,
        );
        if let Some(_collision) = collision {
            // will eventually make it change state to loss
            println!("YOU LOSE");
        }
    }
}
