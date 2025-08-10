use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::{music, sound_effect},
    game::game_sequencer::GameMechanic,
    screens::{Screen, game_over::GameOverData},
};

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
        .add_systems(OnEnter(GameMechanic::Victory), on_victory)
        .add_observer(on_game_over);
}

#[derive(Event)]
pub struct GameOver(GameMechanic);

#[derive(Resource, Asset, Clone, Reflect)]
struct Soundtrack(Handle<AudioSource>);

impl FromWorld for Soundtrack {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self(assets.load("audio\\music\\soundtrack.ogg"))
    }
}

fn on_game_over(
    trigger: Trigger<GameOver>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_over_data: ResMut<GameOverData>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if trigger.event().0 != GameMechanic::Victory {
        let handle = asset_server.load("audio/sound_effects/lose.ogg");
        commands.spawn((Name::new("Lose sound"), sound_effect(handle, 0.4)));
    }

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

fn on_victory(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("audio/sound_effects/victory.ogg");
    commands.spawn((Name::new("Victory sound"), sound_effect(handle, 0.3)));
    commands.trigger(GameOver(GameMechanic::Victory));
}
