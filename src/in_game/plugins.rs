use bevy::{
    app::{Plugin, Update},
    prelude::{in_state, IntoSystemConfigs, OnEnter},
};

use crate::app_state::AppState;

use super::systems::layout::spawn_in_game;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(OnEnter(AppState::InGame), spawn_in_game);
    }
}
