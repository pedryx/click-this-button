use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_add_bar)
        .add_systems(Update, (clamp_current_value, update_progress).chain());
}

#[derive(Event)]
pub struct OnBarEmpty {
    pub sender: Entity,
}

#[derive(Event)]
pub struct OnBarFull {
    pub _sender: Entity,
}

#[derive(Component, Default)]
#[require(BarLayout, BarBehavior, BarEntities, Transform)]
pub struct Bar {
    pub min: f32,
    pub max: f32,
    pub current: f32,
}

#[derive(Component)]
pub struct BarLayout {
    pub size: Vec2,
    pub color: Color,
    pub border_size: f32,
    pub border_color: Color,
}

impl Default for BarLayout {
    fn default() -> Self {
        Self {
            size: vec2(128.0, 16.0),
            color: Color::linear_rgb(1.0, 0.0, 0.0),
            border_size: 5.0,
            border_color: Color::linear_rgb(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Component, Default)]
pub struct BarBehavior {
    pub trigger_on_full: bool,
    pub trigger_on_empty: bool,
}

#[derive(Component)]
struct BarEntities {
    outer: Entity,
    inner: Entity,
}

impl Default for BarEntities {
    fn default() -> Self {
        Self {
            outer: Entity::PLACEHOLDER,
            inner: Entity::PLACEHOLDER,
        }
    }
}

fn on_add_bar(
    trigger: Trigger<OnAdd, Bar>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut BarEntities, &BarLayout), With<Bar>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (entity, mut bar_entities, bar_layout) = query.get_mut(trigger.target()).unwrap();

    commands.entity(entity).with_children(|parent| {
        // outer rectangle
        bar_entities.outer = parent
            .spawn((
                Mesh2d(meshes.add(Rectangle::new(bar_layout.size.x, bar_layout.size.y))),
                MeshMaterial2d(materials.add(bar_layout.border_color)),
            ))
            .with_children(|parent| {
                // inner rectangle
                bar_entities.inner = parent
                    .spawn((
                        Mesh2d(meshes.add(Rectangle::new(
                            bar_layout.size.x - bar_layout.border_size,
                            bar_layout.size.y - bar_layout.border_size,
                        ))),
                        MeshMaterial2d(materials.add(bar_layout.color)),
                    ))
                    .id();
            })
            .id();
    });
}

fn clamp_current_value(
    mut commands: Commands,
    mut query: Query<(&mut Bar, &BarBehavior, Entity), Changed<Bar>>,
) {
    for (mut bar, bar_behavior, entity) in query.iter_mut() {
        bar.current = bar.current.clamp(bar.min, bar.max);

        if bar_behavior.trigger_on_empty && bar.current == bar.min {
            commands.trigger(OnBarEmpty { sender: entity });
        }
        if bar_behavior.trigger_on_full && bar.current == bar.max {
            commands.trigger(OnBarFull { _sender: entity });
        }
    }
}

fn update_progress(
    bar_query: Query<(&Bar, &BarLayout, &BarEntities), Changed<Bar>>,
    mut transform_query: Query<&mut Transform>,
) {
    for (bar, _layout, entities) in bar_query.iter() {
        let progress = (bar.current - bar.min) / (bar.max - bar.min);
        let mut transform = transform_query.get_mut(entities.inner).unwrap();

        transform.scale.x = progress;
    }
}
