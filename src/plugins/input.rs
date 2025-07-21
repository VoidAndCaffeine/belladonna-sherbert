use bevy::input::gamepad::GamepadEvent;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use leafwing_input_manager::prelude::*;
use crate::plugins::game::GameState;
use crate::plugins::world::world2_plugin;
use crate::prelude::player::Player;
pub(crate) fn input_plugin(app: &mut App) {
    app
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .add_plugins(InputModeManagerPlugin)
        .init_resource::<ActionState<PlayerAction>>()
        .insert_resource(PlayerAction::default_input_map())
    ;
}

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
    #[actionlike(Button)]
    Jump,
}

impl PlayerAction{
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);
        input_map.insert_dual_axis(Self::Look, GamepadStick::RIGHT);
        input_map.insert(Self::Interact, GamepadButton::South);
        input_map.insert(Self::Menu, GamepadButton::Start);
        input_map.insert(Self::Jump, GamepadButton::North);

        input_map.insert_dual_axis(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::Interact, MouseButton::Left);
        input_map.insert(Self::Menu, KeyCode::Escape);
        input_map.insert(Self::Jump, KeyCode::Space);

        input_map
    }
}

pub struct InputModeManagerPlugin;
impl Plugin for InputModeManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<ActiveInput>()
            .add_systems(
                Update,
                activate_gamepad.run_if(in_state(ActiveInput::MouseKeyboard)),
            )
            .add_systems(
                Update,
                activate_mkb.run_if(in_state(ActiveInput::Gamepad)),
            )
        ;
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum ActiveInput{
    #[default]
    MouseKeyboard,
    Gamepad,
}

/// switches input to gamepad
fn activate_gamepad(
    mut next_state: ResMut<NextState<ActiveInput>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
){
    for ev in gamepad_evr.read(){
        match ev {
            GamepadEvent::Button(_) | GamepadEvent::Axis(_) =>{
                info!("Switch to gamepad");
                next_state.set(ActiveInput::Gamepad);
                return;
            }
            _ => (),
        }
    }
}

fn activate_mkb(
    mut next_state: ResMut<NextState<ActiveInput>>,
    mut kb_evr: EventReader<KeyboardInput>,
){
    for _ev in kb_evr.read() {
        info!("Switch to keyboard");
        next_state.set(ActiveInput::MouseKeyboard);
    }
}

fn apply_ui_world_controls(action_state: ActionState<PlayerAction>) {
    if action_state.just_pressed(&PlayerAction::Menu){

    }
}