use bevy::prelude::*;

pub mod gen;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Generating,
    Ingame,
    Paused,
    MillUpgrade,
}
