use bevy::prelude::States;

#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Setting,
}
