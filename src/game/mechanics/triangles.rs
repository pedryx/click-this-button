use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{
    PausableSystems,
    audio::sound_effect,
    game::{
        GameOver,
        game_sequencer::GameMechanic,
        mechanics::the_button::{THE_BUTTON_SIZE, TheButton},
        player::Player,
    },
    screens::Screen,
};

const TRIANGLE_SPAWN_INTERVAL: f32 = 2.5;
const TRIANGLE_SIZE: f32 = 48.0;
const TRIANGLE_COLOR: Color = Color::linear_rgb(0.0, 0.0, 1.0);
const TRIANGLE_Z: f32 = 80.0;
const TRIANGLE_SPEED: f32 = 96.0;

const FRAGMENT_SIZE: f32 = 12.0;
const FRAGMENTS_PER_HEXAGON: usize = 8;
const FRAGMENT_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.3);
const FRAGMENT_Z: f32 = 0.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameMechanic::Triangles), spawn_triangle_spawner)
        .add_systems(
            Update,
            (spawn_triangles, move_triangles).in_set(PausableSystems),
        )
        .add_observer(create_triangle_destroyed_effect)
        .add_systems(OnExit(Screen::Gameplay), despawn_triangle_spawner);
}

#[derive(Event)]
struct OnTriangleDestroyed {
    location: Vec2,
}

#[derive(Component)]
struct Triangle;

#[derive(Resource)]
struct TriangleSpawner {
    spawn_timer: Timer,
    triangle_mesh: Handle<Mesh>,
    triangle_material: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct FragmentHandles {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

fn spawn_triangle_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(TriangleSpawner {
        spawn_timer: Timer::from_seconds(TRIANGLE_SPAWN_INTERVAL, TimerMode::Repeating),
        triangle_mesh: meshes.add(Triangle2d::new(
            vec2(0.0, 0.0),
            vec2(-TRIANGLE_SIZE, TRIANGLE_SIZE * 0.6),
            vec2(-TRIANGLE_SIZE, -TRIANGLE_SIZE * 0.6),
        )),
        triangle_material: materials.add(TRIANGLE_COLOR),
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

fn despawn_triangle_spawner(mut commands: Commands) {
    commands.remove_resource::<TriangleSpawner>();
    commands.remove_resource::<FragmentHandles>();
}

fn spawn_triangles(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    spawner: Option<ResMut<TriangleSpawner>>,
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
        window.width() * 0.7,
        rng.random_range((-window.height() * 0.4)..(window.height() * 0.4)),
    );

    commands
        .spawn((
            Name::new("Triangle"),
            Mesh2d(spawner.triangle_mesh.clone()),
            MeshMaterial2d(spawner.triangle_material.clone()),
            Transform::from_translation(spawn_position.extend(TRIANGLE_Z)),
            Pickable::default(),
            Triangle,
            StateScoped(Screen::Gameplay),
        ))
        .observe(destroy_clicked_hexagon);
}

fn move_triangles(
    mut commands: Commands,
    mut query: Query<&mut Transform, (With<Triangle>, Without<TheButton>)>,
    button_transform: Single<&Transform, With<TheButton>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let direction =
            (button_transform.translation.xy() - transform.translation.xy()).normalize_or_zero();
        let delta = direction * TRIANGLE_SPEED * time.delta_secs();

        transform.translation += delta.extend(0.0);
        transform.rotation = Quat::from_rotation_z(direction.to_angle());

        let distance = transform
            .translation
            .xy()
            .distance(button_transform.translation.xy());
        if distance <= THE_BUTTON_SIZE {
            commands.trigger(GameOver(GameMechanic::Triangles));
            return;
        }
    }
}

fn destroy_clicked_hexagon(
    trigger: Trigger<Pointer<Click>>,
    query: Query<&Transform, With<Triangle>>,
    mut player: Single<&mut Player>,
    mut commands: Commands,
) {
    let transform = query.get(trigger.target()).unwrap();

    player.clicked_on_target = true;
    commands.trigger(OnTriangleDestroyed {
        location: transform.translation.xy(),
    });
    commands.entity(trigger.target()).despawn();
}

fn create_triangle_destroyed_effect(
    trigger: Trigger<OnTriangleDestroyed>,
    mut commands: Commands,
    fragment_handles: Res<FragmentHandles>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::rng();

    // spawn fragments
    for _ in 0..FRAGMENTS_PER_HEXAGON {
        let relative_position = vec2(
            rng.random_range((-TRIANGLE_SIZE * 0.3)..(TRIANGLE_SIZE * 0.3)),
            rng.random_range((-TRIANGLE_SIZE * 0.3)..(TRIANGLE_SIZE * 0.3)),
        );
        let position = trigger.event().location + relative_position;
        let rotation = rng.random_range((0.0)..(2.0 * std::f32::consts::PI));

        commands.spawn((
            Name::new("Fragment"),
            Mesh2d(fragment_handles.mesh.clone()),
            MeshMaterial2d(fragment_handles.material.clone()),
            Transform::from_translation(position.extend(FRAGMENT_Z))
                .with_rotation(Quat::from_rotation_z(rotation)),
            Pickable {
                should_block_lower: false,
                ..default()
            },
            StateScoped(Screen::Gameplay),
        ));
    }

    // play sound effect
    let handle = asset_server.load("audio/sound_effects/break.ogg");
    commands.spawn((Name::new("Hexagon break sound"), sound_effect(handle, 0.2)));
}
