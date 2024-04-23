use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};
use rand::prelude::*;

#[derive(Component)]
struct MainGameCamera;

#[derive(Resource)]
struct BlockSpawnTimer(Timer);
#[derive(Resource)]
struct ColorTimer(Timer);

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
    commands.insert_resource(BlockSpawnTimer(Timer::from_seconds(
        0.01,
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
    let x_transform: f32 = rand::thread_rng().gen_range(-1000.0..1000.0);
    if block_timer.0.just_finished() {
        println!("Spawning a block!");
        println!("{x_transform}");
        commands.spawn(MaterialMesh2dBundle {
            mesh: block_mesh.0.clone(),
            material: block_color.0.clone(),
            transform: Transform::from_xyz(x_transform, x_transform, 0.0),
            ..default()
        });
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
        .run();
}
