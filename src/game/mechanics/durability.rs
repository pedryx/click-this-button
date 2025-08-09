use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    game::{
        OnGameOver,
        bar::{Bar, BarBehavior, BarLayout, OnBarEmpty},
        game_sequencer::SpawnMechanic,
        mechanics::button::{BUTTON_Z, OnButtonClicked},
    },
    screens::Screen,
};

const MAX_DURABILITY: f32 = 10.0;
const CLICK_DAMAGE: f32 = 2.0;
const BAR_COLOR: Color = Color::linear_rgb(1.0, 1.0, 0.0);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(SpawnMechanic::Durability), spawn_durability_bar)
        .add_observer(update_durability)
        .add_observer(on_durability_zero);
}

#[derive(Component)]
struct DurabilityBar;

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
        Transform::from_xyz(window.width() * -0.25, window.height() * -0.43, BUTTON_Z),
        StateScoped(Screen::Gameplay),
        DurabilityBar,
        Pickable {
            should_block_lower: false,
            ..default()
        },
    ));
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
    commands.trigger(OnGameOver);
}
