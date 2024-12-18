use bevy::prelude::*;
use player::Player;
use tiles::Collider;

mod player;
mod tiles;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (camera_setup, player_setup, world_setup))
        .add_systems(Update, update_player)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    let circle = Circle::new(32.);
    let color = Color::srgb(255., 0., 0.);

    let circlemesh = meshes.add(circle);
    commands.spawn((
        Mesh2d(circlemesh),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(
            -circle.radius / 2.,
            -(window.single().resolution.height() / 2.) + 48. + circle.radius / 2.,
            0.,
        ),
        Player::new(),
    ));
}

fn world_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let square = Rectangle::new(300., 8.);
    let color = Color::srgb(0., 255., 0.);

    let rect = meshes.add(square);
    commands.spawn((
        Mesh2d(rect),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(-400., 0., 0.),
        Collider::new(),
    ));
}

// Updates

fn update_player(
    mut q: Query<(&Player, &mut Transform)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (pl, mut transform) = q.single_mut();

    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= 1. * pl.speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        transform.translation.x += 1. * pl.speed * time.delta_secs();
    }
}
