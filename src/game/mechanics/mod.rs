use bevy::prelude::*;

mod button;
mod hexagons;
mod timer;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((button::plugin, timer::plugin, hexagons::plugin));
}
