use bevy::prelude::*;
use super::states::SceneState;

fn camera(commands: &mut Commands, transform: Transform, clear_color: ClearColorConfig) {
    commands.spawn(Camera3dBundle {
        camera: Camera {
            clear_color,
            ..default()
        },
        transform,
        ..default()
    });
}

fn sun(commands: &mut Commands, transform: Transform) {
    commands.spawn(DirectionalLightBundle {
        transform,
        ..default()
    });
}

pub fn scene2_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    camera(&mut commands, Transform::from_xyz(10., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y), ClearColorConfig::Custom(Color::BLACK));
    sun(&mut commands, Transform::from_xyz(100., 100., 100.));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(3.).mesh().uv(32, 18)),
        material: materials.add(StandardMaterial {
            base_color: Color::YELLOW,
            ..default()
        }),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}

pub fn scene1_update(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>, mut next_state: ResMut<NextState<SceneState>>) {
    if time.elapsed_seconds() < 10. {
        let mut transform = query.single_mut();
        transform.translation -= Vec3::splat(0.75) * time.delta_seconds();
    } else {
        next_state.set(SceneState::Scene2);
    }
}

pub fn scene1_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    camera(&mut commands, Transform::from_xyz(10., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y), ClearColorConfig::Custom(Color::BLACK));
    sun(&mut commands, Transform::from_xyz(100., 100., 100.));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(3.).mesh().uv(32, 18)),
        material: materials.add(StandardMaterial {
            base_color: Color::YELLOW,
            ..default()
        }),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}

pub fn scene2_update(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>, mut next_state: ResMut<NextState<SceneState>>) {
    if time.elapsed_seconds() < 10. {
        let mut transform = query.single_mut();
        transform.translation -= Vec3::splat(0.75) * time.delta_seconds();
    } else {
        next_state.set(SceneState::Scene2);
    }
}