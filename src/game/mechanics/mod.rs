use bevy::prelude::*;

mod durability;
mod pentagon;
mod square;
mod the_button;
mod timer;
mod triangles;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        the_button::plugin,
        timer::plugin,
        durability::plugin,
        triangles::plugin,
        square::plugin,
        pentagon::plugin,
    ));
}
