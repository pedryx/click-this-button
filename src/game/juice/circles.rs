use bevy::prelude::*;

use crate::PausableSystems;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_circles.in_set(PausableSystems))
        .add_observer(spawn_circles);
}

#[derive(Event)]
pub struct SpawnCircles {
    pub location: Vec3,
    pub start_size: f32,
    pub end_size: f32,
    pub start_color: LinearRgba,
    pub end_color: LinearRgba,
    pub spacing: f32,
    pub ttl: f32,
    pub thickness: f32,
}

impl Default for SpawnCircles {
    fn default() -> Self {
        Self {
            location: Vec3::ZERO,
            start_size: 24.0,
            end_size: 32.0,
            start_color: LinearRgba::new(0.3, 0.3, 0.3, 1.0),
            end_color: LinearRgba::new(0.0, 0.0, 0.0, 0.0),
            spacing: 4.0,
            ttl: 0.5,
            thickness: 2.0,
        }
    }
}

#[derive(Component)]
struct CircleParticle {
    ttl_timer: Timer,
    start_size: f32,
    end_size: f32,
    start_color: LinearRgba,
    end_color: LinearRgba,
}

fn spawn_circles(
    trigger: Trigger<SpawnCircles>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let event = trigger.event();

    // outer circle
    commands.spawn((
        Mesh2d(meshes.add(Annulus::new(1.0 - event.thickness / event.start_size, 1.0))),
        MeshMaterial2d(materials.add(Color::LinearRgba(event.start_color))),
        Transform::from_translation(event.location)
            .with_scale(Vec2::splat(event.start_size).extend(1.0)),
        CircleParticle {
            ttl_timer: Timer::from_seconds(event.ttl, TimerMode::Once),
            start_size: event.start_size,
            end_size: event.end_size,
            start_color: event.start_color,
            end_color: event.end_color,
        },
    ));

    // outer circle
    let start_size = event.start_size - event.spacing;
    let end_size = event.end_size - event.spacing;
    commands.spawn((
        Mesh2d(meshes.add(Annulus::new(1.0 - event.thickness / start_size, 1.0))),
        MeshMaterial2d(materials.add(Color::LinearRgba(event.start_color))),
        Transform::from_translation(event.location).with_scale(Vec2::splat(start_size).extend(1.0)),
        CircleParticle {
            ttl_timer: Timer::from_seconds(event.ttl, TimerMode::Once),
            start_size,
            end_size,
            start_color: event.start_color,
            end_color: event.end_color,
        },
    ));
}

fn update_circles(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut MeshMaterial2d<ColorMaterial>,
        &mut CircleParticle,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut material, mut particle) in query.iter_mut() {
        particle.ttl_timer.tick(time.delta());
        let t = particle.ttl_timer.fraction();
        if particle.ttl_timer.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        // update scale
        let scale = particle.start_size.lerp(particle.end_size, t);
        transform.scale = Vec2::splat(scale).extend(transform.scale.z);

        // update color
        let color = particle
            .start_color
            .to_vec4()
            .lerp(particle.end_color.to_vec4(), t);
        material.0 = materials.add(Color::linear_rgba(color.x, color.y, color.z, color.w));
    }
}
