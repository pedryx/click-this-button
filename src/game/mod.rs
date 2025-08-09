use bevy::prelude::*;

mod player;
mod guide;
mod game_sequencer;
mod mechanics;
mod bar;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_game_over)
        .add_plugins((
            player::plugin,
            guide::plugin,
            game_sequencer::plugin,
            mechanics::plugin,
            bar::plugin,
        ));
}

#[derive(Event)]
pub struct OnGameOver;

fn on_game_over(
    _: Trigger<OnGameOver>,
    mut app_exit_ew: EventWriter<AppExit>
 ) {
    app_exit_ew.write(AppExit::Success);
}
