use bevy::prelude::*;

mod player;
mod guide;
mod timer;

mod button;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((player::plugin, button::plugin, timer::plugin, guide::plugin));
}
