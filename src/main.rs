use app_state::AppState;
use bevy::{app::App, DefaultPlugins};

mod app_state;
mod camera;
mod controller;
mod in_game;
mod lobby;
mod main_menu;
mod setting;

use camera::plugins::CameraPlugin;
use in_game::plugins::InGamePlugin;
use main_menu::plugins::MainMenuPlugin;

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_plugins((DefaultPlugins, CameraPlugin, MainMenuPlugin, InGamePlugin))
        .run();
}
