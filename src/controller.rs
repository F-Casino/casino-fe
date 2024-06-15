use bevy::{input::ButtonInput, prelude::{Commands, KeyCode, Res, State}};

use crate::app_state::{self, AppState};

pub fn to_menu (
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>
) {
   if keyboard_input.pressed(KeyCode::KeyH) {
        println!("STATE: {:?}", app_state.get());
   };
}