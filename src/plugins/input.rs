use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::prelude::player;
use crate::prelude::player::Player;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
#[actionlike(DualAxis)]
pub enum PlayerAction {
    // movement
    Move,
    Look,
    // ui
    #[actionlike(Button)]
    Interact,
    #[actionlike(Button)]
    Menu,
}

impl PlayerAction{
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);
        input_map.insert_dual_axis(Self::Look, GamepadStick::RIGHT);
        input_map.insert(Self::Interact, GamepadButton::South);
        input_map.insert(Self::Menu, GamepadButton::Start);

        input_map.insert_dual_axis(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::Interact, MouseButton::Left);
        input_map.insert(Self::Menu, KeyCode::Escape);

        input_map
    }
}
