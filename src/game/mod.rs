use bevy::prelude::*;


mod player;
mod timer;
mod guide;
mod game_sequencer;
mod button;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        player::plugin,
        timer::plugin,
        guide::plugin,
        game_sequencer::plugin,
        button::plugin,
    ));
}


