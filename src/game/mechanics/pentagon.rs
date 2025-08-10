use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    PausableSystems,
    game::{
        GameOver,
        game_sequencer::GameMechanic,
        player::{PLAYER_SIZE, Player},
    },
    screens::Screen,
};

const PENTAGON_Z: f32 = 90.0;
const PENTAGON_COLOR: Color = Color::linear_rgb(1.0, 0.5, 0.0);
const PENTAGON_SIZE: f32 = 48.0;
const PENTAGON_SPEED: f32 = 128.0;

const SPAWN_DISTANCE: f32 = 1024.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameMechanic::Pentagon), spawn_pentagon)
        .add_systems(Update, move_to_player.in_set(PausableSystems));
}

#[derive(Component)]
struct Pentagon;

fn spawn_pentagon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();
    let position = Vec2::from_angle(rng.random_range(0.0..2.0 * PI)) * SPAWN_DISTANCE;

    commands.spawn((
        Name::new("Pentagon"),
        Mesh2d(meshes.add(RegularPolygon::new(PENTAGON_SIZE, 5))),
        MeshMaterial2d(materials.add(PENTAGON_COLOR)),
        Transform::from_translation(position.extend(PENTAGON_Z)),
        StateScoped(Screen::Gameplay),
        Pentagon,
    ));
}

fn move_to_player(
    mut commands: Commands,
    mut pentagon_transform: Single<&mut Transform, With<Pentagon>>,
    player_transform: Single<&mut Transform, (With<Player>, Without<Pentagon>)>,
    time: Res<Time>,
) {
    let pentagon_position = pentagon_transform.translation.xy();
    let player_position = player_transform.translation.xy();

    let direction = (player_position - pentagon_position).normalize_or_zero();
    let delta = direction * PENTAGON_SPEED * time.delta_secs();

    pentagon_transform.translation += delta.extend(0.0);
    pentagon_transform.rotation = Quat::from_rotation_z(direction.to_angle());

    if player_position.distance_squared(pentagon_position) <= (PENTAGON_SIZE + PLAYER_SIZE).powi(2)
    {
        commands.trigger(GameOver(GameMechanic::Pentagon));
    }
}
