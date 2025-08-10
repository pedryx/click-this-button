use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use itertools::Itertools;
use strum_macros::EnumString;
use thiserror::Error;

use crate::{PausableSystems, game::guide::GuideText, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameMechanic>()
        .init_asset::<ActionSequence>()
        .init_asset_loader::<ActionSequenceAssetLoader>()
        .add_systems(OnEnter(Screen::Gameplay), load_action_sequence)
        .add_systems(Update, update_game_sequence.in_set(PausableSystems));
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default, Copy, EnumString)]
#[strum(serialize_all = "title_case")]
#[strum(ascii_case_insensitive)]
pub enum GameMechanic {
    #[default]
    None,
    Button,
    ButtonTime,
    Timer,
    Durability,
    Fix,
    Triangles,
    Square,
    Victory,
}

enum ActionType {
    ChangeText(String),
    SpawnMechanic(GameMechanic),
}

struct Action {
    /// in seconds
    time: f32,
    action_type: ActionType,
}

#[derive(Asset, TypePath)]
struct ActionSequence(Vec<Action>);

#[derive(Resource, Default)]
struct SequencerState {
    sequence: Handle<ActionSequence>,
    elapsed_time: f32,
    action_index: usize,
}

#[derive(Default)]
struct ActionSequenceAssetLoader;

#[derive(Error, Debug)]
pub enum ActionSequenceLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for ActionSequenceAssetLoader {
    type Asset = ActionSequence;
    type Settings = ();
    type Error = ActionSequenceLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut text = String::new();
        reader.read_to_string(&mut text).await?;

        let actions = text
            .lines()
            .map(|l| l.trim())
            .filter(|l| !(l.is_empty() || l.starts_with("#")))
            .map(|l| l.split('|').map(|t| t.trim()).collect_tuple().unwrap())
            .map(|(time, action_type, content)| Action {
                time: time.parse().unwrap(),
                action_type: match action_type {
                    "T" => ActionType::ChangeText(content.into()),
                    "M" => ActionType::SpawnMechanic(content.parse::<GameMechanic>().unwrap()),
                    _ => panic!("Invalid action type."),
                },
            })
            .collect_vec();

        Ok(ActionSequence(actions))
    }

    fn extensions(&self) -> &[&str] {
        &["seq"]
    }
}

fn load_action_sequence(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SequencerState {
        sequence: asset_server.load("sequence.seq"),
        ..default()
    });
}

fn update_game_sequence(
    mut guide_text: Single<&mut Text, With<GuideText>>,
    mut state: ResMut<SequencerState>,
    mut spawn_mechanic: ResMut<NextState<GameMechanic>>,
    action_sequences: Res<Assets<ActionSequence>>,
    time: Res<Time>,
) {
    // get sequence
    let Some(action_sequence) = action_sequences.get(&state.sequence) else {
        return;
    };

    // get action
    if state.action_index >= action_sequence.0.len() {
        return;
    }
    let action = &action_sequence.0[state.action_index];

    // update time
    state.elapsed_time += time.delta_secs();
    if state.elapsed_time < action.time {
        return;
    }
    state.elapsed_time -= action.time;
    state.action_index += 1;

    // invoke action
    match &action.action_type {
        ActionType::ChangeText(text) => guide_text.0 = text.into(),
        ActionType::SpawnMechanic(mechanic) => spawn_mechanic.set(*mechanic),
    }
}
