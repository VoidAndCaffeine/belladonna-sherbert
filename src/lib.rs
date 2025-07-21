#![allow(unused_imports)]

use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod utils;

/// Use this module instead of importing the `components`, `plugins`, `resources`, and `utils`
/// modules directly.
mod prelude {
    pub use super::*;
    pub use {components::*, plugins::*, resources::*, utils::*};
    pub use entity_despawner;
}


pub fn entity_despawner<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}
pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            plugins::default::plugin,
            resources::fonts::plugin,
            plugins::game::plugin,
            plugins::input::input_plugin,
            resources::player::player_plugin,
            plugins::main_menu::main_menu_plugin,
            plugins::world::world_plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins((plugins::dev_tools::plugin, plugins::debug::plugin));
    }
}
