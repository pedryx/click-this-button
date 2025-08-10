use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{
    PausableSystems,
    game::{game_sequencer::SpawnMechanic, mechanics::the_button::TheButton},
};

const SQUARE_SPAWN_INTERVAL: f32 = 3.0;
const SQUARE_SIZE: f32 = 256.0;
const SQUARE_Z: f32 = 90.0;
const SQUARE_COLOR: Color = Color::linear_rgb(0.5, 0.0, 1.0);
const SQUARE_SPEED: f32 = 1024.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(SpawnMechanic::Square), spawn_square_spawner)
        .add_systems(Update, (spawn_square, move_square).in_set(PausableSystems));
}

#[derive(Resource)]
struct SquareSpawner {
    spawn_timer: Timer,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

#[derive(Component, Default)]
struct Square {
    drag_direction: Option<Vec2>,
}

fn spawn_square_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(SquareSpawner {
        spawn_timer: Timer::from_seconds(SQUARE_SPAWN_INTERVAL, TimerMode::Once),
        mesh: meshes.add(Rectangle::new(SQUARE_SIZE, SQUARE_SIZE)),
        material: materials.add(SQUARE_COLOR),
    });
}

fn spawn_square(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    spawner: Option<ResMut<SquareSpawner>>,
    time: Res<Time>,
) {
    let Some(mut spawner) = spawner else {
        return;
    };

    spawner.spawn_timer.tick(time.delta());
    if !spawner.spawn_timer.just_finished() {
        return;
    }

    let mut rng = rand::rng();
    let angle = rng.random_range((0.0)..(2.0 * PI));
    let position = Vec2::from_angle(angle) * window.width() * 0.6;

    commands
        .spawn((
            Name::new("Square"),
            Mesh2d(spawner.mesh.clone()),
            MeshMaterial2d(spawner.material.clone()),
            Transform::from_translation(position.extend(SQUARE_Z)),
            Pickable {
                should_block_lower: true,
                ..default()
            },
            Square::default(),
        ))
        .observe(on_square_drag);
}

fn move_square(
    mut commands: Commands,
    square: Single<(Entity, &mut Transform, &Square)>,
    button_transform: Single<&Transform, (With<TheButton>, Without<Square>)>,
    window: Single<&Window, With<PrimaryWindow>>,
    spawner: Option<ResMut<SquareSpawner>>,
    time: Res<Time>,
) {
    let (entity, mut square_transform, square) = square.into_inner();

    let square_pos = square_transform.translation.xy();
    let button_pos = button_transform.translation.xy();

    if square_pos.distance_squared(button_pos) <= 10.0 && square.drag_direction.is_none() {
        return;
    }

    let direction = square
        .drag_direction
        .unwrap_or_else(|| (button_pos - square_pos).normalize_or_zero());
    let delta = direction * SQUARE_SPEED * time.delta_secs();

    square_transform.translation += delta.extend(0.0);
    square_transform.rotation = Quat::from_rotation_z(direction.to_angle());

    if square.drag_direction.is_some()
        && button_pos.distance_squared(square_pos) >= window.width() * window.width()
    {
        commands.entity(entity).despawn();
        spawner.unwrap().spawn_timer.reset();
    }
}

fn on_square_drag(trigger: Trigger<Pointer<Drag>>, mut square: Single<&mut Square>) {
    square.drag_direction = Some(trigger.distance.normalize_or_zero() * vec2(1.0, -1.0));
}
