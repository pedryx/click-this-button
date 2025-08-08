use bevy::prelude::*;

mod button;
mod player;
mod timer;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((player::plugin, button::plugin, timer::plugin));
}
