use bevy::prelude::*;

use crate::PausableSystems;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, apply_pulse.in_set(PausableSystems));
}

#[derive(Component)]
#[require(PulseEffectState, Transform)]
pub struct PulseEffect {
    pub min: f32,
    pub max: f32,
    pub speed: f32,
}

impl Default for PulseEffect {
    fn default() -> Self {
        Self {
            min: 0.95,
            max: 1.05,
            speed: 0.2,
        }
    }
}

#[derive(Component)]
struct PulseEffectState {
    direction: f32,
}

impl Default for PulseEffectState {
    fn default() -> Self {
        Self { direction: 1.0 }
    }
}

fn apply_pulse(
    mut query: Query<(&mut Transform, &mut PulseEffectState, &PulseEffect)>,
    time: Res<Time>,
) {
    for (mut transform, mut state, effect) in query.iter_mut() {
        // only splat scale is supported
        let scale = transform.scale.x + state.direction * effect.speed * time.delta_secs();

        if scale >= effect.max || scale <= effect.min {
            state.direction *= -1.0;
        }
        let scale = scale.clamp(effect.min, effect.max);

        transform.scale = Vec2::splat(scale).extend(transform.scale.z);
    }
}
