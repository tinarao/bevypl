use bevy::{prelude::*, window::WindowResolution};

enum GameState {
    Running,
    Failed,
    Pause,
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Debug;

#[derive(Resource)]
struct State {
    current_state: GameState,
}

impl State {
    fn to_string(&self) -> String {
        let state_string = match self.current_state {
            GameState::Failed => "failed",
            GameState::Pause => "paused",
            GameState::Running => "running",
        };

        String::from(state_string)
    }
}

const PADDLE_SPEED: f32 = 300.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                resolution: WindowResolution::new(720., 360.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(State {
            current_state: GameState::Pause,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (move_paddle, update_game_state))
        .run();
}

fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
    state: Res<State>,
) {
    cmd.spawn(Camera2d);

    let ball_rect = Circle::new(8.);
    let paddle_rect = Rectangle::new(96., 8.);
    let ball = meshes.add(ball_rect);
    let paddle: Handle<Mesh> = meshes.add(paddle_rect);

    let ball_color = Color::srgb(255., 133., 196.);
    let paddle_color = Color::srgb(128., 215., 255.);

    cmd.spawn((
        Mesh2d(paddle),
        MeshMaterial2d(materials.add(ball_color)),
        Transform::from_xyz(
            -ball_rect.radius,
            -(window.single().height() / 2.) + ball_rect.radius * 2. + ball_rect.radius,
            0.,
        ),
        Paddle,
    ));

    cmd.spawn((
        Mesh2d(ball),
        MeshMaterial2d(materials.add(paddle_color)),
        Transform::default(),
        Ball,
    ));

    let str = format!("State: {}", state.to_string());
    cmd.spawn((Text::new(str), Transform::from_xyz(-450., 150., 0.), Debug));
}

fn create_world() {
    // 8
}

//

fn update_game_state(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<State>,
    mut debug_track: Query<(&mut Text, &Debug)>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match state.current_state {
            GameState::Pause => state.current_state = GameState::Running,
            GameState::Running => state.current_state = GameState::Pause,
            _ => {}
        }
    }

    let (mut dbg_text, _) = debug_track.single_mut();

    dbg_text.0 = format!("State: {}", state.to_string());
}

fn move_paddle(
    mut q: Query<(&Paddle, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State>,
) {
    let (_p, mut transform) = q.single_mut();

    match state.current_state {
        GameState::Running => {
            if keys.pressed(KeyCode::KeyA) {
                transform.translation.x -= PADDLE_SPEED * time.delta_secs();
            }
            if keys.pressed(KeyCode::KeyD) {
                transform.translation.x += PADDLE_SPEED * time.delta_secs();
            }
        }
        _ => {}
    }
}

// fn update_ball(mut b: Query<(&Ball, &mut Transform)>, state: Res<State>) {
//     let (_, mut transform) = b.single_mut();

//     match state.current_state {
//         GameState::Running => {}
//         _ => {}
//     }
// }
