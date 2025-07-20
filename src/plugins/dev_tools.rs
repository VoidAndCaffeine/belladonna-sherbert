//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};


const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

pub(crate) fn plugin(app: &mut App) {
    let toggle_system = toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY));

    // Toggle the debug overlay for UI.
    app
        .add_systems(Update, toggle_system);
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
