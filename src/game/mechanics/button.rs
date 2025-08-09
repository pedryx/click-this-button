use bevy::prelude::*;

use crate::game::{bar::{Bar, BarBehavior, BarLayout, OnBarEmpty}, game_sequencer::SpawnMechanic, OnGameOver};

const BUTTON_SIZE: f32 = 96.0;
const BUTTON_COLOR: Color = Color::linear_rgb(0.0, 1.0, 0.0);
const BUTTON_Z: f32 = 0.0;

const TEXT_SIZE: f32 = 32.0;
const TEXT_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.0);

const TIME_BAR_DURATION: f32 = 5.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(SpawnMechanic::Button), spawn_button)
        .add_systems(OnEnter(SpawnMechanic::ButtonTimeBar), spawn_button_time_bar)
        .add_systems(Update, update_button_time)
        .add_observer(on_button_time_up);
}

#[derive(Component)]
struct Button;

#[derive(Component)]
struct ButtonTimeBar;

fn spawn_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BUTTON_SIZE))),
        MeshMaterial2d(materials.add(BUTTON_COLOR)),
        Transform::from_xyz(0.0, 0.0, BUTTON_Z),
        Button,
    )).with_children(|parent| {
        parent.spawn((
            Text2d::new("Click\nme!"),
            TextFont {
                font: asset_server.load("fonts/Super Vanilla.ttf"),
                font_size: TEXT_SIZE,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
            TextColor(TEXT_COLOR),
        ));
    });
}

fn spawn_button_time_bar(
    mut commands: Commands,
) {
    commands.spawn((
        Bar {
            max: TIME_BAR_DURATION,
            current: TIME_BAR_DURATION,
            ..default()
        },
        BarLayout {
            color: BUTTON_COLOR,
            ..default()
        },
        BarBehavior {
            trigger_on_empty: true,
            ..default()
        },
        Transform::from_xyz(0.0, -BUTTON_SIZE * 1.5, BUTTON_Z),
        ButtonTimeBar,
    ));
}

fn update_button_time(
    mut bar: Single<&mut Bar, With<ButtonTimeBar>>,
    time: Res<Time>,
) {
    bar.current -= time.delta_secs();
}

fn on_button_time_up(
    trigger: Trigger<OnBarEmpty>,
    mut commands: Commands,
    time_bar_entity: Single<Entity, With<ButtonTimeBar>>,
) {
    if trigger.event().sender != *time_bar_entity { return }

    commands.trigger(OnGameOver);
}
