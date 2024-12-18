use bevy::prelude::Component;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

impl Player {
    pub fn new() -> Player {
        Player { speed: 250. }
    }
}
