use bevy::{math::ops::*, prelude::*, state::commands};
use std::f64::consts::PI;
/// Player movement speed factor.
const PLAYER_SPEED: f32 = 100.;
const WALL_SIZE: f32 = 50.;
const MAX_DISTANCE: f32 = 50.;

const MAP: [[i32; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 1, 0, 1, 1],
    [1, 1, 0, 1, 0, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

const TOP_LEFT_X: f32 = -(MAP[0].len() as f32 * WALL_SIZE) / 2.;
const TOP_LEFT_Y: f32 = (MAP.len() as f32 * WALL_SIZE) / 2.;

#[derive(Component, Default)]
struct Player {
    pub angle: f32,
    pub dir: Vec2,
}

#[derive(Component)]
struct Hittable;

#[derive(Component)]
struct Point;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_scene, setup_camera))
        .add_systems(Update, (move_player).chain())
        .add_systems(Update, (rotate_player).chain())
        .add_systems(Update, (move_point).chain())
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for colon in 0..MAP.len() {
        for row in 0..MAP[0].len() {
            if MAP[colon][row] == 1 {
                commands.spawn((
                    Hittable,
                    Mesh2d(meshes.add(Rectangle::new(WALL_SIZE, WALL_SIZE))),
                    MeshMaterial2d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
                    Transform::from_xyz(
                        TOP_LEFT_X + row as f32 * WALL_SIZE,
                        TOP_LEFT_Y - colon as f32 * WALL_SIZE,
                        2.,
                    ),
                ));
            }
        }
    }
    // World where we move the player
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
    ));

    //Point
    commands.spawn((
        Point,
        Mesh2d(meshes.add(Circle::new(8.))),
        MeshMaterial2d(materials.add(Color::srgb(0.9, 0.5, 0.5))), // RGB values exceed 1 to achieve a bright color for the bloom effect
        Transform::from_xyz(-4., 4., 2.),
    ));
    // Player
    commands.spawn((
        Player {
            angle: 0.,
            dir: Vec2::ZERO,
        },
        Mesh2d(meshes.add(Circle::new(10.))),
        MeshMaterial2d(materials.add(Color::srgb(6.25, 9.4, 9.1))), // RGB values exceed 1 to achieve a bright color for the bloom effect
        Transform::from_xyz(-5., 5., 2.),
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true, // HDR is required for the bloom effect
            ..default()
        },
    ));
}

/// Update the player position with keyboard inputs.
/// Note that the approach used here is for demonstration purposes only,
/// as the point of this example is to showcase the camera tracking feature.
///
/// A more robust solution for player movement can be found in `examples/movement/physics_in_fixed_timestep.rs`.
// fn draw_ray(
//     mut player: Single<&mut Transform, With<Player>>,
//     time: Res<Time>,
// ) {

// }

fn rotate_player(
    mut player: Single<&mut Player, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    if kb_input.pressed(KeyCode::KeyE) {
        if player.angle - 1. < 0. {
            player.angle = 359.
        }
        player.angle -= 1.;
    }
    if kb_input.pressed(KeyCode::KeyQ) {
        if player.angle + 1. > 360. {
            player.angle = 1.;
        }
        player.angle += 1.;
    }
    player.dir.x = cos(player.angle * (PI as f32 / 180.));
    player.dir.y = sin(player.angle * (PI as f32 / 180.));
    player.dir = player.dir.normalize();
    println!("{}", player.dir);
}
fn draw_line(
    player_trans: Single<&mut Transform, (With<Player>, Without<Point>)>,
    player_veiw: Single<&mut Player, With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
  commands.spawn((
      Mesh2d(meshes.add(Segment2d::new(player_trans.translation.truncate(), player_veiw.dir*MAX_DISTANCE)))
  )); 
}
    
fn move_point(
    player_trans: Single<&mut Transform, (With<Player>, Without<Point>)>,
    mut point: Single<&mut Transform, (With<Point>, Without<Player>)>,
    player_veiw: Single<&mut Player, With<Player>>,
) {
    point.translation = player_trans.translation + player_veiw.dir.extend(0.) * 10.
}



fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }
    // Progressively update the player's position over time. Normalize the
    // direction vector to prevent it from exceeding a magnitude of 1 when
    // moving diagonally.
    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    player.translation += move_delta.extend(0.);
}
