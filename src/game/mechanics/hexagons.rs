use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{
    audio::sound_effect, game::{game_sequencer::SpawnMechanic, mechanics::button::GameButton, player::Player}, PausableSystems
};

const HEXAGON_SPAWN_INTERVAL: f32 = 2.5;
const HEXAGON_SIZE: f32 = 24.0;
const HEXAGON_COLOR: Color = Color::linear_rgb(0.0, 0.0, 1.0);
const HEXAGON_Z: f32 = 80.0;
const HEXAGON_SPEED: f32 = 128.0;

const FRAGMENT_SIZE: f32 = 12.0;
const FRAGMENTS_PER_HEXAGON: usize = 8;
const FRAGMENT_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.3);
const FRAGMENT_Z: f32 = 0.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(SpawnMechanic::Hexagons), spawn_hexagon_spawner)
        .add_systems(
            Update,
            (spawn_hexagons, move_hexagon).in_set(PausableSystems),
        ).add_observer(create_hexagon_destroyed_effect);
}

#[derive(Event)]
struct OnHexagonDestroyed {
    location: Vec2,
}

#[derive(Component)]
struct Hexagon;

#[derive(Resource)]
struct HexagonSpawner {
    spawn_timer: Timer,
    hexagon_mesh: Handle<Mesh>,
    hexagon_material: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct FragmentHandles {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

fn spawn_hexagon_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(HexagonSpawner {
        spawn_timer: Timer::from_seconds(HEXAGON_SPAWN_INTERVAL, TimerMode::Repeating),
        hexagon_mesh: meshes.add(RegularPolygon::new(HEXAGON_SIZE, 6)),
        hexagon_material: materials.add(HEXAGON_COLOR),
    });

    commands.insert_resource(FragmentHandles {
        mesh: meshes.add(Triangle2d::new(
            vec2(-FRAGMENT_SIZE / 2.0, -FRAGMENT_SIZE / 2.0),
            vec2(FRAGMENT_SIZE / 2.0, -FRAGMENT_SIZE / 2.0),
            vec2(-FRAGMENT_SIZE / 2.0, FRAGMENT_SIZE / 2.0),
        )),
        material: materials.add(FRAGMENT_COLOR),
    });
}

fn spawn_hexagons(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    spawner: Option<ResMut<HexagonSpawner>>,
    time: Res<Time>,
) {
    let Some(mut spawner) = spawner else {
        return;
    };

    spawner.spawn_timer.tick(time.delta());
    if !spawner.spawn_timer.finished() {
        return;
    }

    let mut rng = rand::rng();
    let spawn_position = vec2(
        window.width() / 2.0,
        rng.random_range((-window.height() * 0.4)..(window.height() * 0.4)),
    );

    commands
        .spawn((
            Name::new("Hexagon"),
            Mesh2d(spawner.hexagon_mesh.clone()),
            MeshMaterial2d(spawner.hexagon_material.clone()),
            Transform::from_translation(spawn_position.extend(HEXAGON_Z)),
            Pickable::default(),
            Hexagon,
        ))
        .observe(destroy_clicked_hexagon);
}

fn move_hexagon(
    mut query: Query<&mut Transform, (With<Hexagon>, Without<GameButton>)>,
    button_transform: Single<&Transform, With<GameButton>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let direction =
            (button_transform.translation.xy() - transform.translation.xy()).normalize_or_zero();
        let delta = direction * HEXAGON_SPEED * time.delta_secs();

        transform.translation += delta.extend(0.0);
        transform.rotation = Quat::from_rotation_z(direction.to_angle());
    }
}

fn destroy_clicked_hexagon(
    trigger: Trigger<Pointer<Click>>,
    query: Query<&Transform, With<Hexagon>>,
    mut player: Single<&mut Player>,
    mut commands: Commands,
) {
    let transform = query.get(trigger.target()).unwrap();

    player.clicked_on_target = true;
    commands.trigger(OnHexagonDestroyed {
        location: transform.translation.xy(),
    });
    commands.entity(trigger.target()).despawn();
}

fn create_hexagon_destroyed_effect(
    trigger: Trigger<OnHexagonDestroyed>,
    mut commands: Commands,
    fragment_handles: Res<FragmentHandles>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::rng();

    // spawn fragments
    for _ in 0..FRAGMENTS_PER_HEXAGON {
        let relative_position = vec2(
            rng.random_range((-HEXAGON_SIZE / 2.0)..(HEXAGON_SIZE / 2.0)),
            rng.random_range((-HEXAGON_SIZE / 2.0)..(HEXAGON_SIZE / 2.0)),
        );
        let position = trigger.event().location + relative_position;
        let rotation = rng.random_range((0.0)..(2.0 * std::f32::consts::PI));


        commands.spawn((
            Name::new("Fragment"),
            Mesh2d(fragment_handles.mesh.clone()),
            MeshMaterial2d(fragment_handles.material.clone()),
            Transform::from_translation(position.extend(FRAGMENT_Z)).with_rotation(Quat::from_rotation_z(rotation)),
            Pickable {
                should_block_lower: false,
                ..default()
            },
        ));
    }

    // play sound effect
    let handle = asset_server.load("audio/sound_effects/break.ogg");
    commands.spawn((Name::new("Hexagon break sound"), sound_effect(handle, 0.2)));
}
