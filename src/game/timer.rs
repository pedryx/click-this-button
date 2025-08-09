use bevy::prelude::*;

use crate::{game::game_sequencer::SpawnMechanic, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(SpawnMechanic::Timer), spawn_timer)
        .add_systems(
            Update,
            (increment_time.run_if(resource_exists::<ElapsedTime>), update_timer_text).run_if(in_state(Screen::Gameplay)),
        );
}

#[derive(Resource, Default)]
pub struct ElapsedTime(f32);

#[derive(Component)]
struct TimerText;

fn spawn_timer(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Node {
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexStart,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("00:00"),
                TextFont {
                    font: asset_server.load("fonts/Super Vanilla.ttf"),
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(1.0, 0.0, 1.0)),
                TextLayout::new_with_justify(JustifyText::Center),
                Node {
                    position_type: PositionType::Relative,
                    top: Val::Vh(1.0),
                    ..default()
                },
                TimerText,
            ));
        });

    commands.init_resource::<ElapsedTime>();
}

fn increment_time(time: Res<Time>, mut elapsed_time: ResMut<ElapsedTime>) {
    elapsed_time.0 += time.delta_secs();
}

fn update_timer_text(
    mut text: Single<&mut Text, With<TimerText>>,
    elapsed_time: Res<ElapsedTime>,
) {
    let minutes = elapsed_time.0 / 60.0;
    let seconds = elapsed_time.0 % 60.0;

    text.0 = format!("{minutes:02.0}:{seconds:02.0}");
}
