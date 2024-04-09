use bevy::ecs::schedule::States;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum SceneState {
    /*
        View of Arrakis from afar (15 sec)
        Far in the future... where politics have evolved to be intergalactic... the universe is controlled by the Padishah Emperor Shaddam IV. He assigns the Duke Leto of House Atreides to rule Arrakis.
    */
    #[default]
    Scene1,
    /*
        View of dunes with a character walking by (13 sec)
        Arrakis. It's dunes contain the only source of the spice, melange, needed for space travel. It's landscape is completely barren and harsh to all living things.
    */
    Scene2,
    /*
        View of dunes with large shiney cube (spice factory) (11 sec)
        Duke Leto begins to send out large machines to harvest and refine the spice. Worms are attracted to these machines, so they must work quickly and be transported back.
    */
    Scene3,
    /*
        View of Baron (13 sec)
        Unfortunately, Baron Vladimir Harkonnen, an enemy of House Atreides, invades. He destroys everything... except for the Duke's son, Paul, who escapes into the desert.
    */
    Scene4,
    /*
        (11 sec)
        Will Paul Atreides, the new Duke, be able to send his enemies to paradise? Or will he succumb to the ruthless desert? Read Dune, by Frank Herbert to find out.
    */
    Scene5
}