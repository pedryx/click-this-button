use bevy::prelude::*;

mod player;
mod guide;
mod game_sequencer;
mod mechanics;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        player::plugin,
        guide::plugin,
        game_sequencer::plugin,
        mechanics::plugin,
    ));
}
