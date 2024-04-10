use std::f32::consts::FRAC_PI_2;

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
        directional_light: DirectionalLight {
            color: Color::ORANGE_RED,
            illuminance: 100_000.,
            shadows_enabled: true,
            ..default()
        },
        transform,
        ..default()
    });
}

#[derive(Component)]
pub struct Character;

pub fn scene1_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    camera(&mut commands, Transform::from_xyz(10., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y), ClearColorConfig::Custom(Color::BLACK));
    sun(&mut commands, Transform::from_xyz(100., 100., 100.));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(3.).mesh().uv(64, 36)),
        material: materials.add(StandardMaterial {
            base_color: Color::YELLOW,
            ..default()
        }),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}

pub fn scene1_update(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>, mut next_state: ResMut<NextState<SceneState>>) {
    if time.elapsed_seconds() < 15. {
        let mut transform = query.single_mut();
        transform.translation -= Vec3::splat(0.5) * time.delta_seconds();
    } else {
        next_state.set(SceneState::Scene2);
    }
}

pub fn scene2_setup(mut commands: Commands, assets: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    camera(&mut commands, Transform::from_xyz(5., 3., 5.).looking_at(Vec3::ZERO, Vec3::Y), ClearColorConfig::Custom(Color::rgb(0.67843137254, 0.84705882352, 0.90196078431)));
    sun(&mut commands, Transform::from_xyz(200., 100., 100.).looking_at(Vec3::ZERO, Vec3::Y));

    commands.spawn(SceneBundle {
        scene: assets.load("desert1.glb#Scene0"),
        ..Default::default()
    });

    commands.spawn((PbrBundle {
        mesh: meshes.add(Capsule3d::new(0.15625, 0.25)),
        material: materials.add(StandardMaterial {
            base_color: Color::GRAY,
            ..default()
        }),
        transform: Transform::from_xyz(-5., 0., 5.),
        ..default()
    }, Character));
}

pub fn scene2_update(time: Res<Time>, mut query: Query<&mut Transform, With<Character>>, mut next_state: ResMut<NextState<SceneState>>) {
    if time.elapsed_seconds() < 28. {
        let mut transform = query.single_mut();
        transform.translation += Vec3::from_array([1., 0., -1.]) * time.delta_seconds();
    } else {
        next_state.set(SceneState::Scene3);
    }
}

pub fn scene3_setup(mut commands: Commands, assets: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    camera(&mut commands, Transform::from_xyz(5., 3., 5.).looking_at(Vec3::ZERO, Vec3::Y), ClearColorConfig::Custom(Color::rgb(0.67843137254, 0.84705882352, 0.90196078431)));
    sun(&mut commands, Transform::from_xyz(200., 100., 100.).looking_at(Vec3::ZERO, Vec3::Y));

    commands.spawn(SceneBundle {
        scene: assets.load("desert2.glb#Scene0"),
        ..Default::default()
    });

    commands.spawn((PbrBundle {
        mesh: meshes.add(Cuboid::new(2., 2., 2.)),
        material: materials.add(StandardMaterial {
            base_color: Color::DARK_GRAY,
            metallic: 1.,
            ..default()
        }),
        transform: Transform::from_xyz(0., 1., 0.),
        ..default()
    }, Character));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cylinder::new(0.25, 1.)),
        material: materials.add(StandardMaterial {
            base_color: Color::DARK_GRAY,
            metallic: 1.,
            ..default()
        }),
        transform: Transform::from_xyz(3., 1., 0.),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cylinder::new(0.25, 1.)),
        material: materials.add(StandardMaterial {
            base_color: Color::DARK_GRAY,
            metallic: 1.,
            ..default()
        }),
        transform: Transform::from_xyz(0., 1., 3.),
        ..default()
    });
}

pub fn scene3_update(time: Res<Time>, mut query: Query<&mut Transform, With<Character>>, mut next_state: ResMut<NextState<SceneState>>) {
    if time.elapsed_seconds() < 39. {
        let mut transform = query.single_mut();
        let x = time.elapsed_seconds() - 39.;

        transform.translation += Vec3::from_array([0., x.sin() / 2., 0.]) * time.delta_seconds();
    } else {
        next_state.set(SceneState::Scene4);
    }
}

pub fn scene4_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    camera(&mut commands, Transform::from_xyz(5., 3., 5.).looking_at(Vec3::ZERO, Vec3::Y), ClearColorConfig::Custom(Color::ORANGE_RED));
    sun(&mut commands, Transform::from_xyz(200., 100., 100.).looking_at(Vec3::ZERO, Vec3::Y));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(5.).mesh().resolution(32)),
        material: materials.add(StandardMaterial {
            base_color: Color::YELLOW,
            ..default()
        }),
        transform: Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(1.).mesh().uv(32, 18)),
        material: materials.add(StandardMaterial {
            base_color: Color::DARK_GRAY,
            ..default()
        }),
        transform: Transform::from_xyz(0., 1., 0.),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(0.5).mesh().uv(32, 18)),
        material: materials.add(StandardMaterial {
            base_color: Color::DARK_GRAY,
            ..default()
        }),
        transform: Transform::from_xyz(0., 2.25, 0.),
        ..default()
    });
}

pub fn scene4_update(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>, mut next_state: ResMut<NextState<SceneState>>) {
    if time.elapsed_seconds() < 52. {
        let mut transform = query.single_mut();
        transform.translation += Vec3::splat(0.5) * time.delta_seconds();
    } else {
        next_state.set(SceneState::Scene5);
    }
}

pub fn scene5_setup(mut commands: Commands, assets: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    camera(&mut commands, Transform::from_xyz(5., 3., 5.).looking_at(Vec3::ZERO, Vec3::Y), ClearColorConfig::Custom(Color::rgb(0.67843137254, 0.84705882352, 0.90196078431)));
    sun(&mut commands, Transform::from_xyz(200., 100., 100.).looking_at(Vec3::ZERO, Vec3::Y));

    commands.spawn(SceneBundle {
        scene: assets.load("desert3.glb#Scene0"),
        ..Default::default()
    });

    commands.spawn((PbrBundle {
        mesh: meshes.add(Capsule3d::new(0.15625, 0.25)),
        material: materials.add(StandardMaterial {
            base_color: Color::GRAY,
            ..default()
        }),
        transform: Transform::from_xyz(5., 0., -5.),
        ..default()
    }, Character));
}

pub fn scene5_update(time: Res<Time>, mut query: Query<&mut Transform, With<Character>>) {
    if time.elapsed_seconds() < 63. {
        let mut transform = query.single_mut();
        transform.translation -= Vec3::from_array([1., 0., -1.]) * time.delta_seconds();
    }
}