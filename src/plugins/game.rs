#![allow(unused)]

use bevy::{app::App, prelude::*};

use crate::prelude::{random_number, main_menu, loading, resources::settings};

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

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoadingState {
    MainMenu,
    MainMenuSilent,
    NewGame,
    LoadSave,
}

pub(crate) fn plugin(app: &mut App) {
    // Your game logic here
    app
        .init_state::<GameState>()
        .insert_state(LoadingState::MainMenu)
        .add_plugins((loading::loading_plugin, main_menu::main_menu_plugin))
    ;
}
