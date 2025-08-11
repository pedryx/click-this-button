use bevy::prelude::*;

use crate::{
    PausableSystems,
    audio::sound_effect,
    game::{
        GameOver,
        bar::{Bar, BarBehavior, BarLayout, OnBarEmpty},
        game_sequencer::GameMechanic,
        juice::{circles::SpawnCircles, pulse_effect::PulseEffect},
        player::{CLICK_PARTICLES_Z, Player},
    },
    screens::Screen,
};

pub const THE_BUTTON_Z: f32 = 50.0;
pub const THE_BUTTON_SIZE: f32 = 96.0;
const THE_BUTTON_COLOR: Color = Color::linear_rgb(0.0, 1.0, 0.0);

const TEXT_SIZE: f32 = 32.0;
const TEXT_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.0);

const TIME_BAR_DURATION: f32 = 8.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameMechanic::Button), spawn_button)
        .add_systems(OnEnter(GameMechanic::ButtonTime), spawn_button_time_bar)
        .add_systems(Update, update_button_time.in_set(PausableSystems))
        .add_observer(on_button_time_up);
}

#[derive(Event)]
pub struct OnButtonClicked;

#[derive(Component)]
pub struct TheButton;

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
            Mesh2d(meshes.add(Circle::new(THE_BUTTON_SIZE))),
            MeshMaterial2d(materials.add(THE_BUTTON_COLOR)),
            Transform::from_xyz(0.0, 0.0, THE_BUTTON_Z),
            TheButton,
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
        })
        .observe(handle_button_click)
        .observe(fill_time_bar_on_button_click);
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
            color: THE_BUTTON_COLOR,
            ..default()
        },
        BarBehavior {
            trigger_on_empty: true,
            ..default()
        },
        Transform::from_xyz(0.0, -THE_BUTTON_SIZE * 1.5, THE_BUTTON_Z),
        ButtonTimeBar,
        StateScoped(Screen::Gameplay),
        Pickable {
            should_block_lower: false,
            ..default()
        },
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
    commands.trigger(GameOver(GameMechanic::ButtonTime));
}

fn handle_button_click(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut player: Single<&mut Player>,
    button_transform: Single<&Transform, With<TheButton>>,
    asset_server: Res<AssetServer>,
) {
    player.clicked_on_target = true;
    commands.trigger(OnButtonClicked);

    // play click sound
    let handle = asset_server.load("audio/sound_effects/button_click.ogg");
    commands.spawn((Name::new("Button click sound"), sound_effect(handle, 0.4)));

    // play circles effect
    commands.trigger(SpawnCircles {
        location: button_transform.translation.xy().extend(CLICK_PARTICLES_Z),
        start_size: THE_BUTTON_SIZE * 1.1,
        end_size: THE_BUTTON_SIZE * 1.4,
        start_color: THE_BUTTON_COLOR.to_linear(),
        thickness: 4.0,
        spacing: 8.0,
        ..default()
    });
}

fn fill_time_bar_on_button_click(
    _: Trigger<Pointer<Click>>,
    mut bar: Single<&mut Bar, With<ButtonTimeBar>>,
) {
    // update time bar
    bar.current = bar.max;
}
