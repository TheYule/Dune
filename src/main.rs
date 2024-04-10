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
        .add_systems(OnEnter(SceneState::Scene3), (cleanup, scene3_setup).chain())
        .add_systems(Update, scene3_update.run_if(in_state::<SceneState>(SceneState::Scene3)))
        .add_systems(OnEnter(SceneState::Scene4), (cleanup, scene4_setup).chain())
        .add_systems(Update, scene4_update.run_if(in_state::<SceneState>(SceneState::Scene4)))
        .add_systems(OnEnter(SceneState::Scene5), (cleanup, scene5_setup).chain())
        .add_systems(Update, scene5_update.run_if(in_state::<SceneState>(SceneState::Scene5)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(
        AmbientLight {
            color: Color::YELLOW,
            brightness: 0.2
        }
    );
}

fn cleanup(mut commands: Commands, entities: Query<Entity, Without<Window>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}