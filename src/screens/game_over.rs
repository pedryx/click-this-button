use bevy::prelude::*;

use crate::{game::game_sequencer::GameMechanic, screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GameOverData>()
        .add_systems(OnEnter(Screen::GameOver), spawn_entities);
}

#[derive(Resource, Default)]
pub struct GameOverData {
    pub reason: GameMechanic,
}

fn spawn_entities(mut commands: Commands, game_over_data: Res<GameOverData>) {
    let title = match game_over_data.reason {
        GameMechanic::Victory => "VICTORY",
        _ => "GAME OVER",
    };

    let reason = match game_over_data.reason {
        GameMechanic::ButtonTime => "THE BUTTON was not clicked during the last 6 seconds.",
        GameMechanic::Durability => "THE BUTTON durability reached zero.",
        GameMechanic::Triangles => "THE BUTTON was destroyed by triangle.",
        GameMechanic::Pentagon => "You were caught by pentagon.",
        GameMechanic::Victory => "CG. You managed to survive the chaos.",
        _ => panic!("Died to unsupported game mechanic."),
    };

    commands.spawn((
        widget::ui_root("Game over UI canvas"),
        StateScoped(Screen::GameOver),
        children![
            widget::header(title),
            widget::label(reason),
            widget::button("Retry", on_retry_click),
            widget::button("Exit", on_exit_click),
        ],
    ));
}

fn on_retry_click(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn on_exit_click(_: Trigger<Pointer<Click>>, mut app_exit_ew: EventWriter<AppExit>) {
    app_exit_ew.write(AppExit::Success);
}
