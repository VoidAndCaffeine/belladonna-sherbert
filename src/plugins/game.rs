#![allow(unused)]

use avian2d::{math::*, prelude::*};
use bevy::{app::App, prelude::*};

use crate::prelude::{random_number,menus,resources::settings};

// This is an example of the most simple plugin you can write, without
// having to implement any traits.
//
// If you wanted to toggle this plugin or configure it for the outside
// you can reach for a `PluginGroup`.

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    LoadingScreen,
    MainMenu,
    InGame,
}

pub(crate) fn plugin(app: &mut App) {
    // Your game logic here
    app
        .init_state::<GameState>()
        .add_plugins((menus::loading::loading_plugin, menus::main_menu::main_menu_plugin))
    ;
}
