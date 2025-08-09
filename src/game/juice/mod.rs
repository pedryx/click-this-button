use bevy::prelude::*;

pub mod pulse_effect;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(pulse_effect::plugin);
}
