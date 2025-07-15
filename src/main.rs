use bevy::{math::ops::*, prelude::*, window::*};
use std::f64::consts::PI;
/// Player movement speed factor.
const PLAYER_SPEED: f32 = 100.;
const WALL_SIZE: f32 = 50.;
const MAX_DISTANCE: f32 = 500.;
const MAP_SIZE: usize= 8;
const WIDTH: i32= 32;

const MAP: [[i32; MAP_SIZE]; MAP_SIZE] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 1],
    [1, 1, 0, 1, 0, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];


#[derive(Component, Default)]
struct Player {

    pub angle: f32,
    pub dir: Vec2,
}




#[derive(Component)]
struct Point;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(MAP_SIZE as f32 *WALL_SIZE, MAP_SIZE as f32*WALL_SIZE).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_scene, setup_camera ))
        .add_systems(Update, (draw_grid))
        .add_systems(Update, (move_point).chain())
        .add_systems(Update, (move_player,rotate_player).chain())
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    
) {
    for colon in 0..MAP[0].len() {
        for row in 0..MAP.len() {
            if MAP[colon][row] == 1 {
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(WALL_SIZE, WALL_SIZE))),
                    MeshMaterial2d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
                    Transform::from_xyz(
                        row as f32 * WALL_SIZE+(WALL_SIZE*0.5),
                        colon as f32 * WALL_SIZE+(WALL_SIZE*0.5),
                        2.,
                    ),
                ));
            }
        }
    }
    // World where we move the player
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(10000., 7000.))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
    ));
    //Point
    commands.spawn((
        Point,
        Mesh2d(meshes.add(Circle::new(5.))),
        MeshMaterial2d(materials.add(Color::srgb(0.9, 0.5, 0.5))), // RGB values exceed 1 to achieve a bright color for the bloom effect
        Transform::from_xyz(-4., 4., 2.),
    ));
    // Player
    commands.spawn((
        Player {
            angle: 0.,
            dir: Vec2::ZERO
        },
        Mesh2d(meshes.add(Circle::new(10.))),
        MeshMaterial2d(materials.add(Color::srgb(6.25, 9.4, 9.1))), // RGB values exceed 1 to achieve a bright color for the bloom effect
        Transform::from_xyz(200., 200., 2.),
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            ..default()
        },
        Transform::from_xyz(200.,200.,0.)
    ));
}

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
    if MAP[get_map_cords((player.translation + move_delta.extend(0.)).truncate()).y as usize][get_map_cords((player.translation + move_delta.extend(0.)).truncate()).x as usize] == 0 {
        player.translation += move_delta.extend(0.);
    } 
}

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera.translation = direction}



fn draw_grid(
    player_transform: Single<&mut Transform, (With<Player>,Without<Point>)>,
    player: Single<&mut Player, With<Player>>,
    mut gizmos:Gizmos,
){
    gizmos.grid(Isometry3d{translation:Vec3A::from(Vec3{x:0.,y:0.,z:0.}), ..default()}, UVec2 { x: MAP_SIZE as u32 * 2, y: MAP_SIZE as u32 * 2 },Vec2 { x: (WALL_SIZE), y: (WALL_SIZE)} ,LinearRgba::gray(0.7));

}

fn draw_rays(
    player_transform: Single<&mut Transform, (With<Player>,Without<Point>)>,
    player: Single<&mut Player, With<Player>>,
    mut gizmos:Gizmos,
){
    let plane: Vec2 = player.dir.perp();
    
    //gizmos.ray_2d(player_transform.translation.truncate()+(player.dir+plane)*50., (-plane)*100. ,LinearRgba::gray(0.7) );
    for i in 0..WIDTH{
        let mut new_dir:Vec2 = (-plane + ((plane/WIDTH as f32)*2. * i as f32 ))+player.dir;
        
        gizmos.ray_2d(player_transform.translation.truncate(),new_dir*MAX_DISTANCE  ,LinearRgba::gray(0.7) );
      //gizmos.ray_2d(player_transform.translation.truncate(),(plane+player.dir)*MAX_DISTANCE  ,LinearRgba::gray(0.7) );
      //gizmos.ray_2d(player_transform.translation.truncate(),(-plane+player.dir)*MAX_DISTANCE  ,LinearRgba::gray(0.7) );    
    }
}

fn get_cords(pos : Vec2) -> Vec2{
 Vec2 { x: (pos.x/WALL_SIZE).ceil(), y : (pos.y/WALL_SIZE).ceil() }
}

fn get_map_cords(pos:Vec2) -> Vec2{
 Vec2 { x:(pos.x/WALL_SIZE).ceil()-1.0, y:MAP_SIZE as f32 - ((pos.y/WALL_SIZE).ceil())}
}

// fn cast_ray(dir :Vec2, mut player: Single<&mut Transform, With<Player>>)-> f32{
    
// }


fn find_point(dir :Vec2, mut player: Single<&Transform, (With<Player>)>){
 
}

fn move_point(
    player_trans: Single<&mut Transform, (With<Player>, Without<Point>)>,
    mut point: Single<&mut Transform, (With<Point>, Without<Player>)>,
    player_veiw: Single<&mut Player, With<Player>>,
) {
    point.translation = player_trans.translation + player_veiw.dir.extend(0.) * 10.
}
