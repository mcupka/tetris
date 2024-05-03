use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2d, Mesh2dHandle},
    window::WindowResolution,
};
use rand::prelude::*;

#[derive(Component)]
struct MainGameCamera;

#[derive(Resource)]
struct BlockSpawnTimer(Timer);
#[derive(Resource)]
struct GravityTimer(Timer);
#[derive(Resource)]
struct ColorTimer(Timer);

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Falling;

#[derive(Component)]
struct PlayerRect;

#[derive(Resource)]
struct RectangleMesh(Mesh2dHandle);

#[derive(Resource)]
struct RectangleColorMaterial(Handle<ColorMaterial>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn camera
    commands.spawn((Camera2dBundle::default(), MainGameCamera));

    // Create mesh resources
    let rectangle_mesh_handle = meshes.add(Rectangle {
        half_size: [50.0, 50.0].into(),
    });
    // Create color material resource
    let rectangle_material_handle = materials.add(Color::rgba(0.8, 0.1, 0.1, 1.0));

    // Resource to directly access handle of the rectangle mesh
    commands.insert_resource(RectangleMesh(Mesh2dHandle(rectangle_mesh_handle.clone())));
    commands.insert_resource(RectangleColorMaterial(rectangle_material_handle.clone()));
    commands.insert_resource(ColorTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
    commands.insert_resource(GravityTimer(Timer::from_seconds(
        0.01,
        TimerMode::Repeating,
    )));
    commands.insert_resource(BlockSpawnTimer(Timer::from_seconds(
        5.0,
        TimerMode::Repeating,
    )));
}

fn change_rectangle_color_periodically(
    time: Res<Time>,
    mut color_timer: ResMut<ColorTimer>,
    query: Query<(Entity, &Handle<ColorMaterial>), With<PlayerRect>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    color_timer.0.tick(time.delta());
    if color_timer.0.just_finished() {
        // Change rect  color
        for (entity, material_handle) in query.iter() {
            let current_color = materials.remove(material_handle).unwrap().color;
            let new_color = current_color + Color::rgba(0.01, 0.01, 0.01, 0.01);

            commands.entity(entity).insert(materials.add(ColorMaterial {
                color: new_color,
                ..default()
            }));
        }
    }
}

// Periodically spawn blocks
fn spawn_blocks_periodically(
    mut commands: Commands,
    block_color: Res<RectangleColorMaterial>,
    block_mesh: Res<RectangleMesh>,
    time: Res<Time>,
    mut block_timer: ResMut<BlockSpawnTimer>,
) {
    block_timer.0.tick(time.delta());
    let x_transform: f32 = rand::thread_rng().gen_range(-250.0..250.0);
    if block_timer.0.just_finished() {
        println!("Spawning a block!");
        println!("{x_transform}");
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: block_mesh.0.clone(),
                material: block_color.0.clone(),
                transform: Transform::from_xyz(x_transform, 300.0, 0.0),
                ..default()
            },
            Block,
            Falling,
        ));
    }
}

// Gravity for blocks
fn move_blocks_down(
    mut commands: Commands,
    time: Res<Time>,
    mut gravity_timer: ResMut<GravityTimer>,
    mut falling_block_query: Query<(Entity, &mut Transform), (With<Block>, With<Falling>)>,
    stationary_block_query: Query<&Transform, (With<Block>, Without<Falling>)>,
    //TODO: make a new query without falling. Iterate over this for collision
    // detection instead of using iter_combinations_mut()
) {
    gravity_timer.0.tick(time.delta());
    if gravity_timer.0.just_finished() {
        for (entity, mut transform) in falling_block_query.iter_mut() {
            transform.translation.y -= 3.0;
            if transform.translation.y <= -300.0 {
                transform.translation.y = -300.0;
                commands.entity(entity).remove::<Falling>();
            }
            for transform_other in stationary_block_query.iter() {
                let (x1, x2, y1, y2, w1, w2, h1, h2) = (
                    transform.translation.x,
                    transform_other.translation.x,
                    transform.translation.y,
                    transform_other.translation.y,
                    100.0,
                    100.0,
                    100.0,
                    100.0,
                );
                if (x1 + w1 / 2.0 >= x2 - w2 / 2.0)
                    && (x1 - w2 / 2.0 <= x2 + w2 / 2.0)
                    && (y1 - h1 / 2.0 <= y2 + h2 / 2.0)
                {
                    transform.translation.y = y2 + h2 / 2.0 + h1 / 2.0;
                    commands.entity(entity).remove::<Falling>();
                }
            }
        }
    }
}

fn main() {
    println!("Tetris Clone");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(150.0, 150.0).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, change_rectangle_color_periodically)
        .add_systems(Update, spawn_blocks_periodically)
        .add_systems(Update, move_blocks_down)
        .run();
}
