use bevy::input::gamepad::GamepadEvent;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use leafwing_input_manager::prelude::*;
use crate::plugins::game::GameState;
use crate::plugins::world2::world2_plugin;
use crate::prelude::player::Player;
pub(crate) fn input_plugin(app: &mut App) {
    app
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .add_plugins(InputModeManagerPlugin)
        .init_resource::<ActionState<PlayerAction>>()
        .insert_resource(PlayerAction::default_input_map())
        .add_systems(
            Update,
            player_mouse_look
                .run_if(in_state(GameState::InGame))
                .run_if(in_state(ActiveInput::MouseKeyboard))
        )
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

/// mouse settings
fn player_mouse_look(
    camera_query: Query<(&GlobalTransform, &Camera)>,
    player_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut action_state: ResMut<ActionState<PlayerAction>>,
){
    let (camera_transform, camera) = camera_query.single().expect("expected a single camera element");
    let player_transform = player_query.single().expect("expected a single player element");
    let window = window_query.single().expect("expected a single window element");

    let player_position = player_transform.translation;
    if let Some(p) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .and_then(|ray|{
            Some(ray).zip(ray.intersect_plane(player_position, InfinitePlane3d::new(Vec3::Y)))
        })
        .map(|(ray, p)| ray.get_point(p))
    {
        let diff = (p - player_position).xz();
        if diff.length_squared() > 1e-3f32 {
            let action_data = action_state.dual_axis_data_mut_or_default(&PlayerAction::Look);
            action_data.pair = Vec2::new(diff.x, -diff.y);
        }
    }
}