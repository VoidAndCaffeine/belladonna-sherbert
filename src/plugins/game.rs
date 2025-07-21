#![allow(unused)]

use bevy::{app::App, prelude::*};
use crate::prelude::{random_number, main_menu, world, player, input};

// This is an example of the most simple plugin you can write, without
// having to implement any traits.
//
// If you wanted to toggle this plugin or configure it for the outside
// you can reach for a `PluginGroup`.

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    LoadWorld,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum MenuState {
    #[default]
    None,
    Ui,
    Pause,
    Inventory,
}

pub(crate) fn plugin(app: &mut App) {
    // Your game logic here
    app
        .init_state::<GameState>()
        .init_state::<MenuState>()

        .add_plugins(input::input_plugin)
        .add_plugins(player::player_plugin)
        .add_plugins(main_menu::main_menu_plugin)
        .add_plugins(world::world2_plugin)

        .add_systems(Update, world::load_finished)
    ;
}
