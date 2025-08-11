use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    audio::sound_effect,
    game::{
        GameOver,
        bar::{Bar, BarBehavior, BarLayout, OnBarEmpty},
        game_sequencer::GameMechanic,
        juice::{circles::SpawnCircles, pulse_effect::PulseEffect},
        mechanics::the_button::{OnButtonClicked, THE_BUTTON_Z},
        player::CLICK_PARTICLES_Z,
    },
    screens::Screen,
};

const MAX_DURABILITY: f32 = 6.0;
const CLICK_DAMAGE: f32 = 1.0;
const BAR_COLOR: Color = Color::linear_rgb(1.0, 1.0, 0.0);

const FIX_BUTTON_SIZE: f32 = 40.0;
const TEXT_SIZE: f32 = 32.0;
const TEXT_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.0);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameMechanic::Durability), spawn_durability_bar)
        .add_systems(OnEnter(GameMechanic::Fix), spawn_fix_button)
        .add_observer(update_durability)
        .add_observer(on_durability_zero);
}

#[derive(Component)]
struct DurabilityBar;

#[derive(Component)]
struct FixButton;

fn spawn_durability_bar(mut commands: Commands, window: Single<&Window, With<PrimaryWindow>>) {
    commands.spawn((
        Name::new("Durability bar"),
        Bar {
            max: MAX_DURABILITY,
            current: MAX_DURABILITY,
            ..default()
        },
        BarLayout {
            color: BAR_COLOR,
            size: vec2(448.0, 32.0),
            ..default()
        },
        BarBehavior {
            trigger_on_empty: true,
            ..default()
        },
        Transform::from_xyz(
            window.width() * -0.24,
            window.height() * -0.42,
            THE_BUTTON_Z,
        ),
        StateScoped(Screen::Gameplay),
        DurabilityBar,
        Pickable {
            should_block_lower: false,
            ..default()
        },
    ));
}

fn spawn_fix_button(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            Name::new("Fix button"),
            Mesh2d(meshes.add(Circle::new(FIX_BUTTON_SIZE))),
            MeshMaterial2d(materials.add(BAR_COLOR)),
            Transform::from_xyz(
                window.width() * -0.455,
                window.height() * -0.42,
                THE_BUTTON_Z,
            ),
            StateScoped(Screen::Gameplay),
            PulseEffect {
                min: 0.98,
                max: 1.02,
                speed: 0.1,
            },
            FixButton,
            Pickable::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new("FIX"),
                TextFont {
                    font: asset_server.load("fonts/Super Vanilla.ttf"),
                    font_size: TEXT_SIZE,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                TextColor(TEXT_COLOR),
                Pickable {
                    should_block_lower: false,
                    ..default()
                },
            ));
        })
        .observe(on_fix_button_click);
}

fn update_durability(_: Trigger<OnButtonClicked>, mut bar: Single<&mut Bar, With<DurabilityBar>>) {
    bar.current -= CLICK_DAMAGE;
}

fn on_durability_zero(
    trigger: Trigger<OnBarEmpty>,
    mut commands: Commands,
    durability_bar_entity: Single<Entity, With<DurabilityBar>>,
) {
    if trigger.event().sender != *durability_bar_entity {
        return;
    }
    commands.trigger(GameOver(GameMechanic::Durability));
}

fn on_fix_button_click(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut durability_bar: Single<&mut Bar, With<DurabilityBar>>,
    button_transform: Single<&Transform, With<FixButton>>,
    asset_server: Res<AssetServer>,
) {
    // fill durability
    durability_bar.current = durability_bar.max;

    // play sound effect
    let handle = asset_server.load("audio/sound_effects/button_click.ogg");
    commands.spawn((Name::new("Button click sound"), sound_effect(handle, 0.4)));

    // trigger circles effect
    commands.trigger(SpawnCircles {
        location: button_transform.translation.xy().extend(CLICK_PARTICLES_Z),
        start_size: FIX_BUTTON_SIZE * 1.1,
        end_size: FIX_BUTTON_SIZE * 1.4,
        start_color: BAR_COLOR.to_linear(),
        thickness: 2.0,
        spacing: 4.0,
        ..default()
    });
}
