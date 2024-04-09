use bevy::prelude::*;
use bevy_generative::{noise::{Gradient, Noise, Region}, terrain::{Terrain, TerrainBundle}};
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
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform,
        ..default()
    });
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

pub fn scene1_update(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>, mut next_state: ResMut<NextState<SceneState>>) {
    if time.elapsed_seconds() < 10. {
        let mut transform = query.single_mut();
        transform.translation -= Vec3::splat(0.75) * time.delta_seconds();
    } else {
        next_state.set(SceneState::Scene2);
    }
}

pub fn scene2_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    camera(&mut commands, Transform::from_xyz(5., 3., 5.).looking_at(Vec3::ZERO, Vec3::Y), ClearColorConfig::Custom(Color::rgb(173., 216., 230.)));
    sun(&mut commands, Transform::from_xyz(100., 100., 0.).looking_at(Vec3::ZERO, Vec3::Y));

    let mut noise = Noise::default();
    noise.regions = vec![Region {
        label: String::from("Desert"),
        position: 1000.,
        color: [255, 255, 0, 255],
    }];

    commands.spawn(TerrainBundle {
        terrain: Terrain {
            noise,
            size: [15, 15],
            resolution: 10,
            // height_exponent: 0.75,
            sea_percent: 0.,
            ..default()
        },
        ..default()
    });
}

pub fn scene2_update(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>, mut next_state: ResMut<NextState<SceneState>>) {
    if time.elapsed_seconds() < 10. {
        // let mut transform = query.single_mut();
        // transform.translation -= Vec3::splat(0.75) * time.delta_seconds();
    } else {
        // next_state.set(SceneState::Scene3);
    }
}