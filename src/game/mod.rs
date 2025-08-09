use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, audio::music, screens::Screen};

mod bar;
mod game_sequencer;
mod guide;
mod mechanics;
mod player;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<Soundtrack>()
        .add_plugins((
            player::plugin,
            guide::plugin,
            game_sequencer::plugin,
            mechanics::plugin,
            bar::plugin,
        ))
        .add_systems(OnEnter(Screen::Gameplay), start_soundtrack)
        .add_observer(on_game_over);
}

#[derive(Event)]
pub struct OnGameOver;

#[derive(Resource, Asset, Clone, Reflect)]
pub struct Soundtrack(Handle<AudioSource>);

impl FromWorld for Soundtrack {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self(assets.load("audio\\music\\soundtrack.ogg"))
    }
}

fn on_game_over(_: Trigger<OnGameOver>, mut app_exit_ew: EventWriter<AppExit>) {
    app_exit_ew.write(AppExit::Success);
}

fn start_soundtrack(mut commands: Commands, soundtrack: Res<Soundtrack>) {
    commands.spawn((
        Name::new("Soundtrack"),
        StateScoped(Screen::Gameplay),
        music(soundtrack.0.clone(), 0.6),
    ));
}
