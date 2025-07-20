#![allow(unused)]

use bevy::{app::App, prelude::*};

use crate::prelude::{random_number, main_menu, resources::settings};

// This is an example of the most simple plugin you can write, without
// having to implement any traits.
//
// If you wanted to toggle this plugin or configure it for the outside
// you can reach for a `PluginGroup`.

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGameWorld1,
    InGameWorld2,
}

pub(crate) fn plugin(app: &mut App) {
    // Your game logic here
    app
        .init_state::<GameState>()
        .add_plugins(main_menu::main_menu_plugin)
    ;
}
