use bevy::prelude::*;

pub mod circles;
pub mod pulse_effect;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((pulse_effect::plugin, circles::plugin));
}
