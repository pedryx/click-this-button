use bevy::{prelude::*, window::PrimaryWindow};

use crate::{PausableSystems, game::juice::pulse_effect::PulseEffect, screens::Screen};

const PLAYER_SIZE: f32 = 16.0;
const PLAYER_COLOR: Color = Color::linear_rgb(1.0, 0.0, 0.0);
const PLAYER_Z: f32 = 100.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_player)
        .add_systems(OnExit(Screen::Gameplay), show_cursor)
        .add_systems(Update, move_player.in_set(PausableSystems));
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    window.cursor_options.visible = false;

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
        Player,
        StateScoped(Screen::Gameplay),
    ));
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
