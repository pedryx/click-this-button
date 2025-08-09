use bevy::prelude::*;

mod button;
mod timer;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((button::plugin, timer::plugin));
}
