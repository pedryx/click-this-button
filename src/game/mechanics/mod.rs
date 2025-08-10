use bevy::prelude::*;

mod button;
mod durability;
mod timer;
mod triangles;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        button::plugin,
        timer::plugin,
        durability::plugin,
        triangles::plugin,
    ));
}
