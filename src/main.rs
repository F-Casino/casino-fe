use app_state::AppState;
use bevy::{app::App, DefaultPlugins};

mod app_state;
mod camera;
mod controller;
mod in_game;
mod main_menu;

use camera::plugins::CameraPlugin;
use main_menu::plugins::MainMenuPlugin;

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_plugins((DefaultPlugins, CameraPlugin, MainMenuPlugin))
        .run();
}
