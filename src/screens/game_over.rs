use bevy::prelude::*;

use crate::{game::game_sequencer::SpawnMechanic, screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GameOverData>()
        .add_systems(OnEnter(Screen::GameOver), spawn_entities);
}

#[derive(Resource, Default)]
pub struct GameOverData {
    pub reason: SpawnMechanic,
}

fn spawn_entities(
    mut commands: Commands,
    game_over_data: Res<GameOverData>,
) {
    let reason = match game_over_data.reason {
        SpawnMechanic::ButtonTime => "THE BUTTON was not clicked during the last 6 seconds.",
        SpawnMechanic::Durability => "THE BUTTON durability reached zero.",
        SpawnMechanic::Triangles => "THE BUTTON was destroyed by triangle.",
        _ => panic!("Died to unsupported game mechanic."),
    };

    commands.spawn((
        widget::ui_root("Game over UI canvas"),
        StateScoped(Screen::GameOver),
        children![
            widget::header("Game Over"),
            widget::label(reason),
            widget::button("Retry", on_retry_click),
            widget::button("Exit", on_exit_click),
        ],
    ));
}

fn on_retry_click(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn on_exit_click(
    _: Trigger<Pointer<Click>>,
    mut app_exit_ew: EventWriter<AppExit>,
) {
    app_exit_ew.write(AppExit::Success);
}
