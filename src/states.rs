use bevy::ecs::schedule::States;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum SceneState {
    #[default]
    Scene1, // View of Arrakis from afar (10 sec)
    //Far in the future... where politics have evolved to be intergalactic... the universe is controlled by the Padishah Emperor Shaddam IV. He assigns the Duke Leto of House Atreides to rule Arrakis.
    //Arrakis. It's dunes contain the only source of the spice, melange, needed for space travel.
    Scene2,
    Scene3,
    Scene4,
    Scene5,
    Scene6
}