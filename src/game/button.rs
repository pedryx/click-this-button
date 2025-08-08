use bevy::prelude::*;

use crate::screens::Screen;

const BUTTON_SIZE: f32 = 64.0;
const BUTTON_START_COLOR: Color = Color::linear_rgb(0.0, 1.0, 0.0);
//const BUTTON_END_COLOR: Color = Color::linear_rgb(1.0, 0.0, 0.0);
const BUTTON_Z: f32 = 0.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_button);
}

#[derive(Component)]
struct Button;

fn spawn_button(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BUTTON_SIZE))),
        MeshMaterial2d(materials.add(BUTTON_START_COLOR)),
        Transform::from_xyz(0.0, 0.0, BUTTON_Z),
        Button,
    ));
}
