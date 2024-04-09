use bevy::prelude::*;
use scenes::*;
use states::SceneState;

mod scenes;
mod states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<SceneState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(SceneState::Scene1), scene1_setup)
        .add_systems(Update, scene1_update.run_if(in_state::<SceneState>(SceneState::Scene1)))
        .add_systems(OnEnter(SceneState::Scene2), (cleanup, scene2_setup).chain())
        .add_systems(Update, scene2_update.run_if(in_state::<SceneState>(SceneState::Scene2)))
        // .add_systems(OnEnter(SceneState::Scene2), (cleanup, scene2_setup).chain())
        // .add_systems(Update, scene1_update.run_if(in_state::<SceneState>(SceneState::Scene1)))
        // .add_systems(OnEnter(SceneState::Scene1), (cleanup, scene1_setup).chain())
        // .add_systems(Update, scene1_update.run_if(in_state::<SceneState>(SceneState::Scene1)))
        // .add_systems(OnEnter(SceneState::Scene1), (cleanup, scene1_setup).chain())
        // .add_systems(Update, scene1_update.run_if(in_state::<SceneState>(SceneState::Scene1)))
        // .add_systems(OnEnter(SceneState::Scene1), (cleanup, scene1_setup).chain())
        // .add_systems(Update, scene1_update.run_if(in_state::<SceneState>(SceneState::Scene1)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(
        AmbientLight {
            color: Color::ORANGE_RED,
            brightness: 0.02,
        }
    );
}

fn cleanup(mut commands: Commands, entities: Query<Entity, Without<Window>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}