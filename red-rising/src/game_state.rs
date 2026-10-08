use bevy::prelude::States;

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    CharacterSelect,
    Drilling,
    CallEvent,
    Decision,
    LaurelSnub,
    Vale,
    ValeReaction,
    Caught,
}
