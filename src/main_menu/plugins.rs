use bevy::{
    app::{Plugin, Update},
    prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit},
};

use crate::app_state::AppState;

use super::systems::{
    interaction::{interact_with_connect_button, interact_with_exit_button},
    layout::{despawn_main_menu, spawn_main_menu},
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                (interact_with_connect_button, interact_with_exit_button).run_if(in_state(AppState::MainMenu)),
            )
           .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
