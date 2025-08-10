use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, audio::music, game::game_sequencer::SpawnMechanic, screens::{game_over::GameOverData, Screen}};

mod bar;
pub mod game_sequencer;
mod guide;
mod juice;
mod mechanics;
mod player;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<Soundtrack>()
        .add_plugins((
            game_sequencer::plugin,
            guide::plugin,
            player::plugin,
            bar::plugin,
            mechanics::plugin,
            juice::plugin,
        ))
        .add_systems(OnEnter(Screen::Gameplay), start_soundtrack)
        .add_observer(on_game_over);
}

#[derive(Event)]
pub struct OnGameOver(SpawnMechanic);

#[derive(Resource, Asset, Clone, Reflect)]
struct Soundtrack(Handle<AudioSource>);

impl FromWorld for Soundtrack {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self(assets.load("audio\\music\\soundtrack.ogg"))
    }
}

fn on_game_over(
    trigger: Trigger<OnGameOver>,
    mut game_over_data: ResMut<GameOverData>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    game_over_data.reason = trigger.event().0;
    next_screen.set(Screen::GameOver);
}

fn start_soundtrack(mut commands: Commands, soundtrack: Res<Soundtrack>) {
    commands.spawn((
        Name::new("Soundtrack"),
        StateScoped(Screen::Gameplay),
        music(soundtrack.0.clone(), 0.8),
    ));
}
