use bevy::prelude::*;

use crate::{
    PausableSystems,
    audio::sound_effect,
    game::{
        OnGameOver,
        bar::{Bar, BarBehavior, BarLayout, OnBarEmpty},
        game_sequencer::SpawnMechanic,
        juice::pulse_effect::PulseEffect,
        player::Player,
    },
    screens::Screen,
};

const BUTTON_SIZE: f32 = 96.0;
const BUTTON_COLOR: Color = Color::linear_rgb(0.0, 1.0, 0.0);
const BUTTON_Z: f32 = 50.0;

const TEXT_SIZE: f32 = 32.0;
const TEXT_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.0);

const TIME_BAR_DURATION: f32 = 5.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(SpawnMechanic::Button), spawn_button)
        .add_systems(OnEnter(SpawnMechanic::ButtonTimeBar), spawn_button_time_bar)
        .add_systems(
            Update,
            (update_button_time, handle_button_click).in_set(PausableSystems),
        )
        .add_observer(on_button_time_up)
        .add_observer(play_sound_on_button_click)
        .add_observer(update_time_bar_on_button_click);
}

#[derive(Event)]
struct OnButtonClicked;

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
    commands
        .spawn((
            Name::new("Button"),
            Mesh2d(meshes.add(Circle::new(BUTTON_SIZE))),
            MeshMaterial2d(materials.add(BUTTON_COLOR)),
            Transform::from_xyz(0.0, 0.0, BUTTON_Z),
            Button,
            StateScoped(Screen::Gameplay),
            PulseEffect::default(),
        ))
        .with_children(|parent| {
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

fn spawn_button_time_bar(mut commands: Commands) {
    commands.spawn((
        Name::new("Button time bar"),
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
        StateScoped(Screen::Gameplay),
    ));
}

fn update_button_time(mut bar: Single<&mut Bar, With<ButtonTimeBar>>, time: Res<Time>) {
    bar.current -= time.delta_secs();
}

fn on_button_time_up(
    trigger: Trigger<OnBarEmpty>,
    mut commands: Commands,
    time_bar_entity: Single<Entity, With<ButtonTimeBar>>,
) {
    if trigger.event().sender != *time_bar_entity {
        return;
    }
    commands.trigger(OnGameOver);
}

fn handle_button_click(
    player_transform: Single<&Transform, With<Player>>,
    button_transform: Single<&Transform, With<Button>>,
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let distance = player_transform
        .translation
        .truncate()
        .distance(button_transform.translation.truncate());
    if mouse.just_pressed(MouseButton::Left) && distance <= BUTTON_SIZE {
        commands.trigger(OnButtonClicked);
    }
}

fn update_time_bar_on_button_click(
    _: Trigger<OnButtonClicked>,
    mut bar: Single<&mut Bar, With<ButtonTimeBar>>,
) {
    bar.current = bar.max;
}

fn play_sound_on_button_click(
    _: Trigger<OnButtonClicked>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle = asset_server.load("audio/sound_effects/click.ogg");
    commands.spawn((Name::new("Button click sound"), sound_effect(handle, 0.3)));
}
