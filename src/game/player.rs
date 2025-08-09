use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    PausableSystems,
    audio::sound_effect,
    game::juice::{circles::SpawnCircles, pulse_effect::PulseEffect},
    screens::Screen,
};

pub const CLICK_PARTICLES_Z: f32 = 20.0;
const PLAYER_SIZE: f32 = 16.0;
const PLAYER_COLOR: Color = Color::linear_rgb(1.0, 0.0, 0.0);
const PLAYER_Z: f32 = 100.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_player)
        .add_systems(OnExit(Screen::Gameplay), show_cursor)
        .add_systems(Update, move_player.in_set(PausableSystems));
}

#[derive(Component, Default)]
pub struct Player {
    pub clicked_on_target: bool,
}

fn spawn_player(
    mut commands: Commands,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    window.cursor_options.visible = false;

    // spawn player
    commands.spawn((
        Name::new("Player"),
        Mesh2d(meshes.add(Circle::new(PLAYER_SIZE))),
        MeshMaterial2d(materials.add(PLAYER_COLOR)),
        Transform::from_xyz(0.0, 0.0, PLAYER_Z),
        PulseEffect {
            min: 0.9,
            max: 1.1,
            speed: 0.5,
        },
        Player::default(),
        StateScoped(Screen::Gameplay),
        Pickable {
            should_block_lower: false,
            ..default()
        },
    ));

    // spawn non-target click mesh
    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(window.width(), window.height()))),
            MeshMaterial2d(materials.add(Color::linear_rgba(0.0, 0.0, 0.0, 0.0))),
            StateScoped(Screen::Gameplay),
            Transform::from_xyz(0.0, 0.0, -1000.0),
        ))
        .observe(create_click_effect);
}

fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let (camera, camera_transform) = *camera;
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    player.translation = world_position.extend(PLAYER_Z);
}

fn show_cursor(mut window: Single<&mut Window, With<PrimaryWindow>>) {
    window.cursor_options.visible = true;
}

fn create_click_effect(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle = asset_server.load("audio/sound_effects/click.ogg");
    commands.spawn((Name::new("Cursor click sound"), sound_effect(handle, 0.1)));
    commands.trigger(SpawnCircles {
        location: trigger.hit.position.unwrap().xy().extend(CLICK_PARTICLES_Z),
        ..default()
    });
}
