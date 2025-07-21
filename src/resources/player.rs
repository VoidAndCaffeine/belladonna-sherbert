use bevy::prelude::*;
use leafwing_input_manager::prelude::InputMap;
use crate::plugins::input::PlayerAction;
use crate::prelude::input;

#[derive(Component,Reflect)]
#[reflect(Component)]
pub struct Player;

pub fn player_plugin(app: &mut App) {
    app.register_type::<Player>();
}